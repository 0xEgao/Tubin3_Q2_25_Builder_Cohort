use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum SourceType {
    Solar,
    Wind,
    Water,
    Reforestation,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
pub enum CarbonUnits {
    TCO2e, //this one metric ton of Co2 equivalent
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
pub enum Country {
    UnitedKingdom,
    SouthAfrica,
    China,
    HongKong,
    India,
    Indonesia,
    Japan,
    Korea,
    Thailand,
    Australia,
    NewZealand,
    SaudiArabia,
    Turkey,
    UnitedArabEmirates,
    Canada,
    Mexico,
    UnitedStates,
    Argentina,
    Brazil,
    Austria,
    Belgium,
    Bulgaria,
    Croatia,
    Cyprus,
    CzechRepublic,
    Denmark,
    Estonia,
    Finland,
    France,
    Germany,
    Greece,
    Hungary,
    Iceland,
    Ireland,
    Italy,
    Latvia,
    Lithuania,
    Luxembourg,
    Malta,
    Netherlands,
    Norway,
    Poland,
    Portugal,
    Romania,
    RussianFederation,
    Serbia,
    Slovakia,
    Slovenia,
    Spain,
    Sweden,
    Switzerland,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct Card {
    pub name: String,
    pub uri: String,
}
