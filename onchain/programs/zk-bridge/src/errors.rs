use anchor_lang::prelude::*;

#[error_code]
pub enum PlatformError {
    #[msg("Insufficient deposits")]
    InsufficientDeposits,
    #[msg("Invalid state hash")]
    InvalidStateHash,
}
