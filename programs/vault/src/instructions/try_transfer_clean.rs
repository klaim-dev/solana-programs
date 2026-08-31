use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

#[derive(Accounts)]
pub struct TryTransferClean<'info> {
    #[account(
        mut,
        seeds = [b"vault clean"],
        bump,
        owner = crate::ID,
    )]
    pub vault: UncheckedAccount<'info>,
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> TryTransferClean<'info> {
    pub fn try_transfer(ctx: Context<TryTransferClean>, amount: u64) -> Result<()> {
        let vault = ctx.accounts.vault.to_account_info();
        let recipient = ctx.accounts.recipient.to_account_info();
        let system_program = ctx.accounts.system_program.key;

        let bump = ctx.bumps.vault;
        let cpi_accounts = Transfer {
            from: vault,
            to: recipient,
        };

        let signer_seeds: &[&[&[u8]]] = &[&[b"vault clean", &[bump]]];

        let cpi_ctx = CpiContext::new_with_signer(*system_program, cpi_accounts, signer_seeds);
        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}
