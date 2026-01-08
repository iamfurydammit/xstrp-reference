# XSTRP Phase C5 — Safety Invariants

## Purpose

This document defines the safety invariants for the Phase C5
formal verification model of XSTRP.

These invariants must hold in all reachable states of the model.
Violation of any invariant constitutes a protocol safety failure.

---

## Invariant 1 — Valid State Space

The intent_state variable must always be one of the defined protocol states:

- Created
- Committed
- Completed
- Expired
- Refunded
- Invalid

No other value is permitted.

---

## Invariant 2 — Terminal State Immutability

If intent_state is one of:

- Completed
- Refunded
- Invalid

Then intent_state must never change in any future state.

---

## Invariant 3 — No Completion Without Authorization

If intent_state = Completed, then:

- authorization_valid = TRUE
- proof_present = TRUE
- is_expired = FALSE

Completion without explicit authorization is impossible.

---

## Invariant 4 — No Completion After Expiry

If intent_state = Completed, then:

- is_expired = FALSE

Expired intents cannot complete.

---

## Invariant 5 — Deterministic Termination

From any reachable non-terminal state, the protocol must eventually reach
exactly one terminal state:

- Completed
- Refunded
- Invalid

No execution path may avoid termination indefinitely.

---

## Invariant 6 — No Stranded Funds

There exists no reachable state in which:

- intent_state is non-terminal
- and no transition is possible
- and intent_state is not Expired

All non-terminal states must have a valid outgoing transition
or be eligible for expiry and refund.

---

## Invariant 7 — Authorization Gating

Any transition that advances intent_state toward Completed
must require authorization_valid = TRUE.

Unauthorized advancement is forbidden.

---

## Status

Safety invariants defined.
No formal specification written yet.
