use anchor_lang::{prelude::*, solana_program::hash::Hash};
use runner_types::RampTx;

pub const PLATFORM_SEED_PREFIX: &[u8] = b"platform:";

/// A platform is the account storing state waiting to be sent to the rollup
#[account]
#[derive(Default)]
pub struct Platform {
    pub bump: u8,
    pub sequencer: Pubkey,
    pub id: Pubkey,
    pub last_state_hash: Hash,
    pub ramp_txs: Vec<RampTx>,
    pub deposit: u64,
    pub withdraw: u64,
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
