---
title: "feat(rust-sdk): a polling event stream over getEvents"
labels: [rust-sdk, enhancement, intermediate]
epic: E13
wave: 3
depends_on: []
---

## Summary

`rust-sdk/src/events.rs` decodes individual events, but every consumer still hand-writes the `getEvents` cursor loop. Add an async stream that pages `getEvents` for the registry contract and yields decoded events, carrying the cursor so a consumer can resume after a restart.

## Acceptance criteria

- [ ] The stream yields typed events from `events.rs` and surfaces undecodable events as an error item rather than silently dropping them.
- [ ] The caller can start from a ledger or from a saved cursor, and can read the current cursor at any point.
- [ ] Transient RPC failures go through the crate's existing `RetryPolicy` instead of a second retry implementation.
- [ ] Tests drive the stream against a fake RPC that returns multiple pages, including an empty page.

## Files

- rust-sdk/src/events.rs
- rust-sdk/src/stream.rs
- rust-sdk/src/lib.rs
