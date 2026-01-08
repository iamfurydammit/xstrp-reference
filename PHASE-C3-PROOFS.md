# Phase C3 — Proof Semantics (Abstract)

This document defines the semantic meaning of proofs used in XSTRP.
Proofs are evidence claims, not cryptographic constructions.

No proof formats, algorithms, or ledger mechanisms are defined here.

---

## Proof Categories

XSTRP recognizes the following abstract proof categories:

1. Receiver Acknowledgment Proof
2. Sender Commitment Proof
3. Settlement Fulfillment Proof
4. Expiry Proof (Protocol-derived)

---

## 1. Receiver Acknowledgment Proof

**Purpose:**
Demonstrates that the receiver has explicitly acknowledged
the intent and agrees to participate.

**Semantic Claims:**
- Receiver identity matches intent.receiver
- Receiver has observed intent parameters
- Receiver has not rejected the transfer

**Validity Conditions:**
- Acknowledgment corresponds to a specific intent ID
- No conflicting receiver acknowledgments exist

**Invalidation Conditions:**
- Receiver identity mismatch
- Contradictory acknowledgment detected
- Proof refers to altered intent parameters

---

## 2. Sender Commitment Proof

**Purpose:**
Demonstrates that the sender authorizes commitment
after receiver confirmation.

**Semantic Claims:**
- Sender identity matches intent.sender
- Sender authorizes irreversible commitment
- Intent parameters remain unchanged

**Validity Conditions:**
- ReceiverConfirmed state already reached
- Proof binds to the same intent ID

**Invalidation Conditions:**
- Sender identity mismatch
- Attempted reuse across intents
- Intent mutation detected

---

## 3. Settlement Fulfillment Proof

**Purpose:**
Demonstrates that settlement conditions have been satisfied.

**Semantic Claims:**
- Required settlement condition has occurred
- No contradictory settlement exists

**Validity Conditions:**
- Proof corresponds to committed intent
- Settlement occurs within validity window

**Invalidation Conditions:**
- Partial or ambiguous settlement
- Conflicting settlement evidence
- Settlement after expiry

---

## 4. Expiry Proof (Protocol-Derived)

**Purpose:**
Demonstrates that the intent has exceeded its allowed lifetime.

**Semantic Claims:**
- Expiry condition defined by intent has been reached

**Validity Conditions:**
- Current time exceeds intent expiry
- Intent not already completed

**Invalidation Conditions:**
- Intent already in terminal state

---

## Proof Freshness & Staleness

- Proofs are considered **stale** if submitted after expiry
- Stale proofs MUST NOT advance state
- Stale proofs MAY trigger invalidation or refund paths

---

## General Rules

- Proofs never mutate state directly
- Proofs are evaluated only in context of a requested transition
- Proof evaluation is deterministic and side-effect free
- Invalid proofs never advance state

---

## Non-Goals

This phase does not define:
- Proof encoding
- Signature schemes
- Cryptographic verification
- On-chain representation

These are deferred to later phases.

---

## Status

This document completes abstract proof semantics for Phase C3.
