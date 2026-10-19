---
title: "feat(ts-sdk): helpers to build oracle condition parameters"
labels: [ts-sdk, oracle, intermediate]
epic: E11
wave: 4
depends_on: [0419]
---

## Summary

Registering an oracle-conditioned task means encoding asset identifiers, thresholds and directions as ScVals. Add typed helpers so dApps do not hand-encode them.

## Acceptance criteria

- [ ] Helpers exist for the price-threshold and deviation conditions with typed parameters.
- [ ] Thresholds accept `bigint` and are normalized by the same rule the contract uses.
- [ ] Tests round-trip each helper's output through the contract's decoder via the existing fake RPC.

## Files

- packages/sdk-ts/src/conditions/oracle.ts
- packages/sdk-ts/src/index.ts
