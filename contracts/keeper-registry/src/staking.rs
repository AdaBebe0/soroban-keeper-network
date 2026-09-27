//! Keeper staking and slashing (epic E06). See `docs/STAKING_DESIGN.md` for
//! the full design: trigger model, storage layout, and the decisions this
//! module implements against.
//!
//! Covers backlog issues 0289 (stake storage / `stake_deposit`), 0290
//! (unbonding: `initiate_unbond` / `withdraw_stake`), and 0291 (`slash`).

use soroban_sdk::{contractimpl, Address, BytesN, Env, Symbol};

use crate::constants::{KEEPER_STAKE_BUMP_LEDGERS, KEEPER_STAKE_BUMP_THRESHOLD, UNBOND_DELAY_LEDGERS};
use crate::errors::KeeperError;
use crate::events::*;
use crate::internal::*;
use crate::types::{DataKey, UnbondRequest};
//! Staking, unbonding, and slashing entry points (epic E06).
//!
//! Implements `docs/STAKING_DESIGN.md`: a keeper posts collateral separate
//! from its reward balance (`DataKey::KeeperStake`, never conflated with
//! `DataKey::KeeperReward` — the same separation-of-concerns reasoning that
//! already keeps `FeesAccrued` distinct from task escrow), can request to
//! unbond it after a configured delay, and an admin can slash a keeper's
//! stake for off-chain-determined misbehavior, moving the slashed amount to
//! a treasury address exactly like `sweep_fees` already does for protocol
//! fees. A slashed keeper has a fixed post-slash window to appeal; the admin
//! resolves the appeal by upholding it (refunding the stake) or rejecting it
//! (the slash stands).

use soroban_sdk::{contractimpl, Address, Env, Symbol};

use crate::constants::*;
use crate::errors::KeeperError;
use crate::events::*;
use crate::internal::*;
use crate::types::{DataKey, SlashRecord, UnbondRequest};
use crate::{KeeperRegistry, KeeperRegistryArgs, KeeperRegistryClient};

#[contractimpl]
impl KeeperRegistry {
    // ── stake_deposit ────────────────────────────────────────────────────────
    //
    // A keeper posts collateral. Requires the depositing keeper's own auth —
    // no address can stake on behalf of another. Independent of task escrow
    // and reward balances: staking, executing tasks, and withdrawing rewards
    // never interfere with each other's storage (see test/staking.rs).
    // A keeper posts collateral. Requires the keeper's own auth — no address
    // can stake on behalf of another. Escrows `amount` from the keeper into
    // the contract via the same reward-token client every other transfer in
    // this contract uses.

    pub fn stake_deposit(e: Env, keeper: Address, amount: i128) -> Result<(), KeeperError> {
        require_not_paused(&e)?;
        if amount <= 0 {
            return Err(KeeperError::InvalidStakeAmount);
        }
        keeper.require_auth();

        bump_instance(&e);
        let current = read_keeper_stake(&e, &keeper);
        let new_total = current
            return Err(KeeperError::InvalidReward);
        }
        keeper.require_auth();
        bump_instance(&e);

        let key = DataKey::KeeperStake(keeper.clone());
        let current = keeper_stake_of(&e, &keeper);
        let updated = current
            .checked_add(amount)
            .ok_or(KeeperError::ArithmeticOverflow)?;

        // Effects before interaction.
        write_keeper_stake(&e, &keeper, new_total);
        reward_token(&e)?.transfer(&keeper, &e.current_contract_address(), &amount);

        emit_stake_deposited(&e, &keeper, amount, new_total);
        e.storage().persistent().set(&key, &updated);
        e.storage().persistent().extend_ttl(
            &key,
            KEEPER_BALANCE_BUMP_THRESHOLD,
            KEEPER_BALANCE_BUMP_LEDGERS,
        );

        reward_token(&e)?.transfer(&keeper, &e.current_contract_address(), &amount);

