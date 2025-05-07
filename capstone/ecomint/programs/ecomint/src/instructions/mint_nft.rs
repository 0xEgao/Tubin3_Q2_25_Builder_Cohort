use crate::error::ErrorCode;
use crate::Card;
use crate::Ecomint;
use crate::Marketplace;

use anchor_lang::prelude::*;
use mpl_core::{instructions::CreateV2CpiBuilder, ID as MPL_CORE_ID};

#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(mut)]
    pub taker: Signer<'info>, //Buyer account

    #[account(
        mut,
        address=eco_mint.maker
    )]
    pub maker: Signer<'info>, //organisation account

    #[account(
        mut,
        has_one=maker,
        seeds=[b"ecomint",eco_mint.maker.key().as_ref()],
        bump=eco_mint.bump
    )]
    pub eco_mint: Account<'info, Ecomint>,

    #[account(
        seeds=[b"marketplace",marketplace.name.as_str().as_bytes()],
        bump=marketplace.bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(mut)]
    pub asset: Signer<'info>,

    #[account(
        address=MPL_CORE_ID
    )]
    pub mpl_core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
impl<'info> MintNft<'info> {
    pub fn mint_nft(&mut self, args: Card) -> Result<()> {
        require_eq!(self.eco_mint.listed, true, ErrorCode::NotListed);
        CreateV2CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .authority(Some(&self.maker.to_account_info()))
            .payer(&self.taker.to_account_info())
            .owner(Some(&self.taker.to_account_info()))
            .system_program(&self.system_program.to_account_info())
            .name(args.name)
            .uri(args.uri);

        Ok(())
    }
}
