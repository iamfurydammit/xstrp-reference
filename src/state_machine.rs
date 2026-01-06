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
