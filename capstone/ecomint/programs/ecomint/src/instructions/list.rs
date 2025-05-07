use crate::error::ErrorCode;
use crate::{ecomint, Ecomint};
use anchor_lang::prelude::*;
use anchor_spl::token::revoke;

#[derive(Accounts)]
pub struct List<'info> {
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
impl<'info> List<'info> {
    pub fn list(&mut self) -> Result<()> {
        require!(!self.eco_mint.listed, ErrorCode::AlreadyListed);
        self.eco_mint.listed = true;
        Ok(())
    }
}
