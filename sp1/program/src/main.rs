//! A simple program to be proven inside the zkVM.
//!
#![no_main]
sp1_zkvm::entrypoint!(main);
use solana_sdk::{
    instruction::AccountMeta, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey,
    signer::keypair::Keypair, system_instruction::SystemInstruction, system_program,
};
use solana_svm::transaction_processor::ExecutionRecordingConfig;
mod data;
mod mock_bank;
use {
    crate::mock_bank::{
        create_executable_environment, deploy_program, register_builtins, MockBankCallback,
        MockForkGraph,
    },
    data::programs,
    solana_sdk::{
        account::{AccountSharedData, WritableAccount},
        hash::Hash,
        instruction::Instruction,
        signature::Signer,
        transaction::{SanitizedTransaction, Transaction},
    },
    solana_svm::{
        account_loader::CheckedTransactionDetails,
        transaction_processor::{
            TransactionBatchProcessor, TransactionProcessingConfig,
            TransactionProcessingEnvironment,
        },
    },
    solana_type_overrides::sync::{Arc, RwLock},
    // std::collections::HashSet,
};

const DEPLOYMENT_SLOT: u64 = 0;
const EXECUTION_SLOT: u64 = 5; // The execution slot must be greater than the deployment slot
const EXECUTION_EPOCH: u64 = 2; // The execution epoch must be greater than the deployment epoch
const LAST_BLOCKHASH: Hash = Hash::new_from_array([7; 32]); // Arbitrary constant hash for advancing nonce
const LAMPORTS_PER_SIGNATURE: u64 = 20;

pub fn main() {
    let mock_bank = MockBankCallback::default();
    let fee_payer_keypair = Keypair::from_bytes(&[
        242, 163, 63, 40, 200, 71, 226, 241, 76, 119, 227, 51, 189, 168, 214, 55, 102, 211, 213,
        34, 59, 184, 233, 45, 228, 254, 217, 17, 6, 254, 2, 63, 132, 90, 132, 46, 243, 109, 222,
        111, 0, 230, 245, 5, 52, 133, 183, 80, 238, 209, 113, 199, 31, 132, 192, 249, 62, 245, 9,
        77, 127, 158, 60, 52,
    ])
    .unwrap();

    let fee_payer = fee_payer_keypair.pubkey();
    let receiver_pk = Pubkey::new_unique();

    // Setting up the fee payer account
    let mut fee_payer_account = AccountSharedData::default();
    fee_payer_account.set_lamports(LAMPORTS_PER_SOL * 10);
    mock_bank
        .account_shared_data
        .write()
        .unwrap()
        .insert(fee_payer, fee_payer_account.clone());

    let mut txs: Vec<SanitizedTransaction> = vec![];
    let mut txscheck = vec![];

    let account_metas = vec![
        AccountMeta::new(fee_payer, true),
        AccountMeta::new(receiver_pk, false),
    ];
    let ix = Instruction::new_with_bincode(
        system_program::id(),
        &SystemInstruction::Transfer {
            lamports: LAMPORTS_PER_SOL,
        },
        account_metas,
    );
    let transaction = Transaction::new_signed_with_payer(
        &[ix],
        Some(&fee_payer),
        &[&fee_payer_keypair],
        LAST_BLOCKHASH,
    );
    let sanitized_transaction = SanitizedTransaction::from_transaction_for_tests(transaction);
    let transaction_check = Ok(CheckedTransactionDetails {
        nonce: None,
        lamports_per_signature: LAMPORTS_PER_SIGNATURE,
    });
    txs.push(sanitized_transaction);
    txscheck.push(transaction_check);

    // let program = programs()[0];
    // for program in programs() {
    //     println!("Deploying Program: {:#?}", program);
    //     let solana_program = deploy_program(program.to_string(), DEPLOYMENT_SLOT, &mock_bank);
    //     let instruction = Instruction::new_with_bytes(
    //         solana_program,
    //         &[],
    //         vec![AccountMeta::new(fee_payer, true)], // arbitrary mock account
    //     );
    //     let transaction = Transaction::new_signed_with_payer(
    //         &[instruction],
    //         Some(&fee_payer),
    //         &[&fee_payer_keypair],
    //         LAST_BLOCKHASH,
    //     );
    //     let sanitized_transaction = SanitizedTransaction::from_transaction_for_tests(transaction);
    //     let transaction_check = Ok(CheckedTransactionDetails {
    //         nonce: None,
    //         lamports_per_signature: LAMPORTS_PER_SIGNATURE,
    //     });
    //     txs.push(sanitized_transaction);
    //     txscheck.push(transaction_check);
    // }
    // // Load and execute the transaction
    // let batch_processor = TransactionBatchProcessor::<MockForkGraph>::new(
    //     EXECUTION_SLOT,
    //     EXECUTION_EPOCH,
    //     HashSet::new(),
    // );

    let batch_processor = TransactionBatchProcessor::<MockForkGraph>::new_uninitialized(
        EXECUTION_SLOT,
        EXECUTION_EPOCH,
    );

    let fork_graph = Arc::new(RwLock::new(MockForkGraph {}));

    // let fork_graph = Arc::new(RwLock::new(MockForkGraph {}));

    create_executable_environment(
        fork_graph.clone(),
        &mock_bank,
        &mut batch_processor.program_cache.write().unwrap(),
    );

    // The sysvars must be put in the cache
    batch_processor.fill_missing_sysvar_cache_entries(&mock_bank);
    register_builtins(&mock_bank, &batch_processor);

    let config = TransactionProcessingConfig {
        recording_config: ExecutionRecordingConfig {
            enable_cpi_recording: true,
            enable_log_recording: true,
            enable_return_data_recording: true,
        },
        ..TransactionProcessingConfig::default()
    };

    println!("here");

    let result = batch_processor.load_and_execute_sanitized_transactions(
        &mock_bank,
        &txs,
        txscheck,
        &TransactionProcessingEnvironment::default(),
        &config,
    );

    println!("Batch Result {:?}", result.processing_results);
}
