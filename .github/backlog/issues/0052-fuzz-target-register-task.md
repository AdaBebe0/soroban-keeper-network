---
title: "test(fuzz): fuzz register_task's parameter space"
labels: [testing, contract, intermediate]
epic: E03
wave: 2
depends_on: [0051]
---

## Summary

`register_task` takes seven arguments — `reward`, `deadline`, `ttl_ledgers`, `lock_ledgers`, and arbitrary `calldata` bytes among them — and issue 0006 already bounds two of them. A fuzz target should throw arbitrary combinations at it and assert the contract never panics, and that every rejection is a typed `KeeperError`, never a host trap.

## Expected behaviour

A `fuzz_targets/register_task.rs` target that:
- Derives an arbitrary `(i128, u64, u32, u32, Vec<u8>)` tuple via `arbitrary`.
- Calls `register_task` with those values against a freshly initialized registry.
- Asserts the call either succeeds and the returned task is readable via `get_task`, or fails with one of the contract's own `KeeperError` variants — never a panic, never a host-level abort.

## Suggested approach

Reuse the `setup()` test harness pattern from `test.rs` (a mock token, an initialized registry) as a `fuzz_targets`-local helper — fuzz targets can't depend on `#[cfg(test)]` code directly, so lift the minimal setup into a small shared helper module under `fuzz/src/`.

Pay particular attention to `reward` and `calldata` length: `i128::MIN`/`i128::MAX` and a very large `Bytes` should be in the input space, since those are the values most likely to hit an unchecked arithmetic path.

## Acceptance criteria

- [ ] Target compiles and links against the real contract, not a stub.
- [ ] Running for at least 5 minutes locally with no crash.
- [ ] A shared `fuzz/src/support.rs` helper avoids duplicating the mock-token/init boilerplate across this and future targets (0053).
- [ ] Any crash found during development is fixed and its input added to `fuzz/corpus/register_task/` before this closes.

## Files

- `fuzz/fuzz_targets/register_task.rs`
- `fuzz/src/support.rs`
