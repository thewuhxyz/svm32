use anchor_lang::prelude::*;
// use anchor_spl::associated_token::AssociatedToken;
// use anchor_spl::token::*;

use crate::errors::PlatformError;
use crate::state::platform::Platform;
use crate::state::ramp::Ramp;
use crate::state::{RampTx, PLATFORM_SEED_PREFIX, RAMP_SEED_PREFIX};
// use crate::state::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AddRampTxArgs {
    pub is_onramp: bool,
    pub amount: u64,
}

#[event_cpi]
#[derive(Accounts)]
#[instruction(args: AddRampTxArgs)]
pub struct AddRampTx<'info> {
    #[account(mut)]
    pub ramper: Signer<'info>,
    #[account(
        mut,
        realloc = 8 + std::mem::size_of_val(&platform) + std::mem::size_of::<RampTx>(), // allocate space for new ramp tx
        realloc::payer = ramper,
        realloc::zero = false,
        seeds = [
            PLATFORM_SEED_PREFIX,
            platform.id.as_ref(),
        ],
        bump
    )]
    pub platform: Account<'info, Platform>,
    #[account(
        init_if_needed,
        payer = ramper,
        space = 8 + std::mem::size_of::<Ramp>(),
        seeds = [
            RAMP_SEED_PREFIX,
            platform.id.as_ref(),
            ramper.key().as_ref(),
        ],
        bump
    )]
    pub ramp: Account<'info, Ramp>,
    pub system_program: Program<'info, System>,
}

impl AddRampTx<'_> {
    pub fn handle(ctx: Context<Self>, args: AddRampTxArgs) -> Result<()> {
        if ctx.accounts.ramp.ramper.eq(&Pubkey::default()) {
            ctx.accounts.ramp.set_inner(Ramp {
                bump: ctx.bumps.ramp,
                ramper: ctx.accounts.ramper.key(),
                current_state_hash: ctx.accounts.platform.last_state_hash,
                pending_withdraw: 0,
            });
        }

        if args.is_onramp {
            ctx.accounts.platform.deposit += args.amount;
            ctx.accounts.ramper.sub_lamports(args.amount)?;
            ctx.accounts.platform.add_lamports(args.amount)?;
        } else {
            ctx.accounts.platform.withdraw += args.amount;
            if ctx.accounts.platform.withdraw > ctx.accounts.platform.deposit {
                return Err(PlatformError::InsufficientDeposits.into());
            }

            ctx.accounts.ramp.current_state_hash = ctx.accounts.platform.last_state_hash;
            ctx.accounts.ramp.pending_withdraw += args.amount;
        }

        ctx.accounts.platform.ramp_txs.push(RampTx {
            is_onramp: args.is_onramp,
            amount: args.amount,
            user: ctx.accounts.ramper.key(),
        });

        Ok(())
    }
}
