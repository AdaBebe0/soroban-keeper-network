---
title: "design(oracle): oracle-backed task conditions architecture"
labels: [oracle, contract, advanced]
epic: E11
wave: 4
depends_on: []
---

## Summary

Epic E11 lets a task's condition read an on-chain price feed. Before any code, decide which oracles are supported (Reflector first; Band if it has a Soroban deployment), how prices are normalized, how stale or missing data is treated, and whether oracle conditions are separate contracts or configurations of one contract.

## Acceptance criteria

- [ ] The document picks a supported-oracle list with a reason for each inclusion or exclusion.
- [ ] It defines a staleness rule and states whether stale data fails closed (claim refused) or open.
- [ ] It fixes the decimals-normalization rule so every later issue implements the same arithmetic.
- [ ] It states how oracle conditions relate to the condition interface from issue 0390.

## Files

- docs/ORACLE_DESIGN.md
