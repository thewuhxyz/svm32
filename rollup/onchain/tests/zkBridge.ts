import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ZkBridge } from "../target/types/zk_bridge";
import kpSender from "../keypairSender.json";
import kpReceiver from "../keypairReceiver.json";
import { assert } from "chai";
import fs from "fs";

const initialStateHash = "EukGGeg2sN2tETkZQP4kPTQxJQU859P8j5JGNLBKSt87";
const senderKeypair = anchor.web3.Keypair.fromSecretKey(
	Uint8Array.from(Buffer.from(kpSender))
);
const receiverKeypair = anchor.web3.Keypair.fromSecretKey(
	Uint8Array.from(Buffer.from(kpReceiver))
);
const proof = new Uint8Array(fs.readFileSync("../zk/borsh/proof_borsh.bin"));

describe("zk-bridge", () => {
	// Configure the client to use the local cluster.
	const provider = anchor.AnchorProvider.env();
	anchor.setProvider(provider);

	const program = anchor.workspace.ZkBridge as Program<ZkBridge>;

	it("works end to end!", async () => {
		const platformId = anchor.web3.PublicKey.unique();
		const [platformKey, _platformBump] =
			anchor.web3.PublicKey.findProgramAddressSync(
				[Buffer.from("platform:"), platformId.toBuffer()],
				program.programId
			);
		const [rampKey, _rampBump] = anchor.web3.PublicKey.findProgramAddressSync(
			[
				Buffer.from("ramp:"),
				platformId.toBuffer(),
				senderKeypair.publicKey.toBuffer(),
			],
			program.programId
		);

		await provider.connection.confirmTransaction(
			await provider.connection.requestAirdrop(
				senderKeypair.publicKey,
				10 * anchor.web3.LAMPORTS_PER_SOL
			)
		);

		await program.methods
			.createPlatform({
				id: platformId,
				initialStateHash: Array.from(Buffer.from(initialStateHash)),
			})
			.accountsPartial({
				sequencer: senderKeypair.publicKey,
				platform: platformKey,
				systemProgram: anchor.web3.SystemProgram.programId,
			})
			.signers([senderKeypair])
			.rpc();

		await program.methods
			.addRampTx({
				isOnramp: true,
				amount: new anchor.BN(anchor.web3.LAMPORTS_PER_SOL),
			})
			.accountsPartial({
				ramper: senderKeypair.publicKey,
				ramp: rampKey,
				platform: platformKey,
				systemProgram: anchor.web3.SystemProgram.programId,
			})
			.signers([senderKeypair])
			.rpc();

		// Allocate proof account
		const proofKp = anchor.web3.Keypair.generate();
		const allocateIx = anchor.web3.SystemProgram.createAccount({
			fromPubkey: senderKeypair.publicKey,
			lamports: await provider.connection.getMinimumBalanceForRentExemption(
				proof.length
			),
			newAccountPubkey: proofKp.publicKey,
			programId: anchor.web3.SystemProgram.programId,
			space: proof.length,
		});
		const recentBlockhash = (await provider.connection.getRecentBlockhash())
			.blockhash;
		const messageV0 = new anchor.web3.TransactionMessage({
			payerKey: senderKeypair.publicKey,
			recentBlockhash,
			instructions: [allocateIx],
		}).compileToV0Message();
		const versionedTx = new anchor.web3.VersionedTransaction(messageV0);
		versionedTx.sign([senderKeypair, proofKp]);
		await provider.connection.sendTransaction(versionedTx);

		// Upload proof
		const [proofKey, _proofBump] = anchor.web3.PublicKey.findProgramAddressSync(
			[
				Buffer.from("proof:"),
				platformId.toBuffer(),
				senderKeypair.publicKey.toBuffer(),
			],
			program.programId
		);
		let dataLeft = proof;
		let offset = 0;
		while (dataLeft.length > 0) {
			console.log(`uploading`);
			const size = Math.min(dataLeft.length, 800);
			await program.methods
				.uploadProof({
					proofSize: new anchor.BN(proof.length),
					offset: new anchor.BN(offset),
					proofData: Buffer.from(dataLeft.subarray(0, offset)),
				})
				.accountsPartial({
					prover: senderKeypair.publicKey,
					proof: proofKey,
					platform: platformKey,
				})
				.signers([senderKeypair])
				.rpc();

			dataLeft = dataLeft.subarray(size);
			offset += size;
		}

		console.log("upload proof");

		await program.methods
			.prove({ proof: Buffer.from(proof) })
			.accountsPartial({
				prover: senderKeypair.publicKey,
				// proof: proofKey,
				platform: platformKey,
			})
			.preInstructions([
				anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
					units: 1_400_000,
				}),
			])
			.signers([senderKeypair])
			.rpc({ skipPreflight: false });

		// TODO: generate the corresponding proof
		// await program.methods.addRampTx().rpc();
		// await program.methods.prove().rpc();
		// await program.methods.withdraw().rpc();

		// const balance = await provider.connection.getBalance(
		//   receiverKeypair.publicKey
		// );
		// assert.equal(balance, anchor.web3.LAMPORTS_PER_SOL);
	});
});
