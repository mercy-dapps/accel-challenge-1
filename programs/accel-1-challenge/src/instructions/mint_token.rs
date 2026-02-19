use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    self,
    Mint,
    MintTo,
    TokenAccount,
    TokenInterface,
};

use anchor_spl::associated_token::AssociatedToken;

use crate::ID;

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        mint::decimals = 9,
        mint::authority = user,
        mint::token_program = token_program,
        extensions::transfer_hook::authority = user,
        extensions::transfer_hook::program_id = ID,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = user,

        associated_token::mint = mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> MintToken<'info> {
    pub fn init_mint(&mut self, amount: u64) -> Result<()> {
        let mint_to_accounts = MintTo {
            mint: self.mint.to_account_info(),
            to: self.user_token_account.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), mint_to_accounts);
        token_interface::mint_to(cpi_ctx, amount)?;

        msg!(
            "Minted {} tokens to {}",
            amount,
            self.user_token_account.key()
        );

        Ok(())
    }
}
