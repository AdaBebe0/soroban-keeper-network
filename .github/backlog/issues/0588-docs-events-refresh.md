---
title: "docs: bring EVENTS.md up to date with every emitted event"
labels: [docs, good-first-issue]
epic: E20
wave: 5
depends_on: []
---

## Summary

contracts/keeper-registry/src/events.rs now emits staking, dispute, verifier and reputation events. Make docs/EVENTS.md list every one with exact topics and payload fields.

## Acceptance criteria

- [ ] Every `emit_*` function has an entry.
- [ ] Topics and fields match the code exactly.

## Files

- docs/EVENTS.md
