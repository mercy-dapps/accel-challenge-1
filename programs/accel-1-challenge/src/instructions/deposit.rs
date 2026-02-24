use anchor_lang::{prelude::*, solana_program::program::invoke};
use anchor_spl::{
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use spl_token_2022::instruction as token_instruction;

use crate::{error::*, state::*};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub signer_token_account: InterfaceAccount<'info, TokenAccount>,

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
        seeds = [b"whitelist", signer.key().as_ref()],
        bump = whitelist.bump
    )]
    pub whitelist: Account<'info, Whitelist>,

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
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        require!(
            self.whitelist.owner == self.signer.key(),
            VaultWhiteListError::NotWhitelisted
        );
        self.whitelist.amount_deposited += amount;

        //   1. Manually build the `transfer_checked` instruction provided by the SPL Token program.
        let mut transfer_ix = token_instruction::transfer_checked(
            &self.token_program.key(),
            &self.signer_token_account.key(),
            &self.mint.key(),
            &self.vault.key(),
            &self.signer.key(),
            &[], // No multisig signers are needed.
            amount,
            self.mint.decimals,
        )?;

        // // 2. Manually add the extra accounts required by the transfer hook.
        // // The Token 2022 program expects these to follow the core transfer accounts.
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
            .push(AccountMeta::new(self.whitelist.key(), false));

        // // 3. Create a flat list of all AccountInfos that the instruction needs.
        // // This includes all accounts for the core transfer and the hook.
        let account_infos = &[
            self.signer_token_account.to_account_info(),
            self.mint.to_account_info(),
            self.vault.to_account_info(),
            self.signer.to_account_info(),
            self.token_program.to_account_info(), // The Token Program must be in this list for `invoke`
            self.hook_program_id.to_account_info(),
            self.extra_account_meta_list.to_account_info(),
            self.vault_program.to_account_info(),
            self.whitelist.to_account_info(),
        ];

        // // 4. Use the low-level `invoke` function to execute the CPI.
        invoke(&transfer_ix, account_infos)?;

        Ok(())
    }
}
