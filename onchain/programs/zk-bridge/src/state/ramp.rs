use anchor_lang::{prelude::*, solana_program::hash::Hash};
use svm_runner_lib::RampTx;

pub const PLATFORM_SEED_PREFIX: &[u8] = b"platform:";

/// A platform is the account storing state waiting to be sent to the rollup
#[account]
#[derive(Default)]
pub struct Ramp {
    pub bump: u8,
    pub ramper: Pubkey,
    pub current_state_hash: Hash,
    pub pending_withdraw: u64,
}

#[macro_export]
macro_rules! generate_network_seeds {
    ($network:expr) => {{
        &[
            PLATFORM_SEED_PREFIX,
            $network.base_mint.as_ref(),
            &[$amm.bump],
        ]
    }};
}
