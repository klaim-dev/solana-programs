use crate::Vault;
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

#[derive(Accounts)]
pub struct TryTransfer<'info> {
    #[account(
        mut,
        seeds = [b"vault"],
        bump,
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> TryTransfer<'info> {
    pub fn try_transfer(ctx: Context<TryTransfer>, amount: u64) -> Result<()> {
        let vault = ctx.accounts.vault.to_account_info();
        let recipient = ctx.accounts.recipient.to_account_info();
        let system_program = ctx.accounts.system_program.key;

        let bump = ctx.bumps.vault;
        let cpi_accounts = Transfer {
            from: vault,
            to: recipient,
        };

        let signer_seeds: &[&[&[u8]]] = &[&[b"vault", &[bump]]];

        let cpi_ctx = CpiContext::new_with_signer(*system_program, cpi_accounts, signer_seeds);
        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}
