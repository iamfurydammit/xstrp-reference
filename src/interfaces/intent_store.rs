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
