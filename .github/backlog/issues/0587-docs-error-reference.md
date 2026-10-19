---
title: "docs: error code reference for every contract"
labels: [docs, good-first-issue]
epic: E20
wave: 5
depends_on: []
---

## Summary

List every error variant of the registry and treasury with its number, meaning, which entry points return it, and how a caller should react.

## Acceptance criteria

- [ ] Every variant in contracts/*/src/errors.rs is listed.
- [ ] A check fails when a variant is added without documentation.

## Files

- docs/ERRORS.md
