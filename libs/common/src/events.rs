//! Event-driven integration primitives shared by services.
//! RabbitMQ's management HTTP API is used deliberately here: it keeps all
//! services on the existing `reqwest` stack while messages are still persisted
//! and routed by RabbitMQ rather than sent service-to-service synchronously.
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DomainEvent<T: Serialize> {
    pub event_type: &'static str,
    pub occurred_at: String,
    pub data: T,
}

#[derive(Clone)]
pub struct RabbitMqEventPublisher {
    client: reqwest::Client,
    api_url: String,
    username: String,
    password: String,
}

impl RabbitMqEventPublisher {
    pub fn from_env() -> Self {
        Self {
            client: reqwest::Client::new(),
            api_url: std::env::var("RABBITMQ_MANAGEMENT_URL")
                .unwrap_or_else(|_| "http://rabbitmq:15672/api".into()),
            username: std::env::var("RABBITMQ_DEFAULT_USER").unwrap_or_else(|_| "localbite".into()),
            password: std::env::var("RABBITMQ_DEFAULT_PASS").unwrap_or_else(|_| "localbite".into()),
        }
    }

    /// Fire-and-forget is intentional: a broker outage must not roll back a
    /// successful local transaction. Production deployments should add an
    /// outbox relay for guaranteed delivery.
    pub fn publish_async<T: Serialize + Send + 'static>(&self, event: DomainEvent<T>) {
        let publisher = self.clone();
        tokio::spawn(async move {
            if let Err(error) = publisher.publish(event).await {
                tracing::warn!(%error, "domain event was not published");
            }
        });
    }

    async fn publish<T: Serialize>(&self, event: DomainEvent<T>) -> anyhow::Result<()> {
        let exchange = format!(
            "{}/exchanges/%2F/local_bite.events",
            self.api_url.trim_end_matches('/')
        );
        self.client.put(&exchange).basic_auth(&self.username, Some(&self.password)).json(&serde_json::json!({"type":"topic","durable":true,"auto_delete":false,"internal":false,"arguments":{}})).send().await?.error_for_status()?;
        self.client.post(format!("{}/publish", exchange)).basic_auth(&self.username, Some(&self.password)).json(&serde_json::json!({"routing_key":event.event_type,"payload":serde_json::to_string(&event)?,"payload_encoding":"string","properties":{"delivery_mode":2}})).send().await?.error_for_status()?;
        Ok(())
    }
}
