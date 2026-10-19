---
title: "security(registry): storage key collision and TTL review"
labels: [security, contract, intermediate]
epic: E19
wave: 5
depends_on: []
---

## Summary

Confirm no two `DataKey` variants can collide and every persistent and temporary entry has a TTL policy, including entries added by staking, reputation and disputes.

## Acceptance criteria

- [ ] Every storage key is listed with its storage type and TTL policy.
- [ ] Entries whose expiry would lose funds are identified and tested.

## Files

- contracts/keeper-registry/src/types.rs
- docs/STORAGE_LAYOUT.md
