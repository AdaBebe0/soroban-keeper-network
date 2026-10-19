---
title: "design(registry): decide permissionless vs admin-curated verifier addresses"
labels: [contract, security, advanced]
epic: E04
wave: 2
depends_on: [0071]
---

## Summary

0071 flagged this as an open trust-model question and deferred it here: should any address be attachable as a task's verifier (fully permissionless, consistent with the protocol's general design philosophy of not gatekeeping task registration), or should the registry maintain an admin-curated allow-list of vetted verifier contracts?

## The tension

Permissionless is consistent with everything else in the registry — anyone can register a task, anyone can be a keeper, no admin approval gates participation. But a verifier is different in kind from a task: it's code the *keeper* (not the owner attaching it) has to trust enough to claim against, and an unvetted verifier is exactly the griefing/DoS surface 0075/0082/0089 are all reasoning about. An allow-list would let the registry vouch for a known-safe set of verifier implementations (e.g. the reference ones from 0077–0079) while still allowing anyone to register a *task* against one of them.

## Suggested approach

Consider a middle ground: fully permissionless by default (any address, matching the rest of the protocol), but expose an on-chain "vetted verifier" registry — a separate, optional list an admin can curate — that keeper bots and dApp UIs can consult to warn users when a task's verifier isn't on it, without the base contract enforcing anything. This keeps `execute_task` itself simple (no admin gate in the hot path) while giving the ecosystem a trust signal.

## Expected output

A decision recorded in `docs/VERIFIER_DESIGN.md`, and if the allow-list approach is chosen, a scoped follow-up issue for implementing it (do not silently expand this issue's scope from "decide" to "also implement" without an explicit acceptance criterion saying so).

## Acceptance criteria

- [ ] The tension above is explicitly weighed, not skipped.
- [ ] A decision is recorded with rationale.
- [ ] If an allow-list is chosen, a new issue is filed for its implementation with its own acceptance criteria, rather than bolting it onto this one.

## Files

- `docs/VERIFIER_DESIGN.md`
