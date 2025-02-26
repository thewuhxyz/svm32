use borsh::{BorshDeserialize, BorshSerialize};
use merkle_tree::MerkleTree;
use serde::{Deserialize, Serialize};
use solana_account::{Account, AccountSharedData};
use solana_program::{clock::Epoch, hash::Hash};
use solana_pubkey::Pubkey;
use solana_sha256_hasher::hashv;
pub use merkle_tree;


#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone)]
pub struct RampTx {
    pub is_onramp: bool,
    pub user: Pubkey,
    pub amount: u64,
}

pub type ExecutionOutput = Hash;


/// Generate the Merkle root of the `RollupState`
pub fn hash_state(rollup_state: &RollupState) -> Hash {
    let mut merkle_tree = MerkleTree::new();
    for state in &rollup_state.states {
        let account = state.account.clone().into();
        merkle_tree.insert(state.pubkey, &account);
    }
    merkle_tree.get_root()
}

/// Generate a Merkle proof for a specific account
pub fn generate_merkle_proof(rollup_state: &RollupState, pubkey: &Pubkey) -> Option<Vec<Hash>> {
    let mut merkle_tree = MerkleTree::new();
    for state in &rollup_state.states {
        let account = state.account.clone().into();
        merkle_tree.insert(state.pubkey, &account);
    }
    merkle_tree.generate_proof(pubkey)
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

#[cfg(test)]
mod tests {
    use super::*;
    pub use merkle_tree::MerkleTree;
    use solana_sdk::{pubkey::Pubkey};

    fn create_dummy_state(pubkey: Pubkey) -> State {
        State {
            pubkey,
            account: SerializableAccount {
                lamports: 1000,
                data: vec![1, 2, 3],
                executable: false,
                rent_epoch: 1,
                owner: pubkey,
            },
        }
    }

    fn create_dummy_state2(pubkey: Pubkey, lamports: u64, data: Vec<u8>) -> State {
        State {
            pubkey,
            account: SerializableAccount {
                lamports,
                data,
                executable: false,
                rent_epoch: 1,
                owner: pubkey,
            },
        }
    }

    #[test]
    fn test_merklization_of_rollup_state() {
        let mut rollup_state = RollupState { states: Vec::new() };

        for _ in 0..4 {
            let pubkey = Pubkey::new_unique();
            rollup_state.states.push(create_dummy_state(pubkey));
        }

        let merkle_root = hash_state(&rollup_state);
        assert_ne!(merkle_root, Hash::default(), "The Merkle root should not be empty.");
    }

    #[test]
    fn test_merkle_proof_with_rollup_state() {
        let mut rollup_state = RollupState { states: Vec::new() };

        let target_pubkey = Pubkey::new_unique();
        rollup_state.states.push(create_dummy_state(target_pubkey));

        for _ in 0..3 {
            let pubkey = Pubkey::new_unique();
            rollup_state.states.push(create_dummy_state(pubkey));
        }

        let proof = generate_merkle_proof(&rollup_state, &target_pubkey);
        assert!(proof.is_some(), "The Merkle proof should be generated for a valid account.");
    }

    #[test]
    fn test_invalid_merkle_proof_rollup_state() {
        let mut rollup_state = RollupState { states: Vec::new() };

        for _ in 0..4 {
            let pubkey = Pubkey::new_unique();
            rollup_state.states.push(create_dummy_state(pubkey));
        }

        let invalid_pubkey = Pubkey::new_unique();
        let proof = generate_merkle_proof(&rollup_state, &invalid_pubkey);
        assert!(proof.is_none(), "No Merkle proof should be generated for a non-existent account.");
    }

    #[test]
    fn test_merklization_and_proof_generation() {
        let mut rollup_state = RollupState { states: Vec::new() };
    
        // Generate 8 accounts with different data
        for i in 1..=8 {
            let pubkey = Pubkey::new_unique();
            rollup_state.states.push(create_dummy_state2(pubkey, 1000 * i, vec![i as u8; 3]));
        }
    
        // Verify that RollupState contains exactly 8 accounts
        assert_eq!(rollup_state.states.len(), 8, "RollupState must contain 8 accounts.");
    
        // Generate the Merkle root
        let merkle_root = hash_state(&rollup_state);
        println!("🔗 Merkle Root: {:?}", merkle_root);
        assert_ne!(merkle_root, Hash::default(), "The Merkle root should not be empty.");
    
        // Select an account to test proof generation
        let target_pubkey = rollup_state.states[2].pubkey;
    
        // Generate the Merkle proof
        let proof = generate_merkle_proof(&rollup_state, &target_pubkey);
        assert!(proof.is_some(), "The Merkle proof should be generated for an existing account.");
        
        let proof_vec = proof.unwrap();
        println!("🛠️ Merkle proof for {:?}: {:?}", target_pubkey, proof_vec);
        assert!(!proof_vec.is_empty(), "The Merkle proof should not be empty.");
    
        // Test with a non-existent account
        let invalid_pubkey = Pubkey::new_unique();
        let invalid_proof = generate_merkle_proof(&rollup_state, &invalid_pubkey);
        assert!(invalid_proof.is_none(), "No proof should be generated for a non-existent account.");
    }

    #[test]
    fn test_incremental_merkle_tree_construction() {
        let mut rollup_state = RollupState { states: Vec::new() };
        let mut merkle_tree = MerkleTree::new();

        let mut pubkeys = Vec::new();
        let mut previous_root = merkle_tree.get_root();

        // Insert accounts one by one and check Merkle root updates
        for i in 1..=8 {
            let pubkey = Pubkey::new_unique();
            let state = create_dummy_state2(pubkey, 1000 * i, vec![i as u8; 3]);
            
            // Add state to RollupState
            rollup_state.states.push(state.clone());

            // Insert into the Merkle Tree
            let account: Account = state.account.clone().into();
            merkle_tree.insert(state.pubkey, &account);

            // Store the pubkey for later proof verification
            pubkeys.push(state.pubkey);

            // Compute new Merkle root
            let new_root = merkle_tree.get_root();
            println!("🔗 Merkle Root after inserting state {}: {:?}", i, new_root);
            
            // Print the Merkle Tree structure (only leaves for readability)
            println!("🌳 Merkle Tree after insertion {}: {:?}", i, merkle_tree.tree);

            // Ensure the Merkle root updates after each insertion
            assert_ne!(new_root, previous_root, "Merkle root should change after insertion.");
            previous_root = new_root;
        }

        // Select a random inserted pubkey to test proof generation
        let target_pubkey = pubkeys[3]; // Pick the 4th inserted account

        // Generate the Merkle proof
        let proof = merkle_tree.generate_proof(&target_pubkey);
        assert!(proof.is_some(), "Merkle proof should be generated for an existing account.");
        
        let proof_vec = proof.unwrap();
        println!("🛠️ Merkle proof for {:?}: {:?}", target_pubkey, proof_vec);
        assert!(!proof_vec.is_empty(), "Merkle proof should not be empty.");

        // Test with a non-existent account
        let invalid_pubkey = Pubkey::new_unique();
        let invalid_proof = merkle_tree.generate_proof(&invalid_pubkey);
        assert!(invalid_proof.is_none(), "No proof should be generated for a non-existent account.");
    }
}



