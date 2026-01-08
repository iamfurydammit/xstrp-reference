# XSTRP Phase C5 — Initial State Definition

## Purpose

This document defines the initial conditions for the Phase C5
formal verification model of XSTRP.

The initial state represents the moment immediately after an intent
is created, before any authorization, proof submission, or expiry.

---

## Initial Conditions

At model start, the following conditions hold:

- intent_state = Created
- authorization_valid = FALSE
- proof_present = FALSE
- is_expired = FALSE

No other state is permitted as an initial condition.

---

## Rationale

The Created state is the only valid protocol entry point.

At creation time:
- No authorization has yet been satisfied
- No proof artifacts exist
- The intent has not expired

All other states must be reached through explicit transitions
defined in the protocol.

---

## Exclusions

- No transition is assumed at initialization
- No implicit authorization exists
- No implicit expiry exists
- No terminal state is reachable at initialization

---

## Status

Initial state defined.
No transitions specified yet.
