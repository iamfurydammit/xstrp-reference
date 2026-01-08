# XSTRP Phase C5 — Abstract State Transitions

## Purpose

This document defines the abstract state transitions for the Phase C5
formal verification model of XSTRP.

These transitions correspond exactly to the frozen protocol behavior
defined in Phases C1–C4.

---

## Transition Rules

All transitions are evaluated atomically.
Exactly one transition may occur at a time.

---

## Non-Terminal State Transitions

### Created → Committed

Permitted if and only if:
- authorization_valid = TRUE
- proof_present = TRUE
- is_expired = FALSE

Result:
- intent_state := Committed

---

### Created → Invalid

Permitted if:
- proof_present = TRUE
- authorization_valid = FALSE

Result:
- intent_state := Invalid

---

### Created → Expired

Permitted if:
- is_expired = TRUE

Result:
- intent_state := Expired

---

## Committed State Transitions

### Committed → Completed

Permitted if and only if:
- authorization_valid = TRUE
- proof_present = TRUE
- is_expired = FALSE

Result:
- intent_state := Completed

---

### Committed → Invalid

Permitted if:
- proof_present = TRUE
- authorization_valid = FALSE

Result:
- intent_state := Invalid

---

### Committed → Expired

Permitted if:
- is_expired = TRUE

Result:
- intent_state := Expired

---

## Expired State Transitions

### Expired → Refunded

Permitted unconditionally.

Result:
- intent_state := Refunded

---

## Terminal States

The following states are terminal and immutable:

- Completed
- Refunded
- Invalid

No transitions are permitted from these states.

---

## Forbidden Transitions

Any transition not explicitly listed above is forbidden.

This includes but is not limited to:
- Skipping states
- Leaving terminal states
- Completing without authorization
- Completing after expiry
- Refunding from non-expired states

---

## Status

Abstract transition relation defined.
No formal specification written yet.
