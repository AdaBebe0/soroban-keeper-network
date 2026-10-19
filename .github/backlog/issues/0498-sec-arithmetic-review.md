---
title: "security(contracts): checked-arithmetic review of fee, reward and stake math"
labels: [security, contract, advanced]
epic: E19
wave: 5
depends_on: []
---

## Summary

Review every arithmetic operation on i128 amounts and basis points in the registry and treasury for overflow, rounding direction and dust.

## Acceptance criteria

- [ ] Every amount operation is listed with whether it is checked and its rounding direction.
- [ ] Rounding never favours the caller over the contract; any exception is justified.
- [ ] Findings are fixed with a test.

## Files

- contracts/keeper-registry/src/internal.rs
- contracts/treasury/src/distribution.rs
- docs/SECURITY_ARITHMETIC_REVIEW.md
