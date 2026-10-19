---
title: "test(registry): confirm the full existing test suite passes unmodified for verifier-less tasks"
labels: [testing, contract, good-first-issue]
epic: E04
wave: 2
depends_on: [0074]
---

## Summary

The whole epic's risk profile hinges on not regressing the MVP behavior that exists today for tasks that don't opt into verification. This issue is a deliberate, explicit checkpoint: after 0074 lands, does every test written before this epic (wave 1's full suite) still pass with zero modification beyond the mechanical `register_task` call-site arity change from 0073?

## Expected behaviour

Run the pre-epic test suite (everything in `test.rs` as of the end of wave 1) against the post-0074 contract, changing only what 0073 mechanically requires (the extra `None` argument to `register_task` calls). No assertion, no expected value, no test structure should need to change.

## Suggested approach

This is best done as a literal diff review: check out `test.rs` as it existed right before 0072 started, and compare against its state after 0074, filtering out only the mechanical arity changes. Anything else that changed is worth asking whether it should have.

## Acceptance criteria

- [ ] Every pre-epic test still exists and still passes.
- [ ] The only diffs in existing tests are the mechanical `None` verifier argument additions.
- [ ] If any pre-existing test needed a *behavioral* change (not just the arity mechanical one) to keep passing, that's a regression — stop and fix the regression rather than adjusting the test to match new behavior.

## Files

- `contracts/keeper-registry/src/test.rs`
