use serde::{Deserialize, Serialize};
use solana_sdk::{account::Account, pubkey::Pubkey, transaction::Transaction};

#[derive(Deserialize, Serialize)]
pub struct RampTx {
    pub is_onramp: bool,
    pub user: Pubkey,
    pub amount: u64,
}

#[derive(Deserialize, Serialize)]
pub struct ExecutionInput {
    pub accounts: Vec<(Pubkey, Account)>,
    pub txs: Vec<Transaction>,
    pub ramp_txs: Vec<RampTx>,
}

#[derive(Deserialize, Serialize)]
pub struct ExecutionOutput(pub Vec<(Pubkey, Account)>);
