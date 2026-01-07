//! Transfer Intent definition for XSTRP
//!
//! An Intent is an immutable, cryptographically bound declaration
//! of transfer parameters. No receiver address is included.
use crate::intent_binding::IntentBinding;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntentId(pub [u8; 32]);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Amount(pub u64);

#[derive(Debug, Clone, Serialize)]
pub struct TransferIntent {
    pub amount_xrp_drops: u64,
    pub expiry_unix: u64,
    pub protocol_version: &'static str,
    pub fee_drops: u64,

    /// Optional contextual binding (ledger / participants)
    pub binding: Option<IntentBinding>,
}

impl TransferIntent {
    pub fn new(amount_xrp_drops: u64, expiry_unix: u64) -> Self {
        Self {
            amount_xrp_drops,
            expiry_unix,
            protocol_version: "RFC-XSTRP-0001",
            fee_drops: 10_000, // 0.01 XRP base fee
            binding: None,
        }
    }
}
