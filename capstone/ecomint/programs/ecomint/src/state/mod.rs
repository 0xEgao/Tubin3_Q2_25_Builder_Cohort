use anchor_lang::prelude::*;
pub mod helper;
pub use helper::*;
#[account]
pub struct Ecomint {
    pub country: Country,
    pub organisation_type: SourceType,
    pub carbon_offset: CarbonUnits,
    pub offset_value: u16,
    pub maker: Pubkey,
    pub bump: u8,
    pub listed: bool,
    pub value: u32,
}
impl Space for Ecomint {
    const INIT_SPACE: usize = 8+ //discriminator
    1+ //Country Enum
    1+ //Organisation Type
    1+ //carbon offesetted
    32 + //pubkey of creator
    1 + //bump
    1 + //listed
    4; //value
}

#[account]
pub struct Marketplace {
    pub admin: Pubkey,
    pub fee: u16,
    pub bump: u8,
    pub treasury_bump: u8,
    pub name: String,
}

//i know i can derive space using macro but just doing it for fun ,lol.
impl Space for Marketplace {
    const INIT_SPACE: usize = 8+ //discriminator
    32+ //Pubkey
    2 + //fee
    1 + //bump
    1 + //treasury_bump
    (4+32); //String + limit of 32bytes only
}
