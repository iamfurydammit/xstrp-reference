/// Declarative interface for recording TransferIntent data.
///
/// This trait defines *that* an intent may be recorded,
/// not *how*, *where*, or *with what guarantees*.
///
/// No validation, persistence, or ordering semantics are implied.
pub trait IntentRecorder {
    /// Records a new TransferIntent.
    ///
    /// Implementations may reject duplicates or invalid data,
    /// but this interface makes no such guarantees.
    fn record_intent(&self, intent: crate::TransferIntent);
}
/// Declarative interface for observing the current state of a TransferIntent.
///
/// This trait permits inspection only.
/// It does not authorize transitions, validation, or mutation.
pub trait IntentStateView {
    /// Returns the current state of the intent, if known.
    ///
    /// No guarantees are made about freshness or consistency.
    fn get_state(&self, intent_id: &str) -> Option<crate::IntentState>;
}
/// Declarative interface for requesting a state transition on a TransferIntent.
///
/// This trait defines *that* a transition may be requested,
/// not *whether* it will occur or *how* it is evaluated.
///
/// No validation, authorization, or execution semantics are implied.
pub trait IntentTransitionRequest {
    /// Requests a transition to a new state.
    ///
    /// This method does not guarantee that the transition is valid,
    /// accepted, or applied.
    fn request_transition(
        &self,
        intent_id: &str,
        target_state: crate::IntentState,
    );
}
