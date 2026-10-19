# Production-ready container image for keeper-bot-v2.
#
# SECURITY: This image contains no embedded secrets, signing keys, or credentials.
# All sensitive configuration (KEEPER_SECRET_KEY, database credentials, RPC URLs)
# must be supplied at runtime via environment variables or mounted secrets.
#
# BUILD USAGE (from the repository root, because the bot depends on the local
# TypeScript SDK in packages/sdk-ts):
#   docker build -f examples/keeper-bot-v2/Dockerfile -t keeper-bot-v2:latest .
#
# RUNTIME USAGE:
#   docker run --env-file examples/keeper-bot-v2/.env keeper-bot-v2:latest
#
# DATABASE: Optional PostgreSQL via DATABASE_URL. The schema is applied on
# startup. The connection string is supplied at runtime, never baked in.

# ═══════════════════════════════════════════════════════════════════════════
# Stage 1: Builder
# ═══════════════════════════════════════════════════════════════════════════
FROM node:20-alpine AS builder

# Build the local SDK first: the bot's package.json references it as
# file:../../packages/sdk-ts, so it must exist at that relative path.
WORKDIR /repo/packages/sdk-ts
COPY packages/sdk-ts/package.json packages/sdk-ts/package-lock.json ./
RUN npm ci --no-audit --no-fund
COPY packages/sdk-ts ./
RUN npm run build

WORKDIR /repo/examples/keeper-bot-v2
COPY examples/keeper-bot-v2/package.json examples/keeper-bot-v2/package-lock.json ./
RUN npm ci --no-audit --no-fund
COPY examples/keeper-bot-v2/tsconfig.json ./
COPY examples/keeper-bot-v2/src ./src
RUN npm run build && npm prune --omit=dev

# ═══════════════════════════════════════════════════════════════════════════
# Stage 2: Runtime
# ═══════════════════════════════════════════════════════════════════════════
FROM node:20-alpine

LABEL org.opencontainers.image.title="Soroban Keeper Network Bot v2"
LABEL org.opencontainers.image.description="Production-ready off-chain keeper bot for automated Soroban task execution"
LABEL org.opencontainers.image.vendor="Soroban Keeper Network Contributors"
LABEL org.opencontainers.image.version="0.2.0"

# Non-root user
RUN addgroup -g 1000 keeper && adduser -D -u 1000 -G keeper keeper

# Keep the repository layout so the SDK symlink in node_modules resolves.
WORKDIR /repo/examples/keeper-bot-v2
COPY --from=builder --chown=keeper:keeper /repo/packages/sdk-ts/package.json /repo/packages/sdk-ts/package.json
COPY --from=builder --chown=keeper:keeper /repo/packages/sdk-ts/dist /repo/packages/sdk-ts/dist
COPY --from=builder --chown=keeper:keeper /repo/examples/keeper-bot-v2/package.json ./package.json
COPY --from=builder --chown=keeper:keeper /repo/examples/keeper-bot-v2/node_modules ./node_modules
COPY --from=builder --chown=keeper:keeper /repo/examples/keeper-bot-v2/dist ./dist

USER keeper

# The metrics endpoint listens on METRICS_PORT (default 9090) when enabled.
EXPOSE 9090

# Daemon mode by default. Pass `--once` or set RUN_ONCE=true for one-shot runs.
CMD ["node", "dist/index.js"]

# ═══════════════════════════════════════════════════════════════════════════
# RUNTIME CONFIGURATION
# ═══════════════════════════════════════════════════════════════════════════
#
# Required environment variables:
#   KEEPER_SECRET_KEY      — Stellar account secret key (starts with S...)
#   REGISTRY_CONTRACT_ID   — Deployed KeeperRegistry contract ID (starts with C...)
#   NETWORK                — Stellar network (testnet, futurenet, mainnet)
#   DATABASE_URL           — PostgreSQL connection string (postgres://user:pass@host:port/db)
#
# Optional environment variables (see .env.example for defaults):
#   POLL_INTERVAL_MS       — Poll frequency in milliseconds (default: 10000)
#   WITHDRAW_THRESHOLD     — Min rewards before withdrawal, in stroops (default: 10000000)
#   MAX_TASKS_PER_ROUND    — Max tasks per round (default: 5)
#   MAX_RETRIES            — Retry attempts for transient errors (default: 3)
#   RETRY_BASE_MS          — Base delay for exponential backoff, ms (default: 500)
#   EXPIRE_STALE_TASKS     — Whether to expire past-deadline tasks (default: true)
#   MIN_PROFIT_MARGIN_STROOPS — Min profit margin, stroops (default: 0)
#   SIMULATE_EXECUTION     — Dev only: fabricate synthetic proofs (default: false)
#   RUN_ONCE               — One-shot mode for cron/serverless (default: false)
#
# SECRET HANDLING:
#   Secrets are never logged. Configuration validation errors suppress the actual
#   secret value and log only the variable name and reason for the failure.
#   Example error output (safe to log):
#     Invalid KEEPER_SECRET_KEY — must be a valid Stellar secret seed (starts with S)
#   Never shows the actual key value.
#
# DATABASE SETUP:
#   On first run, the bot applies pending schema migrations automatically via
#   the migration tooling configured for v2. This allows a fresh database to be
#   brought to the current schema with a single `docker run` invocation.
#
#   Migrations are idempotent: re-running the bot against an already-migrated
#   database is safe. Only unapplied migrations execute.
#
#   For persistent storage, mount a Docker volume or use a managed database service:
#     docker run -v keeper-data:/var/lib/postgresql/data keeper-bot-v2:latest
#   Or reference a managed PostgreSQL instance:
#     -e DATABASE_URL='postgresql://user:pass@managed-db.example.com:5432/keeper'
#
# GRACEFUL SHUTDOWN:
#   The bot listens for SIGINT and SIGTERM, allowing `docker stop` to give it
#   time to complete the current round before terminating. Set a reasonable
#   stop timeout:
#     docker run --stop-timeout 30 keeper-bot-v2:latest
#
# ═══════════════════════════════════════════════════════════════════════════
