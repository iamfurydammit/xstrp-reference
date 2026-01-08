# XSTRP — XRP Safe Transfer & Recovery Protocol

This repository contains the **reference Rust implementation** of the
**XRP Safe Transfer & Recovery Protocol (XSTRP)**.

XSTRP is a safety-first transfer protocol designed to guarantee that
funds are **never stranded**, **never misdirected**, and **never released
without explicit receiver participation**, even under adversarial or
failure conditions.

---

## Status

- RFC-XSTRP-0001 **frozen (v1.0)**
- **Core protocol state machine complete and locked for v1**
- Reference implementation written in **Rust**
- Explicit safety-first design with terminal-state immutability
- No XRPL integration yet
- No cryptography yet
- Protocol logic formally verified via TLA+ (Phase C5)

---

## Protocol State Machine (v1)

### States

- `Created`
- `Committed`
- `Completed` *(terminal)*
- `Expired`
- `Refunded` *(terminal)*
- `Invalid` *(terminal)*

### Events

- `ReceiverConfirms`
- `CompletionProofVerified`
- `ProofInvalid`
- `Timeout`
- `RefundProcessed`
- `InvalidDetected`

### Guarantees

- Terminal states are **immutable**
- Invalidity can be detected and enforced from any non-terminal state
- No legal execution path results in unrecoverable or stranded funds
- All transitions are explicit and exhaustively checked

---

## Implementation Status

- Core state machine implemented in `src/state_machine.rs`
- All legal and illegal transitions explicitly handled
- Unit-tested invariants
- `cargo test` passes cleanly

---

## Testing

The reference implementation includes unit tests validating:

- Successful commit and completion paths
- Invalid-proof handling
- Global invalidation handling
- Terminal-state immutability

---

## Milestone Declaration

The **v1 core state machine is complete and frozen**.

Further work will proceed only by **extension**, not modification, and
will focus on:

- XRPL integration
- Proof format definition
- Cryptographic verification
- Ledger interaction modeling

---

## License

Payne Protocol License (PPL-0.1) — see the `LICENSE` file for full terms.



