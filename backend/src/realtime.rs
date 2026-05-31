//! Realtime Hub.
//!
//! Consumes domain events from Kafka and fans them out to subscribed
//! WebSocket/SSE clients, applying the *same* permission masks as the API
//! before pushing. See system-design §3.7 and design-plan §3.4.

use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
    response::Response,
};

use crate::state::AppState;

/// `GET /api/realtime` — upgrade to a WebSocket and subscribe to entity changes.
pub async fn ws_handler(_ws: WebSocketUpgrade, State(_state): State<AppState>) -> Response {
    todo!("P7: upgrade and hand the socket to handle_socket")
}

#[allow(dead_code)]
async fn handle_socket(_socket: WebSocket, _state: AppState) {
    todo!("P7: parse subscriptions, register with the hub, stream permission-filtered events")
}
