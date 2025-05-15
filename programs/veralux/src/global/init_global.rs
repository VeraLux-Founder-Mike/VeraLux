use std::collections::HashSet;

use anchor_lang::prelude::*;

use crate::{
    validate_multisig, ContractState, InitializeEvent, MigrationState, Multisig, ReentrancyGuard,
    Treasury, VeraluxErrorCode, AIRDROP_POOL_PCT, CONTRACT_STATE_SEED, EMERGENCY_FUND_PCT,
    GOVERNANCE_RESERVE_PCT, INITIAL_TAX_RATE, MARKETING_FUND_PCT, MIGRATION_STATE_SEED,
    MULTISIG_SEED, PROGRESSIVE_TAX_THRESHOLD, STAKING_POOL_PCT, TEAM_POOL_PCT, TREASURY_RESERVE,
    TREASURY_SEED,
};

use super::InitGlobalIx;

#[derive(Accounts)]
pub struct InitGlobal<'info> {
    #[account(mut)]
    pub signer1: Signer<'info>,
    pub signer2: Option<Signer<'info>>,
    pub signer3: Option<Signer<'info>>,
    pub signer4: Option<Signer<'info>>,
    pub signer5: Option<Signer<'info>>,

    #[account(
        init,
        payer = signer1,
        space = 8 + ContractState::INIT_SPACE,
        seeds = [CONTRACT_STATE_SEED],
        bump,
    )]
    pub state: Box<Account<'info, ContractState>>,

    #[account(
        init,
        payer = signer1,
        space = 8 + Treasury::INIT_SPACE,
        seeds = [TREASURY_SEED],
        bump,
    )]
    pub treasury: Box<Account<'info, Treasury>>,

    #[account(
        init,
        payer = signer1,
        space = 8 + Multisig::INIT_SPACE,
        seeds = [MULTISIG_SEED],
        bump,
    )]
    pub multisig: Box<Account<'info, Multisig>>,

    #[account(
        init,
        payer = signer1,
        space = 8 + MigrationState::INIT_SPACE,
        seeds = [MIGRATION_STATE_SEED],
        bump,
    )]
    pub migration_state: Box<Account<'info, MigrationState>>,

    // pub token_mint: Account<'info, Mint>,

    // #[account(
    //     init,
    //     payer = signer1,
    //     token::mint = token_mint,
    //     token::authority = lp_reward_holding_pda,
    // )]
    // pub holding_pool_token_account: Account<'info, TokenAccount>,

    // #[account(
    //     seeds = [b"lp_reward_holding"],
    //     bump
    // )]
    // /// CHECK:
    // pub lp_reward_holding_pda: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    // pub token_program: Program<'info, Token>,
}

