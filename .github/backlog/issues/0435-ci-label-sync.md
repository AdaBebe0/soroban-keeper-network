---
title: "chore(ci): sync repository labels from .github/labels.yml"
labels: [tooling, good-first-issue]
epic: E21
wave: 5
depends_on: []
---

## Summary

.github/labels.yml says it should be applied "with a label-sync action or by hand", and it has drifted: labels such as `indexer`, `ts-sdk` and `rust-sdk` exist on GitHub but not in the file. Add a workflow that syncs labels on changes to the file, and bring the file up to date first.

## Acceptance criteria

- [ ] labels.yml lists every label the backlog issues use.
- [ ] A workflow applies the file on push to `main` when it changes, without deleting unlisted labels.
- [ ] push.sh's README section mentions that labels come from this file.

## Files

- .github/labels.yml
- .github/workflows/labels.yml
- .github/backlog/README.md
