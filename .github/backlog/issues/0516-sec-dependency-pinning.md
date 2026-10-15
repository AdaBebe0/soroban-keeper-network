---
title: "chore(security): review dependency pinning and update cadence"
labels: [security, tooling, good-first-issue]
epic: E19
wave: 5
depends_on: []
---

## Summary

Decide where exact pins are required (soroban-sdk, stellar-sdk) and where ranges are fine, and document the update cadence Dependabot follows.

## Acceptance criteria

- [ ] Contract-critical dependencies are pinned exactly.
- [ ] The policy is written down in CONTRIBUTING.md.

## Files

- Cargo.toml
- .github/dependabot.yml
- CONTRIBUTING.md
