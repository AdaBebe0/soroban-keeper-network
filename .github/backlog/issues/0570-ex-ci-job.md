---
title: "chore(ci): build and test every example in CI"
labels: [tooling, testing, intermediate]
epic: E22
wave: 5
depends_on: [0555]
---

## Summary

Examples rot when nothing builds them. Add a CI job that builds and tests each example contract and script.

## Acceptance criteria

- [ ] Each example has a test command the job runs.
- [ ] The job is path-guarded on examples/ and the SDKs.

## Files

- .github/workflows/ci.yml
- docs/CI.md
