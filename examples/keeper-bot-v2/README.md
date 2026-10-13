# Soroban Keeper Bot v2

**This is the operator-grade keeper bot.** It is aimed at teams running a keeper
competitively — persistent task state, concurrent rounds, multi-account
signing, adaptive fees, alerting and metrics — not at newcomers.

> **New to the Keeper Network?** Start with [`examples/keeper-bot`](../keeper-bot),
> a single-file, dependency-light bot kept deliberately beginner-friendly.

## Layout

The package has two halves, kept in separate directories:

| Directory | Language | What it is |
|---|---|---|
| `src/` | TypeScript (compiled to `dist/`) | The `KeeperBotV2` service and its library modules — the runnable bot (`npm start`). Tests: `src/*.test.ts`, `test/*.test.ts` (vitest). |
| `bot/` | CommonJS | Round-loop components: graceful shutdown, withdrawal strategies, lock-window scheduling, spend ceiling, prioritization and hot-reloadable config. Tests: `test/*.test.js` (`node --test`). |
| `benchmark/` | CommonJS | Latency and claim-race comparison against v1 — see [`benchmark/REPORT.md`](benchmark/REPORT.md). |

```
src/
├── index.ts          # KeeperBotV2 service and CLI entry point
├── config.ts         # Startup schema validation (validateConfig) and fee config
├── accounts.ts       # Signing-account pool: round-robin / least-loaded
├── fees.ts           # Adaptive fees from Soroban RPC fee stats, ceiling-bounded
├── profitability.ts  # Net profit = reward - (claim + execute + withdraw fees)
├── ranking.ts        # Rank candidates by profit, apply the per-round budget
├── loop.ts           # Candidate evaluation for a round
├── alerts.ts         # Alert rules, deduplication, webhook transport
├── metrics.ts        # Per-task-type metrics, Prometheus exposition
├── executors/        # Pluggable executor interface and registry
└── db/               # PostgreSQL client, schema and task-state store
```

## Quickstart: a dry-run instance

This starts a real `KeeperBotV2` process — configuration validated, metrics
server started, one round executed, clean shutdown — without a funded account,
a deployed contract or a database. The placeholders in `.env.example` are
format-valid, which is all startup validation checks.

```bash
cd examples/keeper-bot-v2
npm install
cp .env.example .env     # DATABASE_URL is left unset: in-memory state
npm run build
npm start -- --once
```

The output ends with `[KEEPER-BOT-V2] Stopped cleanly.` and exit code `0`. For a
real deployment, set a real `REGISTRY_CONTRACT_ID` and signing key, configure
`DATABASE_URL`, and drop `--once` to run continuously every `POLL_INTERVAL_MS`.

## Configuration

All configuration comes from environment variables. `src/config.ts`'s
`validateConfig` is the single source of truth: it checks every field and the
cross-field consistency (for example, concurrency against the signing-pool
size) and fails fast before any work starts. [`.env.example`](.env.example) is
a ready-to-copy template.

| Variable | Description | Default |
|---|---|---|
| `NETWORK` | `testnet`, `futurenet` or `mainnet` | `testnet` |
| `REGISTRY_CONTRACT_ID` | Deployed `KeeperRegistry` contract (`C...`) | required |
| `SECRET_BACKEND` | `env`, `vault` or `aws_secrets_manager` | `env` |
| `KEEPER_SECRET_KEY` | Single signing key (`S...`); required with `SECRET_BACKEND=env` unless `SIGNING_KEY_POOL` is set | — |
| `SIGNING_KEY_POOL` | Comma-separated signing keys for concurrent submission; supersedes `KEEPER_SECRET_KEY` | — |
| `KEEPER_POOL_STRATEGY` | `round-robin` or `least-loaded` | `round-robin` |
| `VAULT_ADDR` | Vault address, with `SECRET_BACKEND=vault` | — |
| `AWS_SECRET_NAME` | Secret name, with `SECRET_BACKEND=aws_secrets_manager` | — |
| `DATABASE_URL` | PostgreSQL connection string; omit for in-memory state | — |
| `MAX_CONCURRENT_TASKS` | Concurrency within a round; must not exceed the pool size | `1` |
| `MAX_TASKS_PER_ROUND` | Maximum tasks evaluated per round | `5` |
| `POLL_INTERVAL_MS` | Delay between rounds, ms (≥ `1000`) | `10000` |
| `MIN_PROFIT_MARGIN_STROOPS` | Minimum net profit before claiming | `0` |
| `FEE_CEILING_STROOPS` | Hard per-round fee ceiling | `1000000` |
| `FEE_PERCENTILE` | Adaptive fee target: `p10`, `p50`, `p90`, `p99` | `p90` |
| `FEE_MULTIPLIER` | Multiplier on the selected percentile | `1.1` |
| `WITHDRAW_THRESHOLD` | Balance (stroops) that triggers a withdrawal | `10000000` |
| `MAX_RETRIES` | Retries for transient RPC errors | `3` |
| `RETRY_BASE_MS` | Exponential backoff base delay, ms | `500` |
| `EXPIRE_STALE_TASKS` | Expire past-deadline tasks to refund creators | `true` |
| `SIMULATE_EXECUTION` | Development only: fabricate proofs for unsupported task types | `false` |
| `METRICS_ENABLED` | Serve the Prometheus `/metrics` endpoint | `true` |
| `METRICS_PORT` | Metrics port (`1`–`65535`) | `9090` |
| `RUN_ONCE` | Run one round then exit (same as `--once`) | `false` |

