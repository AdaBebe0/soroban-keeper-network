---
title: "test(ops): check metric names against the documented scheme"
labels: [observability, testing, good-first-issue]
epic: E18
wave: 5
depends_on: [0475, 0478]
---

## Summary

Names drift unless something checks them. Add a test per component that scrapes `/metrics` and compares names against docs/OBSERVABILITY.md.

## Acceptance criteria

- [ ] A renamed or undocumented metric fails the test.
- [ ] The check runs in the existing component CI jobs.

## Files

- indexer/tests/metrics_names.rs
- examples/keeper-bot-v2/test/metricsNames.test.ts
