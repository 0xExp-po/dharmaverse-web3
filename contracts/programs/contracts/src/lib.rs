use anchor_lang::prelude::*;

declare_id!("7GgbXgyxz1BXDEHFmRt91c8Pr3pTpZm21fKjQPWw6zAT");

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
