use anchor_lang::prelude::*;

#[error_code]
pub enum VaultWhiteListError {
    #[msg("Not whitelisted")]
    NotWhitelisted,

    #[msg("Insufficient balance")]
    InsufficientBalance,

    #[msg("Overflow")]
    Overflow,
}
