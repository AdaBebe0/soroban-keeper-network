//! `KeeperRegistryClient` against the real keeper-registry contract, run in
//! soroban's local test environment through an in-process `RpcTransport`.

use keeper_registry::{KeeperError, KeeperRegistry, KeeperRegistryClient as RawClient};
use soroban_keeper_sdk::{
    InvocationRequest, KeeperRegistryClient, KeypairSigner, RegistryClientError, RetryPolicy,
    RpcTransport, SignedTransaction, SimulationOutcome,
};
use soroban_keeper_sdk::{RpcCallError, TransportError};
use soroban_sdk::xdr::ScVal;
use soroban_sdk::{testutils::Address as _, Address, Env, Symbol, TryFromVal, Val};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;

/// Runs invocations against a local `Env`. `Env` is not `Send`, so calls are
/// executed synchronously and only the resulting plain data crosses the await.
struct LocalTransport {
    env: Env,
    contract: Address,
    flaky_simulations: AtomicU32,
    simulate_calls: AtomicU32,
    submitted: Mutex<Vec<Vec<u8>>>,
}

// SAFETY-free wrapper: tests are single-threaded (current_thread runtime).
unsafe impl Send for LocalTransport {}
unsafe impl Sync for LocalTransport {}

impl LocalTransport {
    fn invoke(&self, request: &InvocationRequest) -> Result<ScVal, RpcCallError<u32>> {
        let args: soroban_sdk::Vec<Val> = {
            let mut v = soroban_sdk::Vec::new(&self.env);
            for a in &request.args {
                v.push_back(Val::try_from_val(&self.env, a).unwrap());
            }
            v
        };
        let res = self.env.try_invoke_contract::<Val, soroban_sdk::Error>(
            &self.contract,
            &Symbol::new(&self.env, &request.function),
            args,
        );
        match res {
            Ok(Ok(v)) => Ok(ScVal::try_from_val(&self.env, &v).unwrap()),
            Ok(Err(e)) => panic!("conversion error {e:?}"),
            Err(Ok(err)) => {
                assert!(err.is_type(soroban_sdk::xdr::ScErrorType::Contract));
                Err(RpcCallError::Contract(err.get_code()))
            }
            Err(Err(e)) => panic!("invoke error {e:?}"),
        }
    }
}

impl RpcTransport for LocalTransport {
    async fn simulate(
        &self,
        request: &InvocationRequest,
    ) -> Result<SimulationOutcome, RpcCallError<u32>> {
        self.simulate_calls.fetch_add(1, Ordering::SeqCst);
        if self.flaky_simulations.load(Ordering::SeqCst) > 0 {
            self.flaky_simulations.fetch_sub(1, Ordering::SeqCst);
            return Err(RpcCallError::Transport(TransportError::Timeout));
        }
        // Reads must not change state; simulating a write here would apply it
        // to the local ledger, so only read-only functions are executed now.
        let read_only = ["get_task", "get_fee_bps", "is_paused"];
        if read_only.contains(&request.function.as_str()) {
            Ok(SimulationOutcome {
                return_value: self.invoke(request)?,
                transaction: vec![],
            })
        } else {
            Ok(SimulationOutcome {
                return_value: ScVal::Void,
                transaction: request.function.as_bytes().to_vec(),
            })
        }
    }

    async fn submit(&self, signed: &SignedTransaction) -> Result<ScVal, RpcCallError<u32>> {
        self.submitted
            .lock()
            .unwrap()
            .push(signed.signature.clone());
        self.invoke(&signed.request)
    }

    async fn sleep(&self, _: std::time::Duration) {}
}

struct Fixture {
    env: Env,
    admin: Address,
    contract: Address,
}

fn fixture() -> Fixture {
    let env = Env::default();
    env.mock_all_auths();
    let contract = env.register(KeeperRegistry, ());
    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    RawClient::new(&env, &contract).initialize(&admin, &token, &300);
    Fixture {
        env,
        admin,
        contract,
    }
}

fn client(f: &Fixture, flaky: u32) -> KeeperRegistryClient<LocalTransport> {
    let transport = LocalTransport {
        env: f.env.clone(),
        contract: f.contract.clone(),
        flaky_simulations: AtomicU32::new(flaky),
        simulate_calls: AtomicU32::new(0),
        submitted: Mutex::new(vec![]),
    };
    KeeperRegistryClient::new(
        "CTEST",
        "http://localhost:8000/rpc",
        "Standalone Network ; February 2017",
        transport,
    )
    .with_signer(KeypairSigner::new(f.admin.clone()))
    .with_retry_policy(RetryPolicy {
        max_attempts: 3,
        ..RetryPolicy::no_retry()
    })
}

#[tokio::test]
async fn read_and_write_round_trip_against_the_real_contract() {
    let f = fixture();
    let c = client(&f, 0);

    assert_eq!(c.get_fee_bps().await.unwrap(), 300);
    c.set_fee_bps(450).await.unwrap();
    assert_eq!(c.get_fee_bps().await.unwrap(), 450);
    assert!(!c.is_paused().await.unwrap());
}

#[tokio::test]
async fn contract_errors_decode_into_keeper_error_and_are_not_retried() {
    let f = fixture();
    let c = client(&f, 0);

    // fee_bps above 10_000 is rejected by the contract itself.
    let err = c.set_fee_bps(10_001).await.unwrap_err();
    // Since issue #345 every error arrives wrapped in its call context;
    // `root()` is the variant the caller acts on, the context is the call.
    assert!(matches!(
        err.root(),
        RegistryClientError::Contract(KeeperError::InvalidFeeBps)
    ));
    assert_eq!(
        err.context().expect("context attached").method,
        "set_fee_bps"
    );

    // Unknown task: reads surface the contract error too, after one simulate.
    let before = c.get_task(999).await.unwrap_err();
    assert!(matches!(before.root(), RegistryClientError::Contract(_)));
    assert_eq!(
        before.context().expect("context attached").method,
        "get_task"
    );
}

#[tokio::test]
async fn transient_simulation_failures_are_retried() {
    let f = fixture();
    let c = client(&f, 2);
    assert_eq!(c.get_fee_bps().await.unwrap(), 300);

    let c = client(&f, 5);
    let err = c.get_fee_bps().await.unwrap_err();
    assert!(matches!(
        err.root(),
        RegistryClientError::Transport(TransportError::Timeout)
    ));
}

#[tokio::test]
async fn write_without_signer_fails_before_any_rpc() {
    let f = fixture();
    let transport = LocalTransport {
        env: f.env.clone(),
        contract: f.contract.clone(),
        flaky_simulations: AtomicU32::new(0),
        simulate_calls: AtomicU32::new(0),
        submitted: Mutex::new(vec![]),
    };
    let c = KeeperRegistryClient::new("C", "url", "pass", transport);
    assert!(matches!(
        c.set_fee_bps(1).await.unwrap_err().root(),
        RegistryClientError::NoSigner
    ));
}
