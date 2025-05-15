/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/veralux.json`.
 */
export type Veralux = {
  "address": "4BzWp6JCnguTPGEfHhG12hfJR86tGGU3fJSEdfuo2nBZ",
  "metadata": {
    "name": "veralux",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "buyPresale",
      "discriminator": [
        113,
        18,
        193,
        68,
        35,
        36,
        215,
        8
      ],
      "accounts": [
        {
          "name": "buyer",
          "writable": true,
          "signer": true
        },
        {
          "name": "state",
          "writable": true
        },
        {
          "name": "presalePurchase",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  101,
                  115,
                  97,
                  108,
                  101,
                  95,
                  112,
                  117,
                  114,
                  99,
                  104,
                  97,
                  115,
                  101
                ]
              },
              {
                "kind": "account",
                "path": "buyer"
              }
            ]
          }
        },
        {
          "name": "presaleVesting",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  101,
                  115,
                  97,
                  108,
                  101,
                  95,
                  118,
                  101,
                  115,
                  116,
                  105,
                  110,
                  103
                ]
              },
              {
                "kind": "account",
                "path": "buyer"
              }
            ]
          }
        },
        {
          "name": "buyerUsdtAccount",
          "writable": true
        },
        {
          "name": "presaleUsdtAccount",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "initGlobal",
      "discriminator": [
        44,
        238,
        77,
        253,
        76,
        182,
        192,
        162
      ],
      "accounts": [
        {
          "name": "signer1",
          "writable": true,
          "signer": true
        },
        {
          "name": "signer2",
          "signer": true,
          "optional": true
        },
        {
          "name": "signer3",
          "signer": true,
          "optional": true
        },
        {
          "name": "signer4",
          "signer": true,
          "optional": true
        },
        {
          "name": "signer5",
          "signer": true,
          "optional": true
        },
        {
          "name": "state",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  116,
                  114,
                  97,
                  99,
                  116,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "treasury",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  114,
                  101,
                  97,
                  115,
                  117,
                  114,
                  121
                ]
              }
            ]
          }
        },
        {
          "name": "multisig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  117,
                  108,
                  116,
                  105,
                  115,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "migrationState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  105,
                  103,
                  114,
                  97,
                  116,
                  105,
                  111,
                  110,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
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
          "name": "ix",
          "type": {
            "defined": {
              "name": "initGlobalIx"
            }
          }
        }
      ]
    },
    {
      "name": "initPresale",
      "discriminator": [
        172,
        248,
        47,
        226,
        223,
        52,
        94,
        217
      ],
      "accounts": [
        {
          "name": "buyer",
          "writable": true,
          "signer": true
        },
        {
          "name": "presalePurchase",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  101,
                  115,
                  97,
                  108,
                  101,
                  95,
                  112,
                  117,
                  114,
                  99,
                  104,
                  97,
                  115,
                  101
                ]
              },
              {
                "kind": "account",
                "path": "buyer"
              }
            ]
          }
        },
        {
          "name": "presaleVesting",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  101,
                  115,
                  97,
                  108,
                  101,
                  95,
                  118,
                  101,
                  115,
                  116,
                  105,
                  110,
                  103
                ]
              },
              {
                "kind": "account",
                "path": "buyer"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "updateGlobal",
      "discriminator": [
        90,
        152,
        240,
        21,
        199,
        38,
        72,
        20
      ],
      "accounts": [
        {
          "name": "signer",
          "writable": true,
          "signer": true
        },
        {
          "name": "state",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  116,
                  114,
                  97,
                  99,
                  116,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
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
          "name": "ix",
          "type": {
            "defined": {
              "name": "updateGlobalIx"
            }
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "contractState",
      "discriminator": [
        190,
        138,
        10,
        223,
        189,
        116,
        222,
        115
      ]
    },
    {
      "name": "migrationState",
      "discriminator": [
        95,
        146,
        135,
        64,
        145,
        25,
        197,
        115
      ]
    },
    {
      "name": "multisig",
      "discriminator": [
        224,
        116,
        121,
        186,
        68,
        161,
        79,
        236
      ]
    },
    {
      "name": "presalePurchase",
      "discriminator": [
        34,
        110,
        127,
        240,
        83,
        219,
        152,
        227
      ]
    },
    {
      "name": "presaleVesting",
      "discriminator": [
        97,
        146,
        20,
        99,
        2,
        184,
        221,
        191
      ]
    },
    {
      "name": "treasury",
      "discriminator": [
        238,
        239,
        123,
        238,
        89,
        1,
        168,
        253
      ]
    }
  ],
  "events": [
    {
      "name": "initializeEvent",
      "discriminator": [
        206,
        175,
        169,
        208,
        241,
        210,
        35,
        221
      ]
    },
    {
      "name": "presalePurchaseEvent",
      "discriminator": [
        30,
        12,
        195,
        203,
        44,
        187,
        164,
        135
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "paused",
      "msg": "Contract is paused"
    },
    {
      "code": 6001,
      "name": "notPaused",
      "msg": "Contract is not paused"
    },
    {
      "code": 6002,
      "name": "unauthorizedSender",
      "msg": "Unauthorized: Sender does not own the token account"
    },
    {
      "code": 6003,
      "name": "insufficientSigners",
      "msg": "Unauthorized: Insufficient signers for multisig operation"
    },
    {
      "code": 6004,
      "name": "signerNotOwner",
      "msg": "Unauthorized: Signer is not a multisig owner"
    },
    {
      "code": 6005,
      "name": "reentrancyGuardTriggered",
      "msg": "Reentrancy guard triggered: Operation already in progress"
    },
    {
      "code": 6006,
      "name": "insufficientStakingPoolFunds",
      "msg": "Insufficient funds in staking pool"
    },
    {
      "code": 6007,
      "name": "insufficientAirdropFunds",
      "msg": "Insufficient funds in airdrop pool"
    },
    {
      "code": 6008,
      "name": "insufficientLiquidityIncentiveFunds",
      "msg": "Insufficient funds in liquidity incentive pool"
    },
    {
      "code": 6009,
      "name": "insufficientStakedAmount",
      "msg": "Insufficient staked amount for operation"
    },
    {
      "code": 6010,
      "name": "noLockedTokens",
      "msg": "No locked tokens available"
    },
    {
      "code": 6011,
      "name": "maxSellTxnLimitExceeded",
      "msg": "Maximum sell transaction limit exceeded"
    },
    {
      "code": 6012,
      "name": "dailySellLimitExceeded",
      "msg": "Daily sell limit exceeded"
    },
    {
      "code": 6013,
      "name": "maxTransferLimitExceeded",
      "msg": "Maximum transfer limit exceeded"
    },
    {
      "code": 6014,
      "name": "dailyTransferLimitExceeded",
      "msg": "Daily transfer limit exceeded"
    },
    {
      "code": 6015,
      "name": "cooldownActive",
      "msg": "Transfer cooldown active"
    },
    {
      "code": 6016,
      "name": "vestingNotStarted",
      "msg": "Vesting period has not started"
    },
    {
      "code": 6017,
      "name": "noRewards",
      "msg": "No rewards available"
    },
    {
      "code": 6018,
      "name": "proposalExpired",
      "msg": "Proposal has expired"
    },
    {
      "code": 6019,
      "name": "votingPeriodNotEnded",
      "msg": "Voting period has not ended"
    },
    {
      "code": 6020,
      "name": "noticePeriodNotMet",
      "msg": "Notice period for proposal execution not met"
    },
    {
      "code": 6021,
      "name": "proposalAlreadyExecuted",
      "msg": "Proposal has already been executed"
    },
    {
      "code": 6022,
      "name": "withdrawalDelayNotMet",
      "msg": "Withdrawal delay not met"
    },
    {
      "code": 6023,
      "name": "lockPeriodNotMet",
      "msg": "Lock period not met"
    },
    {
      "code": 6024,
      "name": "invalidProposalType",
      "msg": "Invalid proposal type"
    },
    {
      "code": 6025,
      "name": "invalidProposalValueCount",
      "msg": "Invalid proposal value count"
    },
    {
      "code": 6026,
      "name": "invalidTaxRate",
      "msg": "Invalid tax rate"
    },
    {
      "code": 6027,
      "name": "invalidStakingTiers",
      "msg": "Invalid staking tiers"
    },
    {
      "code": 6028,
      "name": "invalidTaxAllocationTotal",
      "msg": "Invalid tax allocation total"
    },
    {
      "code": 6029,
      "name": "invalidReductionThresholds",
      "msg": "Invalid reduction thresholds"
    },
    {
      "code": 6030,
      "name": "invalidReductionFactor",
      "msg": "Invalid reduction factor"
    },
    {
      "code": 6031,
      "name": "invalidSellLimit",
      "msg": "Invalid sell limit"
    },
    {
      "code": 6032,
      "name": "invalidTransferLimit",
      "msg": "Invalid transfer limit"
    },
    {
      "code": 6033,
      "name": "invalidTaxThreshold",
      "msg": "Invalid tax threshold"
    },
    {
      "code": 6034,
      "name": "invalidStakingReward",
      "msg": "Invalid staking reward"
    },
    {
      "code": 6035,
      "name": "invalidAccounts",
      "msg": "Invalid accounts provided"
    },
    {
      "code": 6036,
      "name": "arithmeticOverflow",
      "msg": "Arithmetic overflow occurred"
    },
    {
      "code": 6037,
      "name": "vectorOverflow",
      "msg": "Vector overflow: Too many elements"
    },
    {
      "code": 6038,
      "name": "uninitializedAccount",
      "msg": "Account not initialized"
    },
    {
      "code": 6039,
      "name": "invalidDexProgram",
      "msg": "Provided DEX program ID is not executable"
    },
    {
      "code": 6040,
      "name": "migrationNotActive",
      "msg": "Migration is not active"
    },
    {
      "code": 6041,
      "name": "migrationActive",
      "msg": "Migration is active"
    },
    {
      "code": 6042,
      "name": "presaleNotActive",
      "msg": "Presale is not active"
    },
    {
      "code": 6043,
      "name": "presaleSupplyExceeded",
      "msg": "Presale supply exceeded"
    },
    {
      "code": 6044,
      "name": "presaleMaxPerWalletExceeded",
      "msg": "Presale maximum per wallet exceeded"
    },
    {
      "code": 6045,
      "name": "amountTooSmallAfterTax",
      "msg": "Amount too small after tax"
    },
    {
      "code": 6046,
      "name": "descriptionTooLong",
      "msg": "Description too long"
    },
    {
      "code": 6047,
      "name": "tooManyProposalValues",
      "msg": "Too many proposal values"
    },
    {
      "code": 6048,
      "name": "insufficientTierForVoting",
      "msg": "Insufficient tier for voting"
    },
    {
      "code": 6049,
      "name": "invalidImmediateAmount",
      "msg": "Invalid immediate amount"
    },
    {
      "code": 6050,
      "name": "invalidTeamMember",
      "msg": "Invalid team member"
    },
    {
      "code": 6051,
      "name": "vestingCanceled",
      "msg": "Vesting canceled"
    },
    {
      "code": 6052,
      "name": "claimCooldownNotMet",
      "msg": "Claim cooldown not met"
    },
    {
      "code": 6053,
      "name": "exceedsVestingTotal",
      "msg": "Exceeds vesting total"
    },
    {
      "code": 6054,
      "name": "tooFewOwners",
      "msg": "Too few owners in multisig"
    },
    {
      "code": 6055,
      "name": "duplicateOwners",
      "msg": "Duplicate owners in multisig"
    },
    {
      "code": 6056,
      "name": "tooManyOwners",
      "msg": "Too many owners in multisig"
    },
    {
      "code": 6057,
      "name": "thresholdExceedsOwners",
      "msg": "Threshold exceeds number of owners"
    },
    {
      "code": 6058,
      "name": "tooManyRecipients",
      "msg": "Too many recipients"
    },
    {
      "code": 6059,
      "name": "invalidRecipientAccounts",
      "msg": "Invalid recipient accounts"
    },
    {
      "code": 6060,
      "name": "invalidRecipientAccount",
      "msg": "Invalid recipient account"
    },
    {
      "code": 6061,
      "name": "invalidContract",
      "msg": "Invalid contract"
    },
    {
      "code": 6062,
      "name": "tooManyWhitelistedContracts",
      "msg": "Too many whitelisted contracts"
    },
    {
      "code": 6063,
      "name": "contractNotWhitelisted",
      "msg": "Contract not whitelisted"
    },
    {
      "code": 6064,
      "name": "alreadyMigrated",
      "msg": "Already migrated"
    },
    {
      "code": 6065,
      "name": "invalidMigrationUser",
      "msg": "Invalid migration user"
    },
    {
      "code": 6066,
      "name": "migrationToggleCooldown",
      "msg": "Migration toggle cooldown active"
    },
    {
      "code": 6067,
      "name": "batchSizeTooLarge",
      "msg": "Batch size too large"
    },
    {
      "code": 6068,
      "name": "whitelistDelayNotMet",
      "msg": "Whitelist delay not met"
    },
    {
      "code": 6069,
      "name": "timeLockNotMet",
      "msg": "Time lock requirement not met"
    },
    {
      "code": 6070,
      "name": "callerNotWhitelisted",
      "msg": "Caller is not whitelisted"
    },
    {
      "code": 6071,
      "name": "invalidDestination",
      "msg": "Invalid transfer destination"
    },
    {
      "code": 6072,
      "name": "versionMismatch",
      "msg": "Version mismatch in whitelisted contract"
    },
    {
      "code": 6073,
      "name": "kycRequired",
      "msg": "KYC verification required for this purchase"
    },
    {
      "code": 6074,
      "name": "invalidThreshold",
      "msg": "Invalid threshold"
    },
    {
      "code": 6075,
      "name": "invalidVectorSize",
      "msg": "Invalid vector size"
    },
    {
      "code": 6076,
      "name": "invalidProposal",
      "msg": "Invalid proposal"
    },
    {
      "code": 6077,
      "name": "invalidBuyerUsdtOwner",
      "msg": "Invalid buyer USDT owner"
    },
    {
      "code": 6078,
      "name": "invalidPresaleUsdtReceiver",
      "msg": "Invalid presale USDT receiver"
    }
  ],
  "types": [
    {
      "name": "contractState",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "admin",
            "type": "pubkey"
          },
          {
            "name": "treasury",
            "type": "pubkey"
          },
          {
            "name": "charityWallet",
            "type": "pubkey"
          },
          {
            "name": "teamWallet",
            "type": "pubkey"
          },
          {
            "name": "liquidityPool",
            "type": "pubkey"
          },
          {
            "name": "presaleUsdtReceiver",
            "type": "pubkey"
          },
          {
            "name": "proposalCount",
            "type": "u64"
          },
          {
            "name": "totalVotingPower",
            "type": "u64"
          },
          {
            "name": "launchTimestamp",
            "type": "i64"
          },
          {
            "name": "taxRate",
            "type": "u64"
          },
          {
            "name": "progressiveTaxThreshold",
            "type": "u64"
          },
          {
            "name": "totalPresaleSold",
            "type": "u64"
          },
          {
            "name": "dexPrograms",
            "type": {
              "vec": "pubkey"
            }
          },
          {
            "name": "pauseReason",
            "type": "string"
          },
          {
            "name": "isProcessing",
            "type": "bool"
          },
          {
            "name": "presaleActive",
            "type": "bool"
          },
          {
            "name": "paused",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "initGlobalIx",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "charityWallet",
            "type": "pubkey"
          },
          {
            "name": "teamWallet",
            "type": "pubkey"
          },
          {
            "name": "liquidityPool",
            "type": "pubkey"
          },
          {
            "name": "launchTimestamp",
            "type": "i64"
          },
          {
            "name": "presaleUsdtReceiver",
            "type": "pubkey"
          },
          {
            "name": "initialOwners",
            "type": {
              "vec": "pubkey"
            }
          },
          {
            "name": "initialDexPrograms",
            "type": {
              "vec": "pubkey"
            }
          },
          {
            "name": "threshold",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "initializeEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "launchTimestamp",
            "type": "i64"
          },
          {
            "name": "initialOwners",
            "type": {
              "vec": "pubkey"
            }
          },
          {
            "name": "threshold",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "migrationState",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "totalLocked",
            "type": "u64"
          },
          {
            "name": "migrationToggleTimestamp",
            "type": "i64"
          },
          {
            "name": "migrationActive",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "multisig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owners",
            "type": {
              "vec": "pubkey"
            }
          },
          {
            "name": "threshold",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "presalePurchase",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "wallet",
            "type": "pubkey"
          },
          {
            "name": "totalPurchased",
            "type": "u64"
          },
          {
            "name": "kycVerified",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "presalePurchaseEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "buyer",
            "type": "pubkey"
          },
          {
            "name": "usdtAmount",
            "type": "u64"
          },
          {
            "name": "tokenAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "presaleVesting",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "totalAmount",
            "type": "u64"
          },
          {
            "name": "claimedAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "treasury",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "stakingPool",
            "type": "u64"
          },
          {
            "name": "airdropPool",
            "type": "u64"
          },
          {
            "name": "governanceReserve",
            "type": "u64"
          },
          {
            "name": "marketingFund",
            "type": "u64"
          },
          {
            "name": "emergencyFund",
            "type": "u64"
          },
          {
            "name": "liquidityIncentive",
            "type": "u64"
          },
          {
            "name": "teamPool",
            "type": "u64"
          },
          {
            "name": "launchTimestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "updateGlobalIx",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "presaleUsdtReceiver",
            "type": "pubkey"
          },
          {
            "name": "launchTimestamp",
            "type": "i64"
          },
          {
            "name": "teamWallet",
            "type": "pubkey"
          },
          {
            "name": "charityWallet",
            "type": "pubkey"
          }
        ]
      }
    }
  ]
};
