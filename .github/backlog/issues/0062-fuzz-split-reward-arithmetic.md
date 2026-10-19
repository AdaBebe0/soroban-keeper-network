---
title: "test(fuzz): fuzz split_reward at extreme fee_bps and reward magnitudes"
labels: [testing, contract, good-first-issue]
epic: E03
wave: 2
depends_on: [0051]
---

## Summary

`split_reward` does `reward.checked_mul(fee_bps as i128).expect("overflow").checked_div(10_000).expect("div zero")`. The `.expect()` calls mean an overflow panics the transaction rather than returning a typed error. This is a narrow, fast fuzz target specifically for that function in isolation — smaller in scope than the full property test in 0057, and a good starting point for a first-time fuzzing contributor.

## Expected behaviour

A fuzz target that calls `split_reward(reward, fee_bps)` with `reward: i128` and `fee_bps: u32` drawn from the full type range (not just the contract's currently-validated range — the point is to find out what happens if this function is ever called from a new code path that doesn't pre-validate its inputs the way `execute_task` does today) and determines: does any input in the *full* `i128 × u32` space actually panic?

## Why this matters even though callers today validate their inputs

`fee_bps` is bounded to `[0, 10_000]` by `set_fee_bps`/`initialize` today, and `reward` is bounded to be positive by `register_task`. But `split_reward` itself has no such guard — it's a free function, callable with anything. If a future refactor adds a new caller (batch operations in epic E05 are a plausible one) that doesn't go through the same validated path, this is the function that would panic in production. Knowing today exactly where the overflow boundary sits means that future caller can be reviewed against it.

## Acceptance criteria

- [ ] Fuzz target covers the full `i128`/`u32` input space, not just validated inputs.
- [ ] The exact overflow boundary (largest `reward` × `fee_bps` combination that doesn't panic) is documented in a comment on `split_reward` itself.
- [ ] If the fuzz run finds a panic within the range callers can actually reach today, that's a real bug — file it separately and reference it here rather than silently fixing it as a drive-by.

## Files

- `fuzz/fuzz_targets/split_reward.rs`
- `contracts/keeper-registry/src/lib.rs` (doc comment only)
