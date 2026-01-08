# Phase C6-B Wallet Conformance Criteria — XSTRP

## Status
NON-NORMATIVE — GOVERNANCE ONLY

## Purpose

This document defines non-normative conformance criteria for wallet and
user interface surfaces that claim compatibility with XSTRP, without
imposing implementation requirements or assuming authority over keys,
signing, or transaction submission.

## Conformance Scope

These criteria apply only to how wallets and interfaces present,
confirm, and relay XSTRP intent information to human participants.

They do not apply to ledger interaction, cryptographic validation, or
economic enforcement.

## Minimum Conformance Expectations

A wallet or interface claiming XSTRP compatibility should:

- Surface all relevant intent information accurately and without omission.
- Require explicit, intentional human confirmation before any signing action.
- Avoid implicit, automatic, or time-based consent mechanisms.
- Clearly represent failure, expiration, and abort conditions.
- Preserve user agency by allowing safe cancellation prior to signing.

## Prohibited Behaviors

A wallet or interface claiming XSTRP compatibility must not:

- Infer or predict protocol outcomes.
- Auto-approve, batch, or suppress confirmations.
- Modify, reinterpret, or obscure intent semantics.
- Represent unsigned or unverified states as final.
- Assume authority beyond presentation and confirmation.

## Hardware Wallet Alignment

When used with hardware wallets, interfaces should:

- Treat hardware devices as the final authority on signing approval.
- Avoid bypassing or abstracting explicit device confirmation.
- Preserve clear user understanding of what is being signed.

## Failure Transparency

Interfaces should present protocol failure states truthfully and without
ambiguity, including refund eligibility and non-completion outcomes.

Partial or speculative states must not be represented as success.

## Non-Authorization Clause

This document does not authorize implementation, certification,
endorsement, or enforcement.

It exists solely to define governance expectations.
