# Phase C6-B Charter — Wallet & UX Integration

## Status
DRAFT — NOT AUTHORIZED

## Purpose

Phase C6-B exists to define governance rules and safety boundaries for
how XSTRP intents are presented to, confirmed by, and handed off through
wallet and user interface surfaces, without altering protocol semantics
or assuming control over keys, signing, or transaction broadcast. This
phase ensures that human interaction with XSTRP preserves explicit
authorization, informed consent, and deterministic outcomes while
maintaining strict authority separation between the XSTRP core, wallets,
and external systems.

## Scope — What This Phase Does

- Defines what information about an XSTRP intent must be surfaced to a human participant.
- Governs the timing and conditions under which human confirmation is required.
- Specifies boundaries for wallet and interface participation without granting authority.
- Describes safe handoff expectations between XSTRP and user-facing surfaces.
- Constrains how user experience choices may affect protocol safety guarantees.

## Explicit Non-Goals — What This Phase Does NOT Do

- This phase does not define or modify protocol states, transitions, or invariants.
- This phase does not specify ledger behavior or settlement mechanics.
- This phase does not define cryptographic constructions or signature formats.
- This phase does not mandate wallet features, APIs, or implementations.
- This phase does not define economic models, licensing enforcement, or fees.

## Authority Boundaries

This phase preserves strict authority separation:

- The XSTRP core remains the sole authority over intent semantics and validation.
- Wallets remain the sole authority over keys, signing, and transaction submission.
- User interfaces may display, relay, or request confirmation but hold no authority.
- No component introduced in this phase may override or reinterpret core outcomes.

## Wallet Interaction Model (Abstract)

Wallet and interface interactions under Phase C6-B are observational and confirmatory
only. Interfaces may display intent information and request explicit human consent but
must not infer, predict, or enforce protocol outcomes. All protocol decisions remain
deterministic and independent of interface behavior.

## Human Confirmation Requirements

This phase governs the requirement that any action leading toward completion of an
XSTRP intent must be preceded by explicit, informed human confirmation. Confirmation
must be intentional, unambiguous, and attributable to the correct participant, without
introducing timing assumptions or implicit consent.

## Hardware Wallet Considerations

Hardware wallets are treated as extensions of the signing authority, not as protocol
participants. This phase may describe compatibility expectations and safe handoff
principles but does not impose requirements or assume capabilities beyond secure
key custody and explicit user approval.

## Failure & Abort Handling (UX Perspective)

User-facing surfaces must represent failure, expiration, and abort conditions clearly
and without ambiguity. Interfaces may not suppress, delay, or reinterpret failure
signals, and must not present partial or speculative outcomes as final.

## Security & Threat Considerations (UX-Limited)

This phase considers threats arising from misleading presentation, incomplete
information, coercive interaction patterns, or user confusion. It does not address
network, cryptographic, or ledger-level threats, which remain out of scope.

## Out of Scope by Design

The following are explicitly out of scope for Phase C6-B:

- Ledger binding or transaction construction
- Cryptographic proof realization
- Economic or fee enforcement mechanisms
- Custodial models or delegated authority
- Automated or implicit consent mechanisms

## Deliverables

Phase C6-B may produce:

- Governance documentation for wallet and UX interaction boundaries
- Conformance criteria for interfaces claiming XSTRP compatibility
- Non-normative guidance for safe presentation of XSTRP intents

No implementation is required.

## Exit Criteria

Phase C6-B is complete when:

- Wallet and UX boundaries are clearly documented
- Authority separation is preserved without ambiguity
- No changes to Phases C1–C5 are required
- Remaining concerns are cleanly deferrable to other phases

## Non-Authorization Clause

This charter does not authorize implementation, deployment,
integration, or modification of the XSTRP core.

## Acceptance

This charter becomes active only upon explicit acceptance
and repository commit.
