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
