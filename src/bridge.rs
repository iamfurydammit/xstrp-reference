use crate::state_machine::{IntentState, TransitionEvent};
use crate::proof::CompletionProof;
pub fn apply_completion_proof(
    state: IntentState,
    proof: &CompletionProof,
) -> Result<IntentState, &'static str> {
    if state != IntentState::Committed {
        return Err("Completion proof can only be applied in Committed state");
    }

    let event = if proof.validate().is_ok() {
        TransitionEvent::CompletionProofVerified
    } else {
        TransitionEvent::ProofInvalid
    };

    state.transition(event)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::proof::CompletionProof;
    use crate::state_machine::IntentState;

    fn valid_proof() -> CompletionProof {
        CompletionProof {
            intent_id: "intent123".to_string(),
            amount: 100,
            receiver: "receiver".to_string(),
            expires_at: 999_999_999,
            evidence: Some("evidence".to_string()),
        }
    }

    fn invalid_proof() -> CompletionProof {
        CompletionProof {
            intent_id: "intent123".to_string(),
            amount: 100,
            receiver: "receiver".to_string(),
            expires_at: 0, // expired
            evidence: None,
        }
    }

    #[test]
    fn committed_with_valid_proof_becomes_completed() {
        let state = IntentState::Committed;
        let proof = valid_proof();

        let new_state = apply_completion_proof(state, &proof).unwrap();

        assert_eq!(new_state, IntentState::Completed);
    }

    #[test]
    fn committed_with_invalid_proof_becomes_invalid() {
        let state = IntentState::Committed;
        let proof = invalid_proof();

        let new_state = apply_completion_proof(state, &proof).unwrap();

        assert_eq!(new_state, IntentState::Invalid);
    }

    #[test]
    fn proof_applied_outside_committed_is_rejected() {
        let state = IntentState::Created;
        let proof = valid_proof();

        let result = apply_completion_proof(state, &proof);

        assert!(result.is_err());
    }
}
