---
title: "test(oracle): a mock price-feed contract for tests"
labels: [oracle, testing, good-first-issue]
epic: E11
wave: 4
depends_on: [0417]
---

## Summary

Tests for oracle conditions need a feed whose price, decimals and timestamp the test controls. Add a mock that implements the same interface as the client from issue 0417.

## Acceptance criteria

- [ ] A test can set price, decimals and timestamp per asset.
- [ ] The mock can be told to return no price for an asset, to exercise the missing-data path.
- [ ] It lives in a test-support crate and is never built into a release artifact.

## Files

- contracts/oracle-client/src/mock.rs
