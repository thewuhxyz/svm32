use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use crate::instructions::*;
// use crate::state::*;

declare_id!("6aTn9Wn2ZaU8ToRYBnnSCSnsbayeQtU71KzEAQhWHMWK");

#[program]
pub mod zk_bridge {
    use super::*;

    pub fn create_platform(ctx: Context<CreatePlatform>, args: CreatePlatformArgs) -> Result<()> {
        CreatePlatform::handle(ctx, args)
    }

    #[access_control(ctx.accounts.validate())]
    pub fn withdraw(ctx: Context<Withdraw>, args: WithdrawArgs) -> Result<()> {
        Withdraw::handle(ctx, args)
    }

    /// Add a ramp transaction to the platform.
    ///
    /// **This can currently be used to DoS the platform by adding transactions faster than the sequencer can generate proofs.**
    pub fn add_ramp_tx(ctx: Context<AddRampTx>, args: AddRampTxArgs) -> Result<()> {
        AddRampTx::handle(ctx, args)
    }

    pub fn prove(ctx: Context<Prove>) -> Result<()> {
        Prove::handle(ctx)
    }
}
