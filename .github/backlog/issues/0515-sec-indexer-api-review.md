---
title: "security(indexer): review API authentication and admin routes"
labels: [security, indexer, intermediate]
epic: E19
wave: 5
depends_on: []
---

## Summary

The indexer exposes `/admin/config` behind src/api/auth.rs and a rate limiter. Review auth, rate limiting, input validation and WebSocket resource limits.

## Acceptance criteria

- [ ] Every route is listed with its auth and limits.
- [ ] WebSocket connection and subscription limits are tested.

## Files

- indexer/src/api/auth.rs
- indexer/src/api/websocket.rs
- docs/INDEXER_SECURITY_REVIEW.md
