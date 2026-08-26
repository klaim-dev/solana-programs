use anchor_lang::prelude::*;
use crate::state::Counter;
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = Counter::DISCRIMINATOR.len() + Counter::INIT_SPACE
    )]
    pub counter: Account<'info, Counter>,
    
    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,

}

impl<'info> Initialize<'info> {

pub fn initialize_counter(ctx: Context<Initialize>) -> Result<()> {
    ctx.accounts.counter.value = 0;
    Ok(())
}
}
