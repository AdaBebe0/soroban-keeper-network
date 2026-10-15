---
title: "feat(rust-sdk): helpers to build oracle condition parameters"
labels: [rust-sdk, oracle, intermediate]
epic: E11
wave: 4
depends_on: [0419]
---

## Summary

Rust parity for issue 0424.

## Acceptance criteria

- [ ] Typed builders exist for the price-threshold and deviation conditions.
- [ ] Tests confirm the encoded values match the TypeScript helpers for the same input.

## Files

- rust-sdk/src/conditions.rs
- rust-sdk/src/lib.rs
