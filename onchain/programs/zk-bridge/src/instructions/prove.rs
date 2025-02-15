use anchor_lang::prelude::*;
// use anchor_spl::associated_token::AssociatedToken;
// use anchor_spl::token::*;

use crate::state::platform::Platform;
use crate::state::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ProveArgs {
    pub id: Pubkey,
}

#[event_cpi]
#[derive(Accounts)]
#[instruction(args: ProveArgs)]
pub struct Prove<'info> {
    #[account(mut)]
    pub prover: Signer<'info>,
    #[account(
        mut,
        seeds = [
            PLATFORM_SEED_PREFIX,
            platform.id.as_ref(),
        ],
        bump
    )]
    pub platform: Account<'info, Platform>,
    pub system_program: Program<'info, System>,
}

impl Prove<'_> {
    pub fn handle(_ctx: Context<Self>, _args: ProveArgs) -> Result<()> {
        unimplemented!();

        // Deserialize proof

        // Check that ramps txs match the ones in the platform

        // Empty pending ramp txs

        // Ok(())
    }
}
