
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer as TokenTransfer};
use solana_program::clock::Clock;
use std::collections::HashSet;
use std::cmp::min;

use crate::constants::*;
use crate::errors::ErrorCode;
use crate::state::*;

// Events
#[event]
pub struct TaxRateUpdated { pub new_tax_rate: u64 }
#[event]
pub struct StakingTiersUpdated { pub new_tiers: [u64; 4] }
#[event]
pub struct TaxAllocationUpdated { pub burn: u64, pub treasury: u64, pub liquidity_pool: u64, pub lp_incentive: u64, pub charity: u64, pub team: u64 }
#[event]
pub struct ReductionFactorsUpdated { pub thresholds: [u64; 3], pub factors: [u64; 4] }
#[event]
pub struct LaunchTimestampUpdated { pub new_timestamp: i64 }
#[event]
pub struct MaxSellTxnLimitUpdated { pub new_limit: u64 }
#[event]
pub struct DailySellLimitUpdated { pub new_limit: u64 }
#[event]
pub struct MaxTransferLimitUpdated { pub new_limit: u64 }
#[event]
pub struct DailyTransferLimitUpdated { pub new_limit: u64 }
#[event]
pub struct ProgressiveTaxThresholdUpdated { pub new_threshold: u64 }
#[event]
pub struct TransferEvent { pub from: Pubkey, pub to: Pubkey, pub amount: u64, pub tax: u64, pub burn: u64, pub treasury_tax: u64, pub liquidity_pool_tax: u64, pub lp_incentive_tax: u64, pub charity_tax: u64, pub team_tax: u64 }
#[event]
pub struct StakeEvent { pub user: Pubkey, pub amount: u64, pub tier: u8 }
#[event]
pub struct UnstakeEvent { pub user: Pubkey, pub amount: u64 }
#[event]
pub struct ClaimRewardsEvent { pub user: Pubkey, pub amount: u64 }
#[event]
pub struct ProposalSubmittedEvent { pub proposal_id: u64, pub description: String, pub proposal_type: u8 }
#[event]
pub struct VoteEvent { pub staker: Pubkey, pub proposal_id: u64, pub in_favor: bool, pub power: u64 }
#[event]
pub struct ProposalExecutedEvent { pub proposal_id: u64, pub status: u8 }
#[event]
pub struct AirdropEvent { pub total_amount: u64, pub recipient_count: usize }
#[event]
pub struct PauseEvent { pub timestamp: i64, pub reason: String }
#[event]
pub struct ResumeEvent { pub timestamp: i64 }
#[event]
pub struct TeamVestingUpdatedEvent { pub team_member: Pubkey, pub total_amount: u64 }
#[event]
pub struct TeamVestingClaimedEvent { pub team_member: Pubkey, pub amount: u64 }
#[event]
pub struct TeamVestingCanceledEvent { pub team_member: Pubkey }
#[event]
pub struct FreelancerVestingUpdatedEvent { pub freelancer: Pubkey, pub total_amount: u64 }
#[event]
pub struct FreelancerVestingClaimedEvent { pub freelancer: Pubkey, pub amount: u64 }
#[event]
pub struct FreelancerMilestoneReleasedEvent { pub freelancer: Pubkey, pub amount: u64 }
#[event]
pub struct WithdrawalInitiatedEvent { pub amount: u64, pub initiation_slot: u64, pub delay_slots: u64 }
#[event]
pub struct WithdrawalCompletedEvent { pub amount: u64 }
#[event]
pub struct MultisigUpdatedEvent { pub threshold: u8, pub owner_count: usize }
#[event]
pub struct LPIncentivesDistributedEvent { pub total_amount: u64, pub recipient_count: usize }
#[event]
pub struct DexProgramsUpdatedEvent { pub program_count: usize }
#[event]
pub struct WhitelistedContractAddedEvent { pub contract: Pubkey }
#[event]
pub struct WhitelistedContractRemovedEvent { pub contract: Pubkey }
#[event]
pub struct TokensLockedForMigrationEvent { pub user: Pubkey, pub amount: u64 }
#[event]
pub struct TokensUnlockedForMigrationEvent { pub user: Pubkey, pub amount: u64 }
#[event]
pub struct LockedTokensBurnedEvent { pub user: Pubkey, pub amount: u64 }
#[event]
pub struct MigrationToggledEvent { pub active: bool }
#[event]
pub struct StakingRewardsUpdated { pub new_rewards: [u64; 4] }
#[event]
pub struct VotingPowerUpdated { pub old_power: u64, pub new_power: u64 }
#[event]
pub struct TreasuryPoolAdjusted { pub pool_type: PoolType, pub new_amount: u64 }
#[event]
pub struct InitializeEvent { pub launch_timestamp: i64, pub threshold: u8, pub initial_owners: Vec<Pubkey> }
#[event]
pub struct PresalePurchaseEvent { pub buyer: Pubkey, pub usdt_amount: u64, pub token_amount: u64 }
#[event]
pub struct StakeLPEvent { pub user: Pubkey, pub amount: u64 }
#[event]
pub struct UnstakeLPEvent { pub user: Pubkey, pub amount: u64 }
#[event]
pub struct ClaimLPRewardsEvent { pub user: Pubkey, pub amount: u64 }
#[event]
pub struct NoRewardsEvent { pub user: Pubkey, pub reason: String }
#[event]
pub struct InsufficientFundsEvent { pub required: u64, pub available: u64 }
#[event]
pub struct TransferFailedEvent { pub from: Pubkey, pub to: Pubkey, pub reason: String }
#[event]
pub struct MigrationConfirmedEvent { pub user: Pubkey }
#[event]
pub struct MultisigChangeInitiated { pub initiation_time: i64 }
#[event]
pub struct PauseInitiated { pub initiation_time: i64 }
#[event]
pub struct ResumeInitiated { pub initiation_time: i64 }

// Scope-based reentrancy guard
struct ReentrancyGuard<'a> {
    state: &'a mut ContractState,
}

impl<'a> ReentrancyGuard<'a> {
    fn new(state: &'a mut ContractState) -> Self {
        require!(!state.is_processing, ErrorCode::ReentrancyGuardTriggered);
        state.is_processing = true;
        Self { state }
    }
}

impl<'a> Drop for ReentrancyGuard<'a> {
    fn drop(&mut self) {
        self.state.is_processing = false;
    }
}

// Helper Functions
pub fn get_pending_rewards(
    state: &ContractState,
    staker: &Staker,
    treasury: &Treasury,
    current_time: i64,
) -> Result<u64> {
    if staker.amount == 0 || staker.tier == 255 {
        return Ok(0);
    }
    let time_staked = current_time.checked_sub(staker.start_time).ok_or(ErrorCode::ArithmeticOverflow)?;
    let days = (current_time.checked_sub(staker.last_claim).ok_or(ErrorCode::ArithmeticOverflow)? / 86400) as u64;
    if days == 0 {
        return Ok(0);
    }
    let pool_fraction = (treasury.staking_pool as u128 * 1000)
        .checked_div(TREASURY_RESERVE as u128 * STAKING_POOL_PCT as u128 / 100)
        .ok_or(ErrorCode::ArithmeticOverflow)?;
    let reduction_factor = if pool_fraction < state.reduction_thresholds[0] { state.reduction_factors[0] }
    else if pool_fraction < state.reduction_thresholds[1] { state.reduction_factors[1] }
    else if pool_fraction < state.reduction_thresholds[2] { state.reduction_factors[2] }
    else { state.reduction_factors[3] };
    let base_reward = state.staking_rewards[staker.tier as usize] / 7;
    let reward = ((base_reward as u128 * reduction_factor * days as u128) / 1000)
        .try_into()
        .map_err(|_| ErrorCode::ArithmeticOverflow)?;
    Ok(reward)
}

fn advance_buckets(record: &mut TransactionRecord, now: i64) -> Result<()> {
    const HOUR_SECONDS: i64 = 3600;
    const DAY_HOURS: usize = 24;

    if record.bucket_start_time == 0 {
        record.bucket_start_time = now - (now % HOUR_SECONDS);
        record.current_bucket_index = 0;
        return Ok(());
    }

    let elapsed_seconds = now.checked_sub(record.bucket_start_time)
        .ok_or(ErrorCode::ArithmeticOverflow)?;
    let hours_passed = (elapsed_seconds / HOUR_SECONDS) as u64;

    if hours_passed == 0 {
        return Ok(());
    }

    if hours_passed >= DAY_HOURS as u64 {
        record.sell_buckets = [0; DAY_HOURS];
        record.transfer_buckets = [0; DAY_HOURS];
        record.current_bucket_index = 0;
        record.bucket_start_time = now - (now % HOUR_SECONDS);
    } else {
        let mut index = record.current_bucket_index as usize;
        let steps = hours_passed as usize;
        for _ in 0..steps {
            index = (index + 1) % DAY_HOURS;
            record.sell_buckets[index] = 0;
            record.transfer_buckets[index] = 0;
        }
        record.current_bucket_index = index as u8;
        record.bucket_start_time = record.bucket_start_time
            .checked_add((hours_passed as i64) * HOUR_SECONDS)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
    }

    Ok(())
}

// Updated Voting Power Functions
pub fn calculate_tier(state: &ContractState, amount: u64, time_staked: i64) -> Result<u8> {
    if amount < 20_000 * 10u64.pow(TOKEN_DECIMALS) || time_staked < 7 * 86400 {
        return Ok(255); // Ineligible for any tier
    }
    for i in (0..4).rev() {
        if amount >= state.staking_tiers[i] && time_staked >= STAKING_DURATIONS[i] {
            return Ok(i as u8);
        }
    }
    Ok(0) // Meets minimum for tier 0
}

fn get_highest_eligible_tier(state: &ContractState, amount: u64) -> u8 {
    for i in (0..4).rev() {
        if amount >= state.staking_tiers[i] {
            return i as u8;
        }
    }
    255 // Ineligible if below minimum
}

pub fn calculate_voting_power(staker: &Staker, state: &ContractState, current_time: i64) -> Result<u64> {
    if staker.tier == 255 {
        return Ok(0); // Ineligible stakers have no voting power
    }
    let base_voting_power = match staker.tier {
        0 => 0,  // Tier 0 has no votes
        1 => 1,
        2 => 4,
        3 => 20,
        _ => return Err(ErrorCode::InvalidTier.into()),
    };
    let time_staked = current_time.checked_sub(staker.start_time).ok_or(ErrorCode::ArithmeticOverflow)?;
    let multiplier = if time_staked >= 90 * 86400 {
        1995 // 1.5 * 1.33 ≈ 1.995, scaled by 1000, rounded up
    } else if time_staked >= 60 * 86400 {
        1500 // 1.5, scaled by 1000
    } else {
        1000 // 1.0, scaled by 1000
    };
    let voting_power = ((base_voting_power as u128 * multiplier as u128) + 999) / 1000; // Ceiling division
    let highest_tier = get_highest_eligible_tier(state, staker.amount);
    if highest_tier == 255 {
        return Ok(0); // Below minimum stake
    }
    let cap_base = match highest_tier {
        0 => 0,
        1 => 1,
        2 => 4,
        3 => 20,
        _ => return Err(ErrorCode::InvalidTier.into()),
    };
    let cap = ((cap_base as u128 * multiplier as u128) + 999) / 1000; // Ceiling division
    let final_voting_power = voting_power.min(cap);
    Ok(final_voting_power as u64)
}

