---
title: "feat(indexer): optional OpenTelemetry trace export"
labels: [indexer, observability, advanced]
epic: E18
wave: 5
depends_on: [0477]
---

## Summary

Operators running a tracing backend should be able to export the indexer's spans via OTLP, off by default.

## Acceptance criteria

- [ ] Export is enabled by an environment variable and adds no overhead when disabled.
- [ ] indexer/README.md documents the configuration.

## Files

- indexer/src/main.rs
- indexer/Cargo.toml
- indexer/README.md
