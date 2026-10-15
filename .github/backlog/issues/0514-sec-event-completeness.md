---
title: "security(registry): every state change emits an event"
labels: [security, contract, intermediate]
epic: E19
wave: 5
depends_on: []
---

## Summary

Off-chain consumers (indexer, bots) rely on events to see state. Verify that every write path emits an event and that event payloads carry enough to reconstruct the change.

## Acceptance criteria

- [ ] Each write path is mapped to the event it emits.
- [ ] A test fails if a write path emits no event.

## Files

- contracts/keeper-registry/src/events.rs
- docs/EVENTS.md
