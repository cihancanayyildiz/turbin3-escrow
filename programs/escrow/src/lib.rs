use anchor_lang::prelude::*;

mod contexts;
use contexts::*;

mod state;
use state::*;

declare_id!("FZ5jYzGZVqt1D52AJokQAg29tGPvJmHK8hQXf5bmykcZ");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, amount: u64, recieve: u64) -> Result<()> {
        ctx.accounts.save_escrow(seed, recieve, ctx.bumps.escrow)?;
        ctx.accounts.deposit_to_vault(amount)?;
        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.transfer_to_maker()?;
        ctx.accounts.withdraw_and_close()?;
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.withdraw_and_close()?;
        Ok(())
    }
}


