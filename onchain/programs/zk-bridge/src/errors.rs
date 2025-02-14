use anchor_lang::prelude::*;

#[error]
pub enum PlatformError {
    #[error("Insufficient deposits")]
    InsufficientDeposits,
    #[error("Invalid state hash")]
    InvalidStateHash,
}
