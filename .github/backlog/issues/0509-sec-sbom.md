---
title: "chore(security): generate SBOMs for releases"
labels: [security, tooling, intermediate]
epic: E19
wave: 5
depends_on: []
---

## Summary

Attach CycloneDX SBOMs for the Rust workspace and each npm package to GitHub releases.

## Acceptance criteria

- [ ] release.yml produces and attaches SBOMs.
- [ ] The SBOMs list exact resolved versions from the lockfiles.

## Files

- .github/workflows/release.yml
