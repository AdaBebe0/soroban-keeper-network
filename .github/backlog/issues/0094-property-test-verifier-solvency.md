---
title: "test(property): confirm verifier interaction never breaks the solvency invariant"
labels: [testing, contract, advanced]
epic: E04
wave: 2
depends_on: [0074, 0054]
---

## Summary

Epic E03 built a solvency property test (issue 0054) against the pre-verifier contract. This issue extends that property (or the shared model harness from 0061, if it's ready to be extended) to cover the new verifier-gated `execute_task` path, so the two epics' work is proven compatible rather than assumed to be.

## Expected behaviour

The solvency property from 0054 — `token.balance(&registry) == open escrow + keeper balances + accrued fees` — continues to hold across randomized sequences that include: tasks with no verifier, tasks with an always-approving verifier, and tasks with an always-rejecting verifier (using the same test-only verifier contracts from 0083/0084).

## Suggested approach

If 0061's model-checking harness has landed by the time this is picked up, extend its `Action` enum with verifier-attached task registration and reuse the existing solvency check rather than writing a parallel one. If not, a standalone extension of 0054's property is acceptable.

## Acceptance criteria

- [ ] Solvency property covers verifier-attached tasks in all three states above (none, approve, reject).
- [ ] A verifier rejection never causes any token movement (ties back to 0084's assertions, now generalized across random sequences rather than one fixed scenario).
- [ ] References both `I-1` (docs/ARCHITECTURE.md) and this epic's design doc.

## Files

- `contracts/keeper-registry/src/test.rs` or `contracts/keeper-registry/tests/model.rs`
