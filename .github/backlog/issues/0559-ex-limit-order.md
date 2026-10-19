---
title: "feat(examples): oracle-driven limit order"
labels: [oracle, enhancement, advanced]
epic: E22
wave: 5
depends_on: [0554]
---

## Summary

Combine an oracle price condition (epic E11) with a swap call: a keeper executes a swap once a price is reached.

## Acceptance criteria

- [ ] The task uses the price-threshold condition, not an off-chain check.
- [ ] Slippage protection is enforced in the swap call.

## Files

- examples/limit-order/
