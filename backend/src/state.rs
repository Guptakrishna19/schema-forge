//! Shared application state, cloned cheaply into every handler.

use std::sync::Arc;

use sqlx::PgPool;

use crate::config::Config;
use crate::events::EventPublisher;
use crate::schema::SchemaEngine;

/// Cheap-to-clone handle to all shared services (everything behind `Arc`).
#[derive(Clone)]
pub struct AppState {
    inner: Arc<Inner>,
}

struct Inner {
    db: PgPool,
    engine: SchemaEngine,
    events: EventPublisher,
}

impl AppState {
    /// Build shared state: lazy DB pool, Kafka producer, empty schema engine.
    pub async fn init(_config: &Config) -> anyhow::Result<Self> {
        todo!("P0: construct PgPool (lazy), EventPublisher, and SchemaEngine")
    }

    pub fn db(&self) -> &PgPool {
        &self.inner.db
    }

    pub fn engine(&self) -> &SchemaEngine {
        &self.inner.engine
    }

    pub fn events(&self) -> &EventPublisher {
        &self.inner.events
    }
}
