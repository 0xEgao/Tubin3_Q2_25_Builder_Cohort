use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub seed: u64,
    pub authority: Option<Pubkey>,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub fees: u16,
    pub locked: bool,
    pub config_bump: u8,
    pub lp_bump: u8,
}
impl Space for Config {
    const INIT_SPACE: usize = 120;
}
