---
title: "chore(make): a make setup target that installs toolchains and dependencies"
labels: [tooling, good-first-issue]
epic: E21
wave: 5
depends_on: []
---

## Summary

`make ci` assumes the wasm target, Stellar CLI and every package's node_modules are present. Add `make setup` that installs them and prints what it could not install.

## Acceptance criteria

- [ ] `make setup` adds the wasm target and runs `npm ci` in every package.
- [ ] It reports missing system tools (Stellar CLI, Docker) instead of failing silently.
- [ ] `make help` lists it.

## Files

- Makefile
- CONTRIBUTING.md
