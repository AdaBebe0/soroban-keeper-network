//! Corpus seed generator for the `execute_task` fuzz target (issue #92).
//!
//! A fuzzer starting from an empty corpus spends its early runs
//! rediscovering inputs the unit tests already know are interesting (a
//! proof at exactly `MAX_PROOF_LEN`, a `fee_bps` at exactly `10_000`, etc.).
//! Seeding the corpus with these boundary values up front means fuzzing
//! time goes toward genuinely novel inputs instead.
//!
//! Only `execute_task` is seeded here. `register_task` and `smoke` do not
//! currently compile (see `docs/FUZZING.md`'s "Target status" table) —
//! seeding a corpus for a target that can't be built would fail this
//! issue's own acceptance criterion (`cargo fuzz run <target> -- -runs=0`
//! must pass for every seeded target). Seed those once they're fixed.
//!
//! ## Regenerating the corpus
//!
//! This is a `#[test]` gated behind `#[ignore]`, run manually rather than
//! as part of the normal test suite, because it writes files to disk and
//! only needs to run when the boundary constants it derives from change:
//!
//! ```bash
//! cd fuzz
//! cargo test --features arbitrary -- --ignored generate_execute_task_corpus --nocapture
//! ```
//!
//! It reads the real constants from `keeper_registry` (`MAX_PROOF_LEN`) and
//! `keeper-registry`'s fee bound (`10_000` bps, the same clamp
//! `execute_task.rs` applies to `fee_bps_bytes`), so the seed values stay in
//! sync with the contract rather than drifting as hand-copied numbers.
//!
//! ## Encoding
//!
//! `fuzz_targets/execute_task.rs`'s `ExecuteTaskInput` is (issue 0123 added
//! the three `proof_len_*`/`proof_content` fields to weight proof length
//! toward the `MAX_PROOF_LEN` boundary rather than leaving it to unweighted
//! `Vec<u8>` generation):
//!
//! ```ignore
//! struct ExecuteTaskInput {
//!     reward_bytes: [u8; 16],   // i128, little-endian
//!     fee_bps_bytes: [u8; 4],   // u32, little-endian
//!     proof_len_selector: u8,   // selects the boundary category
//!     proof_len_extra: u32,     // little-endian; drives the two non-fixed categories
//!     proof_content: Vec<u8>,   // repeated/truncated by build_proof to the chosen length
//! }
//! ```
//!
//! `execute_task.rs` decodes it with plain `ExecuteTaskInput::arbitrary`
//! (not `arbitrary_take_rest` — the target's closure takes raw `&[u8]` and
//! builds its own `Unstructured`). The derive calls `arbitrary()` field by
//! field, all reading off the *front* of the buffer: `reward_bytes`,
//! `fee_bps_bytes`, `proof_len_selector`, and `proof_len_extra` all read
//! their bytes directly via `fill_buffer` (no framing, little-endian —
//! `arbitrary`'s integer impls build the value the same way regardless of
//! whether the field is a raw byte array or a primitive integer type), and
//! `proof_content: Vec<u8>` is decoded by `Vec<u8>::arbitrary`, which —
//! despite there being a separate, unrelated `arbitrary_len` helper
//! elsewhere in the crate that other collection impls use — is actually
//! built on `arbitrary_iter`: before each element it reads one `bool` (1
//! byte, true iff the LSB is set) as a "keep going?" flag, and only reads
//! the `u8` element itself if that flag is true. So a seed file's bytes are:
//!
//! ```text
//! [reward: 16 bytes LE] [fee_bps: 4 bytes LE] [selector: 1 byte] [extra: 4 bytes LE]
//! [(keep_going=1, byte) × content_len] [keep_going=0]
//! ```
//!
//! Since `proof_len_for` (in `execute_task.rs`) derives the ACTUAL proof
//! length used by the target from `selector`/`extra` alone — repeating or
//! truncating whatever `proof_content` decodes to, via `build_proof` — a
//! seed only needs a short, fixed `proof_content` (see `PROOF_CONTENT`
//! below); the boundary category is entirely controlled by `selector`.
//!
//! The trailing `keep_going=0` is optional — once the buffer is exhausted,
//! `fill_buffer` zero-pads, so a missing flag byte reads as `0 & 1 == 0`
//! (stop) anyway. `encode()` below builds this directly against the real
//! `arbitrary` crate's `Unstructured`/`ArbitraryIter` (round-tripped by
//! `generated_corpus_decodes_to_intended_boundaries` below) rather than
//! reimplementing the flag/element interleaving as a hand-derived offset
//! formula.

