
use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;
use state::{ContractState, PoolType};

declare_id!("YourProgramIdHere");

#[program]
pub mod the_saviors {
    use super::*;

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
        instructions::initialize(
            ctx,
            charity_wallet,
            team_wallet,
            liquidity_pool,
            launch_timestamp,
            initial_owners,
            threshold,
            initial_dex_programs,
            presale_usdt_receiver,
        )
    }

    pub fn claim_presale_tokens(ctx: Context<ClaimPresaleTokens>) -> Result<()> {
        instructions::claim_presale_tokens(ctx)
    }

    pub fn buy_presale(ctx: Context<BuyPresale>, amount: u64) -> Result<()> {
        instructions::buy_presale(ctx, amount)
    }

    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        instructions::transfer(ctx, amount)
    }

    pub fn whitelisted_transfer(ctx: Context<WhitelistedTransfer>, amount: u64) -> Result<()> {
        instructions::whitelisted_transfer(ctx, amount)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        instructions::stake(ctx, amount)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        instructions::unstake(ctx)
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        instructions::claim_rewards(ctx)
    }

    pub fn stake_lp(ctx: Context<StakeLP>, amount: u64) -> Result<()> {
        instructions::stake_lp(ctx, amount)
    }

    pub fn unstake_lp(ctx: Context<UnstakeLP>, amount: u64) -> Result<()> {
        instructions::unstake_lp(ctx, amount)
    }

    pub fn claim_lp_rewards(ctx: Context<ClaimLPRewards>) -> Result<()> {
        instructions::claim_lp_rewards(ctx)
    }

    pub fn process_daily_rewards(ctx: Context<ProcessDailyRewards>, batch_size: u64) -> Result<()> {
        instructions::process_daily_rewards(ctx, batch_size)
    }

    pub fn submit_proposal(
        ctx: Context<SubmitProposal>,
        description: String,
        proposal_type: u8,
        proposal_values: Vec<u64>,
    ) -> Result<()> {
        instructions::submit_proposal(ctx, description, proposal_type, proposal_values)
    }

    pub fn vote(ctx: Context<Vote>, proposal_id: u64, in_favor: bool) -> Result<()> {
        instructions::vote(ctx, proposal_id, in_favor)
    }

    pub fn execute_proposal(ctx: Context<ExecuteProposal>, proposal_id: u64) -> Result<()> {
        instructions::execute_proposal(ctx, proposal_id)
    }

    pub fn airdrop(ctx: Context<Airdrop>, winners: Vec<(Pubkey, u64)>) -> Result<()> {
        instructions::airdrop(ctx, winners)
    }

    pub fn pause(ctx: Context<InitiatePause>, reason: String) -> Result<()> {
        instructions::initiate_pause(ctx, reason)
    }

    pub fn confirm_pause(ctx: Context<ConfirmPause>) -> Result<()> {
        instructions::confirm_pause(ctx)
    }

    pub fn resume(ctx: Context<InitiateResume>) -> Result<()> {
        instructions::initiate_resume(ctx)
    }

    pub fn confirm_resume(ctx: Context<ConfirmResume>) -> Result<()> {
        instructions::confirm_resume(ctx)
    }

    pub fn update_team_vesting(
        ctx: Context<UpdateTeamVesting>,
        team_member: Pubkey,
        total_amount: u64,
        immediate_amount: u64,
    ) -> Result<()> {
        instructions::update_team_vesting(ctx, team_member, total_amount, immediate_amount)
    }

    pub fn cancel_team_vesting(ctx: Context<CancelTeamVesting>, team_member: Pubkey) -> Result<()> {
        instructions::cancel_team_vesting(ctx, team_member)
    }

    pub fn claim_team_vesting(ctx: Context<ClaimTeamVesting>) -> Result<()> {
        instructions::claim_team_vesting(ctx)
    }

    pub fn update_freelancer_vesting(
        ctx: Context<UpdateFreelancerVesting>,
        freelancer: Pubkey,
        total_amount: u64,
    ) -> Result<()> {
        instructions::update_freelancer_vesting(ctx, freelancer, total_amount)
    }

    pub fn claim_freelancer_vesting(ctx: Context<ClaimFreelancerVesting>) -> Result<()> {
        instructions::claim_freelancer_vesting(ctx)
    }

    pub fn release_freelancer_milestone(ctx: Context<ReleaseFreelancerMilestone>, amount: u64) -> Result<()> {
        instructions::release_freelancer_milestone(ctx, amount)
    }

    pub fn initiate_withdrawal(ctx: Context<InitiateWithdrawal>, amount: u64) -> Result<()> {
        instructions::initiate_withdrawal(ctx, amount)
    }

    pub fn complete_withdrawal(ctx: Context<CompleteWithdrawal>) -> Result<()> {
        instructions::complete_withdrawal(ctx)
    }

    pub fn set_multisig(ctx: Context<InitiateSetMultisig>, owners: Vec<Pubkey>, threshold: u8) -> Result<()> {
        instructions::initiate_set_multisig(ctx, owners, threshold)
    }

    pub fn confirm_set_multisig(ctx: Context<ConfirmSetMultisig>) -> Result<()> {
        instructions::confirm_set_multisig(ctx)
    }

    pub fn distribute_lp_incentives(ctx: Context<DistributeLPIncentives>, recipients: Vec<(Pubkey, u64)>) -> Result<()> {
        instructions::distribute_lp_incentives(ctx, recipients)
    }

    pub fn update_dex_programs(ctx: Context<UpdateDexPrograms>, dex_programs: Vec<Pubkey>) -> Result<()> {
        instructions::update_dex_programs(ctx, dex_programs)
    }

    pub fn add_whitelisted_contract(ctx: Context<AddWhitelistedContract>, contract: Pubkey) -> Result<()> {
        instructions::add_whitelisted_contract(ctx, contract)
    }

    pub fn remove_whitelisted_contract(ctx: Context<RemoveWhitelistedContract>, contract: Pubkey) -> Result<()> {
        instructions::remove_whitelisted_contract(ctx, contract)
    }

    pub fn confirm_whitelist_change(ctx: Context<ConfirmWhitelistChange>) -> Result<()> {
        instructions::confirm_whitelist_change(ctx)
    }

    pub fn lock_for_migration(ctx: Context<LockForMigration>, amount: u64) -> Result<()> {
        instructions::lock_for_migration(ctx, amount)
    }

    pub fn unlock_for_migration(ctx: Context<UnlockForMigration>) -> Result<()> {
        instructions::unlock_for_migration(ctx)
    }

    pub fn burn_locked_tokens(ctx: Context<BurnLockedTokens>, user: Pubkey) -> Result<()> {
        instructions::burn_locked_tokens(ctx, user)
    }

    pub fn confirm_migration(ctx: Context<ConfirmMigration>) -> Result<()> {
        instructions::confirm_migration(ctx)
    }

    pub fn toggle_migration_active(ctx: Context<ToggleMigrationActive>, active: bool) -> Result<()> {
        instructions::toggle_migration_active(ctx, active)
    }

    pub fn transfer_between_pools(
        ctx: Context<TransferBetweenPools>,
        source_pool: PoolType,
        dest_pool: PoolType,
        amount: u64,
    ) -> Result<()> {
        instructions::transfer_between_pools(ctx, source_pool, dest_pool, amount)
    }

    pub fn query_pending_rewards(ctx: Context<QueryPendingRewards>) -> Result<()> {
        instructions::query_pending_rewards(ctx)
    }

    pub fn query_state(ctx: Context<QueryState>) -> Result<ContractState> {
        instructions::query_state(ctx)
    }
}

