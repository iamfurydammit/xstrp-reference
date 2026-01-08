# XSTRP Phase C5 — Formal Verification Scope

## Purpose

This document defines the formal verification boundary for Phase C5 of XSTRP.

Phase C5 extends formal verification rigor applied to the already frozen
protocol behavior defined in Phases C1–C4. It does not replace or reinterpret
any prior verification work.

---

## Authoritative Sources

The following documents are authoritative and frozen:

- RFC-XSTRP-0001 (v1.0)
- PHASE-C1-FROZEN.md
- PHASE-C2-FROZEN.md
- PHASE-C3-FROZEN.md
- PHASE-C4-FROZEN.md
- PHASE-C5-CHARTER.md

---

## Relationship to Prior Verification

A minimal TLA+ model was previously created to sanity-check Phases C1–C4.

That work remains valid, closed, and unchanged.

Phase C5 introduces no new protocol semantics and does not expand
the verified behavior set. It increases verification depth only.

---

## What Is Verified

Phase C5 formal verification applies only to protocol logic defined
in Phases C1–C4.

The following properties are in scope:

- State space correctness
- Valid and invalid state transitions
- Authorization gating at transition boundaries
- Terminal state immutability
- Deterministic outcomes (completion or refund)
- Absence of stranded funds

---

## What Is Explicitly Out of Scope

The following are NOT modeled or verified:

- XRPL behavior or ledger semantics
- Cryptographic primitives or signatures
- Wallet behavior or UX
- Networking, transport, or message ordering
- Persistence or storage mechanisms
- Timing precision beyond abstract expiry
- Performance or scalability characteristics

---

## Verification Constraints

- No protocol behavior may be changed as a result of verification
- No Rust implementation may be altered to satisfy verification
- No new states or transitions may be introduced

Any discrepancy must be resolved by specification clarification
or documented modeling limitation.

---

## Status

Boundary defined.
Phase C5 formal modeling not yet started.
