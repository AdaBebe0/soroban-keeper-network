const js = require("@eslint/js");
const tsParser = require("@typescript-eslint/parser");
const tsPlugin = require("@typescript-eslint/eslint-plugin");
const globals = require("globals");

// Same philosophy as examples/keeper-bot: a small, non-negotiable ruleset
// focused on correctness, not style.
module.exports = [
  { ignores: ["dist/**", "node_modules/**"] },
  {
    // The CommonJS bot (bot/), its tests and the benchmark.
    files: ["**/*.js"],
    languageOptions: {
      ecmaVersion: 2022,
      sourceType: "commonjs",
      globals: globals.node,
    },
    rules: {
      ...js.configs.recommended.rules,
      "no-unused-vars": ["error", { argsIgnorePattern: "^_" }],
      "no-empty": ["error", { allowEmptyCatch: false }],
      "prefer-const": "warn",
      eqeqeq: ["warn", "smart"],
    },
  },
  {
    // The TypeScript library (src/) and its tests.
    files: ["**/*.ts"],
    languageOptions: {
      parser: tsParser,
      parserOptions: { ecmaVersion: 2022, sourceType: "module" },
      globals: globals.node,
    },
    plugins: { "@typescript-eslint": tsPlugin },
    rules: {
      "@typescript-eslint/no-unused-vars": ["error", { argsIgnorePattern: "^_" }],
      "@typescript-eslint/no-explicit-any": "warn",
      "no-empty": ["error", { allowEmptyCatch: false }],
      "prefer-const": "warn",
      eqeqeq: ["warn", "smart"],
    },
  },
];
