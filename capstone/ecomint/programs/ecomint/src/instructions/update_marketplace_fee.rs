use crate::Ecomint;
use crate::Marketplace;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateMarketplaceFee<'info> {
    #[account(mut,address=eco_mint.maker)]
    pub maker: SystemAccount<'info>,
    #[account(mut,address=marketplace.admin)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        has_one = maker,
        seeds = [b"ecomint", eco_mint.maker.key().as_ref()],
        bump
    )]
    pub eco_mint: Account<'info, Ecomint>,
    #[account(
        mut,
        seeds=[b"marketplace",marketplace.name.as_str().as_bytes()],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>, //fee structure
}
impl<'info> UpdateMarketplaceFee<'info> {
    pub fn update_marketplace_fee(&mut self, fee: u16) -> Result<()> {
        self.marketplace.fee = fee;
        Ok(())
    }
}
