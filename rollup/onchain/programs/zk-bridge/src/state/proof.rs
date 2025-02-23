use anchor_lang::prelude::*;

pub const PROOF_SEED_PREFIX: &[u8] = b"proof:";

#[account]
#[derive(Default, InitSpace)]
pub struct Proof {
    pub bump: u8,
    #[max_len(0)]
    pub data: Vec<u8>,
}