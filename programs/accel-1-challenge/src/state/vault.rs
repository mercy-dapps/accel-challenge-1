use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct VaultConfig {
    pub total_amount_deposited: u64,
    pub admin: Pubkey,
    pub bump: u8
}

