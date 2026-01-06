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
