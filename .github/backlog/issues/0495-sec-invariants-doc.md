---
title: "docs(security): consolidated invariants reference"
labels: [security, contract, intermediate]
epic: E19
wave: 5
depends_on: []
---

## Summary

Invariants are spread across contracts/keeper-registry/src/invariants.rs, design documents and property tests. Gather them into one document, each linked to the code or test that enforces it.

## Acceptance criteria

- [ ] Every invariant in invariants.rs appears with a plain-language statement.
- [ ] Each invariant links to at least one test; any without a test is listed as a gap.

## Files

- docs/INVARIANTS.md