extern crate std;

use keeper_registry::MAX_PROOF_LEN;
use std::fs;
use std::path::Path;
use std::vec::Vec;

const FEE_BPS_MAX: u32 = 10_000;

/// Length of the fixed, short `proof_content` every seed encodes. Actual
/// proof length is entirely controlled by `selector`/`extra` (see
/// `proof_len_for` below) — `build_proof` in `execute_task.rs`
/// repeats/truncates whatever `proof_content` decodes to, so seeds never
/// need to hand-encode long byte vectors to reach a large target length.
const PROOF_CONTENT_LEN: usize = 8;

fn encode(reward: i128, fee_bps: u32, selector: u8, extra: u32) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(16 + 4 + 1 + 4 + PROOF_CONTENT_LEN * 2 + 1);
    bytes.extend_from_slice(&reward.to_le_bytes());
    bytes.extend_from_slice(&fee_bps.to_le_bytes());
    bytes.push(selector);
    bytes.extend_from_slice(&extra.to_le_bytes());
    for i in 0..PROOF_CONTENT_LEN {
        bytes.push(1); // keep_going = true (odd LSB)
        bytes.push(0xAB_u8.wrapping_add(i as u8)); // element value (content is unchecked by the target)
    }
    bytes.push(0); // keep_going = false — stop the Vec here
    bytes
}

/// Mirrors `proof_len_for` from `fuzz_targets/execute_task.rs` exactly, so
/// the round-trip test below can assert each seed lands on the proof
/// length it's named for. Kept separate for the same reason as
/// `ExecuteTaskInputMirror` below — that function is private to the fuzz
/// target binary, not exported by this library crate.
fn proof_len_for(selector: u8, extra: u32, max_len: usize) -> usize {
    match selector % 5 {
        0 => max_len.saturating_sub(1),
        1 => max_len,
        2 => max_len + 1,
        3 => (extra as usize) % (max_len + 1),
        _ => max_len + 2 + (extra as usize % 4_096),
    }
}

/// One named seed case: a reward/fee_bps pair paired with a
/// `proof_len_selector`/`proof_len_extra` combination chosen to land
/// exactly on or beside a boundary this target cares about.
struct Seed {
    name: &'static str,
    reward: i128,
    fee_bps: u32,
    selector: u8,
    extra: u32,
}

fn seeds() -> Vec<Seed> {
    std::vec![
        // Issue 0123's boundary quartet: MAX_PROOF_LEN - 1, exactly,
        // + 1, and a randomized case further out.
        Seed {
            name: "proof_under_max_len",
            reward: 1_000_000,
            fee_bps: 500,
            selector: 0,
            extra: 0,
        },
        Seed {
            name: "proof_at_max_len",
            reward: 1_000_000,
            fee_bps: 500,
            selector: 1,
            extra: 0,
        },
        Seed {
            name: "proof_over_max_len",
            reward: 1_000_000,
            fee_bps: 500,
            selector: 2,
            extra: 0,
        },
        Seed {
            name: "proof_far_over_max_len",
            reward: 1_000_000,
            fee_bps: 500,
            selector: 4,
            extra: 100,
        },
        Seed {
            name: "proof_empty",
            reward: 1_000_000,
            fee_bps: 500,
            selector: 3,
            extra: 0,
        },
        Seed {
            name: "fee_bps_zero",
            reward: 1_000_000,
            fee_bps: 0,
            selector: 3,
            extra: 32,
        },
        Seed {
            name: "fee_bps_max",
            reward: 1_000_000,
            fee_bps: FEE_BPS_MAX,
            selector: 3,
            extra: 32,
        },
        Seed {
            name: "fee_bps_over_max",
            // execute_task.rs clamps fee_bps_bytes with `% 10_001`, so this
            // is a raw value that clamps down to a boundary case rather
            // than passing FEE_BPS_MAX + 1 through unclamped.
            reward: 1_000_000,
            fee_bps: FEE_BPS_MAX + 1,
            selector: 3,
            extra: 32,
        },
        Seed {
            name: "reward_minimal",
            reward: 1,
            fee_bps: 500,
            selector: 3,
            extra: 32,
        },
        Seed {
            name: "reward_large",
            reward: 999_999_999,
            fee_bps: 500,
            selector: 3,
            extra: 32,
        },
    ]
}

