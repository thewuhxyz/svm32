use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct RampTx {
  pub is_onramp: bool,
  pub user: Pubkey,
  pub amount: u64,
}

pub type ExecutionOutput = [u8; 32];

/// Borsh Serializable [CommitedValues]
#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct BorshCommitedValues(pub BorshExecutionInput, pub ExecutionOutput);

/// Borsh Serializable [ExecutionInput]
#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct BorshExecutionInput {
    pub rollup_accounts: BorshRollupState,
    pub txs: Vec<u8>, // Vec of serialized transactions: Vec<Transaction>
    pub ramp_txs: Vec<RampTx>,
}

/// Borsh Serializable [RollupState]
#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct BorshRollupState(pub Vec<(Pubkey, BorshAccount)>);

/// Borsh Serializable Solana
#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct BorshAccount {
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

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SP1Groth16Proof {
    pub proof: Vec<u8>,
    pub sp1_public_inputs: Vec<u8>,
    // pub sp1_public_inputs: BorshCommitedValues,
}
