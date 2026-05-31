//! Event backbone — transactional outbox relayed to Apache Kafka.
//!
//! CRUD commits write a domain event to the `outbox` table in the same
//! transaction as the data change. A relay then publishes those rows to Kafka,
//! where the Workflow Engine and Realtime Hub consume them independently.
//! See system-design §3.7.

use rdkafka::producer::FutureProducer;

use crate::config::Config;

/// Domain event published to Kafka after a successful CRUD commit.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DomainEvent {
    /// e.g. `entity.created`, `entity.updated`, `entity.deleted`.
    pub kind: String,
    pub entity: String,
    pub record_id: String,
    pub payload: serde_json::Value,
}

/// Thin wrapper over an rdkafka producer. Cloning is cheap (shares the client).
#[derive(Clone)]
pub struct EventPublisher {
    #[allow(dead_code)]
    producer: FutureProducer,
    #[allow(dead_code)]
    topic: String,
}

impl EventPublisher {
    pub fn new(_config: &Config) -> anyhow::Result<Self> {
        todo!("P0: build a FutureProducer from config.kafka_brokers")
    }

    /// Publish a domain event, keyed by `entity` for per-entity ordering.
    pub async fn publish(&self, _event: &DomainEvent) -> anyhow::Result<()> {
        todo!("P7: serialize and send to the domain-events topic")
    }
}

/// Relay loop: poll the `outbox` table and publish pending rows to Kafka,
/// marking them sent. Runs as a background task.
pub async fn run_outbox_relay(_publisher: EventPublisher) {
    todo!("P7: poll outbox (or wire Debezium CDC) and publish pending rows")
}
