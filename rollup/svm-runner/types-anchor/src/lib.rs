use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
// use solana_sdk::{
//     account::Account,
//     hash::{hashv, Hash},
//     pubkey::Pubkey,
//     transaction::Transaction,
// };
use solana_account::{Account, AccountSharedData};
use solana_program::{clock::Epoch, hash::Hash};
// use solana_hash::Hash;
// use solana_program::clock::Epoch;
use solana_pubkey::Pubkey;
use solana_sha256_hasher::hashv;
use solana_transaction::Transaction;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone)]
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

// // Temporary function used before adding the merklized state
// pub fn hash_state(output: RollupState) -> Hash {
//     let mut data = Vec::new();
//     for (pk, account) in output.0.iter() {
//         data.extend_from_slice(pk.as_ref());
//         data.extend_from_slice(&bincode::serialize(account).unwrap());
//     }
//     hashv(&[data.as_slice()])
// }
// Temporary function used before adding the merklized state
pub fn hash_state(output: BorshRollupState) -> Hash {
    let mut data = Vec::new();
    for (pk, account) in output.0.iter() {
        data.extend_from_slice(pk.as_ref());
        data.extend_from_slice(&bincode::serialize(account).unwrap());
    }
    hashv(&[data.as_slice()])
}

//
// WE WORK WITH THIS
//
//

/// Borsh Serializable [CommitedValues]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
pub struct BorshCommitedValues(pub BorshExecutionInput, pub ExecutionOutput);

/// Borsh Serializable [ExecutionInput]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
pub struct BorshExecutionInput {
    pub accounts: BorshRollupState,
    pub txs: Vec<u8>, // Vec of serialized transactions: Vec<Transaction>
    pub ramp_txs: Vec<RampTx>,
}

/// Borsh Serializable [RollupState]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
pub struct BorshRollupState(pub Vec<(Pubkey, BorshAccount)>);

/// Borsh Serializable Solana [Account]
#[derive(
    BorshDeserialize, BorshSerialize, PartialEq, Eq, Clone, Default, Deserialize, Serialize, Debug
)]
pub struct BorshAccount {
    /// lamports in the account
    pub lamports: u64,
    /// data held in this account
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
    /// the program that owns this account. If executable, the program that loads this account.
    pub owner: Pubkey,
    /// this account's data contains a loaded program (and is now read-only)
    pub executable: bool,
    /// the epoch at which this account will next owe rent
    pub rent_epoch: Epoch,
}

impl From<BorshAccount> for Account {
    fn from(account: BorshAccount) -> Account {
        Account {
            data: account.data,
            executable: account.executable,
            lamports: account.lamports,
            owner: account.owner,
            rent_epoch: account.rent_epoch,
        }
    }
}

impl From<Account> for BorshAccount {
    fn from(account: Account) -> BorshAccount {
        BorshAccount {
            data: account.data,
            executable: account.executable,
            lamports: account.lamports,
            owner: account.owner,
            rent_epoch: account.rent_epoch,
        }
    }
}

impl From<BorshAccount> for AccountSharedData {
    fn from(account: BorshAccount) -> AccountSharedData {
        Account::from(account).into()
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct SP1Groth16Proof {
    pub proof: Vec<u8>,
    pub sp1_public_inputs: Vec<u8>,
    // pub sp1_public_inputs: BorshCommitedValues,
}
