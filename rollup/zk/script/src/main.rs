// use borsh::BorshDeserialize;
// use clap::Parser;
// use solana_sdk::{
//     hash::Hash, native_token::LAMPORTS_PER_SOL, signature::Keypair, signer::Signer,
//     system_instruction, system_program, transaction::Transaction,
// };
// use sp1_sdk::{include_elf, HashableKey, ProverClient, SP1Stdin};
// use std::{fs::File, vec};
// use svm_runner_types::{
//     hash_state, CommitedValues, ExecutionInput, RampTx, RollupState, SP1Groth16Proof,
//     SerializableAccount, State, generate_merkle_proof,
// };
// use merkle_tree::MerkleTree;

// const ELF: &[u8] = include_elf!("zk-svm");

// #[derive(Parser, Debug)]
// #[clap(author, version, about, long_about = None)]
// struct Args {
//     #[clap(long)]
//     execute: bool,

//     #[clap(long)]
//     prove: bool,

//     #[clap(long)]
//     input: Option<Vec<u8>>,

//     #[clap(long, short, default_value = "./proof.bin")]
//     output_path: String,
// }

// fn create_test_input() -> ExecutionInput {
//     let kp_sender_bytes: Vec<u8> =
//         serde_json::from_slice(include_bytes!("../../../onchain/keypairSender.json")).unwrap();
//     let kp_sender = Keypair::from_bytes(&kp_sender_bytes).unwrap();

//     let kp_receiver_bytes: Vec<u8> =
//         serde_json::from_slice(include_bytes!("../../../onchain/keypairReceiver.json")).unwrap();
//     let kp_receiver = Keypair::from_bytes(&kp_receiver_bytes).unwrap();
//     let pk_receiver = kp_receiver.pubkey();

//     let transactions = vec![Transaction::new_signed_with_payer(
//         &[system_instruction::transfer(
//             &kp_sender.try_pubkey().unwrap(),
//             &pk_receiver,
//             LAMPORTS_PER_SOL,
//         )],
//         Some(&kp_sender.try_pubkey().unwrap()),
//         &[&kp_sender],
//         Hash::new_from_array([7; 32]),
//     )];

//     let serialized_transactions = bincode::serialize(&transactions).unwrap();

//     ExecutionInput {
//         accounts: RollupState {
//             states: vec![
//                 State {
//                     pubkey: kp_sender.try_pubkey().unwrap(),
//                     account: SerializableAccount {
//                         lamports: 10 * LAMPORTS_PER_SOL,
//                         data: vec![],
//                         owner: system_program::id(),
//                         executable: false,
//                         rent_epoch: 0,
//                     },
//                 },
//                 State {
//                     pubkey: pk_receiver,
//                     account: SerializableAccount {
//                         lamports: 0,
//                         data: vec![],
//                         owner: system_program::id(),
//                         executable: false,
//                         rent_epoch: 0,
//                     },
//                 },
//             ],
//         },
//         txs: serialized_transactions,
//         ramp_txs: vec![RampTx {
//             is_onramp: true,
//             user: kp_sender.try_pubkey().unwrap(),
//             amount: LAMPORTS_PER_SOL,
//         }],
//     }
// }

// fn main() {
//     let args = Args::parse();

//     if args.execute == args.prove {
//         eprintln!("Error: You must specify either --execute or --prove");
//         std::process::exit(1);
//     }

//     // Default to test input if user does not provide
//     let input = if let Some(input) = args.input {
//         bincode::deserialize(&input).unwrap()
//     } else {
//         create_test_input()
//     };

//     let bytes = borsh::to_vec(&input).unwrap();

//     let client = ProverClient::from_env();
//     let mut stdin = SP1Stdin::new();

//     stdin.write_slice(&bytes);

//     if args.execute {
//         // Execute the program
//         let (output, report) = client.execute(ELF, &stdin).run().unwrap();
//         println!("Program executed successfully.");

//         let commited_values = CommitedValues::try_from_slice(&output.to_vec()).unwrap();
//         println!("committed values: {:#?}", &commited_values);

//         // Record the number of cycles executed.
//         println!("Number of cycles: {}", report.total_instruction_count());
//     } else {
//         println!("Initial Merkle root: {}", hash_state(&input.accounts));

//         // Setup the program for proving.
//         let (pk, vk) = client.setup(ELF);
//         println!("Verifying key: {}", vk.bytes32());

//         println!("Starting proof generation...");
//         let proof = client
//             .prove(&pk, &stdin)
//             .groth16()
//             .run()
//             .expect("failed to generate proof");
//         proof.save(args.output_path).expect("failed to save proof");

//         let output = CommitedValues::try_from_slice(&proof.public_values.to_vec()).unwrap();
//         println!("Final state hash: {:?}", output.output);

