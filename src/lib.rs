//! XSTRP reference library
//!
//! This crate contains protocol-level logic only.
//! No custody, no network authority, no UI assumptions.

pub mod state_machine;
pub mod intent;
pub mod proof;
pub mod bridge;
pub mod intent_binding;
pub mod interfaces;
pub mod execute;
pub mod proofs;
pub mod validation;
pub mod validation_request;

/// Re-export the core intent type for easier use.
pub use intent::TransferIntent;

#[cfg(test)]
mod smoke_test {
    #[test]
    fn it_runs_tests() {
        assert_eq!(2 + 2, 4);
    }
}

