import { defineConfig } from "vitest/config";

// TypeScript suites run under vitest; the CommonJS bot's suites (test/*.test.js)
// run under `node --test` — see the `test` script.
export default defineConfig({
  test: {
    environment: "node",
    include: ["src/**/*.test.ts", "test/**/*.test.ts"],
  },
});
