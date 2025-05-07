use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum SourceType {
    Solar,
    Wind,
    Water,
    Reforestation,
}
