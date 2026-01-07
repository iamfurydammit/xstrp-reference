/// IntentBinding is a passive, non-enforcing metadata container.
///
/// It provides contextual information about where an intent
/// is expected to exist (ledger) and between whom (participants).
///
/// This structure MUST NOT:
/// - Perform validation
/// - Enforce correctness
/// - Interact with the state machine
/// - Assume XRPL semantics
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntentBinding {
    /// Identifier for the target ledger or environment.
    /// Example: "XRPL-mainnet", "XRPL-testnet", "simulated-ledger"
    pub ledger_id: String,

    /// Opaque sender identifier (address, account, or alias).
    pub sender: String,

    /// Opaque receiver identifier (address, account, or alias).
    pub receiver: String,
}