// Account structs (original structs unchanged unless noted)
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer1: Signer<'info>,
    pub signer2: Option<Signer<'info>>,
    pub signer3: Option<Signer<'info>>,
    pub signer4: Option<Signer<'info>>,
    pub signer5: Option<Signer<'info>>,
    #[account(init, payer = signer1, space = ContractState::LEN)]
    pub state: Account<'info, ContractState>,
    #[account(init, payer = signer1, space = Treasury::LEN)]
    pub treasury: Account<'info, Treasury>,
    #[account(init, payer = signer1, space = Multisig::LEN)]
    pub multisig: Account<'info, Multisig>,
    #[account(init, payer = signer1, space = MigrationState::LEN)]
    pub migration_state: Account<'info, MigrationState>,
    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = signer1,
        token::mint = token_mint,
        token::authority = holding_pool_pda
    )]
    pub holding_pool_token_account: Account<'info, TokenAccount>,
    #[account(
        seeds = [b"lp_reward_holding"],
        bump
    )]
    pub holding_pool_pda: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    /// CHECK: DEX program accounts to validate as executable
    pub remaining_accounts: Vec<AccountInfo<'info>>,
}

#[derive(Accounts)]
pub struct ClaimPresaleTokens<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(mut)]
    pub vesting: Account<'info, PresaleVesting>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    #[account(seeds = [b"treasury_authority"], bump)]
    pub treasury_pda: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct BuyPresale<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(
        init_if_needed,
        payer = buyer,
        space = PresalePurchase::LEN,
        seeds = [b"presale_purchase", buyer.key().as_ref()],
        bump
    )]
    pub presale_purchase: Account<'info, PresalePurchase>,
    #[account(
        init_if_needed,
        payer = buyer,
        space = PresaleVesting::LEN,
        seeds = [b"presale_vesting", buyer.key().as_ref()],
        bump
    )]
    pub presale_vesting: Account<'info, PresaleVesting>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub buyer_usdt_account: Account<'info, TokenAccount>,
    #[account(mut, address = state.presale_usdt_receiver)]
    pub presale_usdt_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(mut)]
    pub treasury: Account<'info, Treasury>,
    #[account(
        init_if_needed,
        payer = sender,
        space = TransactionRecord::LEN,
        seeds = [b"txn_record", sender.key().as_ref()],
        bump
    )]
    pub txn_record: Account<'info, TransactionRecord>,
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mut)]
    pub sender_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub liquidity_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub charity_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub team_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WhitelistedTransfer<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(mut)]
    pub treasury: Account<'info, Treasury>,
    #[account(mut)]
    pub sender_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub liquidity_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub charity_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub team_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    #[account(seeds = [b"whitelisted_authority", caller_program.key().as_ref()], bump)]
    pub whitelisted_pda: AccountInfo<'info>,
    pub caller_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(
        init_if_needed,
        payer = user,
        space = Staker::LEN,
        seeds = [b"staker", user.key().as_ref()],
        bump
    )]
    pub staker: Account<'info, Staker>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub staking_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(
        mut,
        close = user,
        seeds = [b"staker", user.key().as_ref()],
        bump
    )]
    pub staker: Account<'info, Staker>,
    #[account(mut)]
    pub treasury: Account<'info, Treasury>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub staking_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    #[account(seeds = [b"staking_authority"], bump)]
    pub staking_pda: AccountInfo<'info>,
    #[account(seeds = [b"treasury_authority"], bump)]
    pub treasury_pda: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct StakeLP<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(
        init_if_needed,
        payer = user,
        space = LPStaker::LEN,
        seeds = [b"lp_staker", user.key().as_ref()],
        bump
    )]
    pub lp_staker: Account<'info, LPStaker>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_lp_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub staking_lp_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UnstakeLP<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(
        mut,
        seeds = [b"lp_staker", user.key().as_ref()],
        bump
    )]
    pub lp_staker: Account<'info, LPStaker>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_lp_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub staking_lp_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    #[account(seeds = [b"lp_staking_authority"], bump)]
    pub lp_staking_pda: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(
        mut,
        seeds = [b"staker", user.key().as_ref()],
        bump
    )]
    pub staker: Account<'info, Staker>,
    #[account(mut)]
    pub treasury: Account<'info, Treasury>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    #[account(seeds = [b"treasury_authority"], bump)]
    pub treasury_pda: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ClaimLPRewards<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(
        mut,
        seeds = [b"lp_staker", user.key().as_ref()],
        bump
    )]
    pub lp_staker: Account<'info, LPStaker>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub holding_pool_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    #[account(seeds = [b"lp_reward_holding"], bump)]
    pub holding_pool_pda: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ProcessDailyRewards<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(mut)]
    pub treasury: Account<'info, Treasury>,
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub holding_pool_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    /// CHECK: LPStaker accounts provided by caller as remaining accounts
    pub remaining_accounts: Vec<AccountInfo<'info>>,
}

