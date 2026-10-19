//! Every client error names its call — and never its secrets (issue #345).
//!
//! An error by itself does not say which call produced it or with what
//! arguments. These tests pin the two halves of the issue: the formatted
//! error is self-explanatory (method name + non-secret arguments), and no
//! signing key, seed, or signature ever appears in any error's output —
//! asserted the way the acceptance criterion asks, by triggering an error
//! from a client whose signer holds a key and scanning the whole formatted
//! chain for the key's bytes.

use keeper_registry::KeeperRegistry;
use soroban_keeper_sdk::{
    CallContext, ClientError, InvocationRequest, KeeperClient, KeeperRegistryClient, Redacted,
    RegistryClientError, RpcCallError, RpcTransport, SignedTransaction, SignerError,
    SimulationOutcome, TransactionSigner,
};
use soroban_sdk::xdr::ScVal;
use soroban_sdk::{testutils::Address as _, Address, Env};

/// Every message in an error's chain, concatenated: what a caller's log
/// would show with `{err}` plus walked sources, and `{err:?}` for good
/// measure. Redaction must hold over ALL of it.
fn full_rendering(err: &dyn std::error::Error) -> String {
    let mut out = format!("{err} // {err:?}");
    let mut source = err.source();
    while let Some(s) = source {
        out.push_str(&format!(" // {s} // {s:?}"));
        source = s.source();
    }
    out
}

// ── the Env-based client ────────────────────────────────────────────────────

#[test]
fn env_client_errors_name_the_method_and_its_arguments() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(KeeperRegistry, ());
    let signer_addr = Address::generate(&env);
    let signer = soroban_keeper_sdk::KeypairSigner::new(signer_addr);
    let client = KeeperClient::new(&env, contract_id, &signer);

    // pause() before initialize(): the contract refuses, and the error must
    // say which call it was without the caller logging the call site too.
    let err = client.pause().expect_err("not initialized");
    let rendered = full_rendering(&err);
    assert!(
        rendered.contains("pause("),
        "error does not name the failing method: {rendered}"
    );
    assert_eq!(err.context().expect("context attached").method, "pause");
    assert!(matches!(err.root(), ClientError::ContractError(_)));

    // A method with a non-trivial argument carries its debug form.
    let err = client.set_fee_bps(10_001).expect_err("not initialized");
    let rendered = full_rendering(&err);
    assert!(
        rendered.contains("set_fee_bps("),
        "missing method name: {rendered}"
    );
    assert!(
        rendered.contains("10001"),
        "missing argument value: {rendered}"
    );
}

// ── the RPC-based client ────────────────────────────────────────────────────

/// Simulation always succeeds with a signable transaction; submission is
/// unreachable in these tests. Enough transport to reach the signing step.
struct SignOnlyTransport;

impl RpcTransport for SignOnlyTransport {
    async fn simulate(
        &self,
        _request: &InvocationRequest,
    ) -> Result<SimulationOutcome, RpcCallError<u32>> {
        Ok(SimulationOutcome {
            return_value: ScVal::Void,
            transaction: vec![1, 2, 3],
        })
    }

    async fn submit(&self, _signed: &SignedTransaction) -> Result<ScVal, RpcCallError<u32>> {
        unreachable!("these tests fail before submission")
    }
}

/// A signer that HOLDS key material — a 32-byte seed with a distinctive
/// pattern — and fails to sign. What the acceptance criterion is about:
/// the seed must not appear anywhere in the formatted error, however the
/// failure is rendered. The seed lives inside [`Redacted`], so even this
/// type's own `Debug` could not print it.
struct SeededSigner {
    seed: Redacted<[u8; 32]>,
    address: Address,
}

impl TransactionSigner for SeededSigner {
    fn address(&self) -> Address {
        self.address.clone()
    }

    fn sign_payload(&self, _payload: &[u8]) -> Result<soroban_sdk::Bytes, SignerError> {
        // A realistic failure message: mentions the signer, not the key.
        let _ = self.seed.expose(); // the key is used, never printed
        Err(SignerError::Failed(anyhow::anyhow!(
            "HSM refused the signing request"
        )))
    }
}

#[tokio::test]
async fn a_signing_failure_names_the_call_but_never_the_key() {
    let env = Env::default();
    let seed = [0xD7u8; 32];
    let signer = SeededSigner {
        seed: Redacted(seed),
        address: Address::generate(&env),
    };
    let client =
        KeeperRegistryClient::new("C", "url", "pass", SignOnlyTransport).with_signer(signer);

    let err = client.set_fee_bps(250).await.expect_err("signing fails");

    // Self-explanatory: the method and its non-secret argument are there.
    let rendered = full_rendering(&err);
    assert!(
        rendered.contains("set_fee_bps("),
        "missing method name: {rendered}"
    );
    assert!(matches!(err.root(), RegistryClientError::Signing(_)));

    // The acceptance check: no form of the seed appears anywhere in the
    // chain — not the decimal bytes a derived Debug would print, not hex.
    assert!(
        !rendered.contains("215, 215"),
        "seed bytes leaked (decimal): {rendered}"
    );
    let lower = rendered.to_lowercase();
    assert!(
        !lower.contains("d7d7"),
        "seed bytes leaked (hex): {rendered}"
    );
    assert!(
        !lower.contains("0xd7"),
        "seed bytes leaked (byte literal): {rendered}"
    );
}

#[tokio::test]
async fn rpc_errors_carry_the_scval_arguments() {
    struct FailingTransport;
    impl RpcTransport for FailingTransport {
        async fn simulate(
            &self,
            _request: &InvocationRequest,
        ) -> Result<SimulationOutcome, RpcCallError<u32>> {
            // KeeperError::InvalidFeeBps = 7 in the contract's error enum;
            // any contract code works for the context assertion.
            Err(RpcCallError::Contract(7))
        }
        async fn submit(&self, _signed: &SignedTransaction) -> Result<ScVal, RpcCallError<u32>> {
            unreachable!()
        }
    }

    let client = KeeperRegistryClient::new("C", "url", "pass", FailingTransport);
    let err = client.get_task(9_999).await.expect_err("contract refuses");

    let context = err.context().expect("context attached");
    assert_eq!(context.method, "get_task");
    assert!(
        context.args.iter().any(|a| a.contains("9999")),
        "argument missing from context: {:?}",
        context.args
    );
}

// ── the redaction primitive itself ──────────────────────────────────────────

#[test]
fn redacted_hides_the_value_from_both_debug_and_display() {
    let secret = Redacted([0xD7u8; 32]);
    assert_eq!(format!("{secret:?}"), "<redacted>");
    assert_eq!(format!("{secret}"), "<redacted>");

    // And a context built around a redacted value stays clean.
    let context = CallContext::new("sign", vec![format!("{secret:?}")]);
    assert_eq!(context.to_string(), "sign(<redacted>)");
}
