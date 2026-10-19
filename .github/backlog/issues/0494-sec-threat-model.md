---
title: "security: system-wide threat model"
labels: [security, docs, advanced]
epic: E19
wave: 5
depends_on: []
---

## Summary

Each epic has its own security review (staking, reputation, treasury, indexer, keeper-bot-v2), but there is no single threat model covering the system: actors (admin, owner, keeper, verifier, condition, oracle, indexer operator), assets, trust boundaries and attack paths.

## Acceptance criteria

- [ ] The document lists actors, assets and trust boundaries with a diagram.
- [ ] Each attack path names its mitigation or is marked accepted with a reason.
- [ ] It links to every per-epic security review.

## Files

- docs/THREAT_MODEL.md
