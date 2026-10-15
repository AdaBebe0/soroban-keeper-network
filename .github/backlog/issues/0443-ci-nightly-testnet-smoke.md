---
title: "chore(ci): nightly smoke test against the testnet deployment"
labels: [tooling, testing, intermediate]
epic: E21
wave: 5
depends_on: []
---

## Summary

Nothing notices when the testnet deployment listed in docs/DEPLOYMENTS.md stops working (expired TTL, reset network, drained test account). A nightly job should call read-only views and open an issue if they fail.

## Acceptance criteria

- [ ] A scheduled workflow calls `version`, `is_paused` and `task_count` on the testnet registry.
- [ ] On failure it opens (or updates) one tracking issue instead of a new issue per night.

## Files

- .github/workflows/testnet-smoke.yml
- docs/DEPLOYMENTS.md