        emit_stake_deposited(&e, &keeper, amount, updated);
        Ok(())
    }

    // ── initiate_unbond ──────────────────────────────────────────────────────
    //
    // Starts the unbonding delay for `amount` of the keeper's stake. A second
    // call while one request is already pending replaces it, using the new
    // total (docs/STAKING_DESIGN.md §3) — not additive, so a keeper who
    // changes their mind about how much to unbond does not need to wait out
    // an old, smaller request first.
    //
    // No token transfer here — the stake stays escrowed in the contract
    // (still slashable, still counted in I-1 solvency) until `withdraw_stake`
    // actually releases it. This is bookkeeping only.

    pub fn initiate_unbond(e: Env, keeper: Address, amount: i128) -> Result<u32, KeeperError> {
        require_not_paused(&e)?;
        if amount <= 0 {
            return Err(KeeperError::InvalidStakeAmount);
        }
        keeper.require_auth();

        let current_stake = read_keeper_stake(&e, &keeper);
    // Starts the unbonding delay for `amount` of a keeper's stake. Only one
    // unbond request may be pending per keeper at a time (a second call
    // while one is outstanding is rejected, not merged or overwritten) —
    // withdraw the first before starting another.

    pub fn initiate_unbond(e: Env, keeper: Address, amount: i128) -> Result<(), KeeperError> {
        require_not_paused(&e)?;
        if amount <= 0 {
            return Err(KeeperError::InvalidReward);
        }
        keeper.require_auth();

        let current_stake = keeper_stake_of(&e, &keeper);
        if amount > current_stake {
            return Err(KeeperError::InsufficientStake);
        }

        bump_instance(&e);
        let release_ledger = e
            .ledger()
            .sequence()
            .checked_add(UNBOND_DELAY_LEDGERS)
            .ok_or(KeeperError::ArithmeticOverflow)?;

        let key = DataKey::UnbondRequest(keeper.clone());
        e.storage().persistent().set(
            &key,
            &UnbondRequest {
                amount,
                release_ledger,
            },
        );
        e.storage()
            .persistent()
            .extend_ttl(&key, KEEPER_STAKE_BUMP_THRESHOLD, KEEPER_STAKE_BUMP_LEDGERS);

        emit_unbond_initiated(&e, &keeper, amount, release_ledger);
        Ok(release_ledger)
        let request_key = DataKey::UnbondRequest(keeper.clone());
        if e.storage().persistent().has(&request_key) {
            return Err(KeeperError::UnbondAlreadyPending);
        }

        bump_instance(&e);

        // The unbonding amount leaves the "effective" stake immediately —
        // `keeper_stake_of` (and therefore the `set_min_stake` gate on
        // `claim_task`, docs/STAKING_DESIGN.md §6) reflects only what is
        // still fully bonded, not what is mid-unbond.
        let stake_key = DataKey::KeeperStake(keeper.clone());
        let remaining = current_stake
            .checked_sub(amount)
            .ok_or(KeeperError::ArithmeticOverflow)?;
        e.storage().persistent().set(&stake_key, &remaining);
        e.storage().persistent().extend_ttl(
            &stake_key,
            KEEPER_BALANCE_BUMP_THRESHOLD,
            KEEPER_BALANCE_BUMP_LEDGERS,
        );

        let unlock_ledger = e.ledger().sequence().saturating_add(UNBOND_DELAY_LEDGERS);
        let request = UnbondRequest {
            amount,
            unlock_ledger,
        };
        e.storage().persistent().set(&request_key, &request);
        e.storage().persistent().extend_ttl(
            &request_key,
            KEEPER_BALANCE_BUMP_THRESHOLD,
            KEEPER_BALANCE_BUMP_LEDGERS,
        );

        emit_unbond_initiated(&e, &keeper, amount, unlock_ledger);
        Ok(())
    }

    // ── withdraw_stake ───────────────────────────────────────────────────────
    //
    // Releases a keeper's pending unbond request once its delay has elapsed
    // (inclusive boundary: `>=`, matching `lock_expired`'s existing
    // convention). Reduces both the pending request and the underlying
    // KeeperStake balance by the same amount, then transfers the tokens out.

    pub fn withdraw_stake(e: Env, keeper: Address) -> Result<i128, KeeperError> {
        keeper.require_auth();

        let key = DataKey::UnbondRequest(keeper.clone());
        let request: UnbondRequest = e
            .storage()
            .persistent()
            .get(&key)
            .ok_or(KeeperError::NoUnbondRequest)?;

        if e.ledger().sequence() < request.release_ledger {
            return Err(KeeperError::UnbondNotReady);
        }

        let current_stake = read_keeper_stake(&e, &keeper);
        // Defensive: a concurrent slash could have reduced the stake below
        // the pending unbond amount since initiate_unbond ran. Release only
        // what remains rather than underflowing.
        let amount = request.amount.min(current_stake);

        bump_instance(&e);
        // Effects before interaction.
        e.storage().persistent().remove(&key);
        write_keeper_stake(&e, &keeper, current_stake - amount);

        if amount > 0 {
            reward_token(&e)?.transfer(&e.current_contract_address(), &keeper, &amount);
        }

        emit_stake_withdrawn(&e, &keeper, amount);
        Ok(amount)
    // Releases a keeper's pending unbond request once the delay has elapsed.
    // The boundary is inclusive — at exactly `unlock_ledger`, the request is
    // already withdrawable (`>=`, not `>`), mirroring `lock_expired`'s
    // boundary convention. Returns the amount withdrawn.

    pub fn withdraw_stake(e: Env, keeper: Address) -> Result<i128, KeeperError> {
        require_not_paused(&e)?;
        keeper.require_auth();

        let request_key = DataKey::UnbondRequest(keeper.clone());
        let request: UnbondRequest = e
            .storage()
            .persistent()
            .get(&request_key)
            .ok_or(KeeperError::NoPendingUnbond)?;

        if e.ledger().sequence() < request.unlock_ledger {
            return Err(KeeperError::UnbondNotReady);
        }

        bump_instance(&e);

        // Effects before interaction: the request is cleared before the
        // transfer, so a re-entrant reward token cannot withdraw twice.
        e.storage().persistent().remove(&request_key);

        reward_token(&e)?.transfer(&e.current_contract_address(), &keeper, &request.amount);

        emit_stake_withdrawn(&e, &keeper, request.amount);
        Ok(request.amount)
    }

    // ── slash ─────────────────────────────────────────────────────────────────
    //
    // Admin-triggered stake reduction (docs/STAKING_DESIGN.md §1 — v1 is
    // admin-only, not automatic, not dispute-based). `incident_id` provides
    // incident-level idempotency (§6): the same incident can never be
    // slashed twice. `amount` must not exceed the keeper's current stake —
    // rejected outright rather than clamped (§1's "Slash bounds decision"),
    // so the admin always knows exactly what happened. Slashed funds move to
    // `treasury`, a caller-supplied parameter per call, mirroring
    // `sweep_fees`'s existing shape rather than introducing a stored
    // treasury-address configuration value.
    // Admin-authorized (docs/STAKING_DESIGN.md §4-5 — dispute-based, not
    // automatic: E04's verifier work never landed, so there is no on-chain
    // check to trigger this from). Moves `amount` of `keeper`'s current
    // stake to `treasury`, exactly the destination pattern `sweep_fees`
    // already uses for protocol fees. `amount` can never exceed the
    // keeper's current stake. Returns a `slash_id` for later reference by
    // `raise_slash_appeal`.

    pub fn slash(
        e: Env,
        admin: Address,
        keeper: Address,
        amount: i128,
        reason: Symbol,
        incident_id: BytesN<32>,
        treasury: Address,
    ) -> Result<(), KeeperError> {
        require_admin(&e, &admin)?;

        if amount <= 0 {
            return Err(KeeperError::InvalidStakeAmount);
        }

        let incident_key = DataKey::SlashIncident(incident_id.clone());
        if e.storage().persistent().has(&incident_key) {
            return Err(KeeperError::DuplicateSlashIncident);
        }

        let current_stake = read_keeper_stake(&e, &keeper);
        if amount > current_stake {
            return Err(KeeperError::SlashExceedsStake);
        }

        bump_instance(&e);
        // Effects before interaction: record the incident and the reduced
        // stake before the token ever leaves the contract, so a reentrant
        // call (from a malicious reward-token `transfer`) sees the incident
        // already recorded and the stake already reduced — it cannot slash
        // the same incident twice or double-count the reduction.
        e.storage().persistent().set(&incident_key, &());
        e.storage().persistent().extend_ttl(
            &incident_key,
            KEEPER_STAKE_BUMP_THRESHOLD,
            KEEPER_STAKE_BUMP_LEDGERS,
        );
        write_keeper_stake(&e, &keeper, current_stake - amount);

        reward_token(&e)?.transfer(&e.current_contract_address(), &treasury, &amount);

        emit_slashed(&e, &keeper, amount, reason, &incident_id, &treasury);
        Ok(())
    }

    // ── Read-only views ──────────────────────────────────────────────────────
    // Never bump storage TTL, matching views.rs's existing policy.

    /// A keeper's current bonded stake (0 if never deposited or fully
    /// withdrawn). Does not include any amount currently mid-unbond — that
    /// amount is still part of this total until `withdraw_stake` actually
    /// releases it (still slashable, still counted in I-1 solvency).
    pub fn keeper_stake(e: Env, keeper: Address) -> i128 {
        read_keeper_stake(&e, &keeper)
    }

    /// A keeper's pending unbond request, if any: `(amount, release_ledger)`.
    pub fn unbonding_status(e: Env, keeper: Address) -> Option<(i128, u32)> {
        let request: Option<UnbondRequest> = e
            .storage()
            .persistent()
            .get(&DataKey::UnbondRequest(keeper));
        request.map(|r| (r.amount, r.release_ledger))
    }

    /// Whether `incident_id` has already been slashed against.
    pub fn is_slash_incident_recorded(e: Env, incident_id: BytesN<32>) -> bool {
        e.storage()
            .persistent()
            .has(&DataKey::SlashIncident(incident_id))
        treasury: Address,
    ) -> Result<u64, KeeperError> {
        require_admin(&e, &admin)?;

        if amount <= 0 {
            return Err(KeeperError::InvalidReward);
        }
        let current_stake = keeper_stake_of(&e, &keeper);
        if amount > current_stake {
            return Err(KeeperError::InsufficientStake);
        }

        bump_instance(&e);

        // Effects before interaction.
        let stake_key = DataKey::KeeperStake(keeper.clone());
        let remaining = current_stake
            .checked_sub(amount)
            .ok_or(KeeperError::ArithmeticOverflow)?;
        e.storage().persistent().set(&stake_key, &remaining);
        e.storage().persistent().extend_ttl(
            &stake_key,
            KEEPER_BALANCE_BUMP_THRESHOLD,
            KEEPER_BALANCE_BUMP_LEDGERS,
        );

        let slash_id = next_slash_id(&e);
        let record = SlashRecord {
            keeper: keeper.clone(),
            amount,
            reason: reason.clone(),
            ledger: e.ledger().sequence(),
            appealed: false,
        };
        let record_key = DataKey::Slash(slash_id);
        e.storage().persistent().set(&record_key, &record);
        e.storage().persistent().extend_ttl(
            &record_key,
            KEEPER_BALANCE_BUMP_THRESHOLD,
            KEEPER_BALANCE_BUMP_LEDGERS,
        );

        reward_token(&e)?.transfer(&e.current_contract_address(), &treasury, &amount);

        emit_slashed(&e, slash_id, &keeper, amount, &reason);
        Ok(slash_id)
    }

    // ── set_min_stake ─────────────────────────────────────────────────────────
    //
    // Admin sets the minimum bonded stake `claim_task` requires. Default 0
    // (no requirement), mirroring `set_min_reward`'s pattern for the
    // task-side floor. Existing claims are unaffected; only future
    // `claim_task` calls are validated.

    pub fn set_min_stake(e: Env, admin: Address, min_stake: i128) -> Result<(), KeeperError> {
        require_admin(&e, &admin)?;
        if min_stake < 0 {
            return Err(KeeperError::InvalidReward);
        }
        bump_instance(&e);
        let old_min = min_stake_floor(&e);
        e.storage().instance().set(&DataKey::MinStake, &min_stake);
        emit_min_stake_updated(&e, old_min, min_stake);
        Ok(())
    }

    // ── raise_slash_appeal ───────────────────────────────────────────────────
    //
    // Only the slashed keeper itself has standing to appeal its own slash
    // (docs/STAKING_DESIGN.md §4.1). Must be called within
    // `DISPUTE_WINDOW_LEDGERS` of the slash, and at most once per
    // `slash_id`. Raising an appeal does not by itself reverse anything —
    // it flags the record for the admin to resolve via
    // `resolve_slash_appeal`.

    pub fn raise_slash_appeal(e: Env, keeper: Address, slash_id: u64) -> Result<(), KeeperError> {
        keeper.require_auth();

        let record_key = DataKey::Slash(slash_id);
        let mut record: SlashRecord = e
            .storage()
            .persistent()
            .get(&record_key)
            .ok_or(KeeperError::SlashNotFound)?;

        if record.keeper != keeper {
            return Err(KeeperError::NotSlashedKeeper);
        }
        if record.appealed {
            return Err(KeeperError::AppealAlreadyRaised);
        }
        let appeal_deadline = record.ledger.saturating_add(DISPUTE_WINDOW_LEDGERS);
        if e.ledger().sequence() > appeal_deadline {
            return Err(KeeperError::AppealWindowClosed);
        }

        record.appealed = true;
        e.storage().persistent().set(&record_key, &record);

        emit_slash_appeal_raised(&e, slash_id, &keeper);
        Ok(())
    }

    // ── resolve_slash_appeal ─────────────────────────────────────────────────
    //
    // Admin-only. `slash` already moved the disputed amount out of this
    // contract to the treasury address the admin chose at slash time — this
    // contract does not hold it in escrow while an appeal is pending (see
    // docs/STAKING_DESIGN.md §4.1: a post-slash appeal, not a pre-slash
    // hold). Upholding an appeal is therefore a genuine refund, not an
    // internal bookkeeping reversal: the admin must supply the funds being
    // returned, exactly as `stake_deposit` requires the depositing party to
    // authorize and fund its own transfer. `uphold_appeal = true` pulls
    // `record.amount` from `admin` into the contract and credits it back to
    // the keeper's stake; `uphold_appeal = false` leaves the slash standing
    // with no transfer. Either way the appeal is considered resolved and
    // the record is removed, so it cannot be re-resolved.

    pub fn resolve_slash_appeal(
        e: Env,
        admin: Address,
        slash_id: u64,
        uphold_appeal: bool,
    ) -> Result<(), KeeperError> {
        require_admin(&e, &admin)?;

        let record_key = DataKey::Slash(slash_id);
        let record: SlashRecord = e
            .storage()
            .persistent()
            .get(&record_key)
            .ok_or(KeeperError::SlashNotFound)?;

        if !record.appealed {
            return Err(KeeperError::SlashNotFound);
        }

        bump_instance(&e);

        // Effects before interaction: the record is removed (so a second
        // resolution attempt fails the same "not found" way a
        // never-appealed id would) before the refund transfer runs.
        e.storage().persistent().remove(&record_key);

        if uphold_appeal {
            let stake_key = DataKey::KeeperStake(record.keeper.clone());
            let current = keeper_stake_of(&e, &record.keeper);
            let restored = current
                .checked_add(record.amount)
                .ok_or(KeeperError::ArithmeticOverflow)?;
            e.storage().persistent().set(&stake_key, &restored);
            e.storage().persistent().extend_ttl(
                &stake_key,
                KEEPER_BALANCE_BUMP_THRESHOLD,
                KEEPER_BALANCE_BUMP_LEDGERS,
            );

            reward_token(&e)?.transfer(&admin, &e.current_contract_address(), &record.amount);
        }

        emit_slash_appeal_resolved(&e, slash_id, uphold_appeal);
        Ok(())
    }
}
