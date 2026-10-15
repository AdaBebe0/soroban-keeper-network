---
title: "chore(ci): coverage reports for the TypeScript packages"
labels: [tooling, testing, intermediate]
epic: E21
wave: 5
depends_on: []
---

## Summary

Rust coverage tooling exists (issue 0030); the TypeScript packages have none. Enable vitest coverage for sdk-ts, indexer-client and keeper-bot-v2 and publish the summary in the job summary.

## Acceptance criteria

- [ ] Each package's test run produces a coverage summary.
- [ ] The summary is written to `$GITHUB_STEP_SUMMARY`; no threshold blocks merges yet.

## Files

- packages/sdk-ts/vitest.config.ts
- packages/indexer-client/vitest.config.ts
- examples/keeper-bot-v2/vitest.config.ts
- .github/workflows/ci.yml
