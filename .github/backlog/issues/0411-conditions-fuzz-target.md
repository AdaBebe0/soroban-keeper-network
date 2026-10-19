---
title: "test(fuzz): a fuzz target for claims against conditioned tasks"
labels: [contract, testing, advanced]
epic: E10
wave: 4
depends_on: []
---

## Summary

The existing fuzz targets (fuzz/fuzz_targets/) predate conditions. Add a target whose condition contract returns arbitrary results, panics, or exhausts its budget, and assert the registry's invariants hold across all of them.

## Acceptance criteria

- [ ] The target covers true, false, panic and budget-exhaustion outcomes from the condition.
- [ ] It asserts that a rejected claim never changes task state or any balance.
- [ ] scripts/run-fuzz-targets.sh and docs/FUZZING.md include the new target.

## Files

- fuzz/fuzz_targets/conditioned_claim.rs
- fuzz/Cargo.toml
- scripts/run-fuzz-targets.sh
- docs/FUZZING.md
