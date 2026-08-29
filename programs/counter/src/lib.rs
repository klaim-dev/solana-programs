pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
pub use instructions::*;

declare_id!("BzDa1WDtCVVhy3PasxqZk4L4BSdxhc68ji9qnw5B42bA");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Initialize::initialize_counter(ctx)
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        Increment::increment_counter(ctx)
    }
}
