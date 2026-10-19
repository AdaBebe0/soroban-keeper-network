---
title: "test(registry): execute_task with a verifier that always approves"
labels: [testing, contract, good-first-issue]
epic: E04
wave: 2
depends_on: [0074]
---

## Summary

The first of four focused test issues (this, 0084, 0085, 0086) covering `execute_task`'s interaction with a verifier under specific behaviors. This one is the straightforward happy path, split out as its own issue so it can be picked up independently and to keep 0074's own PR from having to carry the full test matrix alone.

## Expected behaviour

A minimal test-only verifier contract (in `contracts/keeper-registry/src/test.rs`, following the `mod reentrant_token { ... }` pattern already established there by wave 1's CEI fix PRs) whose `verify` always returns `true`. A test registers a task with this verifier attached, claims and executes it, and asserts the reward is credited exactly as it would be with no verifier at all.

## Acceptance criteria

- [ ] Verifier module added to `test.rs` in the established local-mock-contract style.
- [ ] Test confirms identical outcome (keeper balance, fees accrued, task status, emitted events) to the no-verifier path, modulo the additional verification-related event if 0080's event fires on success too (confirm whether it should — probably not, per 0080's scoping to failures only, but check).

## Files

- `contracts/keeper-registry/src/test.rs`
