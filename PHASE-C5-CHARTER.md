PHASE-C5-CHARTER.md

Phase C5 — Extended Formal Verification

Status

AUTHORIZED — NOT YET STARTED

1. Purpose

Phase C5 authorizes a new, extended formal verification effort for XSTRP.

The purpose of this phase is to increase confidence in the already frozen protocol behavior by applying stronger, more exhaustive formal methods beyond the minimal sanity-check model previously completed.

This phase exists to deepen assurance, not to evolve protocol behavior.

2. Relationship to Prior Work

Phases C1–C4 are final and immutable.

Their specifications remain frozen.

Their Rust implementation remains authoritative.

Their existing minimal TLA+ model remains valid and closed.

Phase C5 does not replace or reinterpret earlier verification work.
It builds on it, under stricter boundaries and clearer claims.

3. Scope of Phase C5 Verification

Phase C5 MAY include:

A full TLA+ specification of the XSTRP state machine

Explicit modeling of:

Authorization gating

Proof presence/absence

Expiry behavior

Terminal state immutability

Exhaustive invariant definition and checking

Refinement mappings between:

Specification documents

Rust reference implementation

Formal model

Verification applies only to protocol logic as defined in Phases C1–C4.

4. Explicitly Out of Scope (Non-Negotiable)

Phase C5 MUST NOT model or assume:

XRPL behavior, consensus, or ledger semantics

Cryptographic primitives or signatures

Wallet behavior or UX

Networking, transport, or message ordering

Persistence or storage layers

Timing precision beyond abstract expiry

Performance, throughput, or scalability

Adversarial network conditions

These exclusions are intentional and permanent for this phase.

5. Authority and Freeze Discipline

Phase C5 is verification-only.

❌ No protocol behavior may be changed

❌ No Rust code may be altered to “satisfy” verification

❌ No new states or transitions may be introduced

If a discrepancy is found:

The resolution MUST be a specification clarification, or

A documented limitation of the model

Behavioral change requires a new phase charter (C6 or later).

6. Verification Methods

Permitted methods include:

TLA+ specification

TLC model checking

Invariant and safety property documentation

Explicit correspondence notes between:

Formal model

Rust types and transitions

Frozen specification text

No probabilistic, heuristic, or empirical methods are permitted.

7. Deliverables

Phase C5 deliverables may include:

/formal/ directory expansion

One or more TLA+ modules

A verification README describing:

Modeled assumptions

Proven invariants

Known limitations

Clear statements of what is proven and what is not

8. Success Criteria

Phase C5 is considered complete when:

All declared invariants hold in the formal model

All reachable behaviors correspond to frozen protocol semantics

No stranded-fund or non-deterministic outcomes are reachable

Results are documented clearly and conservatively

9. Exit Conditions

At completion, Phase C5 MUST end in one of the following states:

Verified within stated bounds

Verified with documented modeling limitations

Phase C5 does not authorize deployment, integration, or extension.

10. Charter Integrity

This charter authorizes only what is written here.

Anything not explicitly allowed is forbidden.
