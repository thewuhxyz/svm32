use solana_hash::Hash;
use solana_sha256_hasher::hashv;
use solana_sdk::{pubkey::Pubkey, account::Account};
use bincode::serialize;
use serde::{Serialize, Deserialize};


// /// Represents a Solana account in the Merkle Tree
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct SolanaAccount {
//     pub lamports: u64,
//     pub data: Vec<u8>,
//     pub executable: bool,
//     pub rent_epoch: u64,
//     pub owner: Pubkey,
// }

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MerkleTree {
    pub leaves: Vec<(Pubkey, Hash)>,    // Tree leaves with Pubkey and Hash
    pub tree: Vec<Hash>,                // All tree nodes (including leaves)
    pub root: Hash,                     // Tree root
}

impl MerkleTree {
    pub fn new() -> Self {
        MerkleTree {
            leaves: Vec::new(),
            tree: Vec::new(),
            root: Hash::default(),
        }
    }

    /// Hash of a Solana account
    fn hash_account(account: &Account) -> Hash {
        let account_bytes = serialize(account).unwrap();
        hashv(&[&account_bytes])
    }

    /// Inserts an account into the Merkle tree
    pub fn insert(&mut self, pubkey: Pubkey, account: &Account) {
        let account_hash = Self::hash_account(account);
        self.leaves.push((pubkey, account_hash));
        self.build_tree();
    }

    /// Builds the tree from leaves
    fn build_tree(&mut self) {
        self.tree.clear();

        if self.leaves.is_empty() {
            self.root = Hash::default();
            return;
        }

        // Copy leaf hashes into the tree
        let mut current_level: Vec<Hash> = self.leaves.iter().map(|(_, hash)| *hash).collect();
        
        // Ensure even number of leaves by duplicating last if necessary
        if current_level.len() % 2 != 0 {
            current_level.push(*current_level.last().unwrap());
        }

        self.tree.extend_from_slice(&current_level);

        while current_level.len() > 1 {
            let mut next_level = Vec::new();

            for chunk in current_level.chunks(2) {
                let left = &chunk[0];
                let right = &chunk[1];
                let parent = Self::hash_nodes(left, right);
                next_level.push(parent);
            }

            // Ensure even number of nodes at each level by duplicating last if necessary
            if next_level.len() % 2 != 0 && next_level.len() > 1 {
                next_level.push(*next_level.last().unwrap());
            }

            self.tree.extend_from_slice(&next_level);
            current_level = next_level;
        }

        self.root = current_level[0];
    }

    /// Hash two nodes to generate a parent node
    fn hash_nodes(left: &Hash, right: &Hash) -> Hash {
        hashv(&[left.as_ref(), right.as_ref()])
    }

    /// Returns the current tree root
    pub fn get_root(&self) -> Hash {
        self.root
    }



    /// Generates a Merkle proof for a given public key
    pub fn generate_proof(&self, pubkey: &Pubkey) -> Option<Vec<Hash>> {
        let mut index = self.leaves.iter().position(|(pk, _)| pk == pubkey)?;
        let mut proof = Vec::new();
        let mut current_level = self.leaves.iter().map(|(_, hash)| *hash).collect::<Vec<_>>();
        
        if current_level.len() % 2 != 0 {
            current_level.push(*current_level.last().unwrap());
        }

        let mut level_start = 0;
        let mut level_size = current_level.len();
        
        while level_size > 1 {
            let is_right_node = index % 2 == 1;
            let sibling_index = if is_right_node { index - 1 } else { index + 1 };
            
            if sibling_index < level_size {
                proof.push(current_level[sibling_index]);
            }
            
            index /= 2;
            level_start += level_size;
            level_size = (level_size + 1) / 2; // If odd, last element is copied
            
            let mut next_level = Vec::new();
            for chunk in current_level.chunks(2) {
                let left = &chunk[0];
                let right = &chunk[1];
                let parent = Self::hash_nodes(left, right);
                next_level.push(parent);
            }
            
            if next_level.len() % 2 != 0 && next_level.len() > 1 {
                next_level.push(*next_level.last().unwrap());
            }
            
            current_level = next_level;
        }
        
        Some(proof)
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::pubkey::Pubkey;
    use solana_sdk::signature::{Keypair, Signer}; // For generating pubkeys


    fn create_example_account(pubkey: Pubkey) -> Account {
        Account {
            lamports: 1000,
            data: vec![1, 2, 3, 4],
            executable: false,
            rent_epoch: 1,
            owner: pubkey,
        }
    }

    #[test]
    fn test_single_leaf_duplication() {
        let mut merkle_tree = MerkleTree::new();

        // Insert a single leaf
        let pubkey1 = Keypair::new().pubkey();
        let account1 = create_example_account(pubkey1);
        merkle_tree.insert(pubkey1, &account1);

        // Verify that the tree contains two copies of the leaf hash
        assert_eq!(merkle_tree.tree.len(), 3); // 2 leaves (duplicated) + 1 root
        assert_eq!(merkle_tree.tree[0], merkle_tree.tree[1]); // First two entries should be identical

        // Verify that proof can be generated
        let proof = merkle_tree.generate_proof(&pubkey1);
        assert!(proof.is_some());
        
        // Proof should contain a single element (the duplicated hash)
        let proof_vec = proof.unwrap();
        assert_eq!(proof_vec.len(), 1);
    }

    #[test]
    fn test_insert_and_build_tree() {
        let mut merkle_tree = MerkleTree::new();

        let pubkey1 = Keypair::new().pubkey(); // Generate a valid pubkey
        let account1 = create_example_account(pubkey1);
        merkle_tree.insert(pubkey1, &account1);

        let pubkey2 = Keypair::new().pubkey(); // Generate another valid pubkey
        let account2 = create_example_account(pubkey2);
        merkle_tree.insert(pubkey2, &account2);

        // Verify that root is generated after insertion
        assert_ne!(merkle_tree.get_root(), Hash::default(), "Root should not be empty.");
    }

    #[test]
    fn test_generate_proof() {
        let mut merkle_tree = MerkleTree::new();

        let pubkey1 = Pubkey::new_unique();
        let account1 = Account {
            lamports: 1000,
            data: vec![1, 2, 3],
            executable: false,
            rent_epoch: 0,
            owner: pubkey1,
        };
        merkle_tree.insert(pubkey1, &account1);

        let pubkey2 = Pubkey::new_unique();
        let account2 = Account {
            lamports: 2000,
            data: vec![4, 5, 6],
            executable: false,
            rent_epoch: 0,
            owner: pubkey2,
        };
        merkle_tree.insert(pubkey2, &account2);

        println!("tree after insert pubkey2: {:?}", merkle_tree.tree);

        let pubkey3 = Pubkey::new_unique();
        let account3 = Account {
            lamports: 2000,
            data: vec![3, 3, 3],
            executable: false,
            rent_epoch: 0,
            owner: pubkey3,
        };
        merkle_tree.insert(pubkey3, &account3);

        let pubkey4 = Pubkey::new_unique();
        let account4 = Account {
            lamports: 2000,
            data: vec![4, 4, 4],
            executable: false,
            rent_epoch: 0,
            owner: pubkey4,
        };
        merkle_tree.insert(pubkey4, &account4);

        let pubkey5 = Pubkey::new_unique();
        let account5 = Account {
            lamports: 2000,
            data: vec![5, 5, 5],
            executable: false,
            rent_epoch: 0,
            owner: pubkey5,
        };
        merkle_tree.insert(pubkey5, &account5);

        

        let pubkey6 = Pubkey::new_unique();
        let account6 = Account {
            lamports: 2000,
            data: vec![6, 6, 6],
            executable: false,
            rent_epoch: 0,
            owner: pubkey6,
        };
        merkle_tree.insert(pubkey6, &account6);

        

        let pubkey7 = Pubkey::new_unique();
        let account7 = Account {
            lamports: 2000,
            data: vec![7, 7, 7],
            executable: false,
            rent_epoch: 0,
            owner: pubkey7,
        };
        merkle_tree.insert(pubkey7, &account7);

        

        let pubkey8 = Pubkey::new_unique();
        let account8 = Account {
            lamports: 2000,
            data: vec![8, 8, 8],
            executable: false,
            rent_epoch: 0,
            owner: pubkey8,
        };
        merkle_tree.insert(pubkey8, &account8);

        println!("tree after insert pubkey8: {:?}", merkle_tree.tree);

        let proof = merkle_tree.generate_proof(&pubkey3);
        assert!(proof.is_some(), "Merkle proof should be generated.");
        let proof_vec = proof.unwrap();
        println!("proof for pubkey3: {:?}", proof_vec);
        assert!(!proof_vec.is_empty(), "Merkle proof should not be empty.");
    }

    #[test]
    fn test_root_changes_after_insert() {
        let mut merkle_tree = MerkleTree::new();

        let pubkey1 = Keypair::new().pubkey(); // Generate a valid pubkey
        let account1 = create_example_account(pubkey1);
        let initial_root = merkle_tree.get_root();
        
        // Add first account
        merkle_tree.insert(pubkey1, &account1);
        
        // Verify that root changed after insertion
        assert_ne!(merkle_tree.get_root(), initial_root, "Root should change after insertion.");

        // Add another account
        let pubkey2 = Keypair::new().pubkey(); // Generate another valid pubkey
        let account2 = create_example_account(pubkey2);
        merkle_tree.insert(pubkey2, &account2);

        // Verify that root changes again
        assert_ne!(merkle_tree.get_root(), initial_root, "Root should change again after another insertion.");
    }

    // #[test]
    // fn test_find_leaf_index() {
    //     let mut merkle_tree = MerkleTree::new();

    //     let pubkey1 = Keypair::new().pubkey(); // Generate a valid pubkey
    //     let account1 = create_example_account(pubkey1);
    //     merkle_tree.insert(pubkey1, &account1);

    //     let pubkey2 = Keypair::new().pubkey(); // Generate another valid pubkey
    //     let account2 = create_example_account(pubkey2);
    //     merkle_tree.insert(pubkey2, &account2);

    //     // Verify that leaf indices are correct
    //     let index1 = merkle_tree.find_leaf_index(&pubkey1);
    //     let index2 = merkle_tree.find_leaf_index(&pubkey2);

    //     assert_eq!(index1, Some(0), "Index of pubkey1 should be 0.");
    //     assert_eq!(index2, Some(1), "Index of pubkey2 should be 1.");
    // }

    #[test]
    fn test_generate_proof_invalid_pubkey() {
        let mut merkle_tree = MerkleTree::new();

        let pubkey1 = Keypair::new().pubkey(); // Generate a valid pubkey
        let account1 = create_example_account(pubkey1);
        merkle_tree.insert(pubkey1, &account1);

        let invalid_pubkey = Keypair::new().pubkey(); // Generate an invalid pubkey

        // Verify that proof fails for a pubkey that doesn't exist in the tree
        let proof = merkle_tree.generate_proof(&invalid_pubkey);

        assert!(proof.is_none(), "Proof for an invalid pubkey should be None.");
    }
    // Not like that, we dedouble instead of considering account hash as root
    // #[test]
    // fn test_integrity_of_solana_account() {
    //     let mut merkle_tree = MerkleTree::new();

    //     let pubkey = Keypair::new().pubkey(); // Generate a valid pubkey
    //     let account = create_example_account(pubkey);
    //     merkle_tree.insert(pubkey, &account);
    //     println!("Tree after insertion of pubkey: {:?}", merkle_tree.tree);

    //     // Get tree root after insertion
    //     let root = merkle_tree.get_root();

    //     // Recreate account hash manually
    //     let account_hash = MerkleTree::hash_account(&account);

    //     // Verify that account hash is correctly used as leaf in the tree
    //     assert_eq!(root, account_hash, "Root does not match account hash.");
    // }
}