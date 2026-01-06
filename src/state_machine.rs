#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntentState {
    Created,
    Committed,
    Completed,
    Expired,
    Refunded,
    Invalid,
}
#[derive(Debug)]
pub enum TransitionEvent {
    ReceiverConfirms,
    CompletionProofVerified,
    ProofInvalid,
    Timeout,
    InvalidDetected,
    RefundProcessed,
}

impl IntentState {
    pub fn transition(
        self,
        event: TransitionEvent,
    ) -> Result<IntentState, &'static str> {
        match (self, event) {
           (
    IntentState::Completed
    | IntentState::Refunded
    | IntentState::Invalid,
    _
) => Err("Terminal state cannot transition"),
            
(IntentState::Committed, TransitionEvent::CompletionProofVerified) => {
    Ok(IntentState::Completed)
},
            (IntentState::Created, TransitionEvent::ReceiverConfirms) => {
                Ok(IntentState::Committed)
            }
            (IntentState::Created, TransitionEvent::Timeout) => {
                Ok(IntentState::Expired)
            }
            (IntentState::Expired, TransitionEvent::RefundProcessed) => {
                Ok(IntentState::Refunded)
            }
            _ => Err("Illegal state transition"),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn created_can_become_committed() {
        let state = IntentState::Created;

        let new_state = state
            .transition(TransitionEvent::ReceiverConfirms)
            .unwrap();

        assert_eq!(new_state, IntentState::Committed);
    }
}
