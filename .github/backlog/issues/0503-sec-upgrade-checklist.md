---
title: "docs(security): contract upgrade safety checklist"
labels: [security, docs, intermediate]
epic: E19
wave: 5
depends_on: []
---

## Summary

`upgrade` replaces the WASM in place. Write the checklist an admin follows: storage compatibility, version bump, testnet rehearsal, announcement and rollback plan.

## Acceptance criteria

- [ ] The checklist is ordered and each step has a verification.
- [ ] It covers storage-schema compatibility with a test reference.

## Files

- docs/UPGRADE_CHECKLIST.md
