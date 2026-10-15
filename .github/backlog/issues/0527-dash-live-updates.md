---
title: "feat(dashboard): live updates over the indexer WebSocket"
labels: [dashboard, enhancement, intermediate]
epic: E17
wave: 5
depends_on: [0525]
---

## Summary

Subscribe to `/v1/stream` so task lists and detail pages update without refresh, using packages/indexer-client's WebSocket client.

## Acceptance criteria

- [ ] New and changed tasks appear without reload.
- [ ] Disconnects reconnect with backoff and show a stale-data notice.

## Files

- apps/dashboard/src/hooks/useLiveEvents.ts