#[derive(Accounts)]
pub struct SubmitProposal<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(init, payer = signer1, space = Proposal::LEN)]
    pub proposal: Account<'info, Proposal>,
    pub multisig: Account<'info, Multisig>,
    pub signer1: Signer<'info>,
    pub signer2: Option<Signer<'info>>,
    pub signer3: Option<Signer<'info>>,
    pub signer4: Option<Signer<'info>>,
    pub signer5: Option<Signer<'info>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(
        seeds = [b"staker", user.key().as_ref()],
        bump
    )]
    pub staker: Account<'info, Staker>,
    #[account(
        init_if_needed,
        payer = user,
        space = VoteRecord::LEN,
        seeds = [b"vote", user.key().as_ref(), &proposal.id.to_le_bytes()],
        bump
    )]
    pub vote_record: Account<'info, VoteRecord>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
}

#[derive(Accounts)]
pub struct Airdrop<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(mut)]
    pub treasury: Account<'info, Treasury>,
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    #[account(seeds = [b"treasury_authority"], bump)]
    pub treasury_pda: AccountInfo<'info>,
    pub multisig: Account<'info, Multisig>,
    pub signer1: Signer<'info>,
    pub signer2: Option<Signer<'info>>,
    pub signer3: Option<Signer<'info>>,
    pub signer4: Option<Signer<'info>>,
    pub signer5: Option<Signer<'info>>,
}

