//! A simple program to be proven inside the zkVM.
//!
#![no_main]
sp1_zkvm::entrypoint!(main);

use runner_types::ExecutionInput;
use svm_runner_lib::runner;

pub fn main() {
    // Read an input to the program.
    let input_bytes = sp1_zkvm::io::read::<Vec<u8>>();

    let input: ExecutionInput = bincode::deserialize(&input_bytes).unwrap();

    let output = runner(input);

    // Commit to the input and output
    sp1_zkvm::io::commit(&input_bytes);
    sp1_zkvm::io::commit(&output);
}
