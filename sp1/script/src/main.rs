use std::vec;

use solana_sdk::{
    account::Account, hash::Hash, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey,
    signature::Keypair, signer::Signer, system_instruction, system_program,
    transaction::Transaction,
};
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};
use svm_runner_lib::{ExecutionInput, RampTx};

const ELF: &[u8] = include_elf!("zk-svm");

fn main() {
    let kp_sender = Keypair::new();
    let pk_receiver = Pubkey::new_unique();
    let input = ExecutionInput {
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
    };
    let bytes: Vec<u8> = bincode::serialize(&input).unwrap();

    let mut stdin = SP1Stdin::new();
    stdin.write(&bytes);

    let client = ProverClient::from_env();
    let (values, result) = client.execute(ELF, &stdin).run().unwrap();

    println!("result: {}", result);
    println!("values: {:?}", values);

    println!("successfully generated and verified proof for the program!")
}
