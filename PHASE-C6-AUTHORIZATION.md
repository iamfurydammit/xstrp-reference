# Phase C6 Authorization Boundary — XSTRP

## Status
DRAFT — NOT AUTHORIZED

## Purpose

This document defines the authorization boundary for Phase C6 of XSTRP.

Phase C6 exists solely to explore external bindings and extensions to the
already frozen and formally verified XSTRP core (Phases C1–C5).

This document does not authorize implementation work.

## What Phase C6 Is Allowed to Discuss

Phase C6 may discuss:

- External system bindings (e.g., ledgers, wallets)
- Adapter patterns that observe or relay XSTRP intent state
- Extension governance and compatibility rules
- Economic or fee signaling models external to the core
- UX flows that do not alter protocol behavior

## What Phase C6 Is NOT Allowed to Do

Phase C6 must not:

- Modify any Phase C1–C5 artifacts
- Introduce new protocol states or transitions
- Change authorization rules
- Introduce custody, signing, or transaction broadcast logic
- Assume trust in wallets, ledgers, or users
- Reinterpret formally verified properties

Any such change requires a new protocol version.

## Authority Separation

- XSTRP Core defines intent semantics
- Wallets control keys and signing
- Ledgers control settlement
- Phase C6 extensions have no authority

## Non-Authorization Clause

This document does not authorize:

- Phase C6 sub-phases
- Implementations
- External integrations
- Economic enforcement mechanisms

Explicit authorization is required for each.

## Acceptance

Phase C6 remains inactive until this document is explicitly accepted
and committed.
