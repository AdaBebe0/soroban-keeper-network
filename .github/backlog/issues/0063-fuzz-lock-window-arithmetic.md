---
title: "test(fuzz): fuzz lock_expired's u32 ledger arithmetic for overflow at the boundary"
labels: [testing, contract, good-first-issue]
epic: E03
wave: 2
depends_on: [0051]
---

## Summary

`lock_expired` computes `claimed_at.saturating_add(task.lock_ledgers)` — the `saturating_add` was a deliberate choice (see the doc comment added by wave 1) to avoid a panic on an enormous `lock_ledgers`. This issue fuzzes the function to confirm the saturating arithmetic actually behaves correctly at and around `u32::MAX`, across the full space of `(claim_ledger, lock_ledgers, current_ledger)`.

## Expected behaviour

A fuzz target (or, given the small input space, this may be better as an exhaustive property test over the boundary region rather than a genuine fuzz target — use judgement) that asserts:
- `lock_expired` never panics for any `u32` combination of `claimed_at`, `lock_ledgers`, and current ledger sequence.
- When `claimed_at.saturating_add(lock_ledgers)` saturates to `u32::MAX`, the lock is correctly reported as still active for any realistic current ledger sequence (i.e., saturation doesn't accidentally make an enormous lock window look expired).

## Suggested approach

Given wave 1 already added `MAX_LOCK_LEDGERS` (issue 0006), the *practically reachable* input space for `lock_ledgers` is now bounded — this fuzz target's value is in confirming the function is safe even for inputs outside that bound, since `lock_expired` itself has no such guard and is called with whatever was stored in `Task.lock_ledgers` at registration time, which could predate a future tightening of the bounds.

## Acceptance criteria

- [ ] Covers the saturation boundary explicitly (`claimed_at` near `u32::MAX`, large `lock_ledgers`).
- [ ] No panic found across the fuzzed space.
- [ ] Findings (or "no findings, boundary confirmed correct") noted in a comment on `lock_expired`.

## Files

- `fuzz/fuzz_targets/lock_expired.rs`