//         println!("Successfully generated proof!");

//         let grooth16_proof = SP1Groth16Proof {
//             proof: proof.bytes(),
//             sp1_public_inputs: output,
//         };

//         println!("Writing borsh serializable grooth16 proof to file...");

//         let mut proof_borsh_file = File::create("grooth16_proof.bin").expect("failed to open file");

//         borsh::to_writer(&mut proof_borsh_file, &grooth16_proof)
//             .expect("borsh unable to write to file");

//         println!("Successfully written to file!");

//         // Verify the proof.
//         client.verify(&proof, &vk).expect("failed to verify proof");
//         println!("Successfully verified proof!");
//     }
// }

use borsh::BorshDeserialize;
use clap::Parser;
use solana_sdk::{
    hash::Hash, native_token::LAMPORTS_PER_SOL, signature::Keypair, signer::Signer,
    system_instruction, system_program, transaction::Transaction, account::Account,
};
use sp1_sdk::{include_elf, HashableKey, ProverClient, SP1Stdin};
use std::{fs::File, vec};
use svm_runner_types::{
    hash_state, CommitedValues, ExecutionInput, RampTx, RollupState, SP1Groth16Proof,
    SerializableAccount, State,
};
use merkle_tree::MerkleTree;

const ELF: &[u8] = include_elf!("zk-svm");

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,

    #[clap(long)]
    input: Option<Vec<u8>>,

    #[clap(long, short, default_value = "./proof.bin")]
    output_path: String,
}

/// Creates a test input with initial accounts
fn create_test_input() -> ExecutionInput {
    let kp_sender_bytes: Vec<u8> =
        serde_json::from_slice(include_bytes!("../../../onchain/keypairSender.json")).unwrap();
    let kp_sender = Keypair::from_bytes(&kp_sender_bytes).unwrap();

    let kp_receiver_bytes: Vec<u8> =
        serde_json::from_slice(include_bytes!("../../../onchain/keypairReceiver.json")).unwrap();
    let kp_receiver = Keypair::from_bytes(&kp_receiver_bytes).unwrap();
    let pk_receiver = kp_receiver.pubkey();

    let transactions = vec![Transaction::new_signed_with_payer(
        &[system_instruction::transfer(
            &kp_sender.try_pubkey().unwrap(),
            &pk_receiver,
            LAMPORTS_PER_SOL,
        )],
        Some(&kp_sender.try_pubkey().unwrap()),
        &[&kp_sender],
        Hash::new_from_array([7; 32]),
    )];

    let serialized_transactions = bincode::serialize(&transactions).unwrap();

    let accounts = vec![
        State {
            pubkey: kp_sender.try_pubkey().unwrap(),
            account: SerializableAccount {
                lamports: 10 * LAMPORTS_PER_SOL,
                data: vec![],
                owner: system_program::id(),
                executable: false,
                rent_epoch: 0,
            },
        },
        State {
            pubkey: pk_receiver,
            account: SerializableAccount {
                lamports: 0,
                data: vec![],
                owner: system_program::id(),
                executable: false,
                rent_epoch: 0,
            },
        },
    ];

    ExecutionInput {
        accounts: RollupState { states: accounts },
        txs: serialized_transactions,
        ramp_txs: vec![RampTx {
            is_onramp: true,
            user: kp_sender.try_pubkey().unwrap(),
            amount: LAMPORTS_PER_SOL,
        }],
    }
}

