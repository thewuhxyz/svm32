use anchor_lang::prelude::*;
use verifier::verify_proof;
// use anchor_spl::associated_token::AssociatedToken;
// use anchor_spl::token::*;

use crate::errors::PlatformError;
use crate::state::platform::Platform;
use crate::state::*;

/// Derived as follows:
///
/// ```
/// let client = sp1_sdk::ProverClient::new();
/// let (pk, vk) = client.setup(YOUR_ELF_HERE);
/// let vkey_hash = vk.bytes32();
/// ```
const ZK_BRIDGE_VKEY_HASH: &str =
    "0x008d5e2aa8fe6d5f0f9b1ad59034a47517fe5f4a5439c7db4e5cc923f783a887";

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct SP1Groth16Proof {
    pub proof: Vec<u8>,
    pub sp1_public_inputs: Vec<u8>,
}

#[event_cpi]
#[derive(Accounts)]
pub struct Prove<'info> {
    #[account(mut)]
    pub prover: Signer<'info>,
    /// CHECK: The proof is verified
    pub proof: UncheckedAccount<'info>,
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
    pub fn handle(ctx: Context<Self>) -> Result<()> {
        // unimplemented!();

        // Deserialize the SP1Groth16Proof from the instruction data.
        let groth16_proof = SP1Groth16Proof::try_from_slice(&ctx.accounts.proof.data.borrow())
            .map_err(|_| PlatformError::InvalidProof)?;

        // Get the SP1 Groth16 verification key from the `sp1-solana` crate.
        let vk = verifier::GROTH16_VK_4_0_0_RC3_BYTES;

        // Verify the proof.
        verify_proof(
            &groth16_proof.proof,
            &groth16_proof.sp1_public_inputs,
            &ZK_BRIDGE_VKEY_HASH,
            vk,
        )
        .map_err(|_| ProgramError::InvalidInstructionData)?;

        // Print out the public values.
        let mut reader = groth16_proof.sp1_public_inputs.as_slice();
        let n = u32::deserialize(&mut reader).unwrap();
        let a = u32::deserialize(&mut reader).unwrap();
        let b = u32::deserialize(&mut reader).unwrap();

        // Deserialize proof

        // Check that ramps txs match the ones in the platform

        // Empty pending ramp txs

        Ok(())
    }
}
