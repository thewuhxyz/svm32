use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::*;

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
    pub sequencer: Signer<'info>,
    #[account(
        init,
        payer = sequencer,
        space = 8 + std::mem::size_of::<Platform>(),
        seeds = [
            PLATFORM_SEED_PREFIX,
            args.id.as_ref(),
        ],
        bump
    )]
    pub platform: Account<'info, Platform>,
    pub system_program: Program<'info, System>,
}

impl Prove<'_> {
    pub fn handle(ctx: Context<Self>, args: ProveArgs) -> Result<()> {
        ctx.accounts.platform.set_inner(Platform {
            bump: ctx.bumps.platform,
            id: args.id,
            sequencer: ctx.accounts.creator.key(),
            last_state_hash: Hash::default(),
            ramp_txs: vec![],
            deposit: 0,
            withdraw: 0,
        });

        Ok(())
    }
}
