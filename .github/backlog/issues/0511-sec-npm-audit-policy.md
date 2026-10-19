---
title: "chore(security): npm audit policy for the TypeScript packages"
labels: [security, tooling, good-first-issue]
epic: E19
wave: 5
depends_on: []
---

## Summary

Rust has an advisory `audit` job; the npm packages have none. Add an advisory `npm audit --omit=dev` per package with a documented triage policy.

## Acceptance criteria

- [ ] Each package is audited in CI as an advisory job.
- [ ] docs/CI.md states how findings are triaged.

## Files

- .github/workflows/ci.yml
- docs/CI.md
