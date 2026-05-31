//! Persistence Adapter.
//!
//! Maps entities to physical storage using the **hybrid** strategy (Decision D0):
//! known/queryable fields become real columns; the rest live in a `data JSONB`
//! column. Relations are FK columns (1-1, 1-n) or join tables (n-n). All other
//! layers go through this adapter and never see the storage strategy. See
//! system-design §3.4 and design-plan §2.

use sqlx::PgPool;

use crate::error::AppError;
use crate::schema::EntityDef;

pub struct PersistenceAdapter<'a> {
    #[allow(dead_code)]
    db: &'a PgPool,
}

impl<'a> PersistenceAdapter<'a> {
    pub fn new(db: &'a PgPool) -> Self {
        Self { db }
    }

    /// Create/alter physical storage for an entity (table, columns, indexes,
    /// FK/join tables) via online additive DDL.
    pub async fn ensure_storage(&self, _entity: &str, _def: &EntityDef) -> Result<(), AppError> {
        todo!("P2: generate and run online DDL from the entity definition")
    }

    /// Build a parameterized SELECT from a list query (filter/sort/paginate/
    /// expand/fields) and execute it.
    pub async fn list(
        &self,
        _entity: &str,
        _def: &EntityDef,
    ) -> Result<Vec<serde_json::Value>, AppError> {
        todo!("P3: query builder over hybrid columns + JSONB")
    }
}
