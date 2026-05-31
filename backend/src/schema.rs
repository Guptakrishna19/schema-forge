//! Schema Registry & Engine.
//!
//! The Registry persists versioned schema definitions and validates/diffs/activates
//! them. The Engine is the in-memory resolver every other component reads — it never
//! exposes raw rows, only the active, validated [`Schema`]. See system-design §3.1–3.2.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

use crate::error::AppError;

// ---------------------------------------------------------------------------
// Meta-schema (the schema-of-schemas). Mirrors docs/design-plan.md §1.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub version: u32,
    #[serde(default)]
    pub entities: HashMap<String, EntityDef>,
    #[serde(default)]
    pub workflows: Vec<WorkflowDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDef {
    #[serde(default)]
    pub fields: HashMap<String, FieldDef>,
    #[serde(default)]
    pub relations: HashMap<String, RelationDef>,
    /// (action -> list of grants). Action is `read|create|update|delete` or `*`.
    #[serde(default)]
    pub permissions: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub indexes: Vec<IndexDef>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    String,
    Text,
    Integer,
    Decimal,
    Boolean,
    Datetime,
    Date,
    Enum,
    Json,
    Reference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDef {
    #[serde(rename = "type")]
    pub field_type: FieldType,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub unique: bool,
    #[serde(default)]
    pub default: Option<serde_json::Value>,
    #[serde(default)]
    pub min: Option<f64>,
    #[serde(default)]
    pub max: Option<f64>,
    #[serde(default, rename = "minLength")]
    pub min_length: Option<u32>,
    #[serde(default, rename = "maxLength")]
    pub max_length: Option<u32>,
    #[serde(default)]
    pub pattern: Option<String>,
    #[serde(default)]
    pub format: Option<String>,
    /// Allowed values when `field_type == Enum`.
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RelationKind {
    OneToOne,
    OneToMany,
    ManyToOne,
    ManyToMany,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationDef {
    pub kind: RelationKind,
    pub target: String,
    #[serde(default)]
    pub inverse: Option<String>,
    /// `restrict | cascade | set-null`
    #[serde(default)]
    pub on_delete: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexDef {
    pub fields: Vec<String>,
    #[serde(default)]
    pub unique: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDef {
    pub name: String,
    pub on: WorkflowTrigger,
    #[serde(default)]
    pub actions: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTrigger {
    pub entity: String,
    pub event: String,
    #[serde(default)]
    pub when: Option<String>,
}

// ---------------------------------------------------------------------------
// Schema Engine — in-memory resolver of the currently active schema.
// ---------------------------------------------------------------------------

#[derive(Clone, Default)]
pub struct SchemaEngine {
    active: Arc<RwLock<Option<Schema>>>,
}

impl SchemaEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Atomically swap in a newly activated schema version (hot reload).
    pub fn load(&self, _schema: Schema) {
        todo!("P1: store the active schema behind the RwLock")
    }

    /// Snapshot of the active schema, or `None` if nothing is activated yet.
    pub fn snapshot(&self) -> Option<Schema> {
        todo!("P1: clone the active schema out of the RwLock")
    }

    /// Resolve an entity definition, erroring with 404 if unknown.
    pub fn entity(&self, _name: &str) -> Result<EntityDef, AppError> {
        todo!("P1: look up the entity in the active schema or AppError::UnknownEntity")
    }
}

// ---------------------------------------------------------------------------
// Registry — validate / diff / activate. Backed by the `schema_version` table.
// ---------------------------------------------------------------------------

/// Classification of a candidate schema relative to the active one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeClass {
    /// Additive only — safe to activate immediately (online DDL).
    Additive,
    /// Breaking (drop field, narrow type, add required) — needs a migration plan.
    Breaking(Vec<String>),
}

/// Validate a candidate schema's internal integrity (types, relation targets,
/// permission references). Returns the parsed [`Schema`] on success.
pub fn validate(_candidate: serde_json::Value) -> Result<Schema, AppError> {
    todo!("P1: parse against the meta-schema + check relation targets / permission refs")
}

/// Diff a candidate against the active schema to classify the change.
pub fn diff(_active: Option<&Schema>, _candidate: &Schema) -> ChangeClass {
    todo!("P1: classify additive vs breaking per system-design §7")
}
