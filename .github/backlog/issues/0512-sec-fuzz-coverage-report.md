---
title: "test(security): fuzz coverage report"
labels: [security, testing, intermediate]
epic: E19
wave: 5
depends_on: []
---

## Summary

Fuzzing runs nightly but nobody knows which code it reaches. Produce a coverage report from the fuzz corpus and list unreached entry points.

## Acceptance criteria

- [ ] The nightly workflow uploads a coverage report.
- [ ] Unreached entry points are listed in docs/FUZZING.md with a plan or a reason.

## Files

- .github/workflows/fuzz-nightly.yml
- docs/FUZZING.md
