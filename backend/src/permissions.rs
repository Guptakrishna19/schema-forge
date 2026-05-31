//! Permission Evaluator.
//!
//! Resolves role/attribute rules from schema metadata per (entity, action, field).
//! ABAC expressions (`record.ownerId == user.id`) evaluate in a sandboxed CEL
//! evaluator. Deny-by-default; fail-closed on errors. See system-design §3.5 and
//! design-plan §1.5.

use crate::error::AppError;
use crate::schema::EntityDef;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Read,
    Create,
    Update,
    Delete,
}

impl Action {
    pub fn as_key(self) -> &'static str {
        match self {
            Action::Read => "read",
            Action::Create => "create",
            Action::Update => "update",
            Action::Delete => "delete",
        }
    }
}

/// Coarse (entity, action) gate. Returns `Forbidden` when no grant matches.
/// Deny-by-default; a later pass threads in subject + record for `role:`/`expr:`
/// grants, field masks, and the CEL sandbox.
pub fn check(_def: &EntityDef, _action: Action) -> Result<(), AppError> {
    todo!("P4: match grants for (entity, action); deny by default")
}
