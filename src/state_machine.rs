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
    pub fn transition(self, event: TransitionEvent) -> Self {
        match (self, event) {
            (IntentState::Created, TransitionEvent::ReceiverConfirms) => {
                IntentState::Committed
            }
            (IntentState::Created, TransitionEvent::Timeout) => {
                IntentState::Expired
            }
            (IntentState::Expired, TransitionEvent::RefundProcessed) => {
                IntentState::Refunded
            }
            _ => self,
        }
    }
}
