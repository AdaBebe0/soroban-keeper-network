---
title: "chore(dx): optional pre-commit hooks for fmt and lint"
labels: [tooling, good-first-issue]
epic: E21
wave: 5
depends_on: []
---

## Summary

Format and lint failures are the most common red CI checks on PRs. Provide an opt-in hook that runs `cargo fmt --check` and the relevant ESLint on staged files.

## Acceptance criteria

- [ ] `make hooks` installs the hook; nothing installs it automatically.
- [ ] The hook only checks packages with staged changes, so it stays fast.
- [ ] CONTRIBUTING.md documents how to install and bypass it.

## Files

- scripts/pre-commit
- Makefile
- CONTRIBUTING.md
