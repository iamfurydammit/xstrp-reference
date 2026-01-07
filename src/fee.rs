/// Represents the context of a protocol-defined "use"
#[derive(Debug, Clone)]
pub struct UseContext {
    pub intent_id: String,
    pub from_state: String,
    pub to_state: String,
    pub timestamp_unix: u64,
}

/// Represents a record that a usage event was registered
#[derive(Debug, Clone)]
pub struct FeeReceipt {
    pub context_hash: String,
}

/// Abstract sink for recording protocol usage
pub trait FeeSink {
    /// Record a single protocol-defined "use"
    fn record_use(&self, context: &UseContext) -> FeeReceipt;
}
