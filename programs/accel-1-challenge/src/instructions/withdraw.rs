use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use spl_token_2022::instruction as token_instruction;
use spl_transfer_hook_interface::solana_cpi::invoke_signed;

use crate::{error::*, state::*};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = vault_config,
        associated_token::token_program = token_program
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
        mut,
        seeds = [b"whitelist", user.key().as_ref()],
        bump = whitelist.bump
    )]
    pub whitelist: Account<'info, Whitelist>,

    /// CHECK: Vault's whitelist for transfer hook
    #[account(
        mut,
        seeds = [b"whitelist", vault_config.key().as_ref()],
        bump
    )]
    pub vault_whitelist: UncheckedAccount<'info>,

    /// CHECK: ExtraAccountMetaList Account,
    #[account(
        seeds = [b"extra-account-metas", mint.key().as_ref()], 
        bump,
        seeds::program = hook_program_id.key()
    )]
    pub extra_account_meta_list: UncheckedAccount<'info>,

    /// CHECK: Hook Program ID Account,
    pub hook_program_id: UncheckedAccount<'info>,

    /// CHECK: Vault Program ID Account
    #[account(address = crate::ID)]
    pub vault_program: UncheckedAccount<'info>,

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

        self.vault_config.total_amount_deposited = self
            .vault_config
            .total_amount_deposited
            .checked_sub(amount)
            .ok_or(VaultWhiteListError::Overflow)?;

        self.whitelist.amount_deposited = self
            .whitelist
            .amount_deposited
            .checked_sub(amount)
            .ok_or(VaultWhiteListError::Overflow)?;

        let mut transfer_ix = token_instruction::transfer_checked(
            &self.token_program.key(),
            &self.vault.key(),
            &self.mint.key(),
            &self.user_token_account.key(),
            &self.vault_config.key(),
            &[], // No multisig signers are needed.
            amount,
            self.mint.decimals,
        )?;

        transfer_ix
            .accounts
            .push(AccountMeta::new_readonly(self.hook_program_id.key(), false));
        transfer_ix.accounts.push(AccountMeta::new_readonly(
            self.extra_account_meta_list.key(),
            false,
        ));
        transfer_ix
            .accounts
            .push(AccountMeta::new_readonly(self.vault_program.key(), false));
        transfer_ix
            .accounts
            .push(AccountMeta::new(self.vault_whitelist.key(), false));

        let account_infos = &[
            self.user_token_account.to_account_info(),
            self.mint.to_account_info(),
            self.vault.to_account_info(),
            self.vault_config.to_account_info(),
            self.user.to_account_info(),
            self.token_program.to_account_info(), // The Token Program must be in this list for `invoke`
            self.hook_program_id.to_account_info(),
            self.extra_account_meta_list.to_account_info(),
            self.vault_program.to_account_info(),
            self.vault_whitelist.to_account_info(),
        ];

        let signer_seeds: &[&[&[u8]]] = &[&[b"vault", &[self.vault_config.bump]]];

        invoke_signed(&transfer_ix, account_infos, signer_seeds)?;

        Ok(())
    }
}
