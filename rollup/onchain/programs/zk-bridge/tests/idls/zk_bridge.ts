/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/zk_bridge.json`.
 */
export type ZkBridge = {
  "address": "CfV42peE578MVCEFiycmeUQNCzfzmMcUcRq4ngrXkayn",
  "metadata": {
    "name": "zkBridge",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "addRampTx",
      "docs": [
        "Add a ramp transaction to the platform.",
        "",
        "**This can currently be used to DoS the platform by adding transactions faster than the sequencer can generate proofs.**"
      ],
      "discriminator": [
        49,
        251,
        151,
        190,
        198,
        165,
        91,
        139
      ],
      "accounts": [
        {
          "name": "ramper",
          "writable": true,
          "signer": true
        },
        {
          "name": "platform",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  116,
                  102,
                  111,
                  114,
                  109,
                  58
                ]
              },
              {
                "kind": "account",
                "path": "platform.id",
                "account": "platform"
              }
            ]
          }
        },
        {
          "name": "ramp",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  97,
                  109,
                  112,
                  58
                ]
              },
              {
                "kind": "account",
                "path": "platform.id",
                "account": "platform"
              },
              {
                "kind": "account",
                "path": "ramper"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": {
              "name": "addRampTxArgs"
            }
          }
        }
      ]
    },
    {
      "name": "createPlatform",
      "discriminator": [
        159,
        106,
        44,
        241,
        53,
        188,
        123,
        238
      ],
      "accounts": [
        {
          "name": "sequencer",
          "writable": true,
          "signer": true
        },
        {
          "name": "platform",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  116,
                  102,
                  111,
                  114,
                  109,
                  58
                ]
              },
              {
                "kind": "arg",
                "path": "args.id"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": {
              "name": "createPlatformArgs"
            }
          }
        }
      ]
    },
    {
      "name": "prove",
      "discriminator": [
        52,
        246,
        26,
        161,
        211,
        170,
        86,
        215
      ],
      "accounts": [
        {
          "name": "prover",
          "writable": true,
          "signer": true
        },
        {
          "name": "platform",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  116,
                  102,
                  111,
                  114,
                  109,
                  58
                ]
              },
              {
                "kind": "account",
                "path": "platform.id",
                "account": "platform"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "proof",
          "type": {
            "defined": {
              "name": "proofArgs"
            }
          }
        }
      ]
    },
    {
      "name": "uploadProof",
      "discriminator": [
        57,
        235,
        171,
        213,
        237,
        91,
        79,
        2
      ],
      "accounts": [
        {
          "name": "prover",
          "writable": true,
          "signer": true
        },
        {
          "name": "proof",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  111,
                  111,
                  102,
                  58
                ]
              },
              {
                "kind": "account",
                "path": "platform.id",
                "account": "platform"
              },
              {
                "kind": "account",
                "path": "prover"
              }
            ]
          }
        },
        {
          "name": "platform",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  116,
                  102,
                  111,
                  114,
                  109,
                  58
                ]
              },
              {
                "kind": "account",
                "path": "platform.id",
                "account": "platform"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": {
              "name": "uploadProofArgs"
            }
          }
        }
      ]
    },
    {
      "name": "withdraw",
      "discriminator": [
        183,
        18,
        70,
        156,
        148,
        109,
        161,
        34
      ],
      "accounts": [
        {
          "name": "ramper",
          "writable": true,
          "signer": true
        },
        {
          "name": "platform",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  116,
                  102,
                  111,
                  114,
                  109,
                  58
                ]
              },
              {
                "kind": "account",
                "path": "platform.id",
                "account": "platform"
              }
            ]
          }
        },
        {
          "name": "ramp",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  97,
                  109,
                  112,
                  58
                ]
              },
              {
                "kind": "account",
                "path": "platform.id",
                "account": "platform"
              },
              {
                "kind": "account",
                "path": "ramper"
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": {
              "name": "withdrawArgs"
            }
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "platform",
      "discriminator": [
        77,
        92,
        204,
        58,
        187,
        98,
        91,
        12
      ]
    },
    {
      "name": "proof",
      "discriminator": [
        163,
        35,
        13,
        71,
        15,
        128,
        63,
        82
      ]
    },
    {
      "name": "ramp",
      "discriminator": [
        11,
        37,
        46,
        133,
        8,
        234,
        246,
        138
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "insufficientDeposits",
      "msg": "Insufficient deposits"
    },
    {
      "code": 6001,
      "name": "invalidStateHash",
      "msg": "Invalid state hash"
    },
    {
      "code": 6002,
      "name": "invalidProofData",
      "msg": "Invalid proof data"
    },
    {
      "code": 6003,
      "name": "invalidProof",
      "msg": "Invalid proof"
    },
    {
      "code": 6004,
      "name": "missingRampTxs",
      "msg": "Missing ramp txs"
    },
    {
      "code": 6005,
      "name": "outOfMemory",
      "msg": "Out of Memory"
    },
    {
      "code": 6006,
      "name": "deserializationError",
      "msg": "Deserialization Error"
    }
  ],
  "types": [
    {
      "name": "addRampTxArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "isOnramp",
            "type": "bool"
          },
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "createPlatformArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "pubkey"
          },
          {
            "name": "initialStateHash",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          }
        ]
      }
    },
    {
      "name": "platform",
      "docs": [
        "A platform is the account storing state waiting to be sent to the rollup"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "sequencer",
            "type": "pubkey"
          },
          {
            "name": "id",
            "type": "pubkey"
          },
          {
            "name": "lastStateHash",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "rampTxs",
            "type": {
              "vec": {
                "defined": {
                  "name": "rampTx"
                }
              }
            }
          },
          {
            "name": "deposit",
            "type": "u64"
          },
          {
            "name": "withdraw",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "proof",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "data",
            "type": "bytes"
          }
        ]
      }
    },
    {
      "name": "proofArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proof",
            "type": "bytes"
          }
        ]
      }
    },
    {
      "name": "ramp",
      "docs": [
        "A platform is the account storing state waiting to be sent to the rollup"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "ramper",
            "type": "pubkey"
          },
          {
            "name": "currentStateHash",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "pendingWithdraw",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "rampTx",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "isOnramp",
            "type": "bool"
          },
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "uploadProofArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proofSize",
            "type": "u64"
          },
          {
            "name": "offset",
            "type": "u64"
          },
          {
            "name": "proofData",
            "type": "bytes"
          }
        ]
      }
    },
    {
      "name": "withdrawArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    }
  ]
};
