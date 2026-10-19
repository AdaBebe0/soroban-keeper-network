---
title: "feat(registry): emit an event when a verifier is attached at registration or via update_verifier"
labels: [contract, enhancement, good-first-issue]
epic: E04
wave: 2
depends_on: [0073, 0081]
---

## Summary

Following this project's established convention (every state-relevant fact is emitted as an event for off-chain indexers and keeper bots, not just logged), attaching a verifier — whether at registration (0073) or via a later update (0081) — should be observable without requiring a keeper bot to `get_task` and diff every task on every poll.

## Expected behaviour

Extend `emit_task_registered` (or add a distinct topic) to include the verifier address when one is attached at registration, and have `update_verifier` (0081) emit its own event carrying the old and new verifier values, following the `emit_fee_updated`-style before/after pattern already used elsewhere in `lib.rs`.

## Acceptance criteria

- [ ] Registration event reflects whether a verifier was attached (and which address), without breaking existing event-topic consumers that don't care about verifiers — confirm whether this needs to be an additive field on the existing event or a genuinely separate event, and justify the choice.
- [ ] `update_verifier` event follows the before/after pattern.
- [ ] README event table updated.
- [ ] Test asserts both events fire with correct data.

## Files

- `contracts/keeper-registry/src/lib.rs`
- `contracts/keeper-registry/src/test.rs`
- `README.md`
