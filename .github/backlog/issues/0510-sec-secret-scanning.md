---
title: "chore(security): secret scanning on pull requests"
labels: [security, tooling, good-first-issue]
epic: E19
wave: 5
depends_on: []
---

## Summary

Keeper examples handle secret keys and `.env` files. Add a secret scanner (gitleaks or GitHub secret scanning with push protection) to block committed secrets.

## Acceptance criteria

- [ ] The scanner runs on PRs and passes on the current history, with documented allowlist entries for the format-valid placeholders in `.env.example` files.
- [ ] CONTRIBUTING.md mentions it.

## Files

- .github/workflows/pr-hygiene.yml
- .gitleaks.toml
