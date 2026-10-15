---
title: "feat(cli): skn task register"
labels: [cli, enhancement, intermediate]
epic: E16
wave: 5
depends_on: [0452, 0453]
---

## Summary

Register a task from flags or a JSON file: target contract, function, arguments, reward, deadline and optional verifier.

## Acceptance criteria

- [ ] Deadlines accept ISO timestamps and relative durations (for example `+2h`).
- [ ] The command simulates first and shows the fee before submitting; `--yes` skips the prompt.
- [ ] It prints the new task ID.

## Files

- packages/cli/src/commands/task/register.ts
