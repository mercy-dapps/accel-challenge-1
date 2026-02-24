use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Whitelist {
    pub owner: Pubkey,
    pub amount_deposited: u64,
    pub bump: u8
}