use anchor_lang::prelude::*;

#[error_code]
pub enum PlatformError {
    #[msg("Insufficient deposits")]
    InsufficientDeposits,
    #[msg("Invalid state hash")]
    InvalidStateHash,
    #[msg("Invalid proof data")]
    InvalidProofData,
    #[msg("Invalid proof")]
    InvalidProof,
    #[msg("Missing ramp txs")]
    MissingRampTxs,
}
