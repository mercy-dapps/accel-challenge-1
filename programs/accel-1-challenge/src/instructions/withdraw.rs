use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::spl_token_2022::onchain,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{error::*, state::*};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = vault_config
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vault"],
        bump = vault_config.bump,

        owner = crate::ID
    )]
    pub vault_config: Account<'info, VaultConfig>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        seeds = [b"whitelist", user.key().as_ref()],
        bump = whitelist.bump
    )]
    pub whitelist: Account<'info, Whitelist>,

    /// CHECK: ExtraAccountMetaList Account,
    #[account(
        seeds = [b"extra-account-metas", mint.key().as_ref()], 
        bump
    )]
    pub extra_account_meta_list: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        require!(
            self.whitelist.owner == self.user.key(),
            VaultWhiteListError::NotWhitelisted
        );
        require!(
            self.whitelist.amount_deposited >= amount,
            VaultWhiteListError::InsufficientBalance
        );

        let additional_accounts = &[
            self.extra_account_meta_list.to_account_info(),
            self.whitelist.to_account_info()
        ]; // &[AccountInfo<'a>]

        let vault_bump = self.vault_config.bump;
        let seeds: &[&[&[u8]]] = &[&[b"vault", &[vault_bump]]];

        onchain::invoke_transfer_checked(
            &self.token_program.key(),
            self.vault.to_account_info(),
            self.mint.to_account_info(),
            self.user_token_account.to_account_info(),
            self.vault_config.to_account_info(),
            additional_accounts,
            amount,
            9,
            seeds,
        )
        .map_err(|e| {
            msg!("Transfer tokens failed: {:?}", e);
            e
        })?;

        Ok(())
    }
}
