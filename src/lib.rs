//! XSTRP reference library
//!
//! This crate contains protocol-level logic only.
//! No custody, no network authority, no UI assumptions.

pub mod state_machine;
pub mod intent;
pub mod proof;
pub mod bridge;

/// Re-export the core intent type for easier use.
pub use intent::TransferIntent;

#[cfg(test)]
mod smoke_test {
    #[test]
    fn it_runs_tests() {
        assert_eq!(2 + 2, 4);
    }
}

