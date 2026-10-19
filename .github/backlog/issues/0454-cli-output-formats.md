---
title: "feat(cli): --json output and stable exit codes"
labels: [cli, enhancement, good-first-issue]
epic: E16
wave: 5
depends_on: [0451]
---

## Summary

Scripts need machine-readable output. Every command supports `--json`, bigints are serialized as strings, and exit codes follow the design document.

## Acceptance criteria

- [ ] Every command's `--json` output is valid JSON with bigint fields as decimal strings.
- [ ] Contract errors exit with a code distinct from usage errors and RPC errors.
- [ ] README documents the exit codes.

## Files

- packages/cli/src/output.ts
- packages/cli/README.md
