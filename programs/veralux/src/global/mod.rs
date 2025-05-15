use anchor_lang::prelude::*;

pub mod init_global;
pub mod update_global;

pub use init_global::*;
pub use update_global::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitGlobalIx {
    pub charity_wallet: Pubkey,
    pub team_wallet: Pubkey,
    pub liquidity_pool: Pubkey,
    pub launch_timestamp: i64,
    pub presale_usdt_receiver: Pubkey,
    pub initial_owners: Vec<Pubkey>,
    pub initial_dex_programs: Vec<Pubkey>,
    pub threshold: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateGlobalIx {
    pub presale_usdt_receiver: Pubkey,
    pub launch_timestamp: i64,
    pub team_wallet: Pubkey,
    pub charity_wallet: Pubkey,
}
