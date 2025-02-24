use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, InitSpace)]
pub struct RampTx {
    pub is_onramp: bool,
    pub user: Pubkey,
    pub amount: u64,
}

pub type ExecutionOutput = [u8; 32];

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct CommitedValues {
    pub input: ExecutionInput,
    pub output: ExecutionOutput,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct ExecutionInput {
    pub rollup_accounts: RollupState, // use Vec<State> instead
    pub txs: Vec<u8>,                 // Vec of serialized transactions: Vec<Transaction>
    pub ramp_txs: Vec<RampTx>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct RollupState {
    pub states: Vec<State>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug)]
pub struct State {
    pub pubkey: Pubkey,
    pub account: SerializableAccount,
}

/// Borsh Serializable Solana Account
#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct SerializableAccount {
    /// lamports in the account
    pub lamports: u64,
    /// data held in this account
    pub data: Vec<u8>,
    /// the program that owns this account. If executable, the program that loads this account.
    pub owner: Pubkey,
    /// this account's data contains a loaded program (and is now read-only)
    pub executable: bool,
    /// the epoch at which this account will next owe rent
    pub rent_epoch: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct SP1Groth16Proof {
    pub proof: Vec<u8>,
    pub sp1_public_inputs: CommitedValues,
}
