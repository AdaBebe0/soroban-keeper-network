---
title: "feat(conditions): a reference time-window condition contract"
labels: [contract, enhancement, good-first-issue]
epic: E10
wave: 4
depends_on: []
---

## Summary

The only reference condition is the price threshold from issue 0397. Add the simplest useful second one: claimable only between two ledger timestamps. It doubles as a minimal template for condition authors.

## Acceptance criteria

- [ ] The contract exposes the condition interface defined in issue 0390 and nothing else.
- [ ] Tests cover before the window, inside it, after it, and the exact boundaries.
- [ ] The contract is built in CI alongside the other WASM targets.

## Files

- contracts/conditions/time-window/src/lib.rs
- contracts/conditions/time-window/src/test.rs
- Cargo.toml
