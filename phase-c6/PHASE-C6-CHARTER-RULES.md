# Phase C6 Charter Construction Rules — XSTRP

## Status
ACTIVE — GOVERNING

## Purpose

This document defines mandatory constraints on the construction of any
Phase C6 charter.

No Phase C6 charter may be proposed unless it conforms to these rules.

## Rule 1 — No Core Mutation

A Phase C6 charter must not propose, imply, or allow changes to any
artifacts, semantics, or guarantees defined in Phases C1–C5.

## Rule 2 — Additive Only

All Phase C6 work must be strictly additive.

Removal, replacement, reinterpretation, or shadowing of core behavior
is prohibited.

## Rule 3 — Authority Preservation

A Phase C6 charter must explicitly preserve authority separation:

- XSTRP Core: intent semantics only
- Wallets: keys and signing
- Ledgers: settlement
- Extensions: non-authoritative

## Rule 4 — Deterministic Failure Preservation

Any extension described under Phase C6 must fail closed and preserve
deterministic refund or non-completion behavior.

No extension may introduce stranded or ambiguous outcomes.

## Rule 5 — Optionality

All Phase C6 outputs must be optional.

The XSTRP core must remain correct, complete, and usable without any
Phase C6 extensions present.

## Rule 6 — Explicit Scope Declaration

Every Phase C6 charter must explicitly list:

- What it does
- What it does not do
- What it intentionally leaves unresolved

## Rule 7 — Single-Concern Charters

A Phase C6 charter must address exactly one concern domain.

Multi-domain charters are prohibited.

## Rule 8 — No Implicit Authorization

A Phase C6 charter does not authorize implementation, deployment, or
integration unless explicitly stated.

## Enforcement

Any charter violating these rules is invalid by definition.