#[derive(Accounts)]
pub struct InitiatePause<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(init, payer = signer1, space = PendingPause::LEN)]
    pub pending_pause: Account<'info, PendingPause>,
    pub multisig: Account<'info, Multisig>,
    pub signer1: Signer<'info>,
    pub signer2: Option<Signer<'info>>,
    pub signer3: Option<Signer<'info>>,
    pub signer4: Option<Signer<'info>>,
    pub signer5: Option<Signer<'info>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConfirmPause<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(mut, close = signer)]
    pub pending_pause: Account<'info, PendingPause>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct InitiateResume<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(init, payer = signer1, space = PendingResume::LEN)]
    pub pending_resume: Account<'info, PendingResume>,
    pub multisig: Account<'info, Multisig>,
    pub signer1: Signer<'info>,
    pub signer2: Option<Signer<'info>>,
    pub signer3: Option<Signer<'info>>,
    pub signer4: Option<Signer<'info>>,
    pub signer5: Option<Signer<'info>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConfirmResume<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(mut, close = signer)]
    pub pending_resume: Account<'info, PendingResume>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateTeamVesting<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(
        init_if_needed,
        payer = signer1,
        space = TeamVesting::LEN,
        seeds = [b"team_vesting", team_member.key().as_ref()],
        bump
    )]
    pub team_vesting: Account<'info, TeamVesting>,
    /// CHECK: Team member's public key
    pub team_member: AccountInfo<'info>,
    pub multisig: Account<'info, Multisig>,
    pub signer1: Signer<'info>,
    pub signer2: Option<Signer<'info>>,
    pub signer3: Option<Signer<'info>>,
    pub signer4: Option<Signer<'info>>,
    pub signer5: Option<Signer<'info>>,
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub team_member_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    #[account(seeds = [b"treasury_authority"], bump)]
    pub treasury_pda: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CancelTeamVesting<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(
        mut,
        seeds = [b"team_vesting", team_member.key().as_ref()],
        bump
    )]
    pub team_vesting: Account<'info, TeamVesting>,
    pub multisig: Account<'info, Multisig>,
    pub signer1: Signer<'info>,
    pub signer2: Option<Signer<'info>>,
    pub signer3: Option<Signer<'info>>,
    pub signer4: Option<Signer<'info>>,
    pub signer5: Option<Signer<'info>>,
    pub team_member: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ClaimTeamVesting<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(mut, seeds = [b"team_vesting", team_member.key().as_ref()], bump)]
    pub team_vesting: Account<'info, TeamVesting>,
    #[account(mut)]
    pub team_member: Signer<'info>,
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub team_member_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    #[account(seeds = [b"treasury_authority"], bump)]
    pub treasury_pda: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateFreelancerVesting<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(
        init_if_needed,
        payer = signer1,
        space = FreelancerVesting::LEN,
        seeds = [b"freelancer_vesting", freelancer.key().as_ref()],
        bump
    )]
    pub freelancer_vesting: Account<'info, FreelancerVesting>,
    pub freelancer: AccountInfo<'info>,
    pub multisig: Account<'info, Multisig>,
    pub signer1: Signer<'info>,
    pub signer2: Option<Signer<'info>>,
    pub signer3: Option<Signer<'info>>,
    pub signer4: Option<Signer<'info>>,
    pub signer5: Option<Signer<'info>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimFreelancerVesting<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(mut, seeds = [b"freelancer_vesting", freelancer.key().as_ref()], bump)]
    pub freelancer_vesting: Account<'info, FreelancerVesting>,
    #[account(mut)]
    pub freelancer: Signer<'info>,
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub freelancer_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    #[account(seeds = [b"treasury_authority"], bump)]
    pub treasury_pda: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ReleaseFreelancerMilestone<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(mut, seeds = [b"freelancer_vesting", freelancer.key().as_ref()], bump)]
    pub freelancer_vesting: Account<'info, FreelancerVesting>,
    /// CHECK: Freelancer's public key
    pub freelancer: AccountInfo<'info>,
    pub multisig: Account<'info, Multisig>,
    pub signer1: Signer<'info>,
    pub signer2: Option<Signer<'info>>,
    pub signer3: Option<Signer<'info>>,
    pub signer4: Option<Signer<'info>>,
    pub signer5: Option<Signer<'info>>,
}

