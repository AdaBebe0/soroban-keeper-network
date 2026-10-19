---
title: "feat(cli): cancel, expire, increase-reward and extend-deadline"
labels: [cli, enhancement, intermediate]
epic: E16
wave: 5
depends_on: [0456]
---

## Summary

Cover the remaining task lifecycle entry points so an owner can manage a task without writing code.

## Acceptance criteria

- [ ] Each subcommand maps to the matching SDK method and prints the resulting task state.
- [ ] `cancel` explains when a claimed task can be cancelled (lock lapsed) if the call is refused.

## Files

- packages/cli/src/commands/task/
