---
title: "chore(dx): a devcontainer with Rust, the wasm target, Stellar CLI and Node"
labels: [tooling, intermediate]
epic: E21
wave: 5
depends_on: []
---

## Summary

New contributors install Rust, `wasm32-unknown-unknown`, the Stellar CLI, Node 20 and Postgres by hand. A devcontainer gets them to a passing `make ci` in one step.

## Acceptance criteria

- [ ] Opening the repo in a devcontainer (or Codespaces) gives a shell where `make ci` passes.
- [ ] Tool versions match CI.
- [ ] CONTRIBUTING.md mentions it as an option, not a requirement.

## Files

- .devcontainer/devcontainer.json
- .devcontainer/Dockerfile
- CONTRIBUTING.md