#[derive(Accounts)]
pub struct InitiateWithdrawal<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(init, payer = signer1, space = PendingWithdrawal::LEN)]
    pub pending_withdrawal: Account<'info, PendingWithdrawal>,
    pub multisig: Account<'info, Multisig>,
    pub signer1: Signer<'info>,
    pub signer2: Option<Signer<'info>>,
    pub signer3: Option<Signer<'info>>,
    pub signer4: Option<Signer<'info>>,
    pub signer5: Option<Signer<'info>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CompleteWithdrawal<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(mut)]
    pub pending_withdrawal: Account<'info, PendingWithdrawal>,
    pub multisig: Account<'info, Multisig>,
    pub signer1: Signer<'info>,
    pub signer2: Option<Signer<'info>>,
    pub signer3: Option<Signer<'info>>,
    pub signer4: Option<Signer<'info>>,
    pub signer5: Option<Signer<'info>>,
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub destination_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    #[account(seeds = [b"treasury_authority"], bump)]
    pub treasury_pda: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitiateSetMultisig<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(init, payer = signer1, space = PendingMultisigChange::LEN)]
    pub pending_multisig_change: Account<'info, PendingMultisigChange>,
    pub multisig: Account<'info, Multisig>,
    pub signer1: Signer<'info>,
    pub signer2: Option<Signer<'info>>,
    pub signer3: Option<Signer<'info>>,
    pub signer4: Option<Signer<'info>>,
    pub signer5: Option<Signer<'info>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConfirmSetMultisig<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(mut, close = signer)]
    pub pending_multisig_change: Account<'info, PendingMultisigChange>,
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct DistributeLPIncentives<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(mut)]
    pub treasury: Account<'info, Treasury>,
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    #[account(seeds = [b"treasury_authority"], bump)]
    pub treasury_pda: AccountInfo<'info>,
    pub multisig: Account<'info, Multisig>,
    pub signer1: Signer<'info>,
    pub signer2: Option<Signer<'info>>,
    pub signer3: Option<Signer<'info>>,
    pub signer4: Option<Signer<'info>>,
    pub signer5: Option<Signer<'info>>,
}

#[derive(Accounts)]
pub struct UpdateDexPrograms<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    pub multisig: Account<'info, Multisig>,
    pub signer1: Signer<'info>,
    pub signer2: Option<Signer<'info>>,
    pub signer3: Option<Signer<'info>>,
    pub signer4: Option<Signer<'info>>,
    pub signer5: Option<Signer<'info>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddWhitelistedContract<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    pub multisig: Account<'info, Multisig>,
    pub signer1: Signer<'info>,
    pub signer2: Option<Signer<'info>>,
    pub signer3: Option<Signer<'info>>,
    pub signer4: Option<Signer<'info>>,
    pub signer5: Option<Signer<'info>>,
    pub system_program: Program<'info, System>,
    /// CHECK: Contract to whitelist, validated in instruction
    pub contract: AccountInfo<'info>,
    #[account(init, payer = signer1, space = PendingWhitelistChange::LEN)]
    pub pending_whitelist_change: Account<'info, PendingWhitelistChange>,
}

