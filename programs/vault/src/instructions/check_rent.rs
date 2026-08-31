use anchor_lang::prelude::*;

use crate::Vault;

#[derive(Accounts)]
pub struct CheckRent<'info> {
    #[account(
        mut,
        seeds = [b"vault"],
        bump,
    )]
    pub vault: Account<'info, Vault>,
}

impl<'info> CheckRent<'info> {
    pub fn check_rent(ctx: Context<CheckRent>) -> Result<()> {
        let rent = Rent::get()?;
        let data_space = Vault::INIT_SPACE + Vault::DISCRIMINATOR.len();
        ctx.accounts.vault.rent = rent.minimum_balance(data_space);
        Ok(())
    }
}
