---
title: "feat(cli): configuration file and named network profiles"
labels: [cli, enhancement, intermediate]
epic: E16
wave: 5
depends_on: [0451]
---

## Summary

Typing the RPC URL, passphrase and contract ID on every command is error-prone. Add a config file with named profiles and a `--profile` flag, seeded from the SDK's `NETWORK_PRESETS`.

## Acceptance criteria

- [ ] `skn config init` writes a config with testnet, futurenet and mainnet profiles.
- [ ] Flags override the profile, which overrides defaults, and `skn config show` prints the effective values.
- [ ] Mainnet commands that send transactions require an explicit confirmation flag.

## Files

- packages/cli/src/config.ts
- packages/cli/src/commands/config.ts