// Instruction Implementations
pub fn initialize(
    ctx: Context<Initialize>,
    charity_wallet: Pubkey,
    team_wallet: Pubkey,
    liquidity_pool: Pubkey,
    launch_timestamp: i64,
    initial_owners: Vec<Pubkey>,
    threshold: u8,
    initial_dex_programs: Vec<Pubkey>,
    presale_usdt_receiver: Pubkey,
) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    require!(threshold >= 2, ErrorCode::InvalidThreshold);
    require!(initial_owners.len() >= 3 && initial_owners.len() <= Multisig::MAX_OWNERS, ErrorCode::TooFewOwners);
    let unique_owners: HashSet<Pubkey> = initial_owners.iter().cloned().collect();
    require!(unique_owners.len() == initial_owners.len(), ErrorCode::DuplicateOwners);

    validate_multisig(&ctx.accounts.multisig, &[
        &Some(&ctx.accounts.signer1),
        ctx.accounts.signer2.as_ref(),
        ctx.accounts.signer3.as_ref(),
        ctx.accounts.signer4.as_ref(),
        ctx.accounts.signer5.as_ref(),
    ])?;

    require!(ctx.remaining_accounts.len() == initial_dex_programs.len(), ErrorCode::InvalidAccounts);
    for (i, program_id) in initial_dex_programs.iter().enumerate() {
        let account_info = &ctx.remaining_accounts[i];
        require!(account_info.key == *program_id && account_info.executable, ErrorCode::InvalidDexProgram);
    }

    let state = &mut ctx.accounts.state;
    require!(initial_dex_programs.len() <= ContractState::MAX_DEXES, ErrorCode::VectorOverflow);
    state.dex_programs = initial_dex_programs;
    state.paused = false;
    state.pause_reason = String::new();
    state.treasury = ctx.accounts.treasury.key();
    state.charity_wallet = charity_wallet;
    state.team_wallet = team_wallet;
    state.liquidity_pool = liquidity_pool;
    state.proposal_count = 0;
    state.total_voting_power = 0;
    state.launch_timestamp = launch_timestamp;
    state.admin = ctx.accounts.multisig.key();
    state.tax_rate = INITIAL_TAX_RATE;
    state.staking_tiers = INITIAL_STAKING_TIERS;
    state.whitelisted_contracts = Vec::new();
    state.allowed_destinations = Vec::new(); // Initialize empty
    state.burn_alloc = 2000;
    state.treasury_alloc = 2000;
    state.liquidity_pool_alloc = 2400;
    state.lp_incentive_alloc = 600;
    state.charity_alloc = 2000;
    state.team_alloc = 1000;
    state.reduction_thresholds = [250, 500, 750];
    state.reduction_factors = [512, 640, 800, 1000];
    state.max_sell_txn_limit = MAX_SELL_TXN_LIMIT;
    state.daily_sell_limit = DAILY_SELL_LIMIT;
    state.max_transfer_limit = MAX_TRANSFER_LIMIT;
    state.daily_transfer_limit = DAILY_TRANSFER_LIMIT;
    state.progressive_tax_threshold = PROGRESSIVE_TAX_THRESHOLD;
    state.staking_rewards = STAKING_REWARDS;
    state.presale_usdt_receiver = presale_usdt_receiver;
    state.presale_active = true;
    state.total_presale_sold = 0;
    state.last_processed_day = 0;
    state.is_processing = false;
    state.last_processed_index = 0;

    let treasury = &mut ctx.accounts.treasury;
    let total_treasury = TREASURY_RESERVE;
    let staking_pool = (total_treasury as u128 * STAKING_POOL_PCT as u128 / 100)
        .try_into()
        .map_err(|_| ErrorCode::ArithmeticOverflow)?;
    let airdrop_pool = (total_treasury as u128 * AIRDROP_POOL_PCT as u128 / 100)
        .try_into()
        .map_err(|_| ErrorCode::ArithmeticOverflow)?;
    let governance_reserve = (total_treasury as u128 * GOVERNANCE_RESERVE_PCT as u128 / 100)
        .try_into()
        .map_err(|_| ErrorCode::ArithmeticOverflow)?;
    let marketing_fund = (total_treasury as u128 * MARKETING_FUND_PCT as u128 / 100)
        .try_into()
        .map_err(|_| ErrorCode::ArithmeticOverflow)?;
    let emergency_fund = (total_treasury as u128 * EMERGENCY_FUND_PCT as u128 / 100)
        .try_into()
        .map_err(|_| ErrorCode::ArithmeticOverflow)?;
    let team_pool = (total_treasury as u128 * TEAM_POOL_PCT as u128 / 100)
        .try_into()
        .map_err(|_| ErrorCode::ArithmeticOverflow)?;
    treasury.staking_pool = staking_pool;
    treasury.airdrop_pool = airdrop_pool;
    treasury.governance_reserve = governance_reserve;
    treasury.marketing_fund = marketing_fund;
    treasury.emergency_fund = emergency_fund;
    treasury.team_pool = team_pool;
    treasury.liquidity_incentive = 0;

    let multisig = &mut ctx.accounts.multisig;
    require!(initial_owners.len() <= Multisig::MAX_OWNERS, ErrorCode::InvalidVectorSize);
    require!(threshold as usize <= initial_owners.len(), ErrorCode::InvalidProposal);
    multisig.owners = initial_owners.clone();
    multisig.threshold = threshold;

    let migration_state = &mut ctx.accounts.migration_state;
    migration_state.total_locked = 0;
    migration_state.migration_active = false;
    migration_state.migration_toggle_timestamp = 0;

    emit!(InitializeEvent {
        launch_timestamp,
        threshold,
        initial_owners,
    });

    Ok(())
}

pub fn buy_presale(ctx: Context<BuyPresale>, usdt_amount: u64) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);
    require!(state.presale_active, ErrorCode::PresaleNotActive);

    let purchase = &mut ctx.accounts.presale_purchase;
    let vesting = &mut ctx.accounts.presale_vesting;

    let token_amount = ((usdt_amount as u128 * 10u64.pow(TOKEN_DECIMALS) as u128) / PRESALE_PRICE_PER_TOKEN as u128)
        .try_into()
        .map_err(|_| ErrorCode::ArithmeticOverflow)?;

    // Atomic check and update for presale supply
    require!(
        state.total_presale_sold + token_amount <= PRESALE_SUPPLY,
        ErrorCode::PresaleSupplyExceeded
    );
    // KYC check for purchases >= $1000 USDT
    require!(
        usdt_amount < 1000 || purchase.kyc_verified,
        ErrorCode::KYCRequired
    );
    require!(
        purchase.total_purchased.checked_add(token_amount).ok_or(ErrorCode::ArithmeticOverflow)? <= PRESALE_MAX_PER_WALLET,
        ErrorCode::PresaleMaxPerWalletExceeded
    );

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.buyer_usdt_account.to_account_info(),
                to: ctx.accounts.presale_usdt_account.to_account_info(),
                authority: ctx.accounts.buyer.to_account_info(),
            },
        ),
        usdt_amount,
    )?;

    purchase.wallet = ctx.accounts.buyer.key();
    purchase.total_purchased = purchase.total_purchased.checked_add(token_amount).ok_or(ErrorCode::ArithmeticOverflow)?;
    vesting.total_amount = vesting.total_amount.checked_add(token_amount).ok_or(ErrorCode::ArithmeticOverflow)?;
    state.total_presale_sold = state.total_presale_sold.checked_add(token_amount).ok_or(ErrorCode::ArithmeticOverflow)?;

    emit!(PresalePurchaseEvent {
        buyer: ctx.accounts.buyer.key(),
        usdt_amount,
        token_amount,
    });

    Ok(())
}

pub fn claim_presale_tokens(ctx: Context<ClaimPresaleTokens>) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    let vesting = &mut ctx.accounts.vesting;
    require!(vesting.total_amount > 0, ErrorCode::UninitializedAccount);
    let clock = Clock::get()?;
    let time_since_launch = clock.unix_timestamp.checked_sub(state.launch_timestamp).ok_or(ErrorCode::ArithmeticOverflow)?;
    require!(time_since_launch >= 0, ErrorCode::VestingNotStarted);
    let weeks_passed = (time_since_launch / (7 * 86400)) as u64;
    let unlock_percent = min(10 + 10 * weeks_passed, 100);
    let claimable = (vesting.total_amount as u128 * unlock_percent as u128 / 100)
        .checked_sub(vesting.claimed_amount as u128)
        .ok_or(ErrorCode::ArithmeticOverflow)?
        .try_into()
        .map_err(|_| ErrorCode::ArithmeticOverflow)?;
    
    if claimable == 0 {
        emit!(NoRewardsEvent { 
            user: ctx.accounts.user.key(), 
            reason: "No tokens available to claim".to_string() 
        });
        return Ok(());
    }

    vesting.claimed_amount = vesting.claimed_amount.checked_add(claimable).ok_or(ErrorCode::ArithmeticOverflow)?;
    let signer_seeds = &[&[b"treasury_authority", &[ctx.bumps.treasury_pda]]];
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.treasury_token_account.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.treasury_pda.to_account_info(),
            },
            signer_seeds,
        ),
        claimable,
    )?;
    emit!(ClaimRewardsEvent {
        user: ctx.accounts.user.key(),
        amount: claimable,
    });

    Ok(())
}

pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused || amount == 0, ErrorCode::Paused); // Allow zero-amount transfers when paused

    let clock = Clock::get()?;
    let record = &mut ctx.accounts.txn_record;
    let sender_key = ctx.accounts.sender.key();
    require!(sender_key == ctx.accounts.sender_token_account.owner, ErrorCode::UnauthorizedSender);
    require!(
        clock.unix_timestamp.checked_sub(record.last_txn_time).ok_or(ErrorCode::ArithmeticOverflow)? >= TXN_COOLDOWN,
        ErrorCode::CooldownActive
    );

    let is_sell = state.dex_programs.iter().any(|&dex| {
        dex == ctx.accounts.recipient_token_account.owner || 
        dex == ctx.accounts.recipient_token_account.to_account_info().owner
    });
    advance_buckets(record, clock.unix_timestamp)?;
    let bucket_idx = record.current_bucket_index as usize;

    let (current_sell, current_transfer) = if is_sell {
        (record.sell_buckets[bucket_idx], record.transfer_buckets[bucket_idx])
    } else {
        (record.sell_buckets[bucket_idx], record.transfer_buckets[bucket_idx])
    };

    let daily_sell = record.sell_buckets.iter().sum::<u64>();
    let daily_transfer = record.transfer_buckets.iter().sum::<u64>();

    if is_sell {
        require!(amount <= state.max_sell_txn_limit, ErrorCode::MaxSellTxnLimitExceeded);
        require!(daily_sell.checked_add(amount).ok_or(ErrorCode::ArithmeticOverflow)? <= state.daily_sell_limit, ErrorCode::DailySellLimitExceeded);
        record.sell_buckets[bucket_idx] = current_sell.checked_add(amount).ok_or(ErrorCode::ArithmeticOverflow)?;
    } else {
        require!(amount <= state.max_transfer_limit, ErrorCode::MaxTransferLimitExceeded);
        require!(daily_transfer.checked_add(amount).ok_or(ErrorCode::ArithmeticOverflow)? <= state.daily_transfer_limit, ErrorCode::DailyTransferLimitExceeded);
        record.transfer_buckets[bucket_idx] = current_transfer.checked_add(amount).ok_or(ErrorCode::ArithmeticOverflow)?;
    }

    let is_whitelisted = state.whitelisted_contracts.iter().any(|&(contract, _)| contract == ctx.accounts.recipient_token_account.owner);
    let tax_rate = if is_whitelisted {
        state.tax_rate / 2
    } else if amount >= state.progressive_tax_threshold {
        state.tax_rate * 3
    } else {
        state.tax_rate
    };
    let tax = ((amount as u128 * tax_rate as u128 * 10u64.pow(7) as u128 + 9999_999) / (10000 * 10u64.pow(7) as u128))
        .try_into()
        .map_err(|_| ErrorCode::ArithmeticOverflow)?;
    if amount <= tax {
        emit!(TransferFailedEvent {
            from: sender_key,
            to: ctx.accounts.recipient_token_account.owner,
            reason: "Amount too small after tax".to_string(),
        });
        return Err(ErrorCode::AmountTooSmallAfterTax.into());
    }

    let burn = ((tax as u128 * state.burn_alloc as u128 * 10u64.pow(7) as u128 + 9999_999) / (10000 * 10u64.pow(7) as u128))
        .try_into()
        .map_err(|_| ErrorCode::ArithmeticOverflow)?;
    let treasury_tax = ((tax as u128 * state.treasury_alloc as u128 * 10u64.pow(7) as u128 + 9999_999) / (10000 * 10u64.pow(7) as u128))
        .try_into()
        .map_err(|_| ErrorCode::ArithmeticOverflow)?;
    let liquidity_pool_tax = ((tax as u128 * state.liquidity_pool_alloc as u128 * 10u64.pow(7) as u128 + 9999_999) / (10000 * 10u64.pow(7) as u128))
        .try_into()
        .map_err(|_| ErrorCode::ArithmeticOverflow)?;
    let lp_incentive_tax = ((tax as u128 * state.lp_incentive_alloc as u128 * 10u64.pow(7) as u128 + 9999_999) / (10000 * 10u64.pow(7) as u128))
        .try_into()
        .map_err(|_| ErrorCode::ArithmeticOverflow)?;
    let charity_tax = ((tax as u128 * state.charity_alloc as u128 * 10u64.pow(7) as u128 + 9999_999) / (10000 * 10u64.pow(7) as u128))
        .try_into()
        .map_err(|_| ErrorCode::ArithmeticOverflow)?;
    let team_tax = ((tax as u128 * state.team_alloc as u128 * 10u64.pow(7) as u128 + 9999_999) / (10000 * 10u64.pow(7) as u128))
        .try_into()
        .map_err(|_| ErrorCode::ArithmeticOverflow)?;

    let treasury = &mut ctx.accounts.treasury;
    treasury.staking_pool = treasury.staking_pool.checked_add(treasury_tax).ok_or(ErrorCode::ArithmeticOverflow)?;
    treasury.liquidity_incentive = treasury.liquidity_incentive.checked_add(lp_incentive_tax).ok_or(ErrorCode::ArithmeticOverflow)?;
    treasury.team_pool = treasury.team_pool.checked_add(team_tax).ok_or(ErrorCode::ArithmeticOverflow)?;

    if burn > 0 {
        token::burn(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Burn {
                    mint: ctx.accounts.token_mint.to_account_info(),
                    from: ctx.accounts.sender_token_account.to_account_info(),
                    authority: ctx.accounts.sender.to_account_info(),
                },
            ),
            burn,
        )?;
    }

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.sender_token_account.to_account_info(),
                to: ctx.accounts.treasury_token_account.to_account_info(),
                authority: ctx.accounts.sender.to_account_info(),
            },
        ),
        treasury_tax,
    )?;

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.sender_token_account.to_account_info(),
                to: ctx.accounts.liquidity_token_account.to_account_info(),
                authority: ctx.accounts.sender.to_account_info(),
            },
        ),
        liquidity_pool_tax,
    )?;

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.sender_token_account.to_account_info(),
                to: ctx.accounts.treasury_token_account.to_account_info(),
                authority: ctx.accounts.sender.to_account_info(),
            },
        ),
        lp_incentive_tax,
    )?;

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.sender_token_account.to_account_info(),
                to: ctx.accounts.charity_token_account.to_account_info(),
                authority: ctx.accounts.sender.to_account_info(),
            },
        ),
        charity_tax,
    )?;

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.sender_token_account.to_account_info(),
                to: ctx.accounts.team_token_account.to_account_info(),
                authority: ctx.accounts.sender.to_account_info(),
            },
        ),
        team_tax,
    )?;

    let net_amount = amount.checked_sub(tax).ok_or(ErrorCode::ArithmeticOverflow)?;
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.sender_token_account.to_account_info(),
                to: ctx.accounts.recipient_token_account.to_account_info(),
                authority: ctx.accounts.sender.to_account_info(),
            },
        ),
        net_amount,
    )?;

    record.last_txn_time = clock.unix_timestamp;
    if amount >= TRANSACTION_TRACKING_THRESHOLD {
        if is_sell && record.sell_cooldown_start == 0 {
            record.sell_cooldown_start = clock.unix_timestamp;
        } else if !is_sell && record.transfer_cooldown_start == 0 {
            record.transfer_cooldown_start = clock.unix_timestamp;
        }
    }

    emit!(TransferEvent {
        from: sender_key,
        to: ctx.accounts.recipient_token_account.owner,
        amount: net_amount,
        tax,
        burn,
        treasury_tax,
        liquidity_pool_tax,
        lp_incentive_tax,
        charity_tax,
        team_tax,
    });

    Ok(())
}

pub fn whitelisted_transfer(ctx: Context<WhitelistedTransfer>, amount: u64) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    // Runtime checks for whitelisted_transfer
    let caller_program_id = ctx.accounts.caller_program.key();
    require!(
        state.whitelisted_contracts.iter().any(|&(contract, _)| contract == caller_program_id),
        ErrorCode::CallerNotWhitelisted
    );
    let transfer_destination = ctx.accounts.recipient_token_account.owner;
    require!(
        state.allowed_destinations.contains(&transfer_destination),
        ErrorCode::InvalidDestination
    );
    let stored_hash = state.whitelisted_contracts.iter()
        .find(|&(contract, _)| contract == caller_program_id)
        .unwrap()
        .1;
    let expected_hash = caller_program_id.to_bytes(); // Using program ID as version identifier
    require!(stored_hash == expected_hash, ErrorCode::VersionMismatch);

    let treasury = &mut ctx.accounts.treasury;
    let tax_rate = state.tax_rate / 2;
    let tax = ((amount as u128 * tax_rate as u128 * 10u64.pow(7) as u128 + 9999_999) / (10000 * 10u64.pow(7) as u128))
        .try_into()
        .map_err(|_| ErrorCode::ArithmeticOverflow)?;
    if amount <= tax {
        emit!(TransferFailedEvent {
            from: ctx.accounts.sender_token_account.owner,
            to: ctx.accounts.recipient_token_account.owner,
            reason: "Amount too small after tax".to_string(),
        });
        return Err(ErrorCode::AmountTooSmallAfterTax.into());
    }

    let burn = ((tax as u128 * state.burn_alloc as u128 * 10u64.pow(7) as u128 + 9999_999) / (10000 * 10u64.pow(7) as u128))
        .try_into()
        .map_err(|_| ErrorCode::ArithmeticOverflow)?;
    let treasury_tax = ((tax as u128 * state.treasury_alloc as u128 * 10u64.pow(7) as u128 + 9999_999) / (10000 * 10u64.pow(7) as u128))
        .try_into()
        .map_err(|_| ErrorCode::ArithmeticOverflow)?;
    let liquidity_pool_tax = ((tax as u128 * state.liquidity_pool_alloc as u128 * 10u64.pow(7) as u128 + 9999_999) / (10000 * 10u64.pow(7) as u128))
        .try_into()
        .map_err(|_| ErrorCode::ArithmeticOverflow)?;
    let lp_incentive_tax = ((tax as u128 * state.lp_incentive_alloc as u128 * 10u64.pow(7) as u128 + 9999_999) / (10000 * 10u64.pow(7) as u128))
        .try_into()
        .map_err(|_| ErrorCode::ArithmeticOverflow)?;
    let charity_tax = ((tax as u128 * state.charity_alloc as u128 * 10u64.pow(7) as u128 + 9999_999) / (10000 * 10u64.pow(7) as u128))
        .try_into()
        .map_err(|_| ErrorCode::ArithmeticOverflow)?;
    let team_tax = ((tax as u128 * state.team_alloc as u128 * 10u64.pow(7) as u128 + 9999_999) / (10000 * 10u64.pow(7) as u128))
        .try_into()
        .map_err(|_| ErrorCode::ArithmeticOverflow)?;

    treasury.staking_pool = treasury.staking_pool.checked_add(treasury_tax).ok_or(ErrorCode::ArithmeticOverflow)?;
    treasury.liquidity_incentive = treasury.liquidity_incentive.checked_add(lp_incentive_tax).ok_or(ErrorCode::ArithmeticOverflow)?;
    treasury.team_pool = treasury.team_pool.checked_add(team_tax).ok_or(ErrorCode::ArithmeticOverflow)?;

    let signer_seeds = &[&[b"whitelisted_authority", ctx.accounts.caller_program.key().as_ref(), &[ctx.bumps.whitelisted_pda]]];

    if burn > 0 {
        token::burn(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Burn {
                    mint: ctx.accounts.token_mint.to_account_info(),
                    from: ctx.accounts.sender_token_account.to_account_info(),
                    authority: ctx.accounts.whitelisted_pda.to_account_info(),
                },
                signer_seeds,
            ),
            burn,
        )?;
    }

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.sender_token_account.to_account_info(),
                to: ctx.accounts.treasury_token_account.to_account_info(),
                authority: ctx.accounts.whitelisted_pda.to_account_info(),
            },
            signer_seeds,
        ),
        treasury_tax,
    )?;

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.sender_token_account.to_account_info(),
                to: ctx.accounts.liquidity_token_account.to_account_info(),
                authority: ctx.accounts.whitelisted_pda.to_account_info(),
            },
            signer_seeds,
        ),
        liquidity_pool_tax,
    )?;

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.sender_token_account.to_account_info(),
                to: ctx.accounts.treasury_token_account.to_account_info(),
                authority: ctx.accounts.whitelisted_pda.to_account_info(),
            },
            signer_seeds,
        ),
        lp_incentive_tax,
    )?;

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.sender_token_account.to_account_info(),
                to: ctx.accounts.charity_token_account.to_account_info(),
                authority: ctx.accounts.whitelisted_pda.to_account_info(),
            },
            signer_seeds,
        ),
        charity_tax,
    )?;

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.sender_token_account.to_account_info(),
                to: ctx.accounts.team_token_account.to_account_info(),
                authority: ctx.accounts.whitelisted_pda.to_account_info(),
            },
            signer_seeds,
        ),
        team_tax,
    )?;

    let net_amount = amount.checked_sub(tax).ok_or(ErrorCode::ArithmeticOverflow)?;
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.sender_token_account.to_account_info(),
                to: ctx.accounts.recipient_token_account.to_account_info(),
                authority: ctx.accounts.whitelisted_pda.to_account_info(),
            },
            signer_seeds,
        ),
        net_amount,
    )?;

    emit!(TransferEvent {
        from: ctx.accounts.sender_token_account.owner,
        to: ctx.accounts.recipient_token_account.owner,
        amount: net_amount,
        tax,
        burn,
        treasury_tax,
        liquidity_pool_tax,
        lp_incentive_tax,
        charity_tax,
        team_tax,
    });

    Ok(())
}

pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    let clock = Clock::get()?;
    let staker = &mut ctx.accounts.staker;
    let old_voting_power = if staker.start_time == 0 { 
        0 
    } else { 
        calculate_voting_power(staker, state, clock.unix_timestamp)? 
    };
    if staker.start_time == 0 {
        staker.start_time = clock.unix_timestamp;
        staker.last_claim = clock.unix_timestamp;
        staker.amount = amount;
    } else {
        staker.amount = staker.amount.checked_add(amount).ok_or(ErrorCode::ArithmeticOverflow)?;
    }
    let time_staked = clock.unix_timestamp.checked_sub(staker.start_time).ok_or(ErrorCode::ArithmeticOverflow)?;
    staker.tier = calculate_tier(state, staker.amount, time_staked)?;
    let new_voting_power = calculate_voting_power(staker, state, clock.unix_timestamp)?;
    let old_total = state.total_voting_power;
    state.total_voting_power = old_total
        .checked_sub(old_voting_power)
        .ok_or(ErrorCode::ArithmeticOverflow)?
        .checked_add(new_voting_power)
        .ok_or(ErrorCode::ArithmeticOverflow)?;
    emit!(VotingPowerUpdated { old_power: old_total, new_power: state.total_voting_power });
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.staking_token_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount,
    )?;
    emit!(StakeEvent {
        user: ctx.accounts.user.key(),
        amount,
        tier: staker.tier,
    });

    Ok(())
}

pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    let clock = Clock::get()?;
    let staker = &mut ctx.accounts.staker;
    let treasury = &mut ctx.accounts.treasury;

    let time_staked = clock.unix_timestamp.checked_sub(staker.start_time).ok_or(ErrorCode::ArithmeticOverflow)?;
    require!(time_staked >= STAKING_DURATIONS[staker.tier as usize], ErrorCode::LockPeriodNotMet);

    let pending_reward = get_pending_rewards(state, staker, treasury, clock.unix_timestamp)?;
    if pending_reward > 0 {
        require!(treasury.staking_pool >= pending_reward, ErrorCode::InsufficientStakingPoolFunds);
        treasury.staking_pool = treasury.staking_pool.checked_sub(pending_reward).ok_or(ErrorCode::ArithmeticOverflow)?;
        emit!(TreasuryPoolAdjusted { pool_type: PoolType::Staking, new_amount: treasury.staking_pool });
        let signer_seeds = &[&[b"treasury_authority", &[ctx.bumps.treasury_pda]]];
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TokenTransfer {
                    from: ctx.accounts.treasury_token_account.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.treasury_pda.to_account_info(),
                },
                signer_seeds,
            ),
            pending_reward,
        )?;
        emit!(ClaimRewardsEvent {
            user: ctx.accounts.user.key(),
            amount: pending_reward,
        });
        staker.last_claim = clock.unix_timestamp;
    } else {
        emit!(NoRewardsEvent {
            user: ctx.accounts.user.key(),
            reason: "No pending rewards".to_string(),
        });
    }

    let current_voting_power = calculate_voting_power(staker, state, clock.unix_timestamp)?;
    let amount = staker.amount;
    staker.amount = 0;
    staker.tier = 0;
    staker.start_time = 0;
    staker.last_claim = 0;
    let old_total = state.total_voting_power;
    state.total_voting_power = old_total.checked_sub(current_voting_power).ok_or(ErrorCode::ArithmeticOverflow)?;
    emit!(VotingPowerUpdated { old_power: old_total, new_power: state.total_voting_power });
    let signer_seeds = &[&[b"staking_authority", &[ctx.bumps.staking_pda]]];
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.staking_token_account.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.staking_pda.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )?;
    emit!(UnstakeEvent {
        user: ctx.accounts.user.key(),
        amount,
    });

    Ok(())
}

pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    let clock = Clock::get()?;
    let staker = &mut ctx.accounts.staker;
    let treasury = &mut ctx.accounts.treasury;

    // Explicit duration check
    let time_staked = clock.unix_timestamp.checked_sub(staker.start_time).ok_or(ErrorCode::ArithmeticOverflow)?;
    require!(
        time_staked >= STAKING_DURATIONS[staker.tier as usize],
        ErrorCode::LockPeriodNotMet
    );

    let rewards = get_pending_rewards(state, staker, treasury, clock.unix_timestamp)?;
    if rewards == 0 {
        emit!(NoRewardsEvent {
            user: ctx.accounts.user.key(),
            reason: "No rewards available".to_string(),
        });
        return Ok(());
    }

    require!(treasury.staking_pool >= rewards, ErrorCode::InsufficientStakingPoolFunds);
    treasury.staking_pool = treasury.staking_pool.checked_sub(rewards).ok_or(ErrorCode::ArithmeticOverflow)?;
    emit!(TreasuryPoolAdjusted { pool_type: PoolType::Staking, new_amount: treasury.staking_pool });

    let signer_seeds = &[&[b"treasury_authority", &[ctx.bumps.treasury_pda]]];
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.treasury_token_account.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.treasury_pda.to_account_info(),
            },
            signer_seeds,
        ),
        rewards,
    )?;

    staker.last_claim = clock.unix_timestamp;
    emit!(ClaimRewardsEvent {
        user: ctx.accounts.user.key(),
        amount: rewards,
    });

    Ok(())
}

pub fn stake_lp(ctx: Context<StakeLP>, amount: u64) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp;

    let lp_staker = &mut ctx.accounts.lp_staker;
    if lp_staker.amount == 0 {
        lp_staker.last_action_time = current_time;
    }
    lp_staker.amount = lp_staker.amount.checked_add(amount).ok_or(ErrorCode::ArithmeticOverflow)?;

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.user_lp_token_account.to_account_info(),
                to: ctx.accounts.staking_lp_token_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount,
    )?;

    emit!(StakeLPEvent {
        user: ctx.accounts.user.key(),
        amount,
    });

    Ok(())
}

pub fn unstake_lp(ctx: Context<UnstakeLP>, amount: u64) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp;

    let lp_staker = &mut ctx.accounts.lp_staker;
    require!(lp_staker.amount >= amount, ErrorCode::InsufficientStakedAmount);
    require!(
        current_time - lp_staker.last_action_time >= 7 * 86400,
        ErrorCode::LockPeriodNotMet
    );

    lp_staker.amount = lp_staker.amount.checked_sub(amount).ok_or(ErrorCode::ArithmeticOverflow)?;
    lp_staker.last_action_time = current_time;

    let signer_seeds = &[&[b"lp_staking_authority", &[ctx.bumps.lp_staking_pda]]];
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.staking_lp_token_account.to_account_info(),
                to: ctx.accounts.user_lp_token_account.to_account_info(),
                authority: ctx.accounts.lp_staking_pda.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )?;

    if lp_staker.amount == 0 {
        let user_account = ctx.accounts.user.to_account_info();
        let lp_staker_account = lp_staker.to_account_info();
        let rent_lamports = Rent::get()?.minimum_balance(LPStaker::LEN);
        let lamports = lp_staker_account.lamports();
        if lamports > rent_lamports {
            **user_account.try_borrow_mut_lamports()? += lamports - rent_lamports;
            **lp_staker_account.try_borrow_mut_lamports()? = rent_lamports;
        }
        lp_staker_account.assign(&anchor_lang::system_program::ID);
        lp_staker_account.realloc(0, false)?;
    }

    emit!(UnstakeLPEvent {
        user: ctx.accounts.user.key(),
        amount,
    });

    Ok(())
}

pub fn claim_lp_rewards(ctx: Context<ClaimLPRewards>) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp;
    let lp_staker = &mut ctx.accounts.lp_staker;

    require!(
        current_time - lp_staker.last_action_time >= 7 * 86400,
        ErrorCode::LockPeriodNotMet
    );
    let rewards = lp_staker.unclaimed_rewards;
    if rewards == 0 {
        emit!(NoRewardsEvent {
            user: ctx.accounts.user.key(),
            reason: "No unclaimed rewards available".to_string(),
        });
        return Ok(());
    }

    lp_staker.unclaimed_rewards = 0;

    let signer_seeds = &[&[b"lp_reward_holding", &[ctx.bumps.holding_pool_pda]]];
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.holding_pool_token_account.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.holding_pool_pda.to_account_info(),
            },
            signer_seeds,
        ),
        rewards,
    )?;

    emit!(ClaimLPRewardsEvent {
        user: ctx.accounts.user.key(),
        amount: rewards,
    });

    Ok(())
}

pub fn process_daily_rewards(ctx: Context<ProcessDailyRewards>, batch_size: u64) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    const MAX_BATCH_SIZE: u64 = 50; // Optimized for Solana compute budget
    require!(batch_size <= MAX_BATCH_SIZE, ErrorCode::BatchSizeTooLarge);

    let treasury = &mut ctx.accounts.treasury;
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp;
    let current_day_start = (current_time / 86400) * 86400;

    if state.last_processed_day >= current_day_start {
        emit!(NoRewardsEvent {
            user: Pubkey::default(),
            reason: "Rewards already processed for today".to_string(),
        });
        return Ok(());
    }

    let daily_reward = treasury.liquidity_incentive;
    if daily_reward == 0 {
        state.last_processed_day = current_day_start;
        emit!(NoRewardsEvent {
            user: Pubkey::default(),
            reason: "No liquidity incentives available".to_string(),
        });
        return Ok(());
    }

    let stakers = &ctx.remaining_accounts;
    let start_idx = state.last_processed_index as usize;
    let end_idx = (start_idx + batch_size as usize).min(stakers.len());
    let mut total_eligible_stake = 0;

    for i in start_idx..end_idx {
        let lp_staker_info = &stakers[i];
        let lp_staker: Account<LPStaker> = Account::try_from(lp_staker_info)?;
        if lp_staker.last_action_time < current_day_start - 7 * 86400 {
            total_eligible_stake += lp_staker.amount;
        }
    }

    if total_eligible_stake == 0 {
        if end_idx == stakers.len() {
            state.last_processed_day = current_day_start;
            state.last_processed_index = 0;
            emit!(NoRewardsEvent {
                user: Pubkey::default(),
                reason: "No eligible stakers in batch".to_string(),
            });
        } else {
            state.last_processed_index = end_idx as u64;
            emit!(NoRewardsEvent {
                user: Pubkey::default(),
                reason: "No eligible stakers in current batch".to_string(),
            });
        }
        return Ok(());
    }

    let signer_seeds = &[&[b"treasury_authority", &[ctx.bumps.treasury_pda]]];
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.treasury_token_account.to_account_info(),
                to: ctx.accounts.holding_pool_token_account.to_account_info(),
                authority: ctx.accounts.treasury_pda.to_account_info(),
            },
            signer_seeds,
        ),
        daily_reward,
    )?;

    for i in start_idx..end_idx {
        let lp_staker_info = &mut stakers[i];
        let mut lp_staker: LPStaker = Account::try_from(lp_staker_info)?;
        if lp_staker.last_action_time < current_day_start - 7 * 86400 {
            let user_share = (lp_staker.amount as u128 * daily_reward as u128 / total_eligible_stake as u128)
                .try_into()
                .map_err(|_| ErrorCode::ArithmeticOverflow)?;
            lp_staker.unclaimed_rewards = lp_staker.unclaimed_rewards
                .checked_add(user_share)
                .ok_or(ErrorCode::ArithmeticOverflow)?;
            let mut data = lp_staker_info.try_borrow_mut_data()?;
            let mut writer = &mut *data;
            lp_staker.try_serialize(&mut writer)?;
        }
    }

    state.last_processed_index = end_idx as u64;
    if end_idx == stakers.len() {
        treasury.liquidity_incentive = 0;
        state.last_processed_day = current_day_start;
        state.last_processed_index = 0;
    }

    Ok(())
}

