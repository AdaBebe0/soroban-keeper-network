---
title: "chore(ci): build, lint and test the CLI in CI"
labels: [cli, tooling, good-first-issue]
epic: E16
wave: 5
depends_on: [0468]
---

## Summary

Add a required CI job for the CLI, path-guarded the same way as the bot jobs, and a matching Makefile target.

## Acceptance criteria

- [ ] The job runs on PRs touching packages/cli or packages/sdk-ts.
- [ ] `make ci` includes the CLI; docs/CI.md lists the job.

## Files

- .github/workflows/ci.yml
- Makefile
- docs/CI.md
