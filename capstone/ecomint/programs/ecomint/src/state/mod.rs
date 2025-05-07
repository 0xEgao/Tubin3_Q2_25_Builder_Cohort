use anchor_lang::prelude::*;
pub mod helper;
pub use helper::*;
#[account]
pub struct Ecomint {
    pub country: Country,
    pub organisation_type: SourceType,
    pub carbon_offset: CarbonUnits,
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
