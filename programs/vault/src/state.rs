use anchor_lang::prelude::*;
use anchor_lang::{account, InitSpace};

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub authority: Pubkey,
    pub amount: u64,
    pub bump: u8,
    pub rent: u64,
}
