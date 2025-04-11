use anchor_lang::{prelude::*, solana_program::vote::instruction};

pub mod instructions;
pub mod state;

pub use instruction::*;

declare_id!("7iLNmdYsiEhKfeATt7mFA3MqB1ZRod6mX8fLj62mTz58");

#[program]
pub mod escrow {
    use instructions::Make;

    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, deposit_amount: u64, recieve: u64) -> Result<()> {
        ctx.accounts.init_escrow_state(seed, recieve, ctx.bumps)?;
        ctx.accounts.deposit(deposit_amount)?;
        OK(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
