use crate::Vault;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [b"vault"],
        bump,
        payer = authority,
        space = Vault::DISCRIMINATOR.len() + Vault::INIT_SPACE,

    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize_vault(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.vault.amount = 0;
        ctx.accounts.vault.authority = ctx.accounts.authority.key();
        ctx.accounts.vault.bump = ctx.bumps.vault;
        Ok(())
    }
}