fn main() {
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Load test input or provided input
    let mut input = if let Some(input) = args.input {
        bincode::deserialize(&input).unwrap()
    } else {
        create_test_input()
    };

    let bytes = borsh::to_vec(&input).unwrap();

    let client = ProverClient::from_env();
    let mut stdin = SP1Stdin::new();

    stdin.write_slice(&bytes);

    // ✅ Step 1: Initialize Merkle Tree and insert accounts
    let mut merkle_tree = MerkleTree::new();
    println!("📥 Inserting initial accounts into Merkle Tree...");

    for (i, state) in input.accounts.states.iter().enumerate() {
        let account: Account = state.account.clone().into();
        merkle_tree.insert(state.pubkey, &account);

        // Print tree state after each insertion
        println!("🌳 Merkle Tree after inserting account {}: {:?}", i + 1, merkle_tree.tree);
        println!("🔗 Merkle Root after inserting account {}: {:?}", i + 1, merkle_tree.get_root());
    }

    println!("✅ Final Initial Merkle Root: {:?}", merkle_tree.get_root());

    if args.execute {
        // 📌 Step 2: Print account states before execution
        println!("📊 Checking account states BEFORE execution...");
        for state in &input.accounts.states {
            println!(" - {:?} -> {:?} lamports", state.pubkey, state.account.lamports);
        }
        
        // 📌 Step 3: Execute transactions
        let (output, report) = client.execute(ELF, &stdin).run().unwrap();
        println!("✅ Program executed successfully.");

        // 📌 Step 4: Simulate state update (since execution doesn't modify states in this script)
        let kp_sender_bytes: Vec<u8> =serde_json::from_slice(include_bytes!("../../../onchain/keypairSender.json")).unwrap();
        let kp_sender = Keypair::from_bytes(&kp_sender_bytes).expect("❌ Failed to parse sender keypair");

        let kp_receiver_bytes: Vec<u8> = serde_json::from_slice(include_bytes!("../../../onchain/keypairReceiver.json")).unwrap();
        let kp_receiver = Keypair::from_bytes(&kp_receiver_bytes).expect("❌ Failed to parse receiver keypair");

        for state in &mut input.accounts.states {
            if state.pubkey == kp_sender.pubkey() {
                state.account.lamports -= LAMPORTS_PER_SOL;
            } else if state.pubkey == kp_receiver.pubkey() {
                state.account.lamports += LAMPORTS_PER_SOL;
            }
        }

        // 📌 Step 5: Print account states after execution
        println!("📊 Checking account states AFTER execution...");
        for state in &input.accounts.states {
            println!(" - {:?} -> {:?} lamports", state.pubkey, state.account.lamports);
        }

        // 📌 Step 6: Incrementally update Merkle Tree
        println!("🔄 Updating Merkle Tree after execution...");
        for state in &input.accounts.states {
            let account: Account = state.account.clone().into();
            merkle_tree.update(state.pubkey, &account); // Only update modified accounts
        }
        println!("✅ Updated Merkle Root after execution: {:?}", merkle_tree.get_root());

        // Track cycle count
        println!("📊 Number of cycles executed: {}", report.total_instruction_count());
    } else {
        // 📌 Step 7: Setup the prover
        println!("🔹 Initial Merkle Root: {}", hash_state(&input.accounts));

        // Simulation..
        let kp_sender_bytes: Vec<u8> =serde_json::from_slice(include_bytes!("../../../onchain/keypairSender.json")).unwrap();
        let kp_sender = Keypair::from_bytes(&kp_sender_bytes).expect("❌ Failed to parse sender keypair");

        let kp_receiver_bytes: Vec<u8> = serde_json::from_slice(include_bytes!("../../../onchain/keypairReceiver.json")).unwrap();
        let kp_receiver = Keypair::from_bytes(&kp_receiver_bytes).expect("❌ Failed to parse receiver keypair");

        for state in &mut input.accounts.states {
            if state.pubkey == kp_sender.pubkey() {
                state.account.lamports -= LAMPORTS_PER_SOL;
            } else if state.pubkey == kp_receiver.pubkey() {
                state.account.lamports += LAMPORTS_PER_SOL;
            }
        }
        
        println!("📊 Checking account states AFTER execution...");
        for state in &input.accounts.states {
            println!(" - {:?} -> {:?} lamports", state.pubkey, state.account.lamports);
        }

        println!("🔄 Updating Merkle Tree after execution...");
        for state in &input.accounts.states {
            let account: Account = state.account.clone().into();
            merkle_tree.update(state.pubkey, &account); // Only update modified accounts
        }
        println!("✅ Updated Merkle Root after execution: {:?}", merkle_tree.get_root());

        let (pk, vk) = client.setup(ELF);
        println!("🔹 Verifying key: {}", vk.bytes32());

        // 📌 Step 8: Generate the proof
        println!("🚀 Starting proof generation...");
        let proof = client
            .prove(&pk, &stdin)
            .groth16()
            .run()
            .expect("❌ Failed to generate proof");
        proof.save(args.output_path).expect("❌ Failed to save proof");

        let output = CommitedValues::try_from_slice(&proof.public_values.to_vec()).unwrap();
        println!("✅ Final state hash: {:?}", output.output); // TODO?

        // 📌 Step 9: Store the proof
        let grooth16_proof = SP1Groth16Proof {
            proof: proof.bytes(),
            sp1_public_inputs: output,
        };

        println!("💾 Writing proof to file...");
        let mut proof_borsh_file = File::create("grooth16_proof.bin").expect("❌ Failed to open file");
        borsh::to_writer(&mut proof_borsh_file, &grooth16_proof)
            .expect("❌ Failed to write to file");
        println!("✅ Proof successfully written!");

        // 📌 Step 10: Verify the proof
        client.verify(&proof, &vk).expect("❌ Proof verification failed");
        println!("✅ Successfully verified proof!");
    }
}
