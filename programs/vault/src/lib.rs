pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("H4mu26X6TMSWsHf85QLSLh6KMP4MJ1hBtH4eAs1QED6R");

#[program]
pub mod vault {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Initialize::initialize_vault(ctx)
    }

    pub fn try_transfer(ctx: Context<TryTransfer>, amount: u64) -> Result<()> {
        TryTransfer::try_transfer(ctx, amount)
    }

    pub fn try_transfer_clean(ctx: Context<TryTransferClean>, amount: u64) -> Result<()> {
        TryTransferClean::try_transfer(ctx, amount)
    }

    pub fn check_rent(ctx: Context<CheckRent>) -> Result<()> {
        CheckRent::check_rent(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        Deposit::deposit(ctx, amount)
    }
}
