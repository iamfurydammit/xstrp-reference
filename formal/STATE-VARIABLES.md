# XSTRP Phase C5 — Abstract State Variables

## Purpose

This document defines the abstract state variables used in the Phase C5
formal verification model of XSTRP.

These variables represent the minimal protocol state required to model
the frozen behavior defined in Phases C1–C4.

No implementation details, ledger semantics, cryptography, or timing
precision beyond abstract expiry are represented.

---

## Core State Variable

### intent_state

Represents the current lifecycle state of a transfer intent.

Abstract domain:

- Created
- Committed
- Completed
- Expired
- Refunded
- Invalid

This variable is authoritative for protocol progression.
Terminal states are immutable.

---

## Authorization Variable

### authorization_valid

Represents whether the authorization requirements for a transition
have been satisfied.

Abstract domain:

- TRUE
- FALSE

This variable abstracts all authorization inputs (proofs, confirmations,
fees) into a single gating signal.

No assumptions are made about how authorization is obtained.

---

## Proof Presence Variable

### proof_present

Represents whether a proof artifact has been supplied for a transition.

Abstract domain:

- TRUE
- FALSE

This variable does not model proof correctness, cryptography, or format.
It represents presence only.

---

## Expiry Variable

### is_expired

Represents whether the intent has reached abstract expiry.

Abstract domain:

- TRUE
- FALSE

This variable abstracts all timing behavior into a single boolean.
No clock or time model exists.

---

## Transition Eligibility Variable

### transition_allowed

Derived variable indicating whether a state transition is permitted
given the current abstract state.

This variable is not independently mutable.
It exists only to express invariants and guards.

---

## Excluded Variables

The following are intentionally excluded and MUST NOT appear in the model:

- Ledger balances
- Account addresses
- Cryptographic keys or signatures
- Network messages
- Wallet behavior
- User interfaces
- Persistent storage
- Fees as numeric values (only abstract authorization)

---

## Notes on Abstraction

All real-world inputs (user actions, wallet signing, hardware confirmation,
fee payments) are abstracted into boolean conditions.

This is intentional and required to preserve scope discipline.

---

## Status

Abstract state variables defined.
No formal specification written yet.