pub fn submit_proposal(ctx: Context<SubmitProposal>, description: String, proposal_type: u8, proposal_values: Vec<u64>) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    let multisig = &ctx.accounts.multisig;
    require!(multisig.owners.contains(&ctx.accounts.signer1.key()), ErrorCode::Unauthorized);
    validate_multisig(multisig, &[
        &Some(&ctx.accounts.signer1),
        ctx.accounts.signer2.as_ref(),
        ctx.accounts.signer3.as_ref(),
        ctx.accounts.signer4.as_ref(),
        ctx.accounts.signer5.as_ref(),
    ])?;

    require!(description.len() <= Proposal::MAX_DESCRIPTION_LEN, ErrorCode::DescriptionTooLong);
    require!(proposal_values.len() <= Proposal::MAX_PROPOSAL_VALUES, ErrorCode::TooManyProposalValues);

    let proposal = &mut ctx.accounts.proposal;
    let clock = Clock::get()?;
    proposal.id = state.proposal_count;
    proposal.description = description.clone();
    proposal.votes_for = 0;
    proposal.votes_against = 0;
    proposal.status = 0;
    proposal.start_time = clock.unix_timestamp;
    proposal.end_time = clock.unix_timestamp.checked_add(14 * 86400).ok_or(ErrorCode::ArithmeticOverflow)?;
    proposal.execution_time = proposal.end_time.checked_add(3 * 86400).ok_or(ErrorCode::ArithmeticOverflow)?;
    proposal.proposal_type = proposal_type;
    proposal.proposal_values = proposal_values;
    state.proposal_count = state.proposal_count.checked_add(1).ok_or(ErrorCode::ArithmeticOverflow)?;

    emit!(ProposalSubmittedEvent {
        proposal_id: proposal.id,
        description,
        proposal_type,
    });

    Ok(())
}

pub fn vote(ctx: Context<Vote>, proposal_id: u64, in_favor: bool) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    let proposal = &mut ctx.accounts.proposal;
    let staker = &ctx.accounts.staker;
    let vote_record = &mut ctx.accounts.vote_record;
    let clock = Clock::get()?;
    require!(staker.tier != 255, ErrorCode::InsufficientTierForVoting); // Must be eligible
    require!(staker.tier >= 1, ErrorCode::InsufficientTierForVoting); // Tier 0 has no votes
    require!(proposal.status == 0 && clock.unix_timestamp <= proposal.end_time, ErrorCode::ProposalExpired);
    let power = calculate_voting_power(staker, state, clock.unix_timestamp)?;
    if vote_record.voted {
        if vote_record.in_favor {
            proposal.votes_for = proposal.votes_for.checked_sub(power).ok_or(ErrorCode::ArithmeticOverflow)?;
        } else {
            proposal.votes_against = proposal.votes_against.checked_sub(power).ok_or(ErrorCode::ArithmeticOverflow)?;
        }
    }
    if in_favor {
        proposal.votes_for = proposal.votes_for.checked_add(power).ok_or(ErrorCode::ArithmeticOverflow)?;
    } else {
        proposal.votes_against = proposal.votes_against.checked_add(power).ok_or(ErrorCode::ArithmeticOverflow)?;
    }
    vote_record.staker = ctx.accounts.user.key();
    vote_record.proposal_id = proposal_id;
    vote_record.voted = true;
    vote_record.in_favor = in_favor;
    emit!(VoteEvent {
        staker: ctx.accounts.user.key(),
        proposal_id,
        in_favor,
        power,
    });

    Ok(())
}

pub fn execute_proposal(ctx: Context<ExecuteProposal>, proposal_id: u64) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    let proposal = &mut ctx.accounts.proposal;
    let clock = Clock::get()?;
    require!(proposal.id == proposal_id, ErrorCode::InvalidProposalType);
    require!(proposal.status == 0, ErrorCode::ProposalAlreadyExecuted);
    require!(clock.unix_timestamp > proposal.end_time, ErrorCode::VotingPeriodNotEnded);
    require!(clock.unix_timestamp >= proposal.execution_time, ErrorCode::NoticePeriodNotMet);

    let total_votes = proposal.votes_for.checked_add(proposal.votes_against).ok_or(ErrorCode::ArithmeticOverflow)?;
    let total_votes_u128 = total_votes as u128;
    let votes_for_u128 = proposal.votes_for as u128;
    let total_voting_power_u128 = state.total_voting_power as u128;

    if total_voting_power_u128 == 0 {
        proposal.status = 2; // Rejected
    } else {
        let quorum = (total_voting_power_u128 * VOTING_QUORUM as u128) / 100;
        let required_approval = (total_votes_u128 * VOTING_APPROVAL as u128 + 99) / 100;
        let required_threshold = (total_voting_power_u128 * VOTING_THRESHOLD as u128) / 100;

        if total_votes_u128 >= quorum && votes_for_u128 >= required_approval && votes_for_u128 >= required_threshold {
            proposal.status = 1; // Approved
            match proposal.proposal_type {
                0 => {
                    require!(proposal.proposal_values.len() == 1, ErrorCode::InvalidProposalValueCount);
                    let new_tax_rate = proposal.proposal_values[0];
                    require!(new_tax_rate >= 100 && new_tax_rate <= 1000, ErrorCode::InvalidTaxRate); // Range check: 1% to 10%
                    state.tax_rate = new_tax_rate;
                    emit!(TaxRateUpdated { new_tax_rate });
                }
                1 => {
                    require!(proposal.proposal_values.len() == 4, ErrorCode::InvalidProposalValueCount);
                    let tiers = proposal.proposal_values;
                    require!(
                        tiers[0] >= 20_000 * 10u64.pow(TOKEN_DECIMALS) && // Minimum for tier 0
                        tiers[0] < tiers[1] && tiers[1] < tiers[2] && tiers[2] < tiers[3] &&
                        tiers[3] <= TOTAL_SUPPLY / 10, // Max 10% of total supply
                        ErrorCode::InvalidStakingTiers
                    );
                    state.staking_tiers = [tiers[0], tiers[1], tiers[2], tiers[3]];
                    emit!(StakingTiersUpdated { new_tiers: state.staking_tiers });
                }
                2 => {
                    require!(proposal.proposal_values.len() == 6, ErrorCode::InvalidProposalValueCount);
                    let total = proposal.proposal_values.iter().sum::<u64>();
                    require!(total == 10000, ErrorCode::InvalidTaxAllocationTotal);
                    for &alloc in &proposal.proposal_values {
                        require!(alloc <= 5000, ErrorCode::InvalidTaxAllocation); // Max 50% per category
                    }
                    state.burn_alloc = proposal.proposal_values[0];
                    state.treasury_alloc = proposal.proposal_values[1];
                    state.liquidity_pool_alloc = proposal.proposal_values[2];
                    state.lp_incentive_alloc = proposal.proposal_values[3];
                    state.charity_alloc = proposal.proposal_values[4];
                    state.team_alloc = proposal.proposal_values[5];
                    emit!(TaxAllocationUpdated {
                        burn: state.burn_alloc,
                        treasury: state.treasury_alloc,
                        liquidity_pool: state.liquidity_pool_alloc,
                        lp_incentive: state.lp_incentive_alloc,
                        charity: state.charity_alloc,
                        team: state.team_alloc,
                    });
                }
                3 => {
                    require!(proposal.proposal_values.len() == 7, ErrorCode::InvalidProposalValueCount);
                    let thresholds = &proposal.proposal_values[0..3];
                    let factors = &proposal.proposal_values[3..7];
                    require!(
                        thresholds[0] < thresholds[1] && thresholds[1] < thresholds[2] &&
                        thresholds[0] >= 100 && thresholds[2] <= 900, // Range 10% to 90%
                        ErrorCode::InvalidReductionThresholds
                    );
                    for &factor in factors {
                        require!(factor >= 100 && factor <= 2000, ErrorCode::InvalidReductionFactor); // 10% to 200%
                    }
                    state.reduction_thresholds = [thresholds[0], thresholds[1], thresholds[2]];
                    state.reduction_factors = [factors[0], factors[1], factors[2], factors[3]];
                    emit!(ReductionFactorsUpdated {
                        thresholds: state.reduction_thresholds,
                        factors: state.reduction_factors,
                    });
                }
                4 => {
                    require!(proposal.proposal_values.len() == 1, ErrorCode::InvalidProposalValueCount);
                    let new_timestamp = proposal.proposal_values[0] as i64;
                    require!(
                        new_timestamp >= LAUNCH_TIMESTAMP - GRACE_PERIOD && 
                        new_timestamp <= LAUNCH_TIMESTAMP + 365 * 86400, // Within 1 year
                        ErrorCode::InvalidTimestamp
                    );
                    state.launch_timestamp = new_timestamp;
                    emit!(LaunchTimestampUpdated { new_timestamp });
                }
                5 => {
                    require!(proposal.proposal_values.len() == 1, ErrorCode::InvalidProposalValueCount);
                    let new_limit = proposal.proposal_values[0];
                    require!(new_limit >= TOTAL_SUPPLY / 1000 && new_limit <= TOTAL_SUPPLY / 50, ErrorCode::InvalidSellLimit); // 0.1% to 2%
                    state.max_sell_txn_limit = new_limit;
                    emit!(MaxSellTxnLimitUpdated { new_limit });
                }
                6 => {
                    require!(proposal.proposal_values.len() == 1, ErrorCode::InvalidProposalValueCount);
                    let new_limit = proposal.proposal_values[0];
                    require!(new_limit >= TOTAL_SUPPLY / 1000 && new_limit <= TOTAL_SUPPLY / 50, ErrorCode::InvalidSellLimit);
                    state.daily_sell_limit = new_limit;
                    emit!(DailySellLimitUpdated { new_limit });
                }
                7 => {
                    require!(proposal.proposal_values.len() == 1, ErrorCode::InvalidProposalValueCount);
                    let new_limit = proposal.proposal_values[0];
                    require!(new_limit >= TOTAL_SUPPLY / 1000 && new_limit <= TOTAL_SUPPLY / 50, ErrorCode::InvalidTransferLimit);
                    state.max_transfer_limit = new_limit;
                    emit!(MaxTransferLimitUpdated { new_limit });
                }
                8 => {
                    require!(proposal.proposal_values.len() == 1, ErrorCode::InvalidProposalValueCount);
                    let new_limit = proposal.proposal_values[0];
                    require!(new_limit >= TOTAL_SUPPLY / 1000 && new_limit <= TOTAL_SUPPLY / 50, ErrorCode::InvalidTransferLimit);
                    state.daily_transfer_limit = new_limit;
                    emit!(DailyTransferLimitUpdated { new_limit });
                }
                9 => {
                    require!(proposal.proposal_values.len() == 1, ErrorCode::InvalidProposalValueCount);
                    let new_threshold = proposal.proposal_values[0];
                    require!(new_threshold >= TOTAL_SUPPLY / 1000 && new_threshold <= TOTAL_SUPPLY / 50, ErrorCode::InvalidTaxThreshold);
                    state.progressive_tax_threshold = new_threshold;
                    emit!(ProgressiveTaxThresholdUpdated { new_threshold });
                }
                10 => {
                    require!(proposal.proposal_values.len() == 4, ErrorCode::InvalidProposalValueCount);
                    for &reward in &proposal.proposal_values {
                        require!(reward >= 100 * 10u64.pow(9) && reward <= 1_000_000 * 10u64.pow(9), ErrorCode::InvalidStakingReward);
                    }
                    state.staking_rewards = [
                        proposal.proposal_values[0],
                        proposal.proposal_values[1],
                        proposal.proposal_values[2],
                        proposal.proposal_values[3],
                    ];
                    emit!(StakingRewardsUpdated { new_rewards: state.staking_rewards });
                }
                11 => { // Batch whitelist update
                    require!(proposal.proposal_values.len() >= 2, ErrorCode::InvalidProposalValueCount);
                    let add_count = proposal.proposal_values[0] as usize;
                    let remove_count = proposal.proposal_values[1] as usize;
                    require!(add_count + remove_count <= proposal.proposal_values.len() - 2, ErrorCode::InvalidProposalValueCount);
                    let contracts = &proposal.proposal_values[2..];
                    let mut new_whitelist = state.whitelisted_contracts.clone();
                    for i in 0..remove_count {
                        let contract_key = Pubkey::new_from_array(contracts[i].to_le_bytes()[..32].try_into().unwrap());
                        if let Some(pos) = new_whitelist.iter().position(|&x| x == contract_key) {
                            new_whitelist.remove(pos);
                            emit!(WhitelistedContractRemovedEvent { contract: contract_key });
                        }
                    }
                    for i in remove_count..remove_count + add_count {
                        if new_whitelist.len() < ContractState::MAX_WHITELISTED {
                            let contract = Pubkey::new_from_array(contracts[i].to_le_bytes()[..32].try_into().unwrap());
                            new_whitelist.push(contract);
                            emit!(WhitelistedContractAddedEvent { contract });
                        }
                    }
                    state.whitelisted_contracts = new_whitelist;
                }
                _ => return Err(ErrorCode::InvalidProposalType.into()),
            }
        } else {
            proposal.status = 2; // Rejected
        }
    }
    emit!(ProposalExecutedEvent {
        proposal_id,
        status: proposal.status,
    });

    Ok(())
}

