//! Resource-cost reporting harness (not a correctness test).

// This module only compiles under cfg(test), where std is always linked.
extern crate std;

use soroban_sdk::{testutils::Address as _, Address, Bytes, Env};

use super::common::*;

// ─────────────────────────────────────────────────────────────────────────────
// Resource cost report (backlog 0100) — not a correctness test. Drives one
// representative call through every state-changing entry point and prints
// its CPU instruction / memory cost, plus a machine-readable JSON file the
// `resource-cost` advisory CI job diffs against a checked-in baseline (see
// scripts/report-resource-cost.sh and docs/CI.md).
//
// `#[ignore]` keeps this out of the default `cargo test` run; CI invokes it
// explicitly with `-- --ignored --nocapture`.
//
// `upgrade` is not covered — it needs a real, separately-deployed WASM hash
// to upgrade to, which is out of scope for a single-entry-point call. Pure
// read-only views (`get_task`, `task_count`, `admin`, etc.) are also not
// covered — they are single storage reads with no interesting cost profile
// to track for regressions.
#[test]
#[ignore]
fn resource_report() {
    let s = setup();
    // `initialize` was the last top-level call `setup()` made; the budget
    // reflects it until the next call resets it (see soroban-sdk's
    // `cost_estimate().budget()` docs: "resets before every top-level
    // contract level invocation").
    let mut rows: std::vec::Vec<(&str, u64, u64)> = std::vec::Vec::new();
    let record = |name: &'static str, env: &Env, rows: &mut std::vec::Vec<(&str, u64, u64)>| {
        let budget = env.cost_estimate().budget();
        rows.push((
            name,
            budget.cpu_instruction_cost(),
            budget.memory_bytes_cost(),
        ));
    };
    record("initialize", &s.env, &mut rows);

    let task_a = register_default_task(&s);
    record("register_task", &s.env, &mut rows);

    s.registry.increase_reward(&s.admin, &task_a, &500_000i128);
    record("increase_reward", &s.env, &mut rows);

    let deadline = s.registry.get_task(&task_a).deadline;
    s.registry
        .extend_deadline(&s.admin, &task_a, &(deadline + 7_200));
    record("extend_deadline", &s.env, &mut rows);

    let keeper1 = Address::generate(&s.env);
    let task_b = register_default_task(&s);
    s.registry.claim_task(&keeper1, &task_b);
    record("claim_task", &s.env, &mut rows);

    s.registry
        .execute_task(&keeper1, &task_b, &Bytes::from_slice(&s.env, b"proof"));
    record("execute_task", &s.env, &mut rows);

    s.registry.withdraw_rewards(&keeper1);
    record("withdraw_rewards", &s.env, &mut rows);

    let task_c = register_default_task(&s);
    s.registry.cancel_task(&s.admin, &task_c);
    record("cancel_task", &s.env, &mut rows);

    let task_d = register_default_task(&s);
    advance(&s.env, 200, 3_601);
    s.registry.expire_task(&task_d);
    record("expire_task", &s.env, &mut rows);

    s.registry.pause(&s.admin);
    record("pause", &s.env, &mut rows);

    s.registry.unpause(&s.admin);
    record("unpause", &s.env, &mut rows);

    s.registry.set_fee_bps(&s.admin, &500u32);
    record("set_fee_bps", &s.env, &mut rows);

    s.registry.set_min_reward(&s.admin, &0i128);
    record("set_min_reward", &s.env, &mut rows);

    let treasury = Address::generate(&s.env);
    let accrued = s.registry.fees_accrued();
    s.registry.sweep_fees(&s.admin, &treasury, &accrued);
    record("sweep_fees", &s.env, &mut rows);

    let new_admin = Address::generate(&s.env);
    s.registry.transfer_admin(&s.admin, &new_admin);
    record("transfer_admin", &s.env, &mut rows);

    std::println!("### Resource cost per entry point");
    std::println!();
    std::println!("| Entry point | CPU instructions | Memory bytes |");
    std::println!("|---|---|---|");
    for (name, cpu, mem) in &rows {
        std::println!("| `{name}` | {cpu} | {mem} |");
    }

    let mut json = std::string::String::from("{\"entry_points\":[");
    for (i, (name, cpu, mem)) in rows.iter().enumerate() {
        if i > 0 {
            json.push(',');
        }
        json.push_str(&std::format!(
            "{{\"name\":\"{name}\",\"cpu_instructions\":{cpu},\"memory_bytes\":{mem}}}"
        ));
    }
    json.push_str("]}");

    let out_path = std::path::Path::new("target/resource-report.json");
    if let Some(parent) = out_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(out_path, json).expect("failed to write target/resource-report.json");
}
