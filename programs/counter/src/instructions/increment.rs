use anchor_lang::prelude::*;

use crate::state::Counter;

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    counter: Account<'info, Counter>,
}

impl<'info> Increment<'info> {
    pub fn increment_counter(ctx: Context<Increment>) -> Result<()> {
        ctx.accounts.counter.value += 1;
        Ok(())
    }
}
