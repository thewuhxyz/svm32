import * as borsh from "borsh";

export type Proof = {
	proof: Uint8Array;
	public_input: Uint8Array;
};

// // Define the schema for the proof data
// export const proofSchema: borsh.Schema = {
// 	struct: {
// 		proof: { array: { type: "u8" } },
// 		publicInput: { array: { type: "u8" } },
// 	},
// };

// export const proofSchema: borsh.Schema = {
// 	struct: {
// 		proof: { array: { type: "u8" } },
// 		publicInput: { struct: {
//       tuple: [],
//     } },
// 	},
// };
