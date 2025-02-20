use anchor_lang::prelude::*;
// use svm_runner_types::CommittedValues;
use bincode::config::Options;
use svm_runner_types_anchor::CommittedValues;
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
    "0x00b5f4f8596951753342637e0ab298e2072459a9aa8ad51116290b32d9206a55";

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct SP1Groth16Proof {
    pub proof: Vec<u8>,
    pub sp1_public_inputs: Vec<u8>,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct ProofArgs {
    pub proof: Vec<u8>,
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

        // let groth16_proof = SP1Groth16Proof::try_from_slice(&ctx.accounts.proof.data)
        let groth16_proof = Box::new(
            SP1Groth16Proof::try_from_slice(&proof.proof)
                .map_err(|_| PlatformError::InvalidProofData)?,
        );

        // let vk = verifier::GROTH16_VK_4_0_0_RC3_BYTES;
        verify_proof(
            &groth16_proof.proof,
            &groth16_proof.sp1_public_inputs,
            ZK_BRIDGE_VKEY_HASH,
            verifier::GROTH16_VK_4_0_0_RC3_BYTES,
        )
        .map_err(|_| PlatformError::InvalidProof)?;

        msg!("Out of memory?");

        let values: CommittedValues = bincode::options()
            .with_limit(1000)
            .with_fixint_encoding() // As per https://github.com/servo/bincode/issues/333, these two options are needed
            .allow_trailing_bytes() // to retain the behavior of bincode::deserialize with the new `options()` method
            .deserialize_from(groth16_proof.sp1_public_inputs.as_slice())
            .map_err(|_| PlatformError::DeserializationError)?;

        // let values = solana_bincode::limited_deserialize::<CommittedValues>(groth16_proof.sp1_public_inputs.as_slice(), 1000).unwrap();

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
        ctx.accounts.platform.last_state_hash = values.1.to_bytes();

        Ok(())
    }
}
