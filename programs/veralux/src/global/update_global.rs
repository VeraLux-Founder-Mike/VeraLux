use anchor_lang::prelude::*;

use crate::{ContractState, CONTRACT_STATE_SEED};

use super::UpdateGlobalIx;

#[derive(Accounts)]
pub struct UpdateGlobal<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        constraint = state.authority == signer.key(),
        seeds = [CONTRACT_STATE_SEED],
        bump,
    )]
    pub state: Box<Account<'info, ContractState>>,

    pub system_program: Program<'info, System>,
}

impl UpdateGlobal<'_> {
    pub fn handler(ctx: &mut Context<UpdateGlobal>, ix: UpdateGlobalIx) -> Result<()> {
        if ix.launch_timestamp > 0 {
            ctx.accounts.state.launch_timestamp = ix.launch_timestamp;
        }

        if ix.team_wallet != ctx.accounts.state.team_wallet {
            ctx.accounts.state.team_wallet = ix.team_wallet;
        }

        if ix.charity_wallet != ctx.accounts.state.charity_wallet {
            ctx.accounts.state.charity_wallet = ix.charity_wallet;
        }

        if ix.presale_usdt_receiver != ctx.accounts.state.presale_usdt_receiver {
            ctx.accounts.state.presale_usdt_receiver = ix.presale_usdt_receiver;
        }

        Ok(())
    }
}
