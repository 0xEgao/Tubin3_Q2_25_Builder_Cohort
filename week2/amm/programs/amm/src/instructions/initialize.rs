use anchor_lang::prelude::*;

use crate::state::Config;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{InitializeMintBumps, Mint, Token, TokenAccount},
};
#[derive(Accounts)]
#[instruction()]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

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
        associated_token::mint=mint_x,
        associated_token::authority=config,
    )]
    pub vault_x: Account<'info, TokenAccount>,

    #[account(
        init,
        payer=admin,
        associated_token::mint=mint_y,
        associated_token::authority=config,
    )]
    pub vault_y: Account<'info, TokenAccount>,

    #[account(
        init,
        payer=admin,
        seeds=[b"config",seed.to_le_bytes().as_ref()];
        bump,
        space=Config::INIT_SPACE
    )]
    pub config: Account<'info,Config>,

    pub token_program:Program<'info,Token>,
    pub associated_token_program:Program<'info,AssociatedToken>,
    pub system_program:Program<'info,System>,

}
impl <'info> Initialize<'info>{
    pub fn Initialize(&mut self,seed:u64,fee:u16,authority:Option<Pubkey>,bump:&InitializeBumps)-> Result<()>{
        self.config.set_inner(Config {
            seed,
            authority,
            mint_x: self.mint_x.key(),
            mint_y: self.mint_y.key(),
            fee,
            locked:true,
            config_bump:bump,
            lp_bump:self.
        })
    }
}
