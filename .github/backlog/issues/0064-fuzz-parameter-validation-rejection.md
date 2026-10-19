---
title: "test(fuzz): confirm register_task's rejection surface is exhaustive after issue 0006"
labels: [testing, contract, good-first-issue]
epic: E03
wave: 2
depends_on: [0051]
---

## Summary

Wave 1's issue 0006 added `MIN_LOCK_LEDGERS`/`MAX_LOCK_LEDGERS`/`MIN_TTL_LEDGERS` bounds to `register_task`. This issue fuzzes specifically the *rejection* paths — every input that should be rejected — to confirm there's no gap where an out-of-range value slips through, and that every rejection returns `InvalidTaskParams` rather than a different error or a panic.

## Expected behaviour

A fuzz target that generates `lock_ledgers` and `ttl_ledgers` values weighted toward the boundary (values just inside, just outside, and far outside the valid ranges) and asserts:
- Every value outside `[MIN_LOCK_LEDGERS, MAX_LOCK_LEDGERS]` is rejected.
- Every `ttl_ledgers` below `MIN_TTL_LEDGERS` is rejected.
- The rejection is always `KeeperError::InvalidTaskParams`, never a different variant, never a panic.
- Every value inside the valid ranges is accepted (assuming other parameters — `reward`, `deadline` — are also valid), confirming the bounds aren't accidentally too strict.

## Suggested approach

This is a good complement to 0052 (which fuzzes the full parameter tuple broadly) by focusing narrowly on just these two parameters with boundary-biased generation, which broad fuzzing is statistically unlikely to hit precisely on its own.

## Acceptance criteria

- [ ] Confirms no gap in the rejection range (no accepted value outside the documented bounds).
- [ ] Confirms no false rejection inside the documented bounds.
- [ ] Every rejection asserted to be `InvalidTaskParams` specifically.

## Files

- `fuzz/fuzz_targets/register_task_bounds.rs`
