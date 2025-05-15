use anchor_lang::prelude::*;

#[event]
pub struct InitializeEvent {
    pub launch_timestamp: i64,
    pub initial_owners: Vec<Pubkey>,
    pub threshold: u8,
}

#[event]
pub struct PresalePurchaseEvent {
    pub buyer: Pubkey,
    pub usdt_amount: u64,
    pub token_amount: u64,
}
