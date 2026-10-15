---
title: "chore(ci): generate GitHub release notes from CHANGELOG.md"
labels: [tooling, intermediate]
epic: E21
wave: 5
depends_on: [0442]
---

## Summary

release.yml attaches WASM and hashes but the release body is empty. Extract the matching CHANGELOG.md section into the release notes.

## Acceptance criteria

- [ ] The release body is the CHANGELOG section for the tag's version.
- [ ] The workflow fails clearly if no section exists for that version.

## Files

- .github/workflows/release.yml
