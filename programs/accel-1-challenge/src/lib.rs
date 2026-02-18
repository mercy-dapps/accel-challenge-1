use anchor_lang::prelude::*;

declare_id!("4pXdfgsYDEvvCHxaCHDfmJWq3yCyrgQJxxaRtuV3RQ2D");

pub mod instructions;
pub use instructions::*;

pub mod state;
pub mod error;

#[program]
pub mod accel_1_challenge {
    use super::*;

}