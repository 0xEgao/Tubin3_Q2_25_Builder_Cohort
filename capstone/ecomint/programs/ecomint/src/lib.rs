pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("BPBKb4ogsKGbF4RGZTj3urRWemC584WT4gESJacNBzF4");

#[program]
pub mod ecomint {
    use super::*;

    pub fn initialize_ecomint(
        ctx: Context<InitializeEcomint>,
        country: Country,
        organisation_type: SourceType,
        carbon_offset: u16,
        value: u32,
    ) -> Result<()> {
        ctx.accounts.init_ecomint(
            country,
            organisation_type,
            CarbonUnits::TCO2e,
            carbon_offset,
            &ctx.bumps,
            value,
        )?;
        Ok(())
    }
    pub fn initialize_marketplace(
        ctx: Context<InitializeMarketplace>,
        name: String,
        fee: u16,
    ) -> Result<()> {
        ctx.accounts.init_marketplace(name, &ctx.bumps, fee)
    }

    pub fn list(ctx: Context<List>) -> Result<()> {
        ctx.accounts.list()
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.delist()?;
        Ok(())
    }
    pub fn mint_nft(ctx: Context<MintNft>, args: Card) -> Result<()> {
        ctx.accounts.mint_nft(args)?;
        Ok(())
    }

    pub fn send_usdc(ctx: Context<Send>, amount: u16) -> Result<()> {
        ctx.accounts.send_usdc(amount)?;
        ctx.accounts.send_fee(amount)?;
        Ok(())
    }

    pub fn update_marketplace_fee(ctx: Context<UpdateMarketplaceFee>, fee: u16) -> Result<()> {
        ctx.accounts.update_marketplace_fee(fee)?;
        Ok(())
    }
}
