//! Discovery / OpenAPI emitter.
//!
//! `/api/schema` returns the full machine-readable active schema; the OpenAPI 3
//! and JSON Schema emitters let agents, codegen, and API explorers self-configure.
//! See system-design §3.8.

use axum::{extract::State, Json};
use serde_json::Value;

use crate::error::AppError;
use crate::state::AppState;

/// `GET /api/schema` — full machine-readable active schema.
pub async fn schema(State(_state): State<AppState>) -> Result<Json<Value>, AppError> {
    todo!("P5: return the active schema snapshot (or an empty descriptor)")
}

/// `GET /api/openapi.json` — OpenAPI 3 spec derived from the active schema.
pub async fn openapi(State(_state): State<AppState>) -> Result<Json<Value>, AppError> {
    todo!("P5: emit OpenAPI 3 paths/components from entities + list grammar")
}
