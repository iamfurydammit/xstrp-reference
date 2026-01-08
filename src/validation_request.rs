//! Phase C4 — Validation Request Types
//!
//! This module defines typed inputs to validation and authorization checks.
//! It binds together intent state, requested transition, and supplied proofs.
//!
//! No validation logic is implemented here.

use crate::proofs::Proof;
use crate::state_machine::IntentState;
use crate::TransferIntent;

/// A request to validate whether a transition may occur.
#[derive(Debug, Clone)]
pub struct ValidationRequest<'a> {
    /// The intent being acted upon.
    pub intent: &'a TransferIntent,

    /// The current state of the intent.
    pub current_state: IntentState,

    /// The state being requested.
    pub requested_state: IntentState,

    /// Proofs supplied in support of the transition.
    pub proofs: Vec<Proof>,
}
