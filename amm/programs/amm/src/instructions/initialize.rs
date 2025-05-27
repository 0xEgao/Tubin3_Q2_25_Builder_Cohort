use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Mint::mint};
use anchor_spl::token::{Token, TokenAccount};

use crate::Config;
#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,

    #[account(
        init,
        payer=admin,
        seeds=[b"lp",config.key.as_ref()],
        bump,
        mint::decimals=6,
        mint::authority=config
    )]
    pub mint_lp: Account<'info, Mint>,

    #[account(
        init,
        payer=admin,
        associated_token::mint=mint_a,
        associated_token::authority=config,
    )]
    pub vault_a: Account<'info, TokenAccount>,
    #[account(
        init,
        payer=admin,
        associated_token::mint=mint_b,
        associated_token::authority=config,
    )]
    pub vault_b: Account<'info, TokenAccount>,

    #[account(
        init,
        payer=admin,
        seeds=[b"config",seed.to_le_bytes().as_ref()],
        bump,
        space=Config::INIT_SPACE,
    )]
    pub config: Account<'info, Config>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(
        &mut self,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>,
        bump: &InitializeBumps,
    ) -> Result<()> {
        self.config.set_inner(Config {
            seed,
            authority,
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            fees: fee,
            locked: false,
            config_bump: bump.config,
            lp_bump: bump.mint_lp,
        });

        Ok(())
    }
}
