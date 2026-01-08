//! Phase C4 — Transition Execution
//!
//! This module executes state transitions only after
//! successful validation.
//!
//! Validation logic is NOT duplicated here.

use crate::state_machine::IntentState;
use crate::validate::validate_transition;
use crate::validation::ValidationOutcome;
use crate::validation_request::ValidationRequest;

/// Attempt to execute a state transition.
///
/// This function mutates state only if validation approves.
/// All other outcomes are returned unchanged.
pub fn execute_transition(
    current_state: IntentState,
    request: &ValidationRequest,
) -> (IntentState, ValidationOutcome) {
    let outcome = validate_transition(request);

    match outcome {
        ValidationOutcome::Approved => (request.requested_state.clone(), outcome),
        _ => (current_state, outcome),
    }
}
