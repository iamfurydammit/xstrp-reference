# Phase C3 — Failure & Refund Guarantees

This document defines all failure paths and refund guarantees
for XSTRP intents across all non-terminal states.

No executable logic is defined here.
This document assigns mandatory outcomes only.

---

## Failure Design Principle

All failures in XSTRP MUST result in one of the following:

- Refund to sender
- Terminal invalidation
- No state change (safe rejection)

There is no failure mode in which funds remain stranded.

---

## State: Created

### Failure Conditions
- Malformed intent detected
- Receiver never acknowledges
- Intent expires

### Outcomes
- Malformed intent → Invalid (terminal)
- No acknowledgment before expiry → Refund to sender
- Expiry reached → Refund to sender

---

## State: ReceiverConfirmed

### Failure Conditions
- Sender never commits
- Proof contradiction detected
- Intent expires

### Outcomes
- No sender action before expiry → Refund to sender
- Proof contradiction → Invalid (terminal)
- Expiry reached → Refund to sender

---

## State: Committed

### Failure Conditions
- Settlement proof invalid
- Settlement never completes
- Conflicting settlement evidence
- Intent expires

### Outcomes
- Invalid settlement proof → Invalid (terminal)
- No settlement before expiry → Refund to sender
- Conflicting evidence → Invalid (terminal)
- Expiry reached → Refund to sender

---

## Terminal States

### Completed
- No failure handling required
- Terminal and immutable

### Invalid
- Terminal and immutable
- Refund path preserved if applicable

---

## Refund Guarantees

- Refunds are mandatory outcomes, not best-effort
- Refunds must be deterministic and unambiguous
- Refund logic is external to Phase C3
- Refund execution mechanism is not defined here

---

## Safety Invariants

The following invariants MUST always hold:

- Funds cannot move without authorization
- Funds cannot remain locked indefinitely
- Invalid actions cannot advance state
- Terminal states cannot be exited

---

## Status

This document completes failure handling and refund guarantees
for Phase C3.
