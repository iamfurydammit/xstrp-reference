//! Phase C2 interface definitions (planning-only).
//!
//! This module contains trait declarations only.
//! No logic, no implementations, no side effects.
//!
//! Governed by PHASE-C2-CHARTER.md
/// Declarative interface for accessing TransferIntent data.
///
/// This trait defines *what* may be accessed, not *how*.
/// No guarantees are made about storage, persistence, or durability.
///
/// Implementations are supplied externally and are non-authoritative.
pub trait IntentStore {
    /// Returns a TransferIntent by its identifier, if available.
    ///
    /// This method must not mutate state or trigger side effects.
    fn get_intent(&self, intent_id: &str) -> Option<crate::TransferIntent>;
}