impl InitGlobal<'_> {
    pub fn handler(ctx: &mut Context<InitGlobal>, ix: InitGlobalIx) -> Result<()> {
        let _guard = ReentrancyGuard::new(&mut ctx.accounts.state)?;

        require!(ix.threshold >= 2, VeraluxErrorCode::InvalidThreshold);
        require!(
            ix.initial_owners.len() >= 3 && ix.initial_owners.len() <= 5,
            VeraluxErrorCode::TooFewOwners
        );

        let unique_owners: HashSet<Pubkey> = ix.initial_owners.iter().cloned().collect();
        require!(
            unique_owners.len() == ix.initial_owners.len(),
            VeraluxErrorCode::DuplicateOwners
        );

        validate_multisig(
            &ctx.accounts.multisig,
            &[
                &Some(&ctx.accounts.signer1),
                &ctx.accounts.signer2.as_ref(),
                &ctx.accounts.signer3.as_ref(),
                &ctx.accounts.signer4.as_ref(),
                &ctx.accounts.signer5.as_ref(),
            ],
        )?;

        require!(
            ctx.remaining_accounts.len() == ix.initial_dex_programs.len(),
            VeraluxErrorCode::InvalidAccounts
        );

        for (i, program_id) in ix.initial_dex_programs.iter().enumerate() {
            let account_info = &ctx.remaining_accounts[i];
            require!(
                account_info.key() == *program_id && account_info.executable,
                VeraluxErrorCode::InvalidDexProgram
            );
        }

        let state = &mut ctx.accounts.state;
        require!(
            ix.initial_dex_programs.len() <= 5,
            VeraluxErrorCode::VectorOverflow
        );
        state.authority = ctx.accounts.signer1.key();
        state.dex_programs = ix.initial_dex_programs.clone();
        state.paused = false;
        state.pause_reason = String::new();
        state.treasury = ctx.accounts.treasury.key();
        state.charity_wallet = ix.charity_wallet;
        state.team_wallet = ix.team_wallet;
        state.liquidity_pool = ix.liquidity_pool;
        state.proposal_count = 0;
        state.total_voting_power = 0;
        state.launch_timestamp = ix.launch_timestamp;
        state.admin = ctx.accounts.multisig.key();
        state.tax_rate = INITIAL_TAX_RATE;
        state.progressive_tax_threshold = PROGRESSIVE_TAX_THRESHOLD;
        state.presale_usdt_receiver = ix.presale_usdt_receiver;
        state.presale_active = true;
        state.total_presale_sold = 0;
        state.is_processing = false;

        let treasury: &mut Account<'_, Treasury> = &mut ctx.accounts.treasury;
        msg!("treasury: {:?}", treasury);
        let total_treasury = TREASURY_RESERVE;
        let staking_pool = (total_treasury as u128 * STAKING_POOL_PCT as u128 / 100)
            .try_into()
            .map_err(|_| VeraluxErrorCode::ArithmeticOverflow)?;
        let airdrop_pool = (total_treasury as u128 * AIRDROP_POOL_PCT as u128 / 100)
            .try_into()
            .map_err(|_| VeraluxErrorCode::ArithmeticOverflow)?;
        let governance_reserve = (total_treasury as u128 * GOVERNANCE_RESERVE_PCT as u128 / 100)
            .try_into()
            .map_err(|_| VeraluxErrorCode::ArithmeticOverflow)?;
        let marketing_fund = (total_treasury as u128 * MARKETING_FUND_PCT as u128 / 100)
            .try_into()
            .map_err(|_| VeraluxErrorCode::ArithmeticOverflow)?;
        let emergency_fund = (total_treasury as u128 * EMERGENCY_FUND_PCT as u128 / 100)
            .try_into()
            .map_err(|_| VeraluxErrorCode::ArithmeticOverflow)?;
        let team_pool = (total_treasury as u128 * TEAM_POOL_PCT as u128 / 100)
            .try_into()
            .map_err(|_| VeraluxErrorCode::ArithmeticOverflow)?;
        treasury.staking_pool = staking_pool;
        treasury.airdrop_pool = airdrop_pool;
        treasury.governance_reserve = governance_reserve;
        treasury.marketing_fund = marketing_fund;
        treasury.emergency_fund = emergency_fund;
        treasury.team_pool = team_pool;
        treasury.liquidity_incentive = 0;

        let multisig = &mut ctx.accounts.multisig;
        require!(
            ix.initial_owners.len() <= 10,
            VeraluxErrorCode::InvalidVectorSize
        );
        require!(
            ix.threshold as usize <= ix.initial_owners.len(),
            VeraluxErrorCode::InvalidProposal
        );
        multisig.owners = ix.initial_owners.clone();
        multisig.threshold = ix.threshold;

        let migration_state = &mut ctx.accounts.migration_state;
        migration_state.total_locked = 0;
        migration_state.migration_active = false;
        migration_state.migration_toggle_timestamp = 0;

        emit!(InitializeEvent {
            launch_timestamp: ix.launch_timestamp,
            initial_owners: ix.initial_owners,
            threshold: ix.threshold,
        });

        Ok(())
    }
}
