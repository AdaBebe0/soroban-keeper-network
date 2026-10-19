/**
 * Test suite for the pluggable executor interface (executeTaskOffChain,
 * EXECUTORS, ttlExtensionExecutor, simulatedExecutor).
 *
 * The core guarantee under test: execute_task must never be reachable for
 * a task type with no registered executor unless SIMULATE_EXECUTION is
 * explicitly on, and an executor that fails (null or throw) must also
 * result in no proof being produced.
 */

"use strict";

const { describe, it } = require("node:test");
const assert = require("node:assert");
const {
  executeTaskOffChain,
  EXECUTORS,
  ttlExtensionExecutor,
  simulatedExecutor,
  TASK_TYPE_NAMES,
} = require("../index.js");

function makeCtx() {
  const logs = [];
  return {
    server: {},
    keypair: {},
    networkPassphrase: "test",
    log: (msg) => logs.push(msg),
    logs,
  };
}

describe("TASK_TYPE_NAMES", () => {
  it("maps every contract TaskType discriminant to a name", () => {
    // Mirrors contracts/keeper-registry/src/lib.rs's TaskType enum.
    assert.strictEqual(TASK_TYPE_NAMES[0], "Liquidation");
    assert.strictEqual(TASK_TYPE_NAMES[1], "OraclePricePush");
    assert.strictEqual(TASK_TYPE_NAMES[2], "FundingRateUpdate");
    assert.strictEqual(TASK_TYPE_NAMES[3], "LiquidityRebalance");
    assert.strictEqual(TASK_TYPE_NAMES[4], "TtlExtension");
    assert.strictEqual(TASK_TYPE_NAMES[5], "Custom");
  });
});

describe("executeTaskOffChain dispatch", () => {
  it("returns null for a task type with no registered executor and simulateExecution=false", async () => {
    const ctx = makeCtx();
    const task = {
      taskId: 1n,
      taskType: 5,
      taskTypeName: "Custom",
      deadline: Math.floor(Date.now() / 1000) + 3600,
    };
    const proof = await executeTaskOffChain(task, ctx, false);
    assert.strictEqual(proof, null);
  });

  it("does not call the simulated executor when simulateExecution is false", async () => {
    const ctx = makeCtx();
    const task = { taskId: 2n, taskType: 5, taskTypeName: "Custom", deadline: 0 };
    await executeTaskOffChain(task, ctx, false);
    assert.ok(
      !ctx.logs.some((l) => l.includes("SIMULATE_EXECUTION")),
      "simulated executor must not run unless simulateExecution=true"
    );
  });

  it("falls back to simulatedExecutor when simulateExecution=true and no real executor is registered", async () => {
    const ctx = makeCtx();
    const task = {
      taskId: 3n,
      taskType: 5,
      taskTypeName: "Custom",
      deadline: Math.floor(Date.now() / 1000) + 3600,
    };
    const proof = await executeTaskOffChain(task, ctx, true);
    assert.ok(Buffer.isBuffer(proof));
    assert.ok(proof.toString().includes("keeper-proof:task:3"));
  });

  it("dispatches to the registered executor for a known task type, ignoring simulateExecution", async () => {
    const ctx = makeCtx();
    const task = {
      taskId: 4n,
      taskType: 4,
      taskTypeName: "TtlExtension",
      deadline: Math.floor(Date.now() / 1000) + 3600,
    };
    const proof = await executeTaskOffChain(task, ctx, false);
    assert.ok(Buffer.isBuffer(proof));
    assert.ok(proof.toString().includes("ttl-extension:task:4"));
  });

  it("returns null (does not throw) when the registered executor throws", async () => {
    const ctx = makeCtx();
    const throwingExecutor = async () => {
      throw new Error("boom");
    };
    const originalExecutor = EXECUTORS.TtlExtension;
    EXECUTORS.TtlExtension = throwingExecutor;
    try {
      const task = { taskId: 5n, taskType: 4, taskTypeName: "TtlExtension", deadline: 0 };
      const proof = await executeTaskOffChain(task, ctx, false);
      assert.strictEqual(proof, null);
    } finally {
      EXECUTORS.TtlExtension = originalExecutor;
    }
  });

  it("has no default executor that fabricates proof for an unknown task type", () => {
    assert.strictEqual(EXECUTORS.default, undefined);
    assert.strictEqual(EXECUTORS.Unknown, undefined);
  });
});

describe("ttlExtensionExecutor", () => {
  it("returns a proof when the task's deadline has not passed", async () => {
    const ctx = makeCtx();
    const task = { taskId: 10n, deadline: Math.floor(Date.now() / 1000) + 3600 };
    const proof = await ttlExtensionExecutor(task, ctx);
    assert.ok(Buffer.isBuffer(proof));
  });

  it("refuses (returns null) when the task's deadline has already passed", async () => {
    const ctx = makeCtx();
    const task = { taskId: 11n, deadline: Math.floor(Date.now() / 1000) - 10 };
    const proof = await ttlExtensionExecutor(task, ctx);
    assert.strictEqual(proof, null);
  });
});

describe("simulatedExecutor", () => {
  it("returns a Buffer proof embedding the task id", async () => {
    const ctx = makeCtx();
    const task = { taskId: 42n };
    const proof = await simulatedExecutor(task, ctx);
    assert.ok(Buffer.isBuffer(proof));
    assert.ok(proof.toString().includes("task:42"));
  });

  it("logs that the proof is fabricated, not real execution", async () => {
    const ctx = makeCtx();
    await simulatedExecutor({ taskId: 43n }, ctx);
    assert.ok(ctx.logs.some((l) => l.includes("SIMULATE_EXECUTION")));
  });
});
