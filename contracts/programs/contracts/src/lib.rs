use anchor_lang::prelude::*;

declare_id!("6k3zHEgbU4MwBjUKbLaDEwEo2bTNSWrn5wuWPYfyUjqH");

mod constants;
mod state;
mod errors;
mod utils;
mod instructions;
#[program]
pub mod contracts {
    pub use super::instructions::*;
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, nfts: Vec<Pubkey>) -> Result<()> {
        instructions::initialize(ctx, nfts)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        instructions::withdraw(ctx)
    }
}

