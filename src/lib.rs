//! XSTRP reference library
//!
//! This crate contains protocol-level logic only.
//! No custody, no network authority, no UI assumptions.

pub mod intent;

/// Re-export the core intent type for easier use.
pub use intent::TransferIntent;
