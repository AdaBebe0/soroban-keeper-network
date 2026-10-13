# @soroban-keeper-network/sdk

A typed TypeScript client for the keeper-registry contract in
[`contracts/keeper-registry`](../../contracts/keeper-registry), plus a client
for the treasury contract and a set of React hooks.

It replaces the hand-built `simulate -> build -> sign -> submit -> confirm`
sequence with one client class and one typed method per contract entry point.

## Install

Not yet published to npm. Reference it as a local path dependency (see
`examples/keeper-bot/package.json` for the pattern):

```json
{
  "dependencies": {
    "@soroban-keeper-network/sdk": "file:../../packages/sdk-ts"
  }
}
```

`@stellar/stellar-sdk` is a peer dependency. React is an optional peer
dependency: importing from the main entry point never pulls it in.

## Quick start

```ts
import { Keypair } from "@stellar/stellar-sdk";
import {
  KeeperRegistryClient,
  NETWORK_PRESETS,
  keypairSigner,
} from "@soroban-keeper-network/sdk";

const { rpcUrl, networkPassphrase } = NETWORK_PRESETS.testnet;

const client = new KeeperRegistryClient({
  contractId: process.env.REGISTRY_CONTRACT_ID!,
  rpcUrl,
  networkPassphrase,
  signer: keypairSigner(Keypair.fromSecret(process.env.KEEPER_SECRET_KEY!)),
});

const task = await client.getTask(42n);
await client.claimTask({ keeper, taskId: 42n });
await client.executeTask({ keeper, taskId: 42n, proof: txHashHex });
const outcome = await client.withdrawRewards({ keeper });
// { status: "withdrawn", amount } or { status: "no_rewards_available", amount: 0n }
```

Read-only methods need no signer and no funded account: they are simulated,
never submitted. State-changing methods need a signer whose public key matches
the address the contract requires authorization from. The SDK checks that
locally rather than spending a fee on a transaction that would fail
`require_auth`.

`NETWORK_PRESETS` holds verified `rpcUrl`/`networkPassphrase` pairs for
`testnet`, `futurenet` and `mainnet`. For a private network, pass your own
values. `NETWORK_NAMES` and the `isNetworkName()` type guard help validate
user-supplied network names.

## Methods

| Area | Methods |
| --- | --- |
| Contract views | `admin`, `getFeeBps`, `isPaused`, `feesAccrued`, `rewardTokenAddress`, `minReward`, `version`, `checkContractCompatibility` |
| Task views | `getTask`, `taskCount`, `isClaimable`, `keeperBalance`, `keeperReputation` |
| Task lifecycle | `registerTask`, `increaseReward`, `extendDeadline`, `claimTask`, `executeTask`, `cancelTask`, `expireTask` |
| Rewards | `withdrawRewards` (reads the balance first and skips the fee when it is zero), `tryWithdrawRewards` (resolves to `0n` instead of rejecting) |
| Staking and disputes | `stakeDeposit`, `initiateUnbond`, `withdrawStake`, `slash`, `setMinStake`, `raiseSlashAppeal`, `resolveSlashAppeal`, `setDisputeWindow`, `disputeExecution`, `resolveExecutionDispute`, `keeperStake`, `pendingUnbond`, `minStake`, `getSlash`, `slashHistory`, `disputeWindow`, `pendingReward` |
| Admin | `pause`, `unpause`, `setFeeBps`, `setMinReward`, and the dual-auth `transferAdmin`, `upgrade`, `sweepFees` |

The treasury contract has its own client at
`@soroban-keeper-network/sdk/treasury` (`TreasuryClient`).

## Conventions

The full rules are in [CONVENTIONS.md](CONVENTIONS.md). In short:

**Large integers are `bigint`.** The contract's `i128` rewards and balances can
exceed `Number.MAX_SAFE_INTEGER`. `u32` fields (`feeBps`, `version`, ledger
counts) stay plain `number`. A `number` is accepted where an integer is
expected, but a non-integer or unsafe one is rejected rather than rounded.

**Timestamps accept `Date | number | bigint` and are returned as `Date`.** A
bare number or bigint is read as Unix *seconds*. A value that looks like
milliseconds is rejected rather than accepted as a deadline in the year 54,000.

**Proofs accept `Uint8Array | Buffer | string`, and a string is always hex.**
The `0x` prefix is optional. A string that is not valid hex is refused instead
of falling back to UTF-8. To submit text, encode it yourself:
`new TextEncoder().encode(text)`.

## Errors

Every failure is typed, so nothing needs to match on an error message:

- `KeeperContractError` — the contract returned `Result::Err`. Its `code` is a
  `KeeperErrorCode`, mirroring `contracts/keeper-registry/src/errors.rs`
  discriminant for discriminant. `error.local` is `true` when the SDK applied
  the same rule locally and never built a transaction.
- `KeeperRpcError` — the call never reached a contract verdict: transport
  failure, malformed response, or a host-level trap.
- Both extend `KeeperSdkError`. `getTask` throws `TaskNotFoundError` for an
  unknown ID.

