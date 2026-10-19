---
title: "feat(examples): TTL-extension task for a dApp's own storage"
labels: [enhancement, intermediate]
epic: E22
wave: 5
depends_on: [0554]
---

## Summary

The most common keeper job on Soroban: keep a contract's storage alive. Ship a small dApp contract and a script that registers a recurring TTL-extension task for it.

## Acceptance criteria

- [ ] The example contract's storage stays live past its natural TTL in a test.
- [ ] The README explains the cost model for the owner.

## Files

- examples/ttl-extension/
