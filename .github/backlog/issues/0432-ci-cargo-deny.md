---
title: "chore(ci): add cargo-deny for licenses, bans and advisories"
labels: [tooling, security, intermediate]
epic: E21
wave: 5
depends_on: []
---

## Summary

The `audit` job checks advisories only. Add a `deny.toml` and a `cargo deny check` job so disallowed licenses, duplicate major versions and yanked crates also surface.

## Acceptance criteria

- [ ] `deny.toml` allows the licenses the workspace currently uses and nothing broader.
- [ ] A CI job runs `cargo deny check` (advisory at first, like `audit`).
- [ ] docs/CI.md lists the job.

## Files

- deny.toml
- .github/workflows/ci.yml
- docs/CI.md
