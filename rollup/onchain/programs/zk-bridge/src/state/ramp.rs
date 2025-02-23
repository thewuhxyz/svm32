use anchor_lang::prelude::*;
// use svm_runner_lib::RampTx;

pub const RAMP_SEED_PREFIX: &[u8] = b"ramp:";

/// A platform is the account storing state waiting to be sent to the rollup
#[account]
#[derive(Default)]
pub struct Ramp {
    pub bump: u8,
    pub ramper: Pubkey,
    pub current_state_hash: [u8; 32],
    pub pending_withdraw: u64,
}
