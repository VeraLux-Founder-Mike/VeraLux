use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum PoolType {
    Staking,
    Airdrop,
    Governance,
    Marketing,
    Emergency,
    LiquidityIncentive,
    Team,
}

/// Global state of the smart contract, holding configuration and operational data.
#[account]
pub struct ContractState {
    /// Multisig account key for administrative actions.
    pub admin: Pubkey,
    /// Flag indicating if the contract is paused.
    pub paused: bool,
    /// Reason for pausing the contract, for transparency.
    pub pause_reason: String,
    /// Treasury wallet address (token account).
    pub treasury: Pubkey,
    /// Charity wallet address.
    pub charity_wallet: Pubkey,
    /// Team wallet address.
    pub team_wallet: Pubkey,
    /// Liquidity pool wallet address.
    pub liquidity_pool: Pubkey,
    /// Total number of governance proposals.
    pub proposal_count: u64,
    /// Total governance voting power across all stakers.
    pub total_voting_power: u64,
    /// Launch timestamp for presale vesting (Unix timestamp).
    pub launch_timestamp: i64,
    /// Current tax rate in basis points (e.g., 500 = 5%).
    pub tax_rate: u64,
    /// Adjustable staking tier thresholds in token amounts.
    pub staking_tiers: [u64; 4],
    /// Allocation for burn in basis points of tax.
    pub burn_alloc: u64,
    /// Allocation for treasury in basis points of tax.
    pub treasury_alloc: u64,
    /// Allocation for liquidity pool in basis points of tax.
    pub liquidity_pool_alloc: u64,
    /// Allocation for LP incentives in basis points of tax.
    pub lp_incentive_alloc: u64,
    /// Allocation for charity in basis points of tax.
    pub charity_alloc: u64,
    /// Allocation for team in basis points of tax.
    pub team_alloc: u64,
    /// Thresholds for reward reduction based on pool depletion.
    pub reduction_thresholds: [u64; 3],
    /// Factors for reward reduction (multipliers in basis points).
    pub reduction_factors: [u64; 4],
    /// Dynamic list of DEX program IDs for sell detection.
    pub dex_programs: Vec<Pubkey>,
    /// Whitelisted external contracts for privileged operations, with version hash.
    pub whitelisted_contracts: Vec<(Pubkey, [u8; 32])>,
    /// Allowed destinations for whitelisted transfers.
    pub allowed_destinations: Vec<Pubkey>,
    /// Maximum sell transaction limit (adjustable via governance).
    pub max_sell_txn_limit: u64,
    /// Daily sell limit per user.
    pub daily_sell_limit: u64,
    /// Maximum transfer limit for non-sell transactions.
    pub max_transfer_limit: u64,
    /// Daily transfer limit for non-sell transactions.
    pub daily_transfer_limit: u64,
    /// Threshold for applying progressive (tripled) tax.
    pub progressive_tax_threshold: u64,
    /// Adjustable staking rewards per week for each tier.
    pub staking_rewards: [u64; 4],
    /// Receiver of USDT payments from presale.
    pub presale_usdt_receiver: Pubkey,
    /// Flag indicating if presale is active.
    pub presale_active: bool,
    /// Total presale tokens sold.
    pub total_presale_sold: u64,
    /// Last day rewards were processed (Unix timestamp of 00:00 UTC).
    pub last_processed_day: i64,
    /// Flag to prevent reentrancy.
    pub is_processing: bool,
    /// Index for batch processing of rewards.
    pub last_processed_index: u64,
}

impl ContractState {
    pub const MAX_DEXES: usize = 10;
    pub const MAX_WHITELISTED: usize = 20;
    pub const MAX_ALLOWED_DESTINATIONS: usize = 10;
    pub const MAX_PAUSE_REASON_LEN: usize = 100;
    pub const LEN: usize = 1130 + 4 + Self::MAX_PAUSE_REASON_LEN + 4 + (32 + 32) * Self::MAX_WHITELISTED + 4 + 32 * Self::MAX_ALLOWED_DESTINATIONS;
}

/// Treasury pools for various allocations.
#[account]
pub struct Treasury {
    pub staking_pool: u64,
    pub airdrop_pool: u64,
    pub governance_reserve: u64,
    pub marketing_fund: u64,
    pub emergency_fund: u64,
    pub liquidity_incentive: u64,
    pub team_pool: u64,
}

impl Treasury {
    pub const LEN: usize = 8 + 8 * 7;
}

/// Individual staker's data for staking operations.
#[account]
pub struct Staker {
    pub tier: u8,
    pub amount: u64,
    pub start_time: i64,
    pub last_claim: i64,
}

impl Staker {
    pub const LEN: usize = 8 + 1 + 8 + 8 + 8;
}

/// Liquidity provider staker's data for LP incentive staking.
#[account]
pub struct LPStaker {
    pub amount: u64,
    pub last_action_time: i64,
    pub unclaimed_rewards: u64,
}

impl LPStaker {
    pub const LEN: usize = 8 + 8 + 8 + 8;
}

/// Tracks user transaction limits and cooldowns.
#[account]
pub struct TransactionRecord {
    pub last_txn_time: i64,
    pub sell_buckets: [u64; 24],
    pub transfer_buckets: [u64; 24],
    pub current_bucket_index: u8,
    pub bucket_start_time: i64,
    pub sell_cooldown_start: i64,
    pub transfer_cooldown_start: i64,
}

