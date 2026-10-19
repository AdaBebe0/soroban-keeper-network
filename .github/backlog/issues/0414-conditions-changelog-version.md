---
title: "chore(release): CHANGELOG entry and VERSION bump for task conditions"
labels: [contract, tooling, good-first-issue]
epic: E10
wave: 4
depends_on: []
---

## Summary

Every previous contract feature closed with a CHANGELOG entry and a `VERSION` bump (verifier, batch registration, staking, reputation). Conditions change `Task` and `register_task`, so integrators need the same signal.

## Acceptance criteria

- [ ] CHANGELOG.md has an entry naming the new parameter, field and event.
- [ ] The contract `VERSION` constant is bumped, and `SUPPORTED_CONTRACT_VERSIONS` in both SDKs is updated in the same PR.

## Files

- CHANGELOG.md
- contracts/keeper-registry/src/constants.rs
- packages/sdk-ts/src/constants.ts
- rust-sdk/src/lib.rs
