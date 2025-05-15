use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ContractState {
    pub authority: Pubkey,
    pub admin: Pubkey,
    pub treasury: Pubkey,
    pub charity_wallet: Pubkey,
    pub team_wallet: Pubkey,
    pub liquidity_pool: Pubkey,
    pub presale_usdt_receiver: Pubkey,
    pub proposal_count: u64,
    pub total_voting_power: u64,
    pub launch_timestamp: i64,
    pub tax_rate: u64,
    pub progressive_tax_threshold: u64,
    pub total_presale_sold: u64,
    #[max_len(5)]
    pub dex_programs: Vec<Pubkey>,
    #[max_len(50)]
    pub pause_reason: String,
    pub is_processing: bool,
    pub presale_active: bool,
    pub paused: bool,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Treasury {
    pub staking_pool: u64,
    pub airdrop_pool: u64,
    pub governance_reserve: u64,
    pub marketing_fund: u64,
    pub emergency_fund: u64,
    pub liquidity_incentive: u64,
    pub team_pool: u64,
    pub launch_timestamp: i64,
}

#[account]
#[derive(InitSpace)]
pub struct Multisig {
    #[max_len(5)]
    pub owners: Vec<Pubkey>,
    pub threshold: u8,
}

#[account]
#[derive(InitSpace)]
pub struct MigrationState {
    pub total_locked: u64,
    pub migration_toggle_timestamp: i64,
    pub migration_active: bool,
}

#[account]
#[derive(InitSpace)]
pub struct PresalePurchase {
    pub wallet: Pubkey,
    pub total_purchased: u64,
    pub kyc_verified: bool,
}

#[account]
#[derive(InitSpace)]
pub struct PresaleVesting {
    pub total_amount: u64,
    pub claimed_amount: u64,
}