```ts
import { KeeperErrorCode, isKeeperError } from "@soroban-keeper-network/sdk";

try {
  await client.executeTask({ keeper, taskId, proof });
} catch (error) {
  if (isKeeperError(error, KeeperErrorCode.NotTaskClaimer)) return;
  throw error;
}
```

Transient RPC failures while sending or polling a transaction are retried
(`maxRetries`, default 3; `retryBaseMs`, default 500). A decoded contract error
is never retried, because it is a deterministic verdict. The same policy is
exported as `withRetry` for your own calls.

## Contract compatibility

`SUPPORTED_CONTRACT_VERSIONS` records the contract `VERSION` range this SDK
release was built against. `client.version()` reads the deployed contract's
version and warns — once per client, through the `warn` option — when it
falls outside that range. `client.checkContractCompatibility()` returns the
same comparison without emitting anything. A mismatch warns rather than
throws: contract versions are additive, and a client that refuses to run
against a newer contract strands every integrator on upgrade day.

The SDK's copies of contract bounds (`MAX_PROOF_LEN`, `MAX_CALLDATA_LEN`, the
lock and TTL ledger limits) only turn a doomed call into a local error instead
of a wasted round trip. The deployed contract always remains authoritative.
See [VERSIONING.md](VERSIONING.md) and [CHANGELOG.md](CHANGELOG.md).

## Wallet signing and sponsored fees

No secret key needs to touch the SDK. Build an unsigned transaction, let a
wallet sign it, then submit:

```ts
import {
  buildFeeBumpTransaction,
  buildTransaction,
  submitSignedTransaction,
} from "@soroban-keeper-network/sdk";

const unsigned = await buildTransaction(client, userAddress, "register_task", args);
const userSignedXdr = await wallet.signTransaction(unsigned.xdr);
await submitSignedTransaction(client, userSignedXdr);

// Or have a sponsor pay the fee for the user's signed transaction:
const feeBump = buildFeeBumpTransaction(client, sponsorAddress, userSignedXdr);
const sponsorSignedXdr = await sponsorSigner.sign(feeBump.xdr);
await submitSignedTransaction(client, sponsorSignedXdr);
```

[`examples/wallet-signing/registerTaskWithFreighter.ts`](examples/wallet-signing/registerTaskWithFreighter.ts)
is the full Freighter example, including the user-rejects-the-signature case.
Read its "Verification status" section before relying on it.
[`examples/node-signing`](examples/node-signing) shows server-side signing.

## React hooks

```tsx
import {
  KeeperRegistryProvider,
  useTask,
  useTaskEvents,
} from "@soroban-keeper-network/sdk/react";

function App() {
  return (
    <KeeperRegistryProvider client={client}>
      <TaskDetail taskId={42} />
    </KeeperRegistryProvider>
  );
}

function TaskDetail({ taskId }: { taskId: number }) {
  const { task, loading, error } = useTask(taskId, { pollIntervalMs: 5000 });
  const { events } = useTaskEvents({ eventTypes: ["TaskClaimed", "TaskExecuted"] });
  // ...
}
```

Available hooks: `useTask`, `useTaskEvents`, `useRegisterTask`,
`useKeeperBalance` and `useKeeperRegistryClient`. See
[docs/REACT_GUIDE.md](docs/REACT_GUIDE.md) and the worked app in
[`examples/react-app`](examples/react-app).

## Development

```bash
npm install
npm run build       # tsc: src/ -> dist/ (ESM + .d.ts)
npm test            # vitest, no network required
npm run lint        # eslint
npm run docs        # API reference into docs/reference/ (gitignored)
npm run docs:check  # same, without output; fails on undocumented exports
```

The client takes its RPC server as an injected dependency (the `server`
option), which is how the test suite exercises every path without a live
network. Argument encoding and result decoding are not faked: tests assert on
values that have been through the real `nativeToScVal`/`scValToNative`.

`docs:check` runs in CI as the advisory `sdk-ts-docs` job (see
[docs/CI.md](../../docs/CI.md)).

### Package layout

| Path | Contents |
| --- | --- |
| `src/index.ts` | Main entry point |
| `src/client.ts` | `KeeperRegistryClient`: the shared read and write paths |
| `src/core/` | The caller seam, auth-entry signing, and conversion helpers |
| `src/methods/` | One module per contract entry point or group |
| `src/errors.ts` | `KeeperErrorCode` and the contract-error decoder |
| `src/constants.ts` | Supported contract `VERSION` range and contract bounds |
| `src/transactionBuilder.ts` | Unsigned-XDR and fee-bump building for wallet flows |
| `src/treasury/` | `@soroban-keeper-network/sdk/treasury` entry point |
| `src/react/` | `@soroban-keeper-network/sdk/react` entry point |
| `src/testing/` | In-memory fake RPC server for tests |
| `test/` | Per-method vitest suites |
| `scripts/report-bundle-size.mjs` | Bundle-size report used by the `sdk-bundle-size` CI job |

### Workspace tooling

This is a standalone npm package, not a workspace member — there is no root
`package.json` in this repository. That matches `examples/keeper-bot` and
`examples/batch-register`, which are each installed independently.
