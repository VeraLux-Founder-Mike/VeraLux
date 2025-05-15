use anchor_lang::prelude::*;

use crate::{PresalePurchase, PresaleVesting, PRESALE_PURCHASSE, PRESALE_VESTING};

#[derive(Accounts)]
pub struct InitPresale<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        init,
        payer = buyer,
        space = 8 + PresalePurchase::INIT_SPACE,
        seeds = [PRESALE_PURCHASSE, buyer.key().as_ref()],
        bump
    )]
    pub presale_purchase: Box<Account<'info, PresalePurchase>>,

    #[account(
        init,
        payer = buyer,
        space = 8 + PresaleVesting::INIT_SPACE,
        seeds = [PRESALE_VESTING, buyer.key().as_ref()],
        bump
    )]
    pub presale_vesting: Box<Account<'info, PresaleVesting>>,

    pub system_program: Program<'info, System>,
}

impl InitPresale<'_> {
    pub fn handler(ctx: &mut Context<InitPresale>) -> Result<()> {
        let purchase = &mut ctx.accounts.presale_purchase;

        purchase.wallet = ctx.accounts.buyer.key();
        purchase.total_purchased = 0;
        purchase.kyc_verified = true;

        Ok(())
    }
}