pub fn airdrop(ctx: Context<Airdrop>, winners: Vec<(Pubkey, u64)>) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    const MAX_RECIPIENTS: usize = 100; // Increased limit for compute budget
    require!(winners.len() <= MAX_RECIPIENTS, ErrorCode::TooManyRecipients);

    validate_multisig(&ctx.accounts.multisig, &[
        &Some(&ctx.accounts.signer1),
        ctx.accounts.signer2.as_ref(),
        ctx.accounts.signer3.as_ref(),
        ctx.accounts.signer4.as_ref(),
        ctx.accounts.signer5.as_ref(),
    ])?;

    let token_accounts = &ctx.remaining_accounts;
    require!(token_accounts.len() == winners.len(), ErrorCode::InvalidRecipientAccounts);

    let total = winners.iter().map(|(_, amt)| *amt).try_fold(0u64, |acc, x| acc.checked_add(x)).ok_or(ErrorCode::ArithmeticOverflow)?;
    let treasury = &mut ctx.accounts.treasury;
    require!(treasury.airdrop_pool >= total, ErrorCode::InsufficientAirdropFunds);
    treasury.airdrop_pool = treasury.airdrop_pool.checked_sub(total).ok_or(ErrorCode::ArithmeticOverflow)?;
    emit!(TreasuryPoolAdjusted { pool_type: PoolType::Airdrop, new_amount: treasury.airdrop_pool });

    let signer_seeds = &[&[b"treasury_authority", &[ctx.bumps.treasury_pda]]];
    for (j, (recipient, amount)) in winners.iter().enumerate() {
        let token_account = Account::<TokenAccount>::try_from(&token_accounts[j])?;
        require!(token_account.owner == *recipient, ErrorCode::InvalidRecipientAccount);
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TokenTransfer {
                    from: ctx.accounts.treasury_token_account.to_account_info(),
                    to: token_account.to_account_info(),
                    authority: ctx.accounts.treasury_pda.to_account_info(),
                },
                signer_seeds,
            ),
            *amount,
        )?;
    }
    emit!(AirdropEvent {
        total_amount: total,
        recipient_count: winners.len(),
    });

    Ok(())
}

pub fn initiate_pause(ctx: Context<InitiatePause>, reason: String) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::AlreadyPaused);
    require!(reason.len() <= ContractState::MAX_PAUSE_REASON_LEN, ErrorCode::DescriptionTooLong);

    validate_multisig(&ctx.accounts.multisig, &[
        &Some(&ctx.accounts.signer1),
        ctx.accounts.signer2.as_ref(),
        ctx.accounts.signer3.as_ref(),
        ctx.accounts.signer4.as_ref(),
        ctx.accounts.signer5.as_ref(),
    ])?;

    let pending_pause = &mut ctx.accounts.pending_pause;
    pending_pause.reason = reason;
    pending_pause.initiation_time = Clock::get()?.unix_timestamp;

    emit!(PauseInitiated { initiation_time: pending_pause.initiation_time });

    Ok(())
}

pub fn confirm_pause(ctx: Context<ConfirmPause>) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::AlreadyPaused);

    let pending_pause = &ctx.accounts.pending_pause;
    let clock = Clock::get()?;
    require!(
        clock.unix_timestamp >= pending_pause.initiation_time + 24 * 3600,
        ErrorCode::TimeLockNotMet
    );

    state.paused = true;
    state.pause_reason = pending_pause.reason.clone();

    emit!(PauseEvent {
        timestamp: clock.unix_timestamp,
        reason: state.pause_reason.clone(),
    });

    Ok(())
}

pub fn initiate_resume(ctx: Context<InitiateResume>) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(state.paused, ErrorCode::NotPaused);

    validate_multisig(&ctx.accounts.multisig, &[
        &Some(&ctx.accounts.signer1),
        ctx.accounts.signer2.as_ref(),
        ctx.accounts.signer3.as_ref(),
        ctx.accounts.signer4.as_ref(),
        ctx.accounts.signer5.as_ref(),
    ])?;

    let pending_resume = &mut ctx.accounts.pending_resume;
    pending_resume.initiation_time = Clock::get()?.unix_timestamp;

    emit!(ResumeInitiated { initiation_time: pending_resume.initiation_time });

    Ok(())
}

pub fn confirm_resume(ctx: Context<ConfirmResume>) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(state.paused, ErrorCode::NotPaused);

    let pending_resume = &ctx.accounts.pending_resume;
    let clock = Clock::get()?;
    require!(
        clock.unix_timestamp >= pending_resume.initiation_time + 24 * 3600,
        ErrorCode::TimeLockNotMet
    );

    state.paused = false;
    state.pause_reason = String::new();

    emit!(ResumeEvent { timestamp: clock.unix_timestamp });

    Ok(())
}

pub fn update_team_vesting(ctx: Context<UpdateTeamVesting>, team_member: Pubkey, total_amount: u64, immediate_amount: u64) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    validate_multisig(&ctx.accounts.multisig, &[
        &Some(&ctx.accounts.signer1),
        ctx.accounts.signer2.as_ref(),
        ctx.accounts.signer3.as_ref(),
        ctx.accounts.signer4.as_ref(),
        ctx.accounts.signer5.as_ref(),
    ])?;
    require!(immediate_amount <= total_amount, ErrorCode::InvalidImmediateAmount);
    let team_vesting = &mut ctx.accounts.team_vesting;
    let clock = Clock::get()?;
    team_vesting.team_member = team_member;
    team_vesting.total_amount = total_amount.checked_sub(immediate_amount).ok_or(ErrorCode::ArithmeticOverflow)?;
    team_vesting.claimed_amount = 0;
    team_vesting.start_time = clock.unix_timestamp;
    team_vesting.canceled = false;

    let signer_seeds = &[&[b"treasury_authority", &[ctx.bumps.treasury_pda]]];
    if immediate_amount > 0 {
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TokenTransfer {
                    from: ctx.accounts.treasury_token_account.to_account_info(),
                    to: ctx.accounts.team_member_token_account.to_account_info(),
                    authority: ctx.accounts.treasury_pda.to_account_info(),
                },
                signer_seeds,
            ),
            immediate_amount,
        )?;
    }

    emit!(TeamVestingUpdatedEvent { team_member, total_amount });

    Ok(())
}

pub fn cancel_team_vesting(ctx: Context<CancelTeamVesting>, team_member: Pubkey) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    validate_multisig(&ctx.accounts.multisig, &[
        &Some(&ctx.accounts.signer1),
        ctx.accounts.signer2.as_ref(),
        ctx.accounts.signer3.as_ref(),
        ctx.accounts.signer4.as_ref(),
        ctx.accounts.signer5.as_ref(),
    ])?;
    let team_vesting = &mut ctx.accounts.team_vesting;
    require!(team_vesting.team_member == team_member, ErrorCode::InvalidTeamMember);
    team_vesting.canceled = true;
    emit!(TeamVestingCanceledEvent { team_member });

    Ok(())
}

