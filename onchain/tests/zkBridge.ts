const initialStateHash = "8VJNjgYfFD9ifndnXUKRLSonUbmEQBQvZkLWdEYDMr1c";

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ZkBridge } from "../target/types/zk_bridge";
import kpSender from "../keypairSender.json";
import kpReceiver from "../keypairReceiver.json";
import { assert } from "chai";

const senderKeypair = anchor.web3.Keypair.fromSecretKey(
  Uint8Array.from(Buffer.from(kpSender))
);
const receiverKeypair = anchor.web3.Keypair.fromSecretKey(
  Uint8Array.from(Buffer.from(kpReceiver))
);

describe("social-swap", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SocialSwap as Program<ZkBridge>;

  it("works end to end!", async () => {
    // TODO: use correct parameters
    await program.methods.createPlatform().rpc();
    await program.methods.addRampTx().rpc();
    await program.methods.prove().rpc();

    // TODO: generate the corresponding proof
    await program.methods.addRampTx().rpc();
    await program.methods.prove().rpc();
    await program.methods.withdraw().rpc();

    const balance = await provider.connection.getBalance(
      receiverKeypair.publicKey
    );
    assert.equal(balance, anchor.web3.LAMPORTS_PER_SOL);
  });
});
