#[cfg(test)]
mod tests {
    use super::*;
    use crate::intent::{Address, IntentId, TransferIntent};
    use crate::proof::CompletionProof;
    use crate::state_machine::IntentState;

    fn sample_intent() -> TransferIntent {
        TransferIntent {
            intent_id: IntentId("intent123".to_string()),
            sender: Address("sender".to_string()),
            receiver: Address("receiver".to_string()),
            amount: 100,
            expires_at: 999_999_999,
        }
    }

    fn valid_proof() -> CompletionProof {
        CompletionProof {
            claimed_amount: 100,
            evidence_ref: Some("evidence".to_string()),
            timestamp: 1,
            metadata: None,
        }
    }

    fn invalid_proof() -> CompletionProof {
        CompletionProof {
            claimed_amount: 999, // mismatched amount
            evidence_ref: None,
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
