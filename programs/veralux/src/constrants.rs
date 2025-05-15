pub const CONTRACT_STATE_SEED: &[u8] = b"contract_state";
pub const TREASURY_SEED: &[u8] = b"treasury";
pub const MULTISIG_SEED: &[u8] = b"multisig";
pub const MIGRATION_STATE_SEED: &[u8] = b"migration_state";
pub const PRESALE_PURCHASSE: &[u8] = b"presale_purchase";
pub const PRESALE_VESTING: &[u8] = b"presale_vesting";

pub const TOTAL_SUPPLY: u64 = 1_000_000_000 * 10u64.pow(9);
pub const TREASURY_RESERVE: u64 = 660_000_000 * 10u64.pow(9); // Updated to 660M tokens
pub const PRESALE_SUPPLY: u64 = 250_000_000 * 10u64.pow(9); // Updated to 250M tokens
pub const INITIAL_TAX_RATE: u64 = 500; // 5%
pub const INITIAL_STAKING_TIERS: [u64; 4] = [
    20_000 * 10u64.pow(9),
    100_000 * 10u64.pow(9),
    500_000 * 10u64.pow(9),
    5_000_000 * 10u64.pow(9),
];
pub const STAKING_REWARDS: [u64; 4] = [
    500 * 10u64.pow(9),
    2_500 * 10u64.pow(9),
    12_500 * 10u64.pow(9),
    125_000 * 10u64.pow(9),
];

pub const VOTING_QUORUM: u64 = 30; // 30%
pub const VOTING_APPROVAL: u64 = 51; // 51%
pub const VOTING_THRESHOLD: u64 = 20; // 20%
pub const STAKING_POOL_PCT: u64 = 30;
pub const AIRDROP_POOL_PCT: u64 = 8;
pub const GOVERNANCE_RESERVE_PCT: u64 = 16;
pub const MARKETING_FUND_PCT: u64 = 18;
pub const EMERGENCY_FUND_PCT: u64 = 5;
pub const TEAM_POOL_PCT: u64 = 18;

// Transaction limits
pub const MAX_SELL_TXN_LIMIT: u64 = TOTAL_SUPPLY / 200; // 0.5%
pub const DAILY_SELL_LIMIT: u64 = TOTAL_SUPPLY / 200; // 0.5%
pub const MAX_TRANSFER_LIMIT: u64 = TOTAL_SUPPLY / 200; // 0.5%
pub const DAILY_TRANSFER_LIMIT: u64 = TOTAL_SUPPLY / 200; // 0.5%
pub const PROGRESSIVE_TAX_THRESHOLD: u64 = TOTAL_SUPPLY / 200; // 0.5%
pub const TRANSACTION_TRACKING_THRESHOLD: u64 = TOTAL_SUPPLY / 1000; // 0.1%

// Presale-specific constants
pub const TOKEN_DECIMALS: u32 = 9;
pub const PRESALE_PRICE_PER_TOKEN: u64 = 1600; // 1600 USDT units per 10^9 token units (0.0016 USDT per token)
pub const PRESALE_MAX_PER_WALLET: u64 = 2_000_000 * 10u64.pow(TOKEN_DECIMALS); // 2M tokens
