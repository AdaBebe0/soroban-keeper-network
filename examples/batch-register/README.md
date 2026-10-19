# Batch Registration Helper

Owner-side tooling for dApp integrators: takes a list of tasks you want
automated and registers them all in a single `batch_register_tasks` call.

This is the counterpart to [`examples/keeper-bot`](../keeper-bot), which is
*keeper*-side — it claims and executes tasks other people registered, and has
no reason to register any itself. Neither example depends on the other; this
one is a standalone script you run when you have work to post.

Design background and the full integration guide live in
[docs/BATCH_OPERATIONS.md](../../docs/BATCH_OPERATIONS.md). This README covers
the file format and how to run the script.

## Quick start

```bash
cd examples/batch-register
npm install
cp .env.example .env
# Fill in OWNER_SECRET_KEY (funded) and REGISTRY_CONTRACT_ID

# Validate and preview without signing or sending anything:
node index.js tasks.example.json --dry-run

# Submit for real:
node index.js tasks.example.json
```

Output on success reports each returned task id next to the entry that
produced it, so you can record the mapping in your own system:

```
→  Chunk 1/1: 3 entries, max_total_reward=4500000
   tx 9f1c…c2d4

  Task id  │  Entry
  ─────────┼──────────────────────────────────────────────
        41  │  liquidate-position-1041
        42  │  liquidate-position-1042
        43  │  push-xlm-usd-price

✅  Registered 3 task(s).
```

## Task list format

Two formats, chosen by file extension — `.csv` is read as CSV, anything else
as JSON. Both produce identical results; use whichever your own tooling emits.

### JSON

Either a bare array or `{ "tasks": [ … ] }`. See
[`tasks.example.json`](tasks.example.json).

```json
{
  "tasks": [
    {
      "label": "liquidate-position-1041",
      "task_type": "Liquidation",
      "calldata": "6c69717569646174653a706f736974696f6e3a31303431",
      "reward": "1000000",
      "deadline": "1790000000",
      "ttl_ledgers": 17280,
      "lock_ledgers": 120
    }
  ]
}
```

### CSV

Header row required; `#` comment lines and blank lines are ignored. See
[`tasks.example.csv`](tasks.example.csv).

```csv
label,task_type,calldata,reward,deadline,ttl_ledgers,lock_ledgers
liquidate-position-1041,Liquidation,6c6971…3431,1000000,1790000000,17280,120
```

The CSV reader is deliberately minimal (comma-separated, optional double
quotes, `""` for an escaped quote) rather than a full RFC 4180 parser — a task
list is a short operator-authored file, and a CSV dependency would add more
surface than it removes. Use JSON if your calldata needs embedded newlines.

### Fields

| Field | Type | Notes |
|---|---|---|
| `label` | string, optional | Only used in this script's own output, to name the entry in logs and in the task-id table. Defaults to `entry #N`. |
| `task_type` | string | One of `Liquidation`, `OraclePricePush`, `FundingRateUpdate`, `LiquidityRebalance`, `TtlExtension`, `Custom`. |
| `calldata` | hex string | The bytes a keeper uses to reconstruct your target call. `0x` prefix optional; empty is allowed. Max 1024 bytes (2048 hex chars). |
| `reward` | integer | Escrowed bounty in token units (stroops for XLM). **Quote it as a string in JSON** — it is an `i128` and can exceed what a JS number holds exactly. |
| `deadline` | integer | Unix timestamp (seconds) after which the task may be expired. Must be in the future. |
| `ttl_ledgers` | integer | Storage lifetime for the task entry. Minimum 1000. |
| `lock_ledgers` | integer | Ledgers the claiming keeper holds exclusively. Must be in `[12, 17280]`. |

Underscores in numeric fields (`1_000_000`) are accepted and stripped.

## How `max_total_reward` is set — and why

**The script sets `max_total_reward` to the exact sum of the chunk it is
submitting.** No padding, no round-number buffer.

`max_total_reward` is a ceiling on how much escrow one call may pull from the
owner. The reasoning for the exact sum:

- The call is **atomic** — either every entry in it registers or none does
  ([docs §3](../../docs/BATCH_OPERATIONS.md)). There is no partial-success
  case that a buffer would rescue; a batch that would exceed a tight ceiling
  fails just as completely as one that exceeds a loose one.
- So padding buys nothing, and costs something: it widens the window in which
  the transaction could move more escrow than you reviewed. That is precisely
  the risk this parameter exists to close. A ceiling set to "some big number to
  be safe" is not a ceiling, it is a rubber stamp.
- The sum is computed with `BigInt`, so a long list of large rewards cannot
  silently lose precision the way floating-point addition would — a rounding
  error here would either fail the whole batch or authorize more than intended.

The script prints the sum before submitting so you can check it against what
you expected.

If your workflow genuinely needs headroom — for instance, an operator appends
entries to the file between review and submission — set
`MAX_TOTAL_REWARD_BUFFER_BPS` in `.env`. It is `0` by default, deliberately,
and the script prints a warning on every run where it is not.

## Chunking and the batch-size ceiling

The contract enforces a `MAX_BATCH_SIZE` (currently **50**) and rejects
anything larger with a typed `BatchTooLarge` error. The script reads the live
value from the registry's `max_batch_size()` view rather than hardcoding it,
then splits a longer list into that many entries per transaction.

Two things worth knowing:

- **50 is a conservative guard, not a measured ceiling.** Measuring the real
  limit against Soroban's per-transaction budget is still open work (backlog
  issue 0104), so the constant may be revised. That is why the script reads it
  from the chain — and why your own integration should too.
- **Entry count is only half the story.** Each entry writes a task whose
  `calldata` may be up to 1024 bytes, so 50 maximum-sized entries is already
  ~50 KB of ledger writes before the per-entry token transfer and event are
  counted. A list combining large payloads *and* many entries can exhaust the
  transaction budget below the 50-entry cap. If you hit resource errors, chunk
  smaller — the script's chunk size is derived from the contract's cap, not
  from your payload sizes.

**Each chunk is an independent atomic transaction** with its own
`max_total_reward` — never the sum across chunks. If chunk 3 of 5 fails, the
script reports that chunks 1–2 already landed, chunk 3 registered nothing and
moved no escrow, and chunks 4–5 were not attempted, along with which entries
to put in the resubmission.

## Pre-flight validation

Before sending anything, the script checks every entry locally against the
contract's own bounds (reward positive and at or above the registry's
`min_reward`, deadline in the future, calldata within 1024 bytes, `lock_ledgers`
and `ttl_ledgers` in range) and reports **all** failures at once.

This matters because the contract rejects the *whole* batch on the first bad
entry it encounters — so without a local pass, fixing a list with three
mistakes in it costs three failed transactions.

## Notes

- The owner account must hold enough of the registry's reward token to cover
  the full sum, plus fees. All escrow is pulled from the one signing account:
  entries carry no per-entry owner, and the whole batch is authorized by a
  single signature.
- Registered tasks are ordinary tasks. Each entry's escrow is refundable
  independently via `cancel_task` or `expire_task`; nothing about having been
  registered in a batch ties them together afterwards.
- The registry must not be paused — `batch_register_tasks` opens new escrow
  exposure and is blocked while paused, like `register_task`.
