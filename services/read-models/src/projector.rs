use common::events::{self, IntegrationEvent, QUEUE};
use futures::StreamExt;
use lapin::{options::*, types::FieldTable};
use sqlx::PgPool;
use std::{sync::{Arc, atomic::{AtomicBool, Ordering}}, time::Duration};

pub fn valid(event: &IntegrationEvent) -> bool {
    let known = match event.source.as_str() {
        "auth" => matches!(event.entity_type.as_str(), "users" | "businesses"),
        "products" => event.entity_type == "products",
        "raw-materials" => event.entity_type == "raw_materials",
        "productions" => matches!(event.entity_type.as_str(), "production_batches" | "process_steps" | "batch_raw_materials"),
        "orders" => matches!(event.entity_type.as_str(), "orders" | "order_items"),
        _ => false,
    };
    known && event.schema_version == 1 && event.sequence > 0
        && matches!(event.operation.as_str(), "INSERT" | "UPDATE" | "DELETE")
        && event.data.get("id").and_then(|v| v.as_str()).and_then(|s| s.parse::<uuid::Uuid>().ok()) == Some(event.entity_id)
}

pub async fn apply(pool: &PgPool, event: &IntegrationEvent) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    let inserted = sqlx::query("INSERT INTO projection_receipts(source,sequence) VALUES($1,$2) ON CONFLICT DO NOTHING")
        .bind(&event.source).bind(event.sequence).execute(&mut *tx).await?.rows_affected();
    if inserted > 0 {
        sqlx::query("INSERT INTO projection_entities(source,entity_type,entity_id,sequence,deleted,data) VALUES($1,$2,$3,$4,$5,$6) ON CONFLICT(source,entity_type,entity_id) DO UPDATE SET sequence=EXCLUDED.sequence,deleted=EXCLUDED.deleted,data=EXCLUDED.data,projected_at=now() WHERE projection_entities.sequence < EXCLUDED.sequence")
            .bind(&event.source).bind(&event.entity_type).bind(event.entity_id).bind(event.sequence)
            .bind(event.operation == "DELETE" || event.data.get("is_deleted").and_then(|v| v.as_bool()).unwrap_or(false))
            .bind(&event.data).execute(&mut *tx).await?;
    }
    tx.commit().await?;
    Ok(())
}

pub fn start(pool: PgPool, connected: Arc<AtomicBool>) {
    tokio::spawn(async move {
        loop {
            if let Err(_error) = consume(&pool, &connected).await {
                tracing::warn!("Projection consumer reconnecting; unacknowledged deliveries will be retried");
            }
            connected.store(false, Ordering::Relaxed);
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    });
}

async fn consume(pool: &PgPool, connected: &AtomicBool) -> anyhow::Result<()> {
    let connection = events::connect().await?;
    let channel = connection.create_channel().await?;
    events::topology(&channel).await?;
    channel.basic_qos(50, BasicQosOptions::default()).await?;
    let mut consumer = channel.basic_consume(QUEUE, "read-models", BasicConsumeOptions::default(), FieldTable::default()).await?;
    connected.store(true, Ordering::Relaxed);
    while let Some(delivery) = consumer.next().await {
        let delivery = delivery?;
        let event = match serde_json::from_slice::<IntegrationEvent>(&delivery.data) {
            Ok(event) if valid(&event) => event,
            _ => {
                tracing::error!("Invalid integration event moved to dead-letter queue");
                delivery.reject(BasicRejectOptions { requeue: false }).await?;
                continue;
            }
        };
        if let Err(_error) = apply(pool, &event).await {
            // Requeue, then disconnect/back off instead of spinning on a failing database.
            delivery.nack(BasicNackOptions { multiple: false, requeue: true }).await?;
            channel.close(200, "Database unavailable").await?;
            return Err(_error);
        }
        delivery.ack(BasicAckOptions::default()).await?;
    }
    Ok(())
}
