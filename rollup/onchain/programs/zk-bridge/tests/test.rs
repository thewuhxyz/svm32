use {
    anchor_client::{
        solana_sdk::{
            compute_budget, hash::Hash, message::Message, native_token::LAMPORTS_PER_SOL,
            pubkey::Pubkey, signature::read_keypair_file, signer::Signer, system_program,
            transaction::Transaction,
        },
        Client, Cluster,
    },
    anchor_lang::AnchorDeserialize,
    litesvm::LiteSVM,
    std::{rc::Rc, str::FromStr},
    zk_bridge::{
        accounts, instruction,
        instructions::{AddRampTxArgs, CreatePlatformArgs, UploadProofArgs},
        utils::SP1Groth16Proof,
    },
};

#[tokio::test]
async fn runs() -> anyhow::Result<()> {
    let payer = read_keypair_file("../../keypairSender.json").unwrap();
    let bytes = include_bytes!("../../../target/deploy/zk_bridge.so");
    let proof = include_bytes!("../../../../zk/script/grooth16_proof.bin");

    let initial_state_hash = Hash::from_str("Aq2kL5qUSQTAPxXyMTWWm618UvyQy3axDweYzEHM9bHa")?;
    let program_id = zk_bridge::ID;
    let mut svm = LiteSVM::new();

    let grooth16_proof = SP1Groth16Proof::try_from_slice(proof)?;
    println!("grooth 16 proof: {:#?}", &grooth16_proof);

    svm.add_program(program_id, bytes);
    svm.airdrop(&payer.pubkey(), 10 * LAMPORTS_PER_SOL).unwrap();

    // use localnet, but we never really use it. We use lite-svm connection instead
    let provider = Client::new(Cluster::Localnet, Rc::new(&payer));

    let program = provider.program(program_id)?;

    // start tests
    let platform_id = Pubkey::new_unique();
    let (platform_key, _platform_bump) =
        Pubkey::find_program_address(&[b"platform:", platform_id.as_ref()], &program_id);
    let (ramp_key, _ramp_bump) = Pubkey::find_program_address(
        &[b"ramp:", platform_id.as_ref(), payer.pubkey().as_ref()],
        &program_id,
    );
    let (proof_key, _proof_bump) = Pubkey::find_program_address(
        &[b"proof:", platform_id.as_ref(), payer.pubkey().as_ref()],
        &program_id,
    );

    // Create Platform & Add Ramp transaction
    let create_platform_ix = program
        .request()
        .accounts(accounts::CreatePlatform {
            platform: platform_key,
            sequencer: payer.pubkey(),
            system_program: system_program::ID,
        })
        .args(instruction::CreatePlatform {
            args: CreatePlatformArgs {
                id: platform_id,
                initial_state_hash: initial_state_hash.to_bytes(),
            },
        })
        .instructions()?
        .remove(0);

    let add_ramp_ix = program
        .request()
        .accounts(accounts::AddRampTx {
            platform: platform_key,
            ramp: ramp_key,
            ramper: payer.pubkey(),
            system_program: system_program::ID,
        })
        .args(instruction::AddRampTx {
            args: AddRampTxArgs {
                is_onramp: true,
                amount: LAMPORTS_PER_SOL,
            },
        })
        .instructions()?
        .remove(0);

    let tx = Transaction::new(
        &[&payer],
        Message::new(&[create_platform_ix, add_ramp_ix], Some(&payer.pubkey())),
        svm.latest_blockhash(),
    );

    let tx_metadata = svm.send_transaction(tx).unwrap();
    println!("tx logs: {:#?}", tx_metadata.logs);

    // Upload proof
    // Can be done in a single tx thanks to LiteSVM
    let upload_proof_ix = program
        .request()
        .args(instruction::UploadProof {
            args: UploadProofArgs {
                proof_size: proof.len() as u64,
                proof_data: proof.to_vec(),
                offset: 0,
            },
        })
        .accounts(accounts::UploadProof {
            platform: platform_key,
            proof: proof_key,
            prover: payer.pubkey(),
            system_program: system_program::ID,
        })
        .instructions()?
        .remove(0);
    let tx = Transaction::new(
        &[&payer],
        Message::new(&[upload_proof_ix], Some(&payer.pubkey())),
        svm.latest_blockhash(),
    );
    let tx_metadata = svm.send_transaction(tx).unwrap();
    println!("tx logs: {:#?}", tx_metadata.logs);

    // Prove
    let prove_tx = program
        .request()
        .args(instruction::Prove {})
        .accounts(accounts::Prove {
            platform: platform_key,
            proof: proof_key,
            prover: payer.pubkey(),
            system_program: system_program::ID,
        })
        .instructions()?
        .remove(0);
    let compute_budget_ix =
        compute_budget::ComputeBudgetInstruction::set_compute_unit_limit(1_400_000);
    let tx = Transaction::new(
        &[&payer],
        Message::new(&[compute_budget_ix, prove_tx], Some(&payer.pubkey())),
        svm.latest_blockhash(),
    );
    let tx_metadata = svm.send_transaction(tx).unwrap();
    println!("tx logs: {:#?}", tx_metadata.logs);

    anyhow::Ok(())
}
