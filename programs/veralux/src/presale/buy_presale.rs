use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Token, TokenAccount, Transfer},
};

use crate::{
    ContractState, PresalePurchase, PresalePurchaseEvent, PresaleVesting, ReentrancyGuard,
    VeraluxErrorCode, PRESALE_MAX_PER_WALLET, PRESALE_PRICE_PER_TOKEN, PRESALE_PURCHASSE,
    PRESALE_SUPPLY, PRESALE_VESTING, TOKEN_DECIMALS,
};

#[derive(Accounts)]
pub struct BuyPresale<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub state: Box<Account<'info, ContractState>>,

    #[account(
        mut,
        seeds = [PRESALE_PURCHASSE, buyer.key().as_ref()],
        bump
    )]
    pub presale_purchase: Box<Account<'info, PresalePurchase>>,

    #[account(
        mut,
        seeds = [PRESALE_VESTING, buyer.key().as_ref()],
        bump
    )]
    pub presale_vesting: Box<Account<'info, PresaleVesting>>,

    #[account(
        mut,
        constraint = buyer_usdt_account.owner == buyer.key() @ VeraluxErrorCode::InvalidBuyerUSDTOwner,
    )]
    pub buyer_usdt_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        address = state.presale_usdt_receiver @ VeraluxErrorCode::InvalidPresaleUSDTReceiver,
    )]
    pub presale_usdt_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl BuyPresale<'_> {
    pub fn handler(ctx: &mut Context<BuyPresale>, usdt_amount: u64) -> Result<()> {
        let _guard = ReentrancyGuard::new(&mut ctx.accounts.state)?;

        let state = &mut ctx.accounts.state;
        require!(!state.paused, VeraluxErrorCode::Paused);
        require!(state.presale_active, VeraluxErrorCode::PresaleNotActive);

        let purchase = &mut ctx.accounts.presale_purchase;
        let vesting = &mut ctx.accounts.presale_vesting;

        let token_amount = ((usdt_amount as u128)
            .checked_mul(10u128.pow(TOKEN_DECIMALS as u32))
            .ok_or(VeraluxErrorCode::ArithmeticOverflow)?
            / PRESALE_PRICE_PER_TOKEN as u128)
            .try_into()
            .map_err(|_| VeraluxErrorCode::ArithmeticOverflow)?;

        require!(
            state
                .total_presale_sold
                .checked_add(token_amount)
                .ok_or(VeraluxErrorCode::ArithmeticOverflow)?
                <= PRESALE_SUPPLY,
            VeraluxErrorCode::PresaleSupplyExceeded
        );
        require!(
            usdt_amount < 1000 || purchase.kyc_verified,
            VeraluxErrorCode::KYCRequired
        );
        require!(
            purchase
                .total_purchased
                .checked_add(token_amount)
                .ok_or(VeraluxErrorCode::ArithmeticOverflow)?
                <= PRESALE_MAX_PER_WALLET,
            VeraluxErrorCode::PresaleMaxPerWalletExceeded
        );

        msg!("buyer: {}", ctx.accounts.buyer.key());

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.buyer_usdt_account.to_account_info(),
                    to: ctx.accounts.presale_usdt_account.to_account_info(),
                    authority: ctx.accounts.buyer.to_account_info(),
                },
            ),
            usdt_amount,
        )?;

        msg!(
            "buyer_usdt_account: {}",
            ctx.accounts.buyer_usdt_account.to_account_info().key()
        );

        purchase.total_purchased = purchase
            .total_purchased
            .checked_add(token_amount)
            .ok_or(VeraluxErrorCode::ArithmeticOverflow)?;
        vesting.total_amount = vesting
            .total_amount
            .checked_add(token_amount)
            .ok_or(VeraluxErrorCode::ArithmeticOverflow)?;
        state.total_presale_sold = state
            .total_presale_sold
            .checked_add(token_amount)
            .ok_or(VeraluxErrorCode::ArithmeticOverflow)?;

        emit!(PresalePurchaseEvent {
            buyer: ctx.accounts.buyer.key(),
            usdt_amount,
            token_amount,
        });

        Ok(())
    }
}
