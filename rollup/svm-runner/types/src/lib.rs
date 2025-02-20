use serde::{Deserialize, Serialize};
use solana_sdk::{
    account::Account,
    hash::{hashv, Hash},
    pubkey::Pubkey,
    transaction::Transaction,
};
// use solana_account::Account;
// use solana_hash::Hash;
// use solana_sha256_hasher::hashv;
// use solana_pubkey::Pubkey;
// use solana_transaction::Transaction;

#[derive(Deserialize, Serialize)]
pub struct RampTx {
    pub is_onramp: bool,
    pub user: Pubkey,
    pub amount: u64,
}

#[derive(Deserialize, Serialize)]
pub struct ExecutionInput {
    pub accounts: RollupState,
    pub txs: Vec<Transaction>,
    pub ramp_txs: Vec<RampTx>,
}

pub type ExecutionOutput = Hash;

#[derive(Deserialize, Serialize)]
pub struct RollupState(pub Vec<(Pubkey, Account)>);

#[derive(Deserialize, Serialize)]
pub struct CommittedValues(pub ExecutionInput, pub ExecutionOutput);

// Temporary function used before adding the merklized state
pub fn hash_state(output: RollupState) -> Hash {
    let mut data = Vec::new();
    for (pk, account) in output.0.iter() {
        data.extend_from_slice(pk.as_ref());
        data.extend_from_slice(&bincode::serialize(account).unwrap());
    }
    hashv(&[data.as_slice()])
}
