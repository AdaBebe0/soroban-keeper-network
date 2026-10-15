---
title: "test(oracle): testnet dry run against a live Reflector feed"
labels: [oracle, testing, intermediate]
epic: E11
wave: 4
depends_on: [0419]
---

## Summary

Unit tests use the mock feed. Before calling the epic done, register a price-threshold task on testnet against the live Reflector deployment and let a keeper claim it when the condition is met.

## Acceptance criteria

- [ ] The dry run records contract IDs, transactions and the observed price at claim time.
- [ ] Any difference from the mock's behaviour is filed as a follow-up issue.

## Files

- docs/ORACLE_GUIDE.md
