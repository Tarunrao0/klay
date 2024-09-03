use anchor_lang::prelude::*;

pub mod states;
pub mod instructions;
pub mod errors;

pub use states::*;
pub use instructions::*;

declare_id!("2fc69n6Xt2xqi3JAa5WXoP9uzoyQhK7LsBjsToE7ZyMG");

#[program]
pub mod klay {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
