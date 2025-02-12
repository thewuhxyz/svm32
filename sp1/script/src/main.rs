use sp1_sdk::{include_elf, ProverClient, SP1ProofWithPublicValues, SP1Stdin};

const ELF: &[u8] = include_elf!("chess-program");

fn main() {
    let stdin = SP1Stdin::new();

    let client = ProverClient::from_env();
    let (values, result) = client.execute(ELF, &stdin).run().unwrap();
    
    println!("result: {}", result);
    println!("values: {:?}", values);

    println!("successfully generated and verified proof for the program!")
}