### Hot reloading (`bot/config.js`)

The CommonJS components re-read some values before every round, so a change
takes effect within one round without a restart: `POLL_INTERVAL_MS`,
`WITHDRAW_THRESHOLD`, `MAX_TASKS_PER_ROUND`, `MAX_RETRIES`, `RETRY_BASE_MS`,
`EXPIRE_STALE_TASKS`, `MIN_PROFIT_MARGIN_STROOPS` and `SIMULATE_EXECUTION`.

- Invalid values abort the whole reload; the previous valid configuration stays.
- Changes are all-or-nothing — never partially applied.
- A change to a restart-required value (`NETWORK`, `REGISTRY_CONTRACT_ID`,
  `KEEPER_SECRET_KEY`, `DRY_RUN`) is logged as a warning and not applied.
- Secrets are replaced with `<redacted>` in all change-detection output.

## Features

- **Multi-account signing.** A pool of signing accounts avoids
  sequence-number serialization. Each account earns its own on-chain
  `keeper_balance`, so rewards are withdrawn per account — see
  [`docs/MULTI_ACCOUNT_SETUP.md`](docs/MULTI_ACCOUNT_SETUP.md).
- **Profitability and ranking.** Candidates are ranked by estimated net profit
  and capped at `MAX_TASKS_PER_ROUND`; tasks below `MIN_PROFIT_MARGIN_STROOPS`
  are skipped.
- **Adaptive fees.** Fees follow the chosen percentile of recent network fees,
  never exceed `FEE_CEILING_STROOPS`, and fall back to `BASE_FEE` when the RPC
  fee stats are unavailable.
- **Spend ceiling and claim races.** A per-round fee ceiling bounds spend; lost
  claim races are recorded as competitive skips, separate from RPC and
  execution failures.
- **Alerting.** Rules for missed executions, consecutive RPC errors and
  stagnant balance, with one notification per incident (re-fires only after the
  condition clears) and a pluggable webhook transport.
- **Metrics.** Per-task-type counts (claimed, executed, skipped by reason) and
  net profit in Prometheus format.
- **Persistence.** Optional PostgreSQL task-state store prevents duplicate
  claims or executions across restarts.
- **Graceful shutdown.** On `SIGINT`/`SIGTERM` the bot stops taking new work,
  lets in-flight submissions finish and persist, and bounds the wait.
- **Lock-window scheduling and withdrawal strategies.** Claimed tasks are
  re-checked at their unlock ledger; withdrawals can use a threshold, a fixed
  schedule or a fee-aware threshold.
- **Verifier support** is deferred until the contract exposes a verifier
  interface; the bot does not code against one that does not exist.

## Docker

The image builds from the repository root, because the bot depends on the
local SDK in `packages/sdk-ts`:

```bash
docker build -f examples/keeper-bot-v2/Dockerfile -t keeper-bot-v2:latest .
```

[`docker-compose.yml`](docker-compose.yml) runs the bot with PostgreSQL:

```bash
cd examples/keeper-bot-v2
cp .env.example .env     # set DB_PASSWORD and real keeper values
docker-compose up -d
docker-compose logs -f keeper-bot
```

Secrets are supplied at runtime and never baked into the image. Kubernetes,
ECS and other topologies are covered in [DEPLOYMENT.md](DEPLOYMENT.md).

## Development

```bash
npm run build   # tsc: src/ -> dist/
npm test        # vitest (TypeScript suites), then node --test (bot/ suites)
npm run lint    # eslint over src/, bot/, test/ and benchmark/
```

Database-backed tests in `test/db.test.ts` run when `DATABASE_URL` (or
`KEEPER_BOT_V2_TEST_DATABASE_URL`) points at a throwaway PostgreSQL; otherwise
they log a skip notice. CI runs them against an ephemeral `postgres:16`
service — see [docs/CI.md](../../docs/CI.md).

## Further reading

- [Migrating from v1](../../docs/KEEPER_BOT_V2_MIGRATION.md)
- [Design rationale](../../docs/KEEPER_BOT_V2_DESIGN.md) — shutdown, scheduling and withdrawal architecture
- [Security review](../../docs/KEEPER_BOT_V2_SECURITY_REVIEW.md)
- [Deployment guide](DEPLOYMENT.md)

## License

Apache 2.0 — see [LICENSE](../../LICENSE).
