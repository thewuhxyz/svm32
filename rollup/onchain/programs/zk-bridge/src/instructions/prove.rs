use crate::errors::PlatformError;
use crate::state::platform::Platform;
use crate::state::*;
use crate::utils::SP1Groth16Proof;
use anchor_lang::prelude::*;
use verifier::verify_proof;

/// Derived as follows:
///
/// ```
/// let client = sp1_sdk::ProverClient::new();
/// let (pk, vk) = client.setup(YOUR_ELF_HERE);
/// let vkey_hash = vk.bytes32();
/// ```
const ZK_BRIDGE_VKEY_HASH: &str =
    "0x004cd8a01c6575b6d58e193d1a8fee5917a96b6e2162ec60163dc7686b2811cb";
// "0x0064e652e64b5b61fc6090d231016260c1ff2ba3746bd7661356a0d780fa0162";
// "0x0039a2ea684d5ebf650341d23f14c448a552e72793827ffe9c54aca424224761";
// "0x00b5f4f8596951753342637e0ab298e2072459a9aa8ad51116290b32d9206a55";

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
    pub fn handle(ctx: Context<Self>, proof: SP1Groth16Proof) -> Result<()> {
        // Taking data from an account because it's too big to fit in an instruction
        let commited_values = &proof.sp1_public_inputs;

        verify_proof(
            &proof.proof,
            &commited_values.try_to_vec()?,
            ZK_BRIDGE_VKEY_HASH,
            verifier::GROTH16_VK_4_0_0_RC3_BYTES,
        )
        .map_err(|_| PlatformError::InvalidProof)?;

        // Check that ramps txs match the ones in the platform
        // Currently only check the count, could be improved to a hash of all txs
        if commited_values.input.ramp_txs.len() != ctx.accounts.platform.ramp_txs.len() {
            return Err(PlatformError::MissingRampTxs.into());
        }

        // Empty pending ramp txs
        ctx.accounts.platform.ramp_txs = vec![];

        // This can currently brick the platform, there should be a limit in number of ramp txs
        for ramp_tx in commited_values
            .input
            .ramp_txs
            .iter()
            .filter(|ramp_tx| !ramp_tx.is_onramp)
        {
            ctx.accounts.platform.withdraw += ramp_tx.amount;
        }

        // Update the platform state
        ctx.accounts.platform.last_state_hash = commited_values.output;

        Ok(())
    }
}