#[test]
#[ignore]
fn generate_execute_task_corpus() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("corpus/execute_task");
    fs::create_dir_all(&dir).expect("create corpus/execute_task directory");

    for seed in seeds() {
        let bytes = encode(seed.reward, seed.fee_bps, seed.selector, seed.extra);
        let path = dir.join(seed.name);
        fs::write(&path, &bytes).unwrap_or_else(|e| panic!("write {path:?}: {e}"));
        std::println!("wrote {} ({} bytes)", path.display(), bytes.len());
    }
}

/// Mirrors `ExecuteTaskInput` from `fuzz_targets/execute_task.rs` field for
/// field. Kept separate (rather than importing it) because that struct is
/// private to the fuzz target binary, not exported by this library crate.
/// Any drift between the two would be caught by `cargo fuzz run
/// execute_task -- -runs=0` failing on these same corpus files in CI.
#[derive(arbitrary::Arbitrary, Debug)]
struct ExecuteTaskInputMirror {
    reward_bytes: [u8; 16],
    fee_bps_bytes: [u8; 4],
    proof_len_selector: u8,
    proof_len_extra: u32,
    proof_content: std::vec::Vec<u8>,
}

/// Decodes each generated seed file through the real `arbitrary` crate —
/// the exact machinery `libfuzzer_sys::fuzz_target!` uses to turn corpus
/// bytes into `ExecuteTaskInput` — and checks it lands on the boundary
/// value each seed is named for. This is a stand-in for `cargo fuzz run
/// execute_task -- -runs=0` (the acceptance criterion), which needs a
/// nightly ASan toolchain that isn't available in every environment; this
/// test gives the same "does the corpus decode into what it's supposed to"
/// guarantee on stable Rust.
#[test]
#[ignore]
fn generated_corpus_decodes_to_intended_boundaries() {
    generate_execute_task_corpus();

    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("corpus/execute_task");
    let max_proof = MAX_PROOF_LEN as usize;

    let cases: &[(&str, i128, u32, u8, u32)] = &[
        ("proof_under_max_len", 1_000_000, 500, 0, 0),
        ("proof_at_max_len", 1_000_000, 500, 1, 0),
        ("proof_over_max_len", 1_000_000, 500, 2, 0),
        ("proof_far_over_max_len", 1_000_000, 500, 4, 100),
        ("proof_empty", 1_000_000, 500, 3, 0),
        ("fee_bps_zero", 1_000_000, 0, 3, 32),
        ("fee_bps_max", 1_000_000, FEE_BPS_MAX, 3, 32),
        ("fee_bps_over_max", 1_000_000, FEE_BPS_MAX + 1, 3, 32),
        ("reward_minimal", 1, 500, 3, 32),
        ("reward_large", 999_999_999, 500, 3, 32),
    ];

    for (name, expected_reward, expected_fee_bps, expected_selector, expected_extra) in cases {
        let bytes = fs::read(dir.join(name)).unwrap_or_else(|e| panic!("read {name}: {e}"));
        let mut u = arbitrary::Unstructured::new(&bytes);
        let decoded = <ExecuteTaskInputMirror as arbitrary::Arbitrary>::arbitrary(&mut u)
            .unwrap_or_else(|e| panic!("{name} failed to decode as ExecuteTaskInput: {e}"));

        assert_eq!(
            i128::from_le_bytes(decoded.reward_bytes),
            *expected_reward,
            "{name}: decoded reward mismatch"
        );
        assert_eq!(
            u32::from_le_bytes(decoded.fee_bps_bytes),
            *expected_fee_bps,
            "{name}: decoded fee_bps mismatch"
        );
        assert_eq!(
            decoded.proof_len_selector, *expected_selector,
            "{name}: decoded proof_len_selector mismatch"
        );
        assert_eq!(
            decoded.proof_len_extra, *expected_extra,
            "{name}: decoded proof_len_extra mismatch"
        );
        assert_eq!(
            decoded.proof_content.len(),
            PROOF_CONTENT_LEN,
            "{name}: decoded proof_content length mismatch"
        );

        // The actual proof length `execute_task.rs` builds for this seed —
        // this is what pins each seed to the boundary it's named for.
        let actual_len = proof_len_for(*expected_selector, *expected_extra, max_proof);
        match *name {
            "proof_under_max_len" => assert_eq!(actual_len, max_proof - 1),
            "proof_at_max_len" => assert_eq!(actual_len, max_proof),
            "proof_over_max_len" => assert_eq!(actual_len, max_proof + 1),
            "proof_far_over_max_len" => assert_eq!(actual_len, max_proof + 2 + 100),
            "proof_empty" => assert_eq!(actual_len, 0),
            _ => assert_eq!(actual_len, 32, "{name}: expected the default 32-byte proof"),
        }
    }
}
