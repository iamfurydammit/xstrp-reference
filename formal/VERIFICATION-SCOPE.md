# XSTRP Formal Verification Scope

## Purpose

This document defines the formal verification boundary for XSTRP.

The goal of this verification effort is to mathematically validate the
safety guarantees of the frozen XSTRP protocol as specified in:

- RFC-XSTRP-0001 (v1.0)
- PHASE-C1-FROZEN.md
- PHASE-C2-FROZEN.md
- PHASE-C3-FROZEN.md
- PHASE-C4-FROZEN.md

This verification substitutes for external adversarial review.

---

## What Is Verified

The following properties are subject to formal verification:

- State space correctness
- Valid and invalid state transitions
- Terminal state immutability
- Deterministic outcomes (completion or refund)
- Absence of stranded funds
- Enforcement of authorization gating at the state-transition level

Verification applies to protocol logic only.

---

## What Is Explicitly Out of Scope

The following are NOT modeled or verified:

- XRPL behavior or semantics
- Cryptographic primitives
- Digital signatures
- Networking, transport, or message ordering
- Persistence or storage
- Timing precision beyond abstract expiry
- Performance or scalability

---

## Authority and Immutability

This verification applies ONLY to Phases C1–C4.

No protocol behavior may be changed as a result of verification.
If a discrepancy is found, it must be resolved by clarifying the
specification, not by altering frozen behavior.

Any future protocol changes require a new phase charter.

---

## Verification Method

Primary method:
- TLA+ specification
- TLC model checking

Secondary artifacts may include:
- Invariant documentation
- Mapping notes between Rust code and formal model

---

## Status

Verification not yet started.
