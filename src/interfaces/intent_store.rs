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
