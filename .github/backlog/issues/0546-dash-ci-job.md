---
title: "chore(ci): build, lint and test the dashboard"
labels: [dashboard, tooling, good-first-issue]
epic: E17
wave: 5
depends_on: [0544]
---

## Summary

Add a path-guarded CI job and a Makefile target for the dashboard.

## Acceptance criteria

- [ ] The job runs on changes to apps/dashboard, packages/sdk-ts or packages/indexer-client.
- [ ] docs/CI.md lists it.

## Files

- .github/workflows/ci.yml
- Makefile
- docs/CI.md
