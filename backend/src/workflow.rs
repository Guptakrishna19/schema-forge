//! Workflow Engine.
//!
//! Consumes domain-event Kafka topics (and schedule ticks), matches declarative
//! triggers, evaluates `when` conditions in the CEL sandbox, and runs ordered
//! actions via the transactional outbox (idempotent, retryable). Records
//! `workflow_run` history. See system-design §3.6 and design-plan §1.6.

use crate::schema::WorkflowDef;

/// One ordered action in a workflow (`set | create | update | delete | webhook
/// | notify | emit`). Modeled loosely until the action grammar is locked (D5).
pub type Action = serde_json::Value;

/// Start the workflow engine: subscribe to the domain-events topic and dispatch
/// matching workflows.
pub async fn run(_workflows: Vec<WorkflowDef>) {
    todo!("P8: Kafka consumer group, trigger matching, CEL `when`, outbox-backed actions")
}

/// Evaluate a single matched workflow against an event.
#[allow(dead_code)]
pub fn evaluate(_workflow: &WorkflowDef, _event: &serde_json::Value) {
    todo!("P8: evaluate `when` and run actions in order")
}