pub fn claim_team_vesting(ctx: Context<ClaimTeamVesting>) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    let clock = Clock::get()?;
    let team_vesting = &mut ctx.accounts.team_vesting;
    require!(!team_vesting.canceled, ErrorCode::VestingCanceled);
    let time_since_start = clock.unix_timestamp.checked_sub(team_vesting.start_time).ok_or(ErrorCode::ArithmeticOverflow)?;
    require!(time_since_start >= 0, ErrorCode::VestingNotStarted);
    let months_passed = (time_since_start / (30 * 86400)) as u64;

    let unlock_percent = if months_passed < 3 {
        0
    } else {
        min(10 * (months_passed - 2), 100)
    };
    let total_vested = (team_vesting.total_amount as u128 * unlock_percent as u128 / 100)
        .try_into()
        .map_err(|_| ErrorCode::ArithmeticOverflow)?;
    let claimable = min(total_vested.checked_sub(team_vesting.claimed_amount).ok_or(ErrorCode::ArithmeticOverflow)?, 20_000_000 * 10u64.pow(9));

    if claimable == 0 {
        emit!(NoRewardsEvent {
            user: ctx.accounts.team_member.key(),
            reason: "No claimable amount available".to_string(),
        });
        return Ok(());
    }

    team_vesting.claimed_amount = team_vesting.claimed_amount.checked_add(claimable).ok_or(ErrorCode::ArithmeticOverflow)?;

    let signer_seeds = &[&[b"treasury_authority", &[ctx.bumps.treasury_pda]]];
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.treasury_token_account.to_account_info(),
                to: ctx.accounts.team_member_token_account.to_account_info(),
                authority: ctx.accounts.treasury_pda.to_account_info(),
            },
            signer_seeds,
        ),
        claimable,
    )?;
    emit!(TeamVestingClaimedEvent {
        team_member: ctx.accounts.team_member.key(),
        amount: claimable,
    });

    Ok(())
}

pub fn update_freelancer_vesting(ctx: Context<UpdateFreelancerVesting>, freelancer: Pubkey, total_amount: u64) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    validate_multisig(&ctx.accounts.multisig, &[
        &Some(&ctx.accounts.signer1),
        ctx.accounts.signer2.as_ref(),
        ctx.accounts.signer3.as_ref(),
        ctx.accounts.signer4.as_ref(),
        ctx.accounts.signer5.as_ref(),
    ])?;
    let freelancer_vesting = &mut ctx.accounts.freelancer_vesting;
    let clock = Clock::get()?;
    freelancer_vesting.freelancer = freelancer;
    freelancer_vesting.total_amount = total_amount;
    freelancer_vesting.released_amount = 0;
    freelancer_vesting.claimed_amount = 0;
    freelancer_vesting.start_time = clock.unix_timestamp;
    freelancer_vesting.last_claim_time = clock.unix_timestamp;
    emit!(FreelancerVestingUpdatedEvent { freelancer, total_amount });

    Ok(())
}

pub fn claim_freelancer_vesting(ctx: Context<ClaimFreelancerVesting>) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    let clock = Clock::get()?;
    let freelancer_vesting = &mut ctx.accounts.freelancer_vesting;
    let time_since_last_claim = clock.unix_timestamp.checked_sub(freelancer_vesting.last_claim_time).ok_or(ErrorCode::ArithmeticOverflow)?;
    require!(time_since_last_claim >= 3 * 86400, ErrorCode::ClaimCooldownNotMet);

    let available = freelancer_vesting.released_amount.checked_sub(freelancer_vesting.claimed_amount).ok_or(ErrorCode::ArithmeticOverflow)?;
    let claimable = min(available, 500_000 * 10u64.pow(TOKEN_DECIMALS));
    if claimable == 0 {
        emit!(NoRewardsEvent {
            user: ctx.accounts.freelancer.key(),
            reason: "No claimable amount available".to_string(),
        });
        return Ok(());
    }

    freelancer_vesting.claimed_amount = freelancer_vesting.claimed_amount.checked_add(claimable).ok_or(ErrorCode::ArithmeticOverflow)?;
    freelancer_vesting.last_claim_time = clock.unix_timestamp;

    let signer_seeds = &[&[b"treasury_authority", &[ctx.bumps.treasury_pda]]];
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.treasury_token_account.to_account_info(),
                to: ctx.accounts.freelancer_token_account.to_account_info(),
                authority: ctx.accounts.treasury_pda.to_account_info(),
            },
            signer_seeds,
        ),
        claimable,
    )?;
    emit!(FreelancerVestingClaimedEvent {
        freelancer: ctx.accounts.freelancer.key(),
        amount: claimable,
    });

    Ok(())
}

pub fn release_freelancer_milestone(ctx: Context<ReleaseFreelancerMilestone>, amount: u64) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    validate_multisig(&ctx.accounts.multisig, &[
        &Some(&ctx.accounts.signer1),
        ctx.accounts.signer2.as_ref(),
        ctx.accounts.signer3.as_ref(),
        ctx.accounts.signer4.as_ref(),
        ctx.accounts.signer5.as_ref(),
    ])?;
    let freelancer_vesting = &mut ctx.accounts.freelancer_vesting;
    let new_released = freelancer_vesting.released_amount.checked_add(amount).ok_or(ErrorCode::ArithmeticOverflow)?;
    require!(new_released <= freelancer_vesting.total_amount, ErrorCode::ExceedsVestingTotal);
    freelancer_vesting.released_amount = new_released;
    emit!(FreelancerMilestoneReleasedEvent {
        freelancer: ctx.accounts.freelancer.key(),
        amount,
    });

    Ok(())
}

pub fn initiate_withdrawal(ctx: Context<InitiateWithdrawal>, amount: u64) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    validate_multisig(&ctx.accounts.multisig, &[
        &Some(&ctx.accounts.signer1),
        ctx.accounts.signer2.as_ref(),
        ctx.accounts.signer3.as_ref(),
        ctx.accounts.signer4.as_ref(),
        ctx.accounts.signer5.as_ref(),
    ])?;
    let clock = Clock::get()?;
    let pending_withdrawal = &mut ctx.accounts.pending_withdrawal;
    pending_withdrawal.amount = amount;
    pending_withdrawal.initiation_slot = clock.slot;
    pending_withdrawal.delay_slots = if amount > WITHDRAWAL_THRESHOLD { WITHDRAWAL_DELAY_SLOTS } else { 0 };
    emit!(WithdrawalInitiatedEvent {
        amount,
        initiation_slot: pending_withdrawal.initiation_slot,
        delay_slots: pending_withdrawal.delay_slots,
    });

    Ok(())
}

pub fn complete_withdrawal(ctx: Context<CompleteWithdrawal>) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    validate_multisig(&ctx.accounts.multisig, &[
        &Some(&ctx.accounts.signer1),
        ctx.accounts.signer2.as_ref(),
        ctx.accounts.signer3.as_ref(),
        ctx.accounts.signer4.as_ref(),
        ctx.accounts.signer5.as_ref(),
    ])?;
    let clock = Clock::get()?;
    let pending_withdrawal = &ctx.accounts.pending_withdrawal;
    require!(
        clock.slot >= pending_withdrawal.initiation_slot.checked_add(pending_withdrawal.delay_slots).ok_or(ErrorCode::ArithmeticOverflow)?,
        ErrorCode::WithdrawalDelayNotMet
    );
    let amount = pending_withdrawal.amount;
    let signer_seeds = &[&[b"treasury_authority", &[ctx.bumps.treasury_pda]]];
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.treasury_token_account.to_account_info(),
                to: ctx.accounts.destination_token_account.to_account_info(),
                authority: ctx.accounts.treasury_pda.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )?;
    emit!(WithdrawalCompletedEvent { amount });

    Ok(())
}

pub fn initiate_set_multisig(ctx: Context<InitiateSetMultisig>, owners: Vec<Pubkey>, threshold: u8) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    let multisig = &ctx.accounts.multisig;
    validate_multisig(multisig, &[
        &Some(&ctx.accounts.signer1),
        ctx.accounts.signer2.as_ref(),
        ctx.accounts.signer3.as_ref(),
        ctx.accounts.signer4.as_ref(),
        ctx.accounts.signer5.as_ref(),
    ])?;

    require!(owners.len() >= 2, ErrorCode::TooFewOwners);
    let unique_owners: HashSet<Pubkey> = owners.iter().cloned().collect();
    require!(unique_owners.len() == owners.len(), ErrorCode::DuplicateOwners);
    require!(owners.len() <= Multisig::MAX_OWNERS, ErrorCode::TooManyOwners);
    require!(threshold as usize <= owners.len(), ErrorCode::ThresholdExceedsOwners);

    let pending_change = &mut ctx.accounts.pending_multisig_change;
    pending_change.new_owners = owners;
    pending_change.new_threshold = threshold;
    pending_change.initiation_time = Clock::get()?.unix_timestamp;

    emit!(MultisigChangeInitiated { initiation_time: pending_change.initiation_time });

    Ok(())
}

pub fn confirm_set_multisig(ctx: Context<ConfirmSetMultisig>) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    let pending_change = &ctx.accounts.pending_multisig_change;
    let clock = Clock::get()?;
    require!(
        clock.unix_timestamp >= pending_change.initiation_time + 24 * 3600,
        ErrorCode::TimeLockNotMet
    );

    let multisig = &mut ctx.accounts.multisig;
    multisig.owners = pending_change.new_owners.clone();
    multisig.threshold = pending_change.new_threshold;

    emit!(MultisigUpdatedEvent {
        threshold: multisig.threshold,
        owner_count: multisig.owners.len(),
    });

    Ok(())
}

pub fn distribute_lp_incentives(ctx: Context<DistributeLPIncentives>, recipients: Vec<(Pubkey, u64)>) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    validate_multisig(&ctx.accounts.multisig, &[
        &Some(&ctx.accounts.signer1),
        ctx.accounts.signer2.as_ref(),
        ctx.accounts.signer3.as_ref(),
        ctx.accounts.signer4.as_ref(),
        ctx.accounts.signer5.as_ref(),
    ])?;
    const MAX_RECIPIENTS: usize = 100;
    require!(recipients.len() <= MAX_RECIPIENTS, ErrorCode::TooManyRecipients);
    let token_accounts = &ctx.remaining_accounts;
    require!(token_accounts.len() == recipients.len(), ErrorCode::InvalidRecipientAccounts);
    let total = recipients.iter().map(|(_, amt)| *amt).try_fold(0u64, |acc, x| acc.checked_add(x)).ok_or(ErrorCode::ArithmeticOverflow)?;
    let treasury = &mut ctx.accounts.treasury;
    require!(total <= treasury.liquidity_incentive, ErrorCode::InsufficientLiquidityIncentiveFunds);
    treasury.liquidity_incentive = treasury.liquidity_incentive.checked_sub(total).ok_or(ErrorCode::ArithmeticOverflow)?;
    emit!(TreasuryPoolAdjusted { pool_type: PoolType::LiquidityIncentive, new_amount: treasury.liquidity_incentive });
    let signer_seeds = &[&[b"treasury_authority", &[ctx.bumps.treasury_pda]]];
    for (j, (recipient, amount)) in recipients.iter().enumerate() {
        let token_account = Account::<TokenAccount>::try_from(&token_accounts[j])?;
        require!(token_account.owner == *recipient, ErrorCode::InvalidRecipientAccount);
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TokenTransfer {
                    from: ctx.accounts.treasury_token_account.to_account_info(),
                    to: token_account.to_account_info(),
                    authority: ctx.accounts.treasury_pda.to_account_info(),
                },
                signer_seeds,
            ),
            *amount,
        )?;
    }
    emit!(LPIncentivesDistributedEvent {
        total_amount: total,
        recipient_count: recipients.len(),
    });

    Ok(())
}

