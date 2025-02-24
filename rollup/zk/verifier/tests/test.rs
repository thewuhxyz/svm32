use borsh::BorshDeserialize;
use svm_runner_types::SP1Groth16Proof;
use verifier::verify_proof;

const ZK_BRIDGE_VKEY_HASH: &str =
    "0x004cd8a01c6575b6d58e193d1a8fee5917a96b6e2162ec60163dc7686b2811cb";

#[test]
fn prove() -> Result<(), Box<dyn std::error::Error>> {
    let proof = include_bytes!("../../script/grooth16_proof.bin");

    let grooth16_proof = SP1Groth16Proof::try_from_slice(proof).unwrap();
    println!("grooth 16 proof: {:#?}", &grooth16_proof);

    verify_proof(
        &grooth16_proof.proof,
        &borsh::to_vec(&grooth16_proof.sp1_public_inputs)?,
        ZK_BRIDGE_VKEY_HASH,
        verifier::GROTH16_VK_4_0_0_RC3_BYTES,
    )?;

    Ok(())
}
