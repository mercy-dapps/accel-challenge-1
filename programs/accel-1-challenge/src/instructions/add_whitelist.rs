use anchor_lang::prelude::*;

use crate::{error::*, state::Whitelist};

#[derive(Accounts)]
#[instruction(address: Pubkey)]
pub struct AddWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + Whitelist::INIT_SPACE,
        seeds = [b"whitelist", address.as_ref()],
        bump
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>,
}

impl<'info> AddWhitelist<'info> {
    pub fn add_whitelist(&mut self, address: Pubkey, bumps: AddWhitelistBumps) -> Result<()> {
        require!(
            self.whitelist.owner == address,
            VaultWhiteListError::AlreadyWhitelisted
        );

        self.whitelist.set_inner(Whitelist {
            owner: address,
            amount_deposited: 0,
            bump: bumps.whitelist,
        });

        Ok(())
    }
}
