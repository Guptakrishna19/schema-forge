//! Runtime configuration, sourced from environment variables.

use std::fmt;

#[derive(Clone)]
pub struct Config {
    /// `host:port` the HTTP server binds to.
    pub bind_addr: String,
    /// PostgreSQL connection string.
    pub database_url: String,
    /// Comma-separated Kafka bootstrap servers (e.g. `localhost:9092`).
    pub kafka_brokers: String,
    /// Topic that domain events (`entity.created/updated/deleted`) are published to.
    pub kafka_domain_topic: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        todo!("P0: read SCHEMA_FORGE_BIND / DATABASE_URL / KAFKA_BROKERS / KAFKA_DOMAIN_TOPIC")
    }
}

impl fmt::Debug for Config {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Redact credentials embedded in the database URL when implementing.
        todo!("P0: debug-format config without leaking the database password")
    }
}
