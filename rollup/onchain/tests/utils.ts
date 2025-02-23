import * as borsh from "borsh";
import * as anchor from "@coral-xyz/anchor"

export type Proof = {
	proof: number[];
	publicInput: {
		input: {
			rollupAccounts: {
				states: {
					pubkey: anchor.web3.PublicKey;
					account: {
						lamports: anchor.BN;
						data: number[];
						owner: anchor.web3.PublicKey;
						executable: boolean;
						rentEpoch: anchor.BN;
					};
				}[];
			};
			txs: number[];
			rampTxs: {
				isOnramp: boolean;
				user: anchor.web3.PublicKey;
				amount: anchor.BN;
			}[];
		};
		output: number[];
	};
};

export type ProofSchema = {
	proof: Uint8Array;
	publicInput: {
		input: {
			rollupAccounts: {
				states: {
					pubkey: Uint8Array;
					account: {
						lamports: BigInt;
						data: Uint8Array;
						owner: Uint8Array;
						executable: boolean;
						rentEpoch: bigint;
					};
				}[];
			};
			txs: Uint8Array;
			rampTxs: {
				isOnramp: boolean;
				user: Uint8Array;
				amount: bigint;
			}[];
		};
		output: Uint8Array;
	};
};

export const proofSchema: borsh.Schema = {
	struct: {
		proof: { array: { type: "u8" } },
		publicInput: {
			struct: {
				input: {
					struct: {
						rollupAcounts: {
							struct: {
								states: {
									array: {
										type: {
											struct: {
												pubkey: { array: { type: "u8", len: 32 } },
												account: {
													struct: {
														lamports: "u64",
														data: { array: { type: "u8" } },
														owner: { array: { type: "u8", len: 32 } },
														executable: "bool",
														rentEpoch: "u64",
													},
												},
											},
										},
									},
								},
							},
						},
						txs: { array: { type: "u8" } },
						rampTxs: {
							array: {
								type: {
									struct: {
										isOnramp: "bool",
										user: { array: { type: "u8", len: 32 } },
										amount: "u64",
									},
								},
							},
						},
					},
				},
				output: { array: { type: "u8", len: 32 } },
			},
		},
	},
};
