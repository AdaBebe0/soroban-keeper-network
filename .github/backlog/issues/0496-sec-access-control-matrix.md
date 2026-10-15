---
title: "docs(security): access-control matrix for every entry point"
labels: [security, contract, good-first-issue]
epic: E19
wave: 5
depends_on: []
---

## Summary

List every public entry point of the registry and treasury with who must authorize it, whether it works while paused, and which checks run before any write.

## Acceptance criteria

- [ ] The matrix covers every `pub fn` in the contract impl blocks.
- [ ] A test or script fails when a new entry point is added without a matrix row.

## Files

- docs/ACCESS_CONTROL.md
