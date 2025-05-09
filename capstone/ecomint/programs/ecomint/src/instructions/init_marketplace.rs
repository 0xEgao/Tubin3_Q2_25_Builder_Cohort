use anchor_lang::prelude::*;

use crate::Marketplace;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializeMarketplace<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        space=Marketplace::INIT_SPACE,
        payer=admin,
        seeds=[b"marketplace",name.as_str().as_bytes()],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,
    #[account(
        seeds=[b"treasury",marketplace.key().as_ref()],
        bump,
    )]
    pub treasury: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
impl<'info> InitializeMarketplace<'info> {
    pub fn init_marketplace(
        &mut self,
        name: String,
        bumps: &InitializeMarketplaceBumps,
        fee: u16,
    ) -> Result<()> {
        self.marketplace.set_inner(Marketplace {
            admin: self.admin.key(),
            fee,
            bump: bumps.marketplace,
            treasury_bump: bumps.treasury,
            name,
        });
        Ok(())
    }
}
