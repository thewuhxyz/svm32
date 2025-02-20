use std::fs::File;
use sp1_sdk::SP1ProofWithPublicValues;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct SP1Groth16Proof {
    pub proof: Vec<u8>,
    pub sp1_public_inputs: Vec<u8>,
}

fn main() -> anyhow::Result<()> {
    let sp1_proof_with_public_values = SP1ProofWithPublicValues::load("proof.bin")?;

    let grooth16_proof = SP1Groth16Proof {
        proof: sp1_proof_with_public_values.bytes(),
        sp1_public_inputs: sp1_proof_with_public_values.public_values.to_vec(),
    };

    let mut proof_borsh_file = File::create("proof_borsh.bin").expect("failed to open file");

    borsh::to_writer(&mut proof_borsh_file, &grooth16_proof)?;

    anyhow::Ok(())
}
