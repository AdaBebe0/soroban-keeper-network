---
title: "feat(cli): signing-key sources that never echo secrets"
labels: [cli, security, intermediate]
epic: E16
wave: 5
depends_on: [0451]
---

## Summary

Support keys from an environment variable, a file with restrictive permissions, and the Stellar CLI's identity store. A key must never appear in output, logs or error messages.

## Acceptance criteria

- [ ] All three sources work and their precedence is documented.
- [ ] A key file readable by other users is refused with a clear message.
- [ ] A test asserts that no command's output, including errors, contains the secret.

## Files

- packages/cli/src/keys.ts
- packages/cli/test/keys.test.ts
