/**
 * Test suite for withRetry().
 *
 * The retry mechanism is critical for keeper reliability. It must:
 *   - Retry transient failures with exponential backoff
 *   - Apply jitter to avoid thundering herd
 *   - Abort immediately on permanent errors
 *   - Respect the maximum retry limit
 *
 * withRetry takes its retry policy and sleep function as an optional third
 * argument, so these tests drive it without a loaded CONFIG and without ever
 * waiting on a real timer.
 */

"use strict";

const { describe, it } = require("node:test");
const assert = require("node:assert");

const { withRetry } = require("../index.js");

const BASE_MS = 100;

/** Retry policy used by every test here, with sleep captured instead of awaited. */
function policy(overrides = {}) {
  const delays = [];
  const options = {
    maxRetries: 3,
    retryBaseMs: BASE_MS,
    sleepFn: async (ms) => {
      delays.push(ms);
    },
    ...overrides,
  };
  return { options, delays };
}

describe("withRetry", () => {
  describe("success cases", () => {
    it("returns immediately on first success", async () => {
      const { options } = policy();
      let attempts = 0;
      const result = await withRetry(
        "test-op",
        async () => {
          attempts++;
          return "success";
        },
        options
      );
      assert.strictEqual(result, "success");
      assert.strictEqual(attempts, 1);
    });

    it("returns the result from a successful retry", async () => {
      const { options } = policy();
      let attempts = 0;
      const result = await withRetry(
        "test-op",
        async () => {
          attempts++;
          if (attempts < 2) throw new Error("transient failure");
          return "eventually succeeded";
        },
        options
      );
      assert.strictEqual(result, "eventually succeeded");
      assert.strictEqual(attempts, 2);
    });

    it("succeeds on the last allowed attempt", async () => {
      const { options } = policy();
      let attempts = 0;
      const result = await withRetry(
        "test-op",
        async () => {
          attempts++;
          if (attempts <= 3) throw new Error("transient failure");
          return "success";
        },
        options
      );
      assert.strictEqual(result, "success");
      assert.strictEqual(attempts, 4); // initial attempt plus three retries
    });
  });

  describe("retry exhaustion", () => {
    it("throws after maxRetries attempts", async () => {
      const { options } = policy();
      let attempts = 0;
      await assert.rejects(
        () =>
          withRetry(
            "test-op",
            async () => {
              attempts++;
              throw new Error("persistent transient failure");
            },
            options
          ),
        { message: "persistent transient failure" }
      );
      assert.strictEqual(attempts, 4);
    });

    it("throws the last error encountered", async () => {
      const { options } = policy();
      await assert.rejects(
        () =>
          withRetry(
            "test-op",
            async () => {
              throw new Error("final error");
            },
            options
          ),
        { message: "final error" }
      );
    });
  });

  describe("permanent errors", () => {
    const permanent = [
      ["simulation failure", "Simulation failed: InvalidAction"],
      ["unauthorized error", "Unauthorized keeper"],
      ["already claimed", "Task already claimed"],
    ];

    for (const [name, message] of permanent) {
      it(`does not retry on ${name}`, async () => {
        const { options } = policy();
        let attempts = 0;
        await assert.rejects(
          () =>
            withRetry(
              "test-op",
              async () => {
                attempts++;
                throw new Error(message);
              },
              options
            ),
          { message }
        );
        assert.strictEqual(attempts, 1, "permanent errors must not be retried");
      });
    }
  });

  describe("exponential backoff", () => {
    it("grows the delay exponentially across retries", async () => {
      const { options, delays } = policy();
      let attempts = 0;
      await withRetry(
        "test-op",
        async () => {
          attempts++;
          if (attempts <= 3) throw new Error("retry me");
          return "ok";
        },
        options
      );

      assert.strictEqual(delays.length, 3, "one delay before each retry");
      assert.ok(delays[0] >= BASE_MS, `first delay ${delays[0]} >= ${BASE_MS}`);
      assert.ok(delays[1] >= BASE_MS * 2, `second delay ${delays[1]} >= ${BASE_MS * 2}`);
      assert.ok(delays[2] >= BASE_MS * 4, `third delay ${delays[2]} >= ${BASE_MS * 4}`);
    });

    it("keeps jitter within one base interval of the backoff", async () => {
      const { options, delays } = policy();
      let attempts = 0;
      await withRetry(
        "test-op",
        async () => {
          attempts++;
          if (attempts <= 2) throw new Error("retry me");
          return "ok";
        },
        options
      );

      // Jitter adds a random [0, retryBaseMs) on top of the exponential term.
      delays.forEach((delay, i) => {
        const min = BASE_MS * 2 ** i;
        const max = min + BASE_MS;
        assert.ok(
          delay >= min && delay < max,
          `delay ${delay} should be in [${min}, ${max})`
        );
      });
    });
  });

  describe("edge cases", () => {
    it("handles a synchronous exception", async () => {
      const { options } = policy();
      await assert.rejects(
        () =>
          withRetry(
            "test-op",
            () => {
              throw new Error("sync error");
            },
            options
          ),
        { message: "sync error" }
      );
    });

    it("handles a rejected promise", async () => {
      const { options } = policy();
      await assert.rejects(
        () => withRetry("test-op", () => Promise.reject(new Error("rejected")), options),
        { message: "rejected" }
      );
    });

    it("attempts exactly once when maxRetries is 0", async () => {
      const { options, delays } = policy({ maxRetries: 0 });
      let attempts = 0;
      await assert.rejects(
        () =>
          withRetry(
            "test-op",
            async () => {
              attempts++;
              throw new Error("no retries allowed");
            },
            options
          ),
        { message: "no retries allowed" }
      );
      assert.strictEqual(attempts, 1);
      assert.strictEqual(delays.length, 0, "no sleep when no retry is allowed");
    });
  });
});
