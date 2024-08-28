use anchor_lang::prelude::*;

declare_id!("6k3zHEgbU4MwBjUKbLaDEwEo2bTNSWrn5wuWPYfyUjqH");

mod state;
#[program]
pub mod contracts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
