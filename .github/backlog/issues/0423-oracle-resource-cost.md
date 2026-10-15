---
title: "test(oracle): measure the resource cost of an oracle-backed claim"
labels: [oracle, testing, intermediate]
epic: E11
wave: 4
depends_on: [0419]
---

## Summary

A claim against an oracle-conditioned task adds a cross-contract call chain (registry → condition → oracle). Measure its CPU and memory cost against the budget guard from issue 0396 and the resource-cost baseline.

## Acceptance criteria

- [ ] The resource-cost report includes a claim against an oracle-conditioned task.
- [ ] The cost stays under the guard from issue 0396 with a stated margin, or the issue records why the guard must change.

## Files

- scripts/report-resource-cost.sh
- contracts/keeper-registry/src/test/perf.rs
