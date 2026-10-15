---
title: "feat(oracle): price-threshold condition backed by Reflector"
labels: [oracle, contract, advanced]
epic: E11
wave: 4
depends_on: [0417, 0418]
---

## Summary

Issue 0397's reference price-threshold condition reads a price from a stand-in source. This issue backs it with the real Reflector client: a task becomes claimable when an asset's price crosses an owner-chosen threshold.

## Acceptance criteria

- [ ] Above and below directions are both supported.
- [ ] Stale prices are handled by the rule chosen in issue 0416, with a test at the exact staleness boundary.
- [ ] Thresholds are compared after the normalization rule from the design document, with tests for feeds of different decimals.

## Files

- contracts/conditions/oracle-price-threshold/src/lib.rs
- contracts/conditions/oracle-price-threshold/src/test.rs
