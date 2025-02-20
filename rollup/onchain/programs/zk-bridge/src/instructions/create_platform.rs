use anchor_lang::prelude::*;
// use anchor_spl::associated_token::AssociatedToken;
// use anchor_spl::token::*;

use crate::state::{platform::Platform, PLATFORM_SEED_PREFIX};
// use crate::state::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreatePlatformArgs {
    pub id: Pubkey,
    pub initial_state_hash: [u8; 32],
}

#[derive(Accounts)]
#[instruction(args: CreatePlatformArgs)]
pub struct CreatePlatform<'info> {
    #[account(mut)]
    pub sequencer: Signer<'info>,
    #[account(
        init,
        payer = sequencer,
        space = 8 + Platform::INIT_SPACE,
        seeds = [
            PLATFORM_SEED_PREFIX,
            args.id.as_ref(),
        ],
        bump
    )]
    pub platform: Account<'info, Platform>,
    pub system_program: Program<'info, System>,
}

impl CreatePlatform<'_> {
    pub fn handle(ctx: Context<Self>, args: CreatePlatformArgs) -> Result<()> {
        ctx.accounts.platform.set_inner(Platform {
            bump: ctx.bumps.platform,
            id: args.id,
            sequencer: ctx.accounts.sequencer.key(),
            last_state_hash: args.initial_state_hash,
            ramp_txs: vec![],
            deposit: 0,
            withdraw: 0,
        });

        Ok(())
    }
}
