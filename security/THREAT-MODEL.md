# XSTRP Threat Model

## Status
Draft (Internal)

## Scope
This document outlines known and anticipated threat classes relevant to the
XRP Safe Transfer & Recovery Protocol (XSTRP), based on its reference architecture
and frozen RFC-XSTRP-0001.

## Assumptions
- Standard cryptographic primitives are secure
- Adversaries may control network timing, ordering, and partial delivery
- Adversaries may attempt replay, spoofing, or proof forgery
- No trusted third party is assumed

## Threat Classes
1. Replay attacks
2. Proof forgery or tampering
3. Partial network failure
4. Message reordering or delay
5. Sender or receiver key compromise
6. Denial-of-service during intent resolution

## Mitigations (High-Level)
- Explicit intent expiration
- Deterministic refund paths
- Proof verification bound to intent hash
- No permanent fund lock states

## Open Items
- Formal cryptographic audit
- Adversarial timing analysis
- Ledger integration edge cases
