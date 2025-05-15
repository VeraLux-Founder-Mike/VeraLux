use anchor_lang::prelude::*;

pub mod constrants;
pub mod errors;
pub mod events;
pub mod global;
pub mod presale;
pub mod state;
pub mod utils;

pub use constrants::*;
pub use errors::*;
pub use events::*;
pub use global::*;
pub use presale::*;
pub use state::*;
pub use utils::*;

declare_id!("4BzWp6JCnguTPGEfHhG12hfJR86tGGU3fJSEdfuo2nBZ");

#[program]
pub mod veralux {
    use super::*;

    pub fn init_global(mut ctx: Context<InitGlobal>, ix: InitGlobalIx) -> Result<()> {
        InitGlobal::handler(&mut ctx, ix)
    }

    pub fn update_global(mut ctx: Context<UpdateGlobal>, ix: UpdateGlobalIx) -> Result<()> {
        UpdateGlobal::handler(&mut ctx, ix)
    }

    pub fn init_presale(mut ctx: Context<InitPresale>) -> Result<()> {
        InitPresale::handler(&mut ctx)
    }

    pub fn buy_presale(mut ctx: Context<BuyPresale>, amount: u64) -> Result<()> {
        BuyPresale::handler(&mut ctx, amount)
    }
}
