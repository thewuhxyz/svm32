use borsh::BorshDeserialize;
use clap::Parser;
use solana_sdk::{
    hash::Hash, native_token::LAMPORTS_PER_SOL, signature::Keypair, signer::Signer,
    system_instruction, system_program, transaction::Transaction,
};
use sp1_sdk::{include_elf, HashableKey, ProverClient, SP1Stdin};
use std::{fs::File, vec};
use svm_runner_types::{
    hash_state, CommitedValues, ExecutionInput, RampTx, RollupState, SP1Groth16Proof,
    SerializableAccount, State,
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

    #[clap(long, short, default_value = "./proof.bin")]
    output_path: String,
}

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

    ExecutionInput {
        accounts: RollupState {
            states: vec![
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
            ],
        },
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

    let client = ProverClient::from_env();
    let mut stdin = SP1Stdin::new();

    stdin.write_slice(&bytes);

    if args.execute {
        // Execute the program
        let (output, report) = client.execute(ELF, &stdin).run().unwrap();
        println!("Program executed successfully.");

        let commited_values = CommitedValues::try_from_slice(&output.to_vec()).unwrap();
        println!("committed values: {:#?}", &commited_values);

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
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

        let output = CommitedValues::try_from_slice(&proof.public_values.to_vec()).unwrap();
        println!("Final state hash: {:?}", output.output);

        println!("Successfully generated proof!");

        let grooth16_proof = SP1Groth16Proof {
            proof: proof.bytes(),
            sp1_public_inputs: output,
        };

        println!("Writing borsh serializable grooth16 proof to file...");

        let mut proof_borsh_file = File::create("grooth16_proof.bin").expect("failed to open file");

        borsh::to_writer(&mut proof_borsh_file, &grooth16_proof)
            .expect("borsh unable to write to file");

        println!("Successfully written to file!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
