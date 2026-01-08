//! Phase C4 — Core Validation Logic
//!
//! Deterministic, side-effect-free validation of requested state transitions.
//! No state mutation occurs here.

use crate::proofs::Proof;
use crate::state_machine::IntentState;
use crate::validation::{InvalidateReason, RejectReason, RefundReason, ValidationOutcome};
use crate::validation_request::ValidationRequest;

/// Validate whether a requested transition is permitted.
///
/// Enforces Phase C3 rules while respecting frozen Phase C1 states.
pub fn validate_transition(request: &ValidationRequest) -> ValidationOutcome {
    // Terminal states may not transition
    if matches!(
        request.current_state,
        IntentState::Completed | IntentState::Refunded | IntentState::Invalid
    ) {
        return ValidationOutcome::Rejected(RejectReason::WrongState);
    }

    // Expiry always wins
    if matches!(request.current_state, IntentState::Expired) {
        return ValidationOutcome::RefundRequired(RefundReason::Expired);
    }

    // Expiry proof has priority
    if request.proofs.iter().any(|p| matches!(p, Proof::Expiry(_))) {
        return ValidationOutcome::RefundRequired(RefundReason::Expired);
    }

    match (request.current_state.clone(), request.requested_state.clone()) {
        // Created → Committed (Receiver confirms)
        (IntentState::Created, IntentState::Committed) => {
            if request
                .proofs
                .iter()
                .any(|p| matches!(p, Proof::ReceiverAcknowledgment(_)))
            {
                ValidationOutcome::Approved
            } else {
                ValidationOutcome::Rejected(RejectReason::InvalidProof)
            }
        }

        // Committed → Completed
        (IntentState::Committed, IntentState::Completed) => {
            if request
                .proofs
                .iter()
                .any(|p| matches!(p, Proof::SettlementFulfillment(_)))
            {
                ValidationOutcome::Approved
            } else {
                ValidationOutcome::Rejected(RejectReason::InvalidProof)
            }
        }

        // Any → Invalid (protocol-driven)
        (_, IntentState::Invalid) => {
            ValidationOutcome::Invalidated(InvalidateReason::MalformedIntent)
        }

        // Everything else is illegal
        _ => ValidationOutcome::Rejected(RejectReason::WrongState),
    }
}
