use borsh::BorshDeserialize;
use clap::Parser;
// use serde::Serialize;
use solana_sdk::{
    hash::Hash, native_token::LAMPORTS_PER_SOL, signature::Keypair, signer::Signer,
    system_instruction, system_program, transaction::Transaction,
};
use sp1_sdk::{include_elf, HashableKey, ProverClient, SP1Stdin};
use std::vec;
// use svm_runner_types::{hash_state, CommittedValues, ExecutionInput, RampTx, RollupState};
use svm_runner_types_anchor::{
    hash_state, BorshAccount, BorshCommitedValues, BorshExecutionInput, BorshRollupState, RampTx,
};

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

    // #[clap(long, short, default_value = "./proof.bin")]
    #[clap(long, short, default_value = "./proof_borsh_input.bin")]
    output_path: String,
}

// fn create_test_input() -> ExecutionInput {
fn create_test_input() -> BorshExecutionInput {
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

    // ExecutionInput {
    BorshExecutionInput {
        // accounts: RollupState(vec![
        accounts: BorshRollupState(vec![
            (
                kp_sender.try_pubkey().unwrap(),
                // Account {
                BorshAccount {
                    lamports: 10 * LAMPORTS_PER_SOL,
                    // lamports: 10 * LAMPORTS_PER_SOL,
                    data: vec![],
                    owner: system_program::id(),
                    executable: false,
                    rent_epoch: 0,
                },
            ),
            (
                pk_receiver,
                // Account {
                BorshAccount {
                    lamports: 0,
                    data: vec![],
                    owner: system_program::id(),
                    executable: false,
                    rent_epoch: 0,
                },
            ),
        ]),
        // txs: vec![Transaction::new_signed_with_payer(
        //     &[system_instruction::transfer(
        //         &kp_sender.try_pubkey().unwrap(),
        //         &pk_receiver,
        //         10 * LAMPORTS_PER_SOL,
        //     )],
        //     Some(&kp_sender.try_pubkey().unwrap()),
        //     &[&kp_sender],
        //     Hash::new_from_array([7; 32]),
        // )],
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

    // Default to test input if user does not provide
    let input = if let Some(input) = args.input {
        bincode::deserialize(&input).unwrap()
    } else {
        create_test_input()
    };

    let bytes = borsh::to_vec(&input).unwrap();
    // let bytes = bincode::serialize(&input).unwrap();

    let client = ProverClient::from_env();
    let mut stdin = SP1Stdin::new();
    
    stdin.write_slice(&bytes);

    if args.execute {
        // Execute the program
        let (output, report) = client.execute(ELF, &stdin).run().unwrap();
        println!("Program executed successfully.");

        // Read the output.
        // let decoded = PublicValuesStruct::abi_decode(output.as_slice(), true).unwrap();
        // let PublicValuesStruct { n, a, b } = decoded;
        // println!("n: {}", n);
        // println!("a: {}", a);
        
        // println!("output buffer: {}", output.raw());
        // let sa: BorshExecutionInput = bincode::deserialize::<BorshExecutionInput>(&bytes).unwrap();
        // println!("byes buffer: {:#?}", sa);
        
        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
        
        // let so = output.read::<BorshExecutionInput>();
        // println!("output buffer: {:#?}", so);
        
        // let commited_values = bincode::deserialize::<BorshCommitedValues>(&output.to_vec()).unwrap();
        let commited_values = BorshCommitedValues::try_from_slice(&output.to_vec()).unwrap();
        println!("committed values: {:#?}", &commited_values);

        // let output = BorshExecutionInput::try_from_slice(&output.to_vec()).unwrap();
        // println!("output buffer: {:#?}", output);
        // println!("output buffer: {}", output.raw());

    } else {
        println!("Initial state hash: {}", hash_state(input.accounts));

        // Setup the program for proving.
        let (pk, vk) = client.setup(ELF);
        println!("Verifying key: {}", vk.bytes32());

        println!("Starting proof generation...");
        let proof = client
            .prove(&pk, &stdin)
            .groth16()
            .run()
            .expect("failed to generate proof");
        proof.save(args.output_path).expect("failed to save proof");

        // let output: BorshCommitedValues = proof.public_values.read();
        // let output = bincode::deserialize::<BorshCommitedValues>(&proof.public_values.to_vec()).unwrap();
        
        let output = BorshCommitedValues::try_from_slice(&proof.public_values.to_vec()).unwrap();
        println!("Final state hash: {:?}", output.1);

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
