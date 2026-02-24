use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta, seeds::Seed, state::ExtraAccountMetaList,
};

use crate::vault;

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    /// CHECK: ExtraAccountMetaList Account, must use these seeds
    #[account(
        init,
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump,
        space = ExtraAccountMetaList::size_of(
            InitializeExtraAccountMetaList::extra_account_metas()?.len()
        ).unwrap(),
        payer = payer
    )]
    pub extra_account_meta_list: AccountInfo<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeExtraAccountMetaList<'info> {
    pub fn extra_account_metas() -> Result<Vec<ExtraAccountMeta>> {
        Ok(vec![
            // index 5: vault program (static)
            // Fixed accounts: 0=source, 1=mint, 2=destination, 3=owner, 4=extra_account_meta_list
            // Extra accounts start at index 5
            ExtraAccountMeta::new_with_pubkey(&vault::ID, false, false).unwrap(),
            // index 6: whitelist PDA — derived externally using vault program
            ExtraAccountMeta::new_external_pda_with_seeds(
                5, // index of the program to use = vault program at index 5
                &[
                    Seed::Literal {
                        bytes: b"whitelist".to_vec(),
                    },
                    Seed::AccountKey { index: 3 }, // owner at index 3
                ],
                false,
                true,
            )
            .unwrap(),
        ])
    }
}
