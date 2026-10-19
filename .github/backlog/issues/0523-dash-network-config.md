---
title: "feat(dashboard): network selection and configuration"
labels: [dashboard, enhancement, good-first-issue]
epic: E17
wave: 5
depends_on: [0522]
---

## Summary

Select testnet or mainnet, with contract IDs and indexer URLs from a config file, and show the active network clearly.

## Acceptance criteria

- [ ] Switching networks updates every page's data.
- [ ] The active network is always visible; mainnet is visually distinct.

## Files

- apps/dashboard/src/config.ts
