//! HTTP router — wires the documented API surface (system-design §5) to handlers.

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde_json::Value;

use crate::error::AppError;
use crate::state::AppState;
use crate::{crud, discovery, realtime};

pub fn router(state: AppState) -> Router {
    Router::new()
        // Health / liveness
        .route("/health", get(health))
        // Discovery
        .route("/api/schema", get(discovery::schema))
        .route("/api/openapi.json", get(discovery::openapi))
        // Dynamic CRUD (generic, parameterized by {entity})
        .route("/api/:entity", get(crud::list).post(crud::create))
        .route(
            "/api/:entity/:id",
            get(crud::read).patch(crud::update).delete(crud::delete),
        )
        // Realtime
        .route("/api/realtime", get(realtime::ws_handler))
        // Admin — schema version lifecycle
        .route("/api/admin/schema/versions", post(submit_version))
        .route(
            "/api/admin/schema/versions/:id/activate",
            post(activate_version),
        )
        .with_state(state)
}

async fn health() -> Json<Value> {
    todo!("return { \"status\": \"ok\" }")
}

/// `POST /api/admin/schema/versions` — submit a candidate schema version.
async fn submit_version(
    State(_state): State<AppState>,
    Json(_candidate): Json<Value>,
) -> Result<Json<Value>, AppError> {
    todo!("P1: validate + diff, persist as a draft schema_version row")
}

/// `POST /api/admin/schema/versions/{id}/activate` — validate + migrate + activate.
async fn activate_version(
    State(_state): State<AppState>,
    Path(_id): Path<String>,
    Json(_candidate): Json<Value>,
) -> Result<Json<Value>, AppError> {
    todo!("P1/P2: load draft, run migrations, activate atomically, hot-reload engine")
}
