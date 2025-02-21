//! SVM runner executing transactions on the given accounts
//!
use solana_account::Account;
// use svm_runner_types::{ExecutionInput, RollupState};
use svm_runner_types_anchor::{BorshExecutionInput, BorshRollupState};
use solana_sdk::{account::{ReadableAccount, WritableAccount}, transaction::Transaction};
// use solana_account::Account;
use solana_svm::transaction_processor::ExecutionRecordingConfig;
mod data;
mod mock_bank;
use {
    crate::mock_bank::{
        create_executable_environment, register_builtins, MockBankCallback, MockForkGraph,
    },
    solana_sdk::transaction::SanitizedTransaction,
    solana_svm::{
        account_loader::CheckedTransactionDetails,
        transaction_processor::{
            TransactionBatchProcessor, TransactionProcessingConfig,
            TransactionProcessingEnvironment,
        },
    },
    solana_type_overrides::sync::{Arc, RwLock},
};

// const DEPLOYMENT_SLOT: u64 = 0;
// const LAST_BLOCKHASH: Hash = Hash::new_from_array([7; 32]); // Arbitrary constant hash for advancing nonce
const EXECUTION_SLOT: u64 = 5; // The execution slot must be greater than the deployment slot
const EXECUTION_EPOCH: u64 = 2; // The execution epoch must be greater than the deployment epoch
const LAMPORTS_PER_SIGNATURE: u64 = 20;

pub fn runner(input: BorshExecutionInput) -> BorshRollupState {
    let mock_bank = MockBankCallback::default();
    // Insert accounts in the bank
    for (pk, account) in &input.accounts.0 {
        // let _account = Account::from(account);
        // let _account: Account = account.into();
        mock_bank
            .account_shared_data
            .write()
            .unwrap()
            .insert(*pk, account.clone().into());
    }

    // Process ramp txs
    for tx in &input.ramp_txs {
        let mut writer = mock_bank.account_shared_data.write().unwrap();

        let account = writer.get_mut(&tx.user).unwrap();

        if tx.is_onramp {
            account.set_lamports(account.lamports() + tx.amount);
        } else {
            account.set_lamports(account.lamports() - tx.amount);
        }
    }

    let mut txs = vec![];
    let mut txscheck = vec![];

    let transactions = bincode::deserialize::<Vec<Transaction>>(&input.txs).unwrap();

    // for tx in input.txs {
    for tx in transactions {
        let sanitized_transaction = SanitizedTransaction::from_transaction_for_tests(tx);
        let transaction_check = Ok(CheckedTransactionDetails {
            nonce: None,
            lamports_per_signature: LAMPORTS_PER_SIGNATURE,
        });
        txs.push(sanitized_transaction);
        txscheck.push(transaction_check);
    }

    let batch_processor = TransactionBatchProcessor::<MockForkGraph>::new_uninitialized(
        EXECUTION_SLOT,
        EXECUTION_EPOCH,
    );

    let fork_graph = Arc::new(RwLock::new(MockForkGraph {}));

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

    let result = batch_processor.load_and_execute_sanitized_transactions(
        &mock_bank,
        &txs,
        txscheck,
        &TransactionProcessingEnvironment::default(),
        &config,
    );

    println!("Batch Result {:#?}", result.processing_results);

    // RollupState(
    BorshRollupState(
        input
            .accounts
            .0
            .iter()
            .map(|(pk, _account)| {
                let account: Account = mock_bank
                        .account_shared_data
                        .read()
                        .unwrap()
                        .get(pk)
                        .unwrap()
                        .clone()
                        .into();
                (
                    *pk,
                    account.into(),
                )
            })
            .collect(),
    )
}