pub fn update_dex_programs(ctx: Context<UpdateDexPrograms>, dex_programs: Vec<Pubkey>) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    validate_multisig(&ctx.accounts.multisig, &[
        &Some(&ctx.accounts.signer1),
        ctx.accounts.signer2.as_ref(),
        ctx.accounts.signer3.as_ref(),
        ctx.accounts.signer4.as_ref(),
        ctx.accounts.signer5.as_ref(),
    ])?;
    require!(dex_programs.len() <= ContractState::MAX_DEXES, ErrorCode::TooManyDexPrograms);
    require!(ctx.remaining_accounts.len() == dex_programs.len(), ErrorCode::InvalidDexAccounts);
    for (j, program_id) in dex_programs.iter().enumerate() {
        let account_info = &ctx.remaining_accounts[j];
        require!(account_info.key == *program_id && account_info.executable, ErrorCode::InvalidDexProgram);
    }
    state.dex_programs = dex_programs.clone();
    emit!(DexProgramsUpdatedEvent { program_count: dex_programs.len() });

    Ok(())
}

pub fn add_whitelisted_contract(ctx: Context<AddWhitelistedContract>, contract: Pubkey) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    validate_multisig(&ctx.accounts.multisig, &[
        &Some(&ctx.accounts.signer1),
        ctx.accounts.signer2.as_ref(),
        ctx.accounts.signer3.as_ref(),
        ctx.accounts.signer4.as_ref(),
        ctx.accounts.signer5.as_ref(),
    ])?;
    require!(ctx.accounts.contract.key() == contract && ctx.accounts.contract.executable, ErrorCode::InvalidContract);

    let pending_change = &mut ctx.accounts.pending_whitelist_change;
    pending_change.contract = contract;
    pending_change.add = true;
    pending_change.initiation_time = Clock::get()?.unix_timestamp;

    Ok(())
}

pub fn remove_whitelisted_contract(ctx: Context<RemoveWhitelistedContract>, contract: Pubkey) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    validate_multisig(&ctx.accounts.multisig, &[
        &Some(&ctx.accounts.signer1),
        ctx.accounts.signer2.as_ref(),
        ctx.accounts.signer3.as_ref(),
        ctx.accounts.signer4.as_ref(),
        ctx.accounts.signer5.as_ref(),
    ])?;

    let pending_change = &mut ctx.accounts.pending_whitelist_change;
    pending_change.contract = contract;
    pending_change.add = false;
    pending_change.initiation_time = Clock::get()?.unix_timestamp;

    Ok(())
}

pub fn confirm_whitelist_change(ctx: Context<ConfirmWhitelistChange>) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    let pending_change = &ctx.accounts.pending_whitelist_change;
    let clock = Clock::get()?;
    let delay = 72 * 60 * 60; // 72 hours
    require!(
        clock.unix_timestamp >= pending_change.initiation_time + delay,
        ErrorCode::WhitelistDelayNotMet
    );

    if pending_change.add {
        require!(state.whitelisted_contracts.len() < ContractState::MAX_WHITELISTED, ErrorCode::TooManyWhitelistedContracts);
        state.whitelisted_contracts.push((pending_change.contract, pending_change.contract.to_bytes())); // Using program ID as hash
        emit!(WhitelistedContractAddedEvent { contract: pending_change.contract });
    } else {
        if let Some(i) = state.whitelisted_contracts.iter().position(|&(x, _)| x == pending_change.contract) {
            state.whitelisted_contracts.remove(i);
            emit!(WhitelistedContractRemovedEvent { contract: pending_change.contract });
        } else {
            return Err(ErrorCode::ContractNotWhitelisted.into());
        }
    }

    Ok(())
}

pub fn lock_for_migration(ctx: Context<LockForMigration>, amount: u64) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    let migration_state = &mut ctx.accounts.migration_state;
    require!(migration_state.migration_active, ErrorCode::MigrationNotActive);
    let migration_record = &mut ctx.accounts.migration_record;
    require!(!migration_record.migrated, ErrorCode::AlreadyMigrated);
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.migration_token_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount,
    )?;
    migration_record.user = ctx.accounts.user.key();
    migration_record.locked_amount = migration_record.locked_amount.checked_add(amount).ok_or(ErrorCode::ArithmeticOverflow)?;
    migration_state.total_locked = migration_state.total_locked.checked_add(amount).ok_or(ErrorCode::ArithmeticOverflow)?;
    emit!(TokensLockedForMigrationEvent {
        user: ctx.accounts.user.key(),
        amount,
    });

    Ok(())
}

pub fn unlock_for_migration(ctx: Context<UnlockForMigration>) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    let migration_state = &mut ctx.accounts.migration_state;
    require!(!migration_state.migration_active, ErrorCode::MigrationActive);
    let migration_record = &mut ctx.accounts.migration_record;
    require!(!migration_record.migrated, ErrorCode::AlreadyMigrated);
    let amount = migration_record.locked_amount;
    require!(amount > 0, ErrorCode::NoLockedTokens);
    let signer_seeds = &[&[b"migration_authority", &[ctx.bumps.migration_pda]]];
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenTransfer {
                from: ctx.accounts.migration_token_account.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.migration_pda.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )?;
    migration_record.locked_amount = 0;
    migration_state.total_locked = migration_state.total_locked.checked_sub(amount).ok_or(ErrorCode::ArithmeticOverflow)?;
    emit!(TokensUnlockedForMigrationEvent {
        user: ctx.accounts.user.key(),
        amount,
    });

    Ok(())
}

pub fn burn_locked_tokens(ctx: Context<BurnLockedTokens>, user: Pubkey) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    validate_multisig(&ctx.accounts.multisig, &[
        &Some(&ctx.accounts.signer1),
        ctx.accounts.signer2.as_ref(),
        ctx.accounts.signer3.as_ref(),
        ctx.accounts.signer4.as_ref(),
        ctx.accounts.signer5.as_ref(),
    ])?;
    let migration_state = &mut ctx.accounts.migration_state;
    let migration_record = &mut ctx.accounts.migration_record;
    require!(migration_record.user == user, ErrorCode::InvalidMigrationUser);
    let amount = migration_record.locked_amount;
    require!(amount > 0, ErrorCode::NoLockedTokens);
    let signer_seeds = &[&[b"migration_authority", &[ctx.bumps.migration_pda]]];
    token::burn(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.token_mint.to_account_info(),
                from: ctx.accounts.migration_token_account.to_account_info(),
                authority: ctx.accounts.migration_pda.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )?;
    migration_record.locked_amount = 0;
    migration_record.migrated = true;
    migration_state.total_locked = migration_state.total_locked.checked_sub(amount).ok_or(ErrorCode::ArithmeticOverflow)?;
    emit!(LockedTokensBurnedEvent {
        user,
        amount,
    });
    emit!(MigrationConfirmedEvent { user });

    Ok(())
}

pub fn toggle_migration_active(ctx: Context<ToggleMigrationActive>, active: bool) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    validate_multisig(&ctx.accounts.multisig, &[
        &Some(&ctx.accounts.signer1),
        ctx.accounts.signer2.as_ref(),
        ctx.accounts.signer3.as_ref(),
        ctx.accounts.signer4.as_ref(),
        ctx.accounts.signer5.as_ref(),
    ])?;
    let migration_state = &mut ctx.accounts.migration_state;
    let clock = Clock::get()?;
    let time_since_last_toggle = clock.unix_timestamp.checked_sub(migration_state.migration_toggle_timestamp).ok_or(ErrorCode::ArithmeticOverflow)?;
    require!(time_since_last_toggle >= 7 * 86400, ErrorCode::MigrationToggleCooldown);
    migration_state.migration_active = active;
    migration_state.migration_toggle_timestamp = clock.unix_timestamp;
    emit!(MigrationToggledEvent { active });

    Ok(())
}

pub fn transfer_between_pools(ctx: Context<TransferBetweenPools>, source_pool: PoolType, dest_pool: PoolType, amount: u64) -> Result<()> {
    let _guard = ReentrancyGuard::new(&mut ctx.accounts.state);

    let state = &mut ctx.accounts.state;
    require!(!state.paused, ErrorCode::Paused);

    validate_multisig(&ctx.accounts.multisig, &[
        &Some(&ctx.accounts.signer1),
        ctx.accounts.signer2.as_ref(),
        ctx.accounts.signer3.as_ref(),
        ctx.accounts.signer4.as_ref(),
        ctx.accounts.signer5.as_ref(),
    ])?;
    let treasury = &mut ctx.accounts.treasury;
    macro_rules! adjust_pool {
        ($pool:ident, $amount:expr, $op:ident) => {
            treasury.$pool = treasury.$pool.$op($amount).ok_or(ErrorCode::ArithmeticOverflow)?;
            emit!(TreasuryPoolAdjusted { pool_type: PoolType::$pool, new_amount: treasury.$pool });
        };
    }
    match source_pool {
        PoolType::Staking => adjust_pool!(staking_pool, amount, checked_sub),
        PoolType::Airdrop => adjust_pool!(airdrop_pool, amount, checked_sub),
        PoolType::Governance => adjust_pool!(governance_reserve, amount, checked_sub),
        PoolType::Marketing => adjust_pool!(marketing_fund, amount, checked_sub),
        PoolType::Emergency => adjust_pool!(emergency_fund, amount, checked_sub),
        PoolType::LiquidityIncentive => adjust_pool!(liquidity_incentive, amount, checked_sub),
        PoolType::Team => adjust_pool!(team_pool, amount, checked_sub),
    }
    match dest_pool {
        PoolType::Staking => adjust_pool!(staking_pool, amount, checked_add),
        PoolType::Airdrop => adjust_pool!(airdrop_pool, amount, checked_add),
        PoolType::Governance => adjust_pool!(governance_reserve, amount, checked_add),
        PoolType::Marketing => adjust_pool!(marketing_fund, amount, checked_add),
        PoolType::Emergency => adjust_pool!(emergency_fund, amount, checked_add),
        PoolType::LiquidityIncentive => adjust_pool!(liquidity_incentive, amount, checked_add),
        PoolType::Team => adjust_pool!(team_pool, amount, checked_add),
    }

    Ok(())
}

pub fn query_pending_rewards(ctx: Context<QueryPendingRewards>) -> Result<()> {
    let state = &ctx.accounts.state;
    let treasury = &ctx.accounts.treasury;
    let user = &ctx.accounts.user;
    let staker_info = &ctx.accounts.staker;

    let (expected_staker_pda, _bump) = Pubkey::find_program_address(&[b"staker", user.key().as_ref()], ctx.program_id);

    let rewards = if staker_info.key() == expected_staker_pda && staker_info.data_len() > 0 {
        let staker: Account<Staker> = Account::try_from(staker_info)?;
        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp;
        get_pending_rewards(state, &staker, treasury, current_time)?
    } else {
        0
    };

    msg!("Pending Rewards: {}", rewards);
    Ok(())
}

pub fn validate_multisig(multisig: &Multisig, signers: &[&Option<&Signer>]) -> Result<()> {
    let mut unique_signers = HashSet::new();
    for signer in signers.iter().filter_map(|s| *s) {
        unique_signers.insert(signer.key());
    }
    require!(unique_signers.len() >= multisig.threshold as usize, ErrorCode::InsufficientSigners);
    for signer in &unique_signers {
        require!(multisig.owners.contains(signer), ErrorCode::SignerNotOwner);
    }
    Ok(())
}

