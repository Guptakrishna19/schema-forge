//! SchemaForge backend library.
//!
//! A metadata-driven application platform: behavior derives from versioned
//! schema definitions at runtime, not from per-entity code. The modules here
//! map 1:1 to the components in `docs/system-design.md`.

pub mod config;
pub mod crud;
pub mod discovery;
pub mod error;
pub mod events;
pub mod permissions;
pub mod persistence;
pub mod realtime;
pub mod routes;
pub mod schema;
pub mod state;
pub mod workflow;

pub use error::AppError;
pub use state::AppState;
