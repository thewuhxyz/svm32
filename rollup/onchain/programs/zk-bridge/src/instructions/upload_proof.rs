use anchor_lang::prelude::*;

use crate::state::platform::Platform;
use crate::state::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UploadProofArgs {
    pub proof_size: u64,
    pub offset: u64,
    pub proof_data: Vec<u8>,
}

#[derive(Accounts)]
#[instruction(args: UploadProofArgs)]
pub struct UploadProof<'info> {
    #[account(mut)]
    pub prover: Signer<'info>,
    #[account(
        init_if_needed,
        payer = prover,
        space = 8 + Proof::INIT_SPACE + args.proof_size as usize,
        seeds = [
            PROOF_SEED_PREFIX,
            platform.id.as_ref(),
            prover.key().as_ref(),
        ],
        bump
    )]
    pub proof: Account<'info, Proof>,
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

impl UploadProof<'_> {
    pub fn handle(ctx: Context<Self>, args: UploadProofArgs) -> Result<()> {
        let proof = &mut ctx.accounts.proof;
        if proof.bump != ctx.bumps.proof {
            proof.bump = ctx.bumps.proof;
            proof.data = vec![0; args.proof_size as usize];
        }

        let offset = args.offset as usize;
        let end = (offset + args.proof_data.len()).min(proof.data.len());
        msg!("offset: {}, end: {}", offset, end);
        proof.data[offset..end].copy_from_slice(&args.proof_data);

        Ok(())
    }
}