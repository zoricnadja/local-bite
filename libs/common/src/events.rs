//! Transactional outbox relay. Database triggers write integration events in the
//! same transaction as domain changes; AMQP confirms precede marking them sent.
use anyhow::{anyhow, Context};
use lapin::{options::*, types::{AMQPValue, FieldTable}, BasicProperties, Channel, Connection, ConnectionProperties, ExchangeKind, publisher_confirm::Confirmation};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use std::time::Duration;

pub const EXCHANGE: &str = "local_bite.events.v1";
pub const QUEUE: &str = "local_bite.read_models.v1";
pub const DEAD_QUEUE: &str = "local_bite.read_models.dead.v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationEvent {
    pub schema_version: u32,
    pub source: String,
    pub sequence: i64,
    pub entity_type: String,
    pub entity_id: uuid::Uuid,
    pub operation: String,
    pub data: Value,
}

pub async fn connect() -> anyhow::Result<Connection> {
    let address = std::env::var("RABBITMQ_URL").context("RABBITMQ_URL is required")?;
    Ok(tokio::time::timeout(Duration::from_secs(10), Connection::connect(&address, ConnectionProperties::default())).await??)
}

pub async fn topology(channel: &Channel) -> anyhow::Result<()> {
    channel.exchange_declare(EXCHANGE, ExchangeKind::Topic, ExchangeDeclareOptions { durable: true, ..Default::default() }, FieldTable::default()).await?;
    channel.queue_declare(DEAD_QUEUE, QueueDeclareOptions { durable: true, ..Default::default() }, FieldTable::default()).await?;
    let mut args = FieldTable::default();
    args.insert("x-dead-letter-exchange".into(), AMQPValue::LongString("".into()));
    args.insert("x-dead-letter-routing-key".into(), AMQPValue::LongString(DEAD_QUEUE.into()));
    channel.queue_declare(QUEUE, QueueDeclareOptions { durable: true, ..Default::default() }, args).await?;
    channel.queue_bind(QUEUE, EXCHANGE, "#", QueueBindOptions::default(), FieldTable::default()).await?;
    channel.queue_declare("local_bite.production_outputs.v1", QueueDeclareOptions { durable: true, ..Default::default() }, FieldTable::default()).await?;
    channel.queue_bind("local_bite.production_outputs.v1", EXCHANGE, "productions.production_batches.*", QueueBindOptions::default(), FieldTable::default()).await?;
    Ok(())
}

pub fn start_outbox(pool: PgPool, source: &'static str) {
    tokio::spawn(async move {
        loop {
            if let Err(_error) = relay(&pool, source).await {
                tracing::warn!(source, "Outbox relay disconnected; unsent events remain in the database");
            }
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    });
}

async fn relay(pool: &PgPool, source: &str) -> anyhow::Result<()> {
    let connection = connect().await?;
    let channel = connection.create_channel().await?;
    topology(&channel).await?;
    channel.confirm_select(ConfirmSelectOptions::default()).await?;
    loop {
        if !connection.status().connected() { return Err(anyhow!("Broker connection closed")); }
        let mut tx = pool.begin().await?;
        // One relay per source database, including when multiple instances run.
        let locked: bool = sqlx::query_scalar("SELECT pg_try_advisory_xact_lock(7142026)").fetch_one(&mut *tx).await?;
        if !locked { drop(tx); tokio::time::sleep(Duration::from_millis(500)).await; continue; }
        let rows: Vec<(i64, String, uuid::Uuid, String, Value)> = sqlx::query_as(
            "SELECT sequence, entity_type, entity_id, operation, data FROM integration_outbox WHERE published_at IS NULL ORDER BY sequence LIMIT 50 FOR UPDATE")
            .fetch_all(&mut *tx).await?;
        if rows.is_empty() { tx.commit().await?; tokio::time::sleep(Duration::from_millis(500)).await; continue; }
        for (sequence, entity_type, entity_id, operation, data) in rows {
            let routing_key = format!("{}.{}.{}", source, entity_type, operation.to_lowercase());
            let event = IntegrationEvent { schema_version: 1, source: source.into(), sequence, entity_type, entity_id, operation, data };
            let payload = serde_json::to_vec(&event)?;
            let confirmation = tokio::time::timeout(Duration::from_secs(10), async {
                channel.basic_publish(EXCHANGE, &routing_key, BasicPublishOptions { mandatory: true, ..Default::default() }, &payload,
                    BasicProperties::default().with_delivery_mode(2).with_content_type("application/json".into()).with_message_id(format!("{source}:{sequence}").into()))
                    .await?.await
            }).await??;
            if !matches!(confirmation, Confirmation::Ack(None)) { return Err(anyhow!("Broker did not confirm routing and persistence")); }
            sqlx::query("UPDATE integration_outbox SET published_at=now() WHERE sequence=$1").bind(sequence).execute(&mut *tx).await?;
        }
        tx.commit().await?;
    }
}