impl TransactionRecord {
    pub const LEN: usize = 8 + 8 + 8 * 24 + 8 * 24 + 1 + 8 + 8 + 8;
}

/// Governance proposal data.
#[account]
pub struct Proposal {
    pub id: u64,
    pub description: String,
    pub votes_for: u64,
    pub votes_against: u64,
    pub status: u8,
    pub start_time: i64,
    pub end_time: i64,
    pub execution_time: i64,
    pub proposal_type: u8,
    pub proposal_values: Vec<u64>,
}

impl Proposal {
    pub const MAX_DESCRIPTION_LEN: usize = 200;
    pub const MAX_PROPOSAL_VALUES: usize = 7;
    pub const LEN: usize = 8 + 8 + (4 + Self::MAX_DESCRIPTION_LEN) + 8 + 8 + 1 + 8 + 8 + 8 + 1 + (4 + 8 * Self::MAX_PROPOSAL_VALUES);
}

/// Pending withdrawal request from treasury.
#[account]
pub struct PendingWithdrawal {
    pub amount: u64,
    pub initiation_slot: u64,
    pub delay_slots: u64,
}

impl PendingWithdrawal {
    pub const LEN: usize = 8 + 8 * 3;
}

/// Multisig configuration for administrative actions.
#[account]
pub struct Multisig {
    pub owners: Vec<Pubkey>,
    pub threshold: u8,
}

impl Multisig {
    pub const MAX_OWNERS: usize = 5;
    pub const LEN: usize = 8 + 4 + 32 * Self::MAX_OWNERS + 1;
}

/// Presale vesting data for a user.
#[account]
pub struct PresaleVesting {
    pub total_amount: u64,
    pub claimed_amount: u64,
}

impl PresaleVesting {
    pub const LEN: usize = 8 + 8 * 2;
}

/// Record of a staker's vote on a proposal.
#[account]
pub struct VoteRecord {
    pub staker: Pubkey,
    pub proposal_id: u64,
    pub voted: bool,
    pub in_favor: bool,
}

impl VoteRecord {
    pub const LEN: usize = 8 + 32 + 8 + 1 + 1;
}

/// Team member's vesting schedule.
#[account]
pub struct TeamVesting {
    pub team_member: Pubkey,
    pub total_amount: u64,
    pub claimed_amount: u64,
    pub start_time: i64,
    pub canceled: bool,
}

impl TeamVesting {
    pub const LEN: usize = 8 + 32 + 8 + 8 + 8 + 1;
}

/// Freelancer vesting schedule.
#[account]
pub struct FreelancerVesting {
    pub freelancer: Pubkey,
    pub total_amount: u64,
    pub released_amount: u64,
    pub claimed_amount: u64,
    pub start_time: i64,
    pub last_claim_time: i64,
}

impl FreelancerVesting {
    pub const LEN: usize = 8 + 32 + 8 + 8 + 8 + 8 + 8;
}

/// Global migration state for token upgrades.
#[account]
pub struct MigrationState {
    pub total_locked: u64,
    pub migration_active: bool,
    pub migration_toggle_timestamp: i64,
}

impl MigrationState {
    pub const LEN: usize = 8 + 8 + 1 + 8;
}

/// Individual user's migration record.
#[account]
pub struct MigrationRecord {
    pub user: Pubkey,
    pub locked_amount: u64,
    pub migrated: bool,
    pub migration_confirmed: bool,
}

impl MigrationRecord {
    pub const LEN: usize = 8 + 32 + 8 + 1 + 1;
}

/// Tracks presale purchases per wallet.
#[account]
pub struct PresalePurchase {
    pub wallet: Pubkey,
    pub total_purchased: u64,
    pub kyc_verified: bool,
}

impl PresalePurchase {
    pub const LEN: usize = 8 + 32 + 8 + 1;
}

/// Pending change for whitelisted contracts with a time-lock.
#[account]
pub struct PendingWhitelistChange {
    pub contract: Pubkey,
    pub add: bool,
    pub initiation_time: i64,
}

impl PendingWhitelistChange {
    pub const LEN: usize = 8 + 32 + 1 + 8;
}

/// Pending change for multisig with a time-lock.
#[account]
pub struct PendingMultisigChange {
    pub new_owners: Vec<Pubkey>,
    pub new_threshold: u8,
    pub initiation_time: i64,
}

impl PendingMultisigChange {
    pub const MAX_OWNERS: usize = 5;
    pub const LEN: usize = 8 + 4 + 32 * Self::MAX_OWNERS + 1 + 8;
}

/// Pending pause action with a time-lock.
#[account]
pub struct PendingPause {
    pub reason: String,
    pub initiation_time: i64,
}

impl PendingPause {
    pub const MAX_REASON_LEN: usize = 100;
    pub const LEN: usize = 8 + 4 + Self::MAX_REASON_LEN + 8;
}

/// Pending resume action with a time-lock.
#[account]
pub struct PendingResume {
    pub initiation_time: i64,
}

impl PendingResume {
    pub const LEN: usize = 8 + 8;
}


