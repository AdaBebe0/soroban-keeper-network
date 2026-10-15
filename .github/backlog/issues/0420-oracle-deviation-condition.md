---
title: "feat(oracle): price-deviation condition (move of N basis points from a reference)"
labels: [oracle, contract, intermediate]
epic: E11
wave: 4
depends_on: [0419]
---

## Summary

Rebalancing and hedging keepers trigger on relative moves, not fixed prices. Add a condition that is met when the price has moved at least N basis points from a reference price recorded at registration.

## Acceptance criteria

- [ ] The reference price is captured once and cannot be changed by anyone after registration.
- [ ] Basis-point arithmetic uses checked operations and is tested at 0, 1 and 10000 bps.
- [ ] Both upward and downward moves are covered by tests.

## Files

- contracts/conditions/oracle-deviation/src/lib.rs
- contracts/conditions/oracle-deviation/src/test.rs
