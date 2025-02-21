use anchor_lang::prelude::*;
use verifier::verify_proof;
use crate::errors::PlatformError;
use crate::state::platform::Platform;
use crate::state::*;
use crate::utils::{BorshCommitedValues, SP1Groth16Proof};

/// Derived as follows:
///
/// ```
/// let client = sp1_sdk::ProverClient::new();
/// let (pk, vk) = client.setup(YOUR_ELF_HERE);
/// let vkey_hash = vk.bytes32();
/// ```
const ZK_BRIDGE_VKEY_HASH: &str =
    "0x00b5f4f8596951753342637e0ab298e2072459a9aa8ad51116290b32d9206a55";

// #[derive(AnchorDeserialize, AnchorSerialize)]
// pub struct SP1Groth16Proof {
//     pub proof: Vec<u8>,
//     pub sp1_public_inputs: Vec<u8>,
// }

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct ProofArgs {
    pub proof: Vec<u8>,
    pub public_input: Vec<u8>,
    // pub public_input: BorshCommitedValues,
}

#[derive(Accounts)]
pub struct Prove<'info> {
    #[account(mut)]
    pub prover: Signer<'info>,
    /// CHECK: The proof is verified
    // pub proof: UncheckedAccount<'info>,
    // pub proof: Account<'info, Proof>,
    #[account(
        mut,
        seeds = [
            PLATFORM_SEED_PREFIX,
            platform.id.as_ref(),
        ],
        bump
    )]
    pub platform: Account<'info, Platform>,
    pub system_program: Program<'info, System>,
}

impl Prove<'_> {
    pub fn handle(ctx: Context<Self>, proof: ProofArgs) -> Result<()> {
        // unimplemented!();

        // Taking data from an account because it's too big to fit in an instruction.

        msg!("Out of memory?");

        // let groth16_proof = SP1Groth16Proof::try_from_slice(&ctx.accounts.proof.data)
        // let groth16_proof = SP1Groth16Proof::try_from_slice(&proof.proof)?;

        msg!("Out of memory?");

        // let vk = verifier::GROTH16_VK_4_0_0_RC3_BYTES;
        // let _proof = groth16_proof.proof;
        let _proof = &proof.proof;
        // let public_input = groth16_proof.sp1_public_inputs.clone();
        // let _public_input = groth16_proof.sp1_public_inputs;
        let _public_input = &proof.public_input;
        verify_proof(
            _proof,
            // &_public_input.clone(),
            _public_input,
            ZK_BRIDGE_VKEY_HASH,
            verifier::GROTH16_VK_4_0_0_RC3_BYTES,
        )
        .map_err(|_| PlatformError::InvalidProof)?;

        msg!("Out of memory?");

        // msg!("public input len: {:#?}", &proof.public_input.len());

        // let values = groth16_proof.sp1_public_inputs;
        let values = BorshCommitedValues::try_from_slice(&proof.public_input)?;
        // let values = _public_input;

        msg!("Not out of memory!");

        // Check that ramps txs match the ones in the platform
        // Currently only check the count, could be improved to a hash of all txs
        if values.0.ramp_txs.len() != ctx.accounts.platform.ramp_txs.len() {
            return Err(PlatformError::MissingRampTxs.into());
        }

        // Empty pending ramp txs
        ctx.accounts.platform.ramp_txs = vec![];

        // This can currently brick the platform, there should be a limit in number of ramp txs
        for ramp_tx in values
            .0
            .ramp_txs
            .iter()
            .filter(|ramp_tx| !ramp_tx.is_onramp)
        {
            ctx.accounts.platform.withdraw += ramp_tx.amount;
        }

        // Update the platform state
        ctx.accounts.platform.last_state_hash = values.1;

        Ok(())
    }
}
