use crate::error::ErrorCode;
use crate::Ecomint;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Delist<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        mut,
        seeds=[b"carbon_credit",maker.key().as_ref()],
        bump=eco_mint.bump
    )]
    pub eco_mint: Account<'info, Ecomint>,
    pub system_program: Program<'info, System>,
}
impl<'info> Delist<'info> {
    pub fn delist(&mut self) -> Result<()> {
        require!(self.eco_mint.listed, ErrorCode::NotListed);
        self.eco_mint.listed = false;
        Ok(())
    }
}
