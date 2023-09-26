use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {
    #[msg("User Exists")]
    UserExists,
    #[msg("User profile not found")]
    UserProfileNotFound,
    #[msg("User already  in Pot")]
    UserAlreadyInPot,
    #[msg("Maximum capacity reached")]
    MaximumCapacityReached
}