use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Zero value error")]
    ValueZero,
    #[msg("Carbon offset can't be zero")]
    OffsetZero,
    #[msg("This is already listed")]
    AlreadyListed,
    #[msg("This is not listed,can't delist")]
    NotListed,
}
