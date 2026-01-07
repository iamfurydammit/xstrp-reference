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
use crate::intent::TransferIntent;

impl CompletionProof {
    pub fn validate_against(
        &self,
        intent: &TransferIntent,
    ) -> Result<(), &'static str> {

        if self.intent_id.0.len() != 32 {
            return Err("invalid intent id length");
        }

        if self.claimed_amount.0 != intent.amount_xrp_drops {
            return Err("amount mismatch");
        }

        if self.timestamp > intent.expiry_unix {
            return Err("proof expired");
        }

        if self.evidence_ref.0.is_empty() {
            return Err("missing evidence");
        }

        Ok(())
    }
}
