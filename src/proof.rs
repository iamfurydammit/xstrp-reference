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
    #[cfg(test)]
mod tests {
    use super::*;
    use crate::intent::{TransferIntent, IntentId, Address, Amount};

    fn sample_intent() -> TransferIntent {
        TransferIntent::new(1_000_000, 1_700_000_000)
    }

    fn valid_proof(intent: &TransferIntent) -> CompletionProof {
        CompletionProof {
            intent_id: IntentId([0u8; 32]),
            receiver: Address("rReceiverAddress".to_string()),
            claimed_amount: Amount(intent.amount_xrp_drops),
            evidence_ref: EvidenceRef(vec![1, 2, 3]),
            timestamp: intent.expiry_unix,
            metadata: None,
        }
    }

    #[test]
    fn proof_validates_successfully() {
        let intent = sample_intent();
        let proof = valid_proof(&intent);

        assert!(proof.validate_against(&intent).is_ok());
    }

    #[test]
    fn proof_fails_on_amount_mismatch() {
        let intent = sample_intent();
        let mut proof = valid_proof(&intent);

        proof.claimed_amount = Amount(intent.amount_xrp_drops + 1);

        let err = proof.validate_against(&intent).unwrap_err();
        assert_eq!(err, "amount mismatch");
    }

    #[test]
    fn proof_fails_when_expired() {
        let intent = sample_intent();
        let mut proof = valid_proof(&intent);

        proof.timestamp = intent.expiry_unix + 1;

        let err = proof.validate_against(&intent).unwrap_err();
        assert_eq!(err, "proof expired");
    }

    #[test]
    fn proof_fails_when_missing_evidence() {
        let intent = sample_intent();
        let mut proof = valid_proof(&intent);

        proof.evidence_ref = EvidenceRef(Vec::new());

        let err = proof.validate_against(&intent).unwrap_err();
        assert_eq!(err, "missing evidence");
    }
}

