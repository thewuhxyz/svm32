use clap::Parser;
use solana_sdk::{
    account::Account, hash::Hash, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey,
    signature::Keypair, signer::Signer, system_instruction, system_program,
    transaction::Transaction,
};
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};
use std::vec;
use svm_runner_lib::{ExecutionInput, RampTx};

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
    let kp_sender = Keypair::new();
    let pk_receiver = Pubkey::new_unique();
    ExecutionInput {
        accounts: vec![
            (
                kp_sender.try_pubkey().unwrap(),
                Account {
                    lamports: 0,
                    data: vec![],
                    owner: system_program::id(),
                    executable: false,
                    rent_epoch: 0,
                },
            ),
            (
                pk_receiver,
                Account {
                    lamports: 0,
                    data: vec![],
                    owner: system_program::id(),
                    executable: false,
                    rent_epoch: 0,
                },
            ),
        ],
        txs: vec![Transaction::new_signed_with_payer(
            &[system_instruction::transfer(
                &kp_sender.try_pubkey().unwrap(),
                &pk_receiver,
                LAMPORTS_PER_SOL,
            )],
            Some(&kp_sender.try_pubkey().unwrap()),
            &[&kp_sender],
            Hash::new_from_array([7; 32]),
        )],
        ramp_txs: vec![RampTx {
            is_onramp: true,
            user: kp_sender.try_pubkey().unwrap(),
            amount: 10 * LAMPORTS_PER_SOL,
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
    let bytes = if let Some(input) = args.input {
        input
    } else {
        let input = create_test_input();
        bincode::serialize(&input).unwrap()
    };

    let client = ProverClient::from_env();
    let mut stdin = SP1Stdin::new();
    stdin.write(&bytes);

    if args.execute {
        // Execute the program
        let (output, report) = client.execute(ELF, &stdin).run().unwrap();
        println!("Program executed successfully.");

        // Read the output.
        // let decoded = PublicValuesStruct::abi_decode(output.as_slice(), true).unwrap();
        // let PublicValuesStruct { n, a, b } = decoded;
        // println!("n: {}", n);
        // println!("a: {}", a);
        println!("output buffer: {}", output.raw());

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(ELF);

        // Generate the proof
        println!("Starting proof generation...");
        let proof = client
            .prove(&pk, &stdin)
            .groth16()
            .run()
            .expect("failed to generate proof");
        proof.save(args.output_path).expect("failed to save proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
