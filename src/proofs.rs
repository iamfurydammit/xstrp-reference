//! Phase C4 — Abstract Proof Inputs
//!
//! This module defines the concrete data structures used to
//! represent abstract proofs described in Phase C3.
//!
//! No cryptography, verification, or logic is implemented here.


/// Proofs are provided as inputs to validation and transition requests.
/// They are not trusted by default.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Proof {
    ReceiverAcknowledgment(ReceiverAcknowledgmentProof),
    SenderCommitment(SenderCommitmentProof),
    SettlementFulfillment(SettlementFulfillmentProof),
    Expiry(ExpiryProof),
}

/// Proof that the receiver has acknowledged the intent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiverAcknowledgmentProof {
    pub intent_id: String,
    pub receiver_id: String,
}

/// Proof that the sender commits after receiver confirmation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SenderCommitmentProof {
    pub intent_id: String,
    pub sender_id: String,
}

/// Proof that settlement conditions have been fulfilled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettlementFulfillmentProof {
    pub intent_id: String,
    /// Abstract settlement reference (ledger-agnostic)
    pub settlement_reference: String,
}

/// Proof that an intent has expired.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpiryProof {
    pub intent_id: String,
    /// Abstract time reference (not bound to wall-clock or ledger time)
    pub observed_time: u64,
}
