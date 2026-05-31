//! Dynamic CRUD & Query Engine.
//!
//! One generic set of handlers parameterized by entity name — no per-entity
//! code. Each handler follows the request lifecycle in system-design §6:
//! resolve entity → check permission → validate → persist → emit event.

use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde_json::Value;

use crate::error::AppError;
use crate::state::AppState;

/// `GET /api/{entity}` — list with filter/sort/paginate/expand/fields.
pub async fn list(
    State(_state): State<AppState>,
    Path(_entity): Path<String>,
    Query(_params): Query<HashMap<String, String>>,
) -> Result<Json<Value>, AppError> {
    todo!("P3: resolve entity → check Read → query via PersistenceAdapter::list")
}

/// `POST /api/{entity}` — create.
pub async fn create(
    State(_state): State<AppState>,
    Path(_entity): Path<String>,
    Json(_payload): Json<Value>,
) -> Result<Json<Value>, AppError> {
    todo!("P3: resolve entity → check Create → validate → insert (tx) → audit + outbox event")
}

/// `GET /api/{entity}/{id}` — read one.
pub async fn read(
    State(_state): State<AppState>,
    Path((_entity, _id)): Path<(String, String)>,
) -> Result<Json<Value>, AppError> {
    todo!("P3: resolve entity → check Read → fetch by id → apply read mask")
}

/// `PATCH /api/{entity}/{id}` — update.
pub async fn update(
    State(_state): State<AppState>,
    Path((_entity, _id)): Path<(String, String)>,
    Json(_payload): Json<Value>,
) -> Result<Json<Value>, AppError> {
    todo!("P3: resolve entity → check Update → validate → update (tx) → audit + outbox event")
}

/// `DELETE /api/{entity}/{id}` — delete.
pub async fn delete(
    State(_state): State<AppState>,
    Path((_entity, _id)): Path<(String, String)>,
) -> Result<Json<Value>, AppError> {
    todo!("P3: resolve entity → check Delete → delete (tx) → audit + outbox event")
}
