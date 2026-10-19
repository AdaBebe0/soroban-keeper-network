---
title: "test(fuzz): stand up a cargo-fuzz harness for keeper-registry"
labels: [testing, contract, intermediate]
epic: E03
wave: 2
depends_on: []
---

## Summary

Nothing in the repository fuzzes the contract today. Every test is a hand-written scenario. This issue is the scaffolding the rest of epic E03 builds on: a `fuzz/` crate wired to `cargo-fuzz`, runnable locally and in CI, with no fuzz targets yet beyond a smoke test.

## Why this first

Every other fuzzing issue in this wave (0052–0053, 0062–0064) needs somewhere to put its target. Standing up the harness in isolation, with a trivial target, means the plumbing (corpus directory, `Cargo.toml` wiring, `libfuzzer-sys` version pin) gets reviewed once instead of once per target.

## Expected behaviour

- A `fuzz/` directory at the workspace root, structured the way `cargo fuzz init` produces.
- `fuzz/Cargo.toml` depends on `keeper-registry` and `soroban-sdk` with `testutils`.
- One placeholder target (`fuzz/fuzz_targets/smoke.rs`) that constructs an `Env`, registers the contract, and calls `version()` — enough to prove the harness links and runs, not enough to find real bugs.
- `cargo fuzz run smoke -- -runs=1000` succeeds locally.

## Suggested approach

```
fuzz/
├── Cargo.toml
└── fuzz_targets/
    └── smoke.rs
```

`fuzz/Cargo.toml` should NOT be a workspace member with a resolver conflict — check `cargo fuzz`'s own guidance on excluding the fuzz crate from the parent workspace `[workspace] members`, since `libfuzzer-sys` pulls in nightly-only flags that shouldn't leak into the normal build.

## Acceptance criteria

- [ ] `cargo fuzz build` succeeds from a clean checkout.
- [ ] `cargo fuzz run smoke -- -runs=1000` passes.
- [ ] The fuzz crate is excluded from the default `cargo build`/`cargo test` workspace so it doesn't slow down every contributor's inner loop.
- [ ] `CONTRIBUTING.md` gets a short "Fuzzing" section pointing at this directory (full guide comes in 0070).
- [ ] `.gitignore` excludes `fuzz/corpus/` and `fuzz/artifacts/` (crash inputs get committed deliberately via 0069, not by accident).

## Files

- `fuzz/Cargo.toml`
- `fuzz/fuzz_targets/smoke.rs`
- `Cargo.toml` (workspace exclusion)
- `.gitignore`
- `CONTRIBUTING.md`
