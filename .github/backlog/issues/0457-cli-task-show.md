---
title: "feat(cli): skn task show <id>"
labels: [cli, enhancement, good-first-issue]
epic: E16
wave: 5
depends_on: [0451]
---

## Summary

Print one task's fields with human formatting: status, reward in token units and stroops, deadline as both timestamp and time remaining, claimer and lock expiry.

## Acceptance criteria

- [ ] Unknown IDs exit with the not-found code and a clear message.
- [ ] `--json` returns the raw fields.

## Files

- packages/cli/src/commands/task/show.ts
