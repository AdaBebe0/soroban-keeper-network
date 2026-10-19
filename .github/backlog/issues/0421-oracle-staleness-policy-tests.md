---
title: "test(oracle): staleness and missing-data behaviour across every oracle condition"
labels: [oracle, testing, intermediate]
epic: E11
wave: 4
depends_on: [0419, 0420]
---

## Summary

Each oracle condition implements the staleness rule separately. A shared table-driven test makes sure they all agree on stale, missing and future-dated prices.

## Acceptance criteria

- [ ] One test table runs against every oracle condition contract.
- [ ] Cases include a price exactly at the staleness limit, one ledger past it, and a missing asset.
- [ ] A future-dated timestamp is treated as invalid, not as fresh.

## Files

- contracts/conditions/tests/staleness.rs
