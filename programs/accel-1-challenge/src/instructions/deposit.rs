use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::spl_token_2022::onchain,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{error::*, state::*};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer
    )]
    pub signer_token_account: InterfaceAccount<'info, TokenAccount>,

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
        seeds = [b"whitelist", signer.key().as_ref()],
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

    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        require!(
            self.whitelist.owner == self.signer.key(),
            VaultWhiteListError::NotWhitelisted
        );

        self.whitelist.amount_deposited += amount;

        let additional_accounts = &[
            self.extra_account_meta_list.to_account_info(),
            self.whitelist.to_account_info(),
            self.vault_config.to_account_info(),
        ]; // &[AccountInfo<'a>]

        let seeds = &[]; // &[&[&[u8]]]

        onchain::invoke_transfer_checked(
            &self.token_program.key(),                   // token program
            self.signer_token_account.to_account_info(), // source token account
            self.mint.to_account_info(),                 // mint account
            self.vault.to_account_info(),                // destination token account
            self.signer.to_account_info(),               // owner of the source token account
            additional_accounts,                         // extra accounts for transfer hooks
            amount,                                      // amount to transfer
            9,                                           // decimals
            seeds,                                       // signer seeds
        )
        .map_err(|e| {
            msg!("Transfer tokens failed: {:?}", e);
            e
        })?;

        Ok(())
    }
}
