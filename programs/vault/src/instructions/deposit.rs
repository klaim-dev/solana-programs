use crate::error::VaultError;
use crate::Vault;
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [b"vault"],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.authority.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };

        let cpi_context = CpiContext::new(ctx.accounts.system_program.key(), cpi_accounts);
        require!(amount > 0, VaultError::InvalidDepositAmount);

        transfer(cpi_context, amount)?;

        ctx.accounts.vault.amount += amount;
        Ok(())
    }
}
