use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_account::{Account, AccountSharedData};
use solana_program::{clock::Epoch, hash::Hash};
use solana_pubkey::Pubkey;
use solana_sha256_hasher::hashv;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone)]
pub struct RampTx {
    pub is_onramp: bool,
    pub user: Pubkey,
    pub amount: u64,
}

pub type ExecutionOutput = Hash;

pub fn hash_state(output: RollupState) -> Hash {
    let mut data = Vec::new();
    for state in output.states.iter() {
        data.extend_from_slice(state.pubkey.as_ref());
        data.extend_from_slice(&bincode::serialize(&state.account).unwrap());
    }
    hashv(&[data.as_slice()])
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone)]
pub struct CommitedValues {
    pub input: ExecutionInput,
    pub output: ExecutionOutput,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone)]
pub struct ExecutionInput {
    pub accounts: RollupState, // use Vec<State> instead
    pub txs: Vec<u8>,          // Vec of serialized transactions: Vec<Transaction>
    pub ramp_txs: Vec<RampTx>,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone)]
pub struct RollupState {
    pub states: Vec<State>,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone)]
pub struct State {
    pub pubkey: Pubkey,
    pub account: SerializableAccount,
}

/// Serializable Solana [Account]
#[derive(
    BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default,
)]
pub struct SerializableAccount {
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

impl From<SerializableAccount> for Account {
    fn from(account: SerializableAccount) -> Account {
        Account {
            data: account.data,
            executable: account.executable,
            lamports: account.lamports,
            owner: account.owner,
            rent_epoch: account.rent_epoch,
        }
    }
}

impl From<Account> for SerializableAccount {
    fn from(account: Account) -> SerializableAccount {
        SerializableAccount {
            data: account.data,
            executable: account.executable,
            lamports: account.lamports,
            owner: account.owner,
            rent_epoch: account.rent_epoch,
        }
    }
}

impl From<SerializableAccount> for AccountSharedData {
    fn from(account: SerializableAccount) -> AccountSharedData {
        Account::from(account).into()
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct SP1Groth16Proof {
    pub proof: Vec<u8>,
    pub sp1_public_inputs: CommitedValues,
}
