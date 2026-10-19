---
title: "refactor(test): extract a shared invariant-checker module used by both property tests and fuzz targets"
labels: [testing, intermediate]
epic: E03
wave: 2
depends_on: [0054, 0057, 0058, 0059, 0060]
---

## Summary

By the time issues 0054–0060 land, the same checks (solvency, fee bounding, escrow isolation, etc.) will exist in `contracts/keeper-registry/src/test.rs`. Some of the fuzz targets (0053 in particular) want to assert a subset of the same properties inline. This issue extracts each invariant assertion into a small, named, reusable function so both the property tests and the fuzz targets call the same code rather than maintaining parallel copies that can drift apart.

## Expected behaviour

A module (e.g. `contracts/keeper-registry/src/invariants.rs`, `#[cfg(any(test, fuzzing))]`) exposing one function per invariant from issue 0050 — `assert_solvent(&env, &registry, &token) -> Result<(), String>`, `assert_fee_bounded(reward, fee_bps, keeper_net, fee) -> Result<(), String>`, and so on — each returning a descriptive error rather than panicking directly, so callers can decide whether to `panic!`, `assert!`, or accumulate failures.

## Suggested approach

This is a refactor, not new test coverage — it should be a no-op change in what's tested, just where the assertion logic lives. Do this *after* 0054–0060 land with their own inline assertions, so the extraction is grounded in real, working code rather than speculative design.

## Acceptance criteria

- [ ] One function per invariant, named after its `I-N` identifier from `docs/ARCHITECTURE.md`.
- [ ] Every property test from 0054–0060 is refactored to call the shared function instead of its own inline assertion.
- [ ] The fuzz target from 0053 uses the same `assert_fee_bounded` rather than a separate copy.
- [ ] No behavior change — all existing tests still pass, asserting the same things they did before the refactor.

## Files

- `contracts/keeper-registry/src/invariants.rs`
- `contracts/keeper-registry/src/test.rs`
- `fuzz/fuzz_targets/execute_task.rs`
