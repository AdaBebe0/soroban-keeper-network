---
title: "feat(examples): lending liquidation keeper in TypeScript"
labels: [keeper-bot, enhancement, advanced]
epic: E22
wave: 5
depends_on: []
---

## Summary

rust-sdk/examples/liquidation-keeper shows liquidations in Rust. Add the TypeScript counterpart on @soroban-keeper-network/sdk against a mock lending contract.

## Acceptance criteria

- [ ] The example detects an under-collateralized position and executes the liquidation task.
- [ ] It shares the mock lending contract with the Rust example.

## Files

- examples/liquidation-keeper-ts/
