use anchor_lang::prelude::*;

declare_id!("4pXdfgsYDEvvCHxaCHDfmJWq3yCyrgQJxxaRtuV3RQ2D");

pub mod instructions;
pub use instructions::*;

pub mod error;

pub mod state;
pub use state::*;

#[program]
pub mod accel_1_challenge {
    use super::*;

    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        ctx.accounts.init_mint(amount)
    }
    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        ctx.accounts.initialize_vault(ctx.bumps)
    }

    pub fn add_whitelist(ctx: Context<AddWhitelist>, address: Pubkey) -> Result<()> {
        ctx.accounts.add_whitelist(address, ctx.bumps)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }
}
