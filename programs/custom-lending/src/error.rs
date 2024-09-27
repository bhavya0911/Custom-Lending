use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Name must be between 1 and 32 characters")]
    NameTooLong,
    #[msg("Listing is not active")]
    NotActive,
    #[msg("Listing is already taken")]
    AlreadyTaken,
    #[msg("Loan has not expired")]
    CannotReclaim,
    #[msg("Loan has expired")]
    CannotRepay,
    #[msg("Listing has expired")]
    Expired,
    #[msg("Unauthorised to repay")]
    Unauthorised,
}