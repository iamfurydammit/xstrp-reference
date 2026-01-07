// src/proof.rs

use crate::intent::{IntentId, Address, Amount};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionProof {
    pub intent_id: IntentId,
    pub receiver: Address,
    pub claimed_amount: Amount,
    pub evidence_ref: EvidenceRef,
    pub timestamp: u64,
    pub metadata: Option<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceRef(pub Vec<u8>);
