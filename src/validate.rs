//! Phase C4 — Core Validation Logic
//!
//! This module implements deterministic, side-effect-free validation
//! of requested state transitions.
//!
//! No state mutation occurs here.
//! No transitions are executed here.

use crate::proofs::Proof;
use crate::state_machine::IntentState;
use crate::validation::{InvalidateReason, RejectReason, RefundReason, ValidationOutcome};
use crate::validation_request::ValidationRequest;

/// Validate whether a requested transition is permitted.
///
/// This function enforces Phase C3 rules.
/// It does not execute transitions or mutate state.
pub fn validate_transition(request: &ValidationRequest) -> ValidationOutcome {
    // Reject immediately if already in terminal state
    if matches!(request.current_state, IntentState::Completed | IntentState::Invalid) {
        return ValidationOutcome::Rejected(RejectReason::WrongState);
    }

    // Expiry proof always has priority
    if request.proofs.iter().any(|p| matches!(p, Proof::Expiry(_))) {
        return ValidationOutcome::RefundRequired(RefundReason::Expired);
    }

    match (request.current_state, request.requested_state) {
        // Created → ReceiverConfirmed
        (IntentState::Created, IntentState::ReceiverConfirmed) => {
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

        // ReceiverConfirmed → Committed
        (IntentState::ReceiverConfirmed, IntentState::Committed) => {
            if request
                .proofs
                .iter()
                .any(|p| matches!(p, Proof::SenderCommitment(_)))
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

        // All other transitions are illegal
        _ => ValidationOutcome::Rejected(RejectReason::WrongState),
    }
}
