---
title: "feat(dashboard): register-task form"
labels: [dashboard, enhancement, advanced]
epic: E17
wave: 5
depends_on: [0524]
---

## Summary

Let an owner register a task from the browser: target, function, arguments, reward, deadline and optional verifier, signed by the wallet.

## Acceptance criteria

- [ ] Inputs are validated with the SDK's rules before signing.
- [ ] The fee is shown before the wallet prompt, and the new task ID after.

## Files

- apps/dashboard/src/pages/RegisterTask.tsx
