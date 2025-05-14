use anchor_lang::prelude::*;

use crate::error::ErrorCode;
use crate::CarbonUnits;
use crate::Country;
use crate::Ecomint;
use crate::SourceType;

#[derive(Accounts)]
pub struct InitializeEcomint<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        init,
        payer=maker,
        space=Ecomint::INIT_SPACE,
        seeds=[b"ecomint",maker.to_account_info().key().as_ref()],
        bump
    )]
    pub eco_mint: Account<'info, Ecomint>,
    pub system_program: Program<'info, System>,
}
impl<'info> InitializeEcomint<'info> {
    pub fn init_ecomint(
        &mut self,
        country: Country,
        organisation_type: SourceType,
        carbon_offset: CarbonUnits,
        offset_value: u16,
        bumps: &InitializeEcomintBumps,
        value: u32,
    ) -> Result<()> {
        require!(value != 0, ErrorCode::ValueZero);
        require!(offset_value != 0, ErrorCode::OffsetZero);

        self.eco_mint.set_inner(Ecomint {
            country,
            organisation_type,
            carbon_offset,
            offset_value,
            maker: self.maker.key(),
            bump: bumps.eco_mint,
            listed: false,
            value,
        });

        Ok(())
    }
}
