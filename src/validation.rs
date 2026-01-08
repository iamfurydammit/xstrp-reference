//! Phase C4 — Validation Result Types
//!
//! This module defines explicit, typed outcomes for validation
//! and authorization checks.
//!
//! No logic is implemented here.
//! These types are consumed by transition execution code.

/// Result of a validation or authorization check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationOutcome {
    /// Action is permitted and may proceed.
    Approved,

    /// Action is rejected without state change.
    Rejected(RejectReason),

    /// Action causes terminal invalidation.
    Invalidated(InvalidateReason),

    /// Action triggers refund to sender.
    RefundRequired(RefundReason),
}

/// Reason an action was rejected without state change.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RejectReason {
    Unauthorized,
    InvalidProof,
    StaleProof,
    WrongState,
}

/// Reason an intent is invalidated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidateReason {
    ProofContradiction,
    TamperingDetected,
    MalformedIntent,
}

/// Reason a refund is required.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefundReason {
    Expired,
    SettlementFailed,
}
