use anchor_lang::prelude::*;

#[error_code]
pub enum VaultWhiteListError {
    #[msg("Not whitelisted")]
    NotWhitelisted,

    #[msg("Already whitelisted")]
    AlreadyWhitelisted,

    #[msg("Insufficient balance")]
    InsufficientBalance,

}