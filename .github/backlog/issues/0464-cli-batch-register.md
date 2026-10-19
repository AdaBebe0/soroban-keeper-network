---
title: "feat(cli): skn batch register (port examples/batch-register)"
labels: [cli, enhancement, intermediate]
epic: E16
wave: 5
depends_on: [0456]
---

## Summary

examples/batch-register is a standalone script that registers tasks from JSON or CSV. Port it into `skn batch register` so it shares config, keys and output handling, then point the example's README at the CLI.

## Acceptance criteria

- [ ] The command accepts the same JSON and CSV formats as the example.
- [ ] Batches larger than `max_batch_size` are split automatically and reported per chunk.
- [ ] `--dry-run` validates and prints without submitting.

## Files

- packages/cli/src/commands/batch/register.ts
- examples/batch-register/README.md
