
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
pub use instructions::*;

declare_id!("2WrkWYs4m32JzPnV557x9sPTCo5S8mYibAJ8ehpmBDsi");

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
