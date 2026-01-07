use crate::intent::TransferIntent;
use crate::proof::CompletionProof;
use crate::state_machine::{IntentState, TransitionEvent};

pub fn apply_completion_proof(
    state: IntentState,
    intent: &TransferIntent,
    proof: &CompletionProof,
) -> Result<IntentState, &'static str> {
    if state != IntentState::Committed {
        return Err("Completion proof can only be applied in Committed state");
    }

    let event = if proof.validate_against(intent).is_ok() {
        TransitionEvent::CompletionProofVerified
    } else {
        TransitionEvent::ProofInvalid
    };

    state.transition(event)
}
#[cfg(test)]
mod tests {
    use super::apply_completion_proof;
    use crate::intent::{IntentId, TransferIntent, Address, Amount};
    use crate::proof::{CompletionProof, EvidenceRef};
    use crate::state_machine::IntentState;


    fn test_intent_id() -> IntentId {
        IntentId([1u8; 32])
    }

    fn sample_intent() -> TransferIntent {
        TransferIntent {
            amount_xrp_drops: 100,
            expiry_unix: 999_999_999,
            protocol_version: "v1",
            fee_drops: 1,
        }
    }

    fn valid_proof() -> CompletionProof {
        CompletionProof {
            intent_id: test_intent_id(),
            receiver: Address("receiver".to_string()),
            claimed_amount: Amount(100),
            evidence_ref: EvidenceRef(b"evidence".to_vec()),
            timestamp: 1,
            metadata: None,
        }
    }

    fn invalid_proof() -> CompletionProof {
        CompletionProof {
            intent_id: test_intent_id(),
            receiver: Address("receiver".to_string()),
            claimed_amount: Amount(999),
            evidence_ref: EvidenceRef(b"evidence".to_vec()),
            timestamp: 1,
            metadata: None,
        }
    }

    #[test]
    fn committed_with_valid_proof_becomes_completed() {
        let state = IntentState::Committed;
        let intent = sample_intent();
        let proof = valid_proof();

        let new_state =
            apply_completion_proof(state, &intent, &proof).unwrap();

        assert_eq!(new_state, IntentState::Completed);
    }

    #[test]
    fn committed_with_invalid_proof_becomes_invalid() {
        let state = IntentState::Committed;
        let intent = sample_intent();
        let proof = invalid_proof();

        let new_state =
            apply_completion_proof(state, &intent, &proof).unwrap();

        assert_eq!(new_state, IntentState::Invalid);
    }

    #[test]
    fn proof_applied_outside_committed_is_rejected() {
        let state = IntentState::Created;
        let intent = sample_intent();
        let proof = valid_proof();

        let result =
            apply_completion_proof(state, &intent, &proof);

        assert!(result.is_err());
    }
}

