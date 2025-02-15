use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RampTx {
    pub is_onramp: bool,
    pub user: Pubkey,
    pub amount: u64,
}
