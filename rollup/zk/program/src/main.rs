//! A simple program to be proven inside the zkVM.
//!
#![no_main]
sp1_zkvm::entrypoint!(main);

use borsh::BorshDeserialize;
// use svm_runner_types::{hash_state, ExecutionInput};
use svm_runner_lib::runner;
use svm_runner_types_anchor::{hash_state, BorshCommitedValues, BorshExecutionInput};

pub fn main() {
    // Read an input to the program.
    let input_bytes = sp1_zkvm::io::read_vec();

    let input = BorshExecutionInput::try_from_slice(&input_bytes).unwrap();

    let rollup_state = runner(input.clone());
    let hash = hash_state(rollup_state);

    let output = BorshCommitedValues(input, hash);
    let output_slice = borsh::to_vec(&output).unwrap();

    sp1_zkvm::io::commit_slice(&output_slice);
}