#[derive(Accounts)]
pub struct RemoveWhitelistedContract<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    pub multisig: Account<'info, Multisig>,
    pub signer1: Signer<'info>,
    pub signer2: Option<Signer<'info>>,
    pub signer3: Option<Signer<'info>>,
    pub signer4: Option<Signer<'info>>,
    pub signer5: Option<Signer<'info>>,
    pub system_program: Program<'info, System>,
    /// CHECK: Contract to remove, validated in instruction
    pub contract: AccountInfo<'info>,
    #[account(init, payer = signer1, space = PendingWhitelistChange::LEN)]
    pub pending_whitelist_change: Account<'info, PendingWhitelistChange>,
}

#[derive(Accounts)]
pub struct ConfirmWhitelistChange<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(mut)]
    pub pending_whitelist_change: Account<'info, PendingWhitelistChange>,
}

#[derive(Accounts)]
pub struct LockForMigration<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(mut)]
    pub migration_state: Account<'info, MigrationState>,
    #[account(
        init_if_needed,
        payer = user,
        space = MigrationRecord::LEN,
        seeds = [b"migration_record", user.key().as_ref()],
        bump
    )]
    pub migration_record: Account<'info, MigrationRecord>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub migration_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UnlockForMigration<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(mut)]
    pub migration_state: Account<'info, MigrationState>,
    #[account(
        mut,
        seeds = [b"migration_record", user.key().as_ref()],
        bump
    )]
    pub migration_record: Account<'info, MigrationRecord>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub migration_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    #[account(seeds = [b"migration_authority"], bump)]
    pub migration_pda: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct BurnLockedTokens<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(mut)]
    pub migration_state: Account<'info, MigrationState>,
    #[account(mut, seeds = [b"migration_record", user.key().as_ref()], bump)]
    pub migration_record: Account<'info, MigrationRecord>,
    #[account(mut)]
    pub migration_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    pub multisig: Account<'info, Multisig>,
    pub signer1: Signer<'info>,
    pub signer2: Option<Signer<'info>>,
    pub signer3: Option<Signer<'info>>,
    pub signer4: Option<Signer<'info>>,
    pub signer5: Option<Signer<'info>>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ConfirmMigration<'info> {
    #[account(mut)]
    pub migration_record: Account<'info, MigrationRecord>,
    pub multisig: Account<'info, Multisig>,
    pub signer1: Signer<'info>,
    pub signer2: Option<Signer<'info>>,
    pub signer3: Option<Signer<'info>>,
    pub signer4: Option<Signer<'info>>,
    pub signer5: Option<Signer<'info>>,
}

#[derive(Accounts)]
pub struct ToggleMigrationActive<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(mut)]
    pub migration_state: Account<'info, MigrationState>,
    pub multisig: Account<'info, Multisig>,
    pub signer1: Signer<'info>,
    pub signer2: Option<Signer<'info>>,
    pub signer3: Option<Signer<'info>>,
    pub signer4: Option<Signer<'info>>,
    pub signer5: Option<Signer<'info>>,
}

#[derive(Accounts)]
pub struct TransferBetweenPools<'info> {
    #[account(mut)]
    pub state: Account<'info, ContractState>,
    #[account(mut)]
    pub treasury: Account<'info, Treasury>,
    pub multisig: Account<'info, Multisig>,
    pub signer1: Signer<'info>,
    pub signer2: Option<Signer<'info>>,
    pub signer3: Option<Signer<'info>>,
    pub signer4: Option<Signer<'info>>,
    pub signer5: Option<Signer<'info>>,
}

#[derive(Accounts)]
pub struct QueryPendingRewards<'info> {
    pub state: Account<'info, ContractState>,
    /// CHECK: Staker account, may not exist
    pub staker: AccountInfo<'info>,
    pub treasury: Account<'info, Treasury>,
    /// CHECK: User's public key for deriving staker PDA
    pub user: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct QueryState<'info> {
    pub state: Account<'info, ContractState>,
}



