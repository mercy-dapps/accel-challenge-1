use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::state::VaultConfig;

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = 8 + VaultConfig::INIT_SPACE,

        seeds = [b"vault"],
        bump
    )]
    pub vault_config: Account<'info, VaultConfig>,

    #[account(
        init,
        payer = admin,

        associated_token::mint = mint,
        associated_token::authority = vault_config
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub mint: InterfaceAccount<'info, Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>
}

impl<'info> InitializeVault<'info>  {
    pub fn initialize_vault(
        &mut self,
        bumps: InitializeVaultBumps
    ) -> Result<()> {

        self.vault_config.set_inner(VaultConfig { 
            total_amount_deposited: 0,
            admin: self.admin.key(),
            bump: bumps.vault_config 
        });

        msg!("Vault initialized");

        Ok(())
    }
}