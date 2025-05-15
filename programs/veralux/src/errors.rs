use anchor_lang::prelude::*;

#[error_code]
pub enum VeraluxErrorCode {
    #[msg("Contract is paused")]
    Paused,
    #[msg("Contract is not paused")]
    NotPaused,
    #[msg("Unauthorized: Sender does not own the token account")]
    UnauthorizedSender,
    #[msg("Unauthorized: Insufficient signers for multisig operation")]
    InsufficientSigners,
    #[msg("Unauthorized: Signer is not a multisig owner")]
    SignerNotOwner,
    #[msg("Reentrancy guard triggered: Operation already in progress")]
    ReentrancyGuardTriggered,
    #[msg("Insufficient funds in staking pool")]
    InsufficientStakingPoolFunds,
    #[msg("Insufficient funds in airdrop pool")]
    InsufficientAirdropFunds,
    #[msg("Insufficient funds in liquidity incentive pool")]
    InsufficientLiquidityIncentiveFunds,
    #[msg("Insufficient staked amount for operation")]
    InsufficientStakedAmount,
    #[msg("No locked tokens available")]
    NoLockedTokens,
    #[msg("Maximum sell transaction limit exceeded")]
    MaxSellTxnLimitExceeded,
    #[msg("Daily sell limit exceeded")]
    DailySellLimitExceeded,
    #[msg("Maximum transfer limit exceeded")]
    MaxTransferLimitExceeded,
    #[msg("Daily transfer limit exceeded")]
    DailyTransferLimitExceeded,
    #[msg("Transfer cooldown active")]
    CooldownActive,
    #[msg("Vesting period has not started")]
    VestingNotStarted,
    #[msg("No rewards available")]
    NoRewards,
    #[msg("Proposal has expired")]
    ProposalExpired,
    #[msg("Voting period has not ended")]
    VotingPeriodNotEnded,
    #[msg("Notice period for proposal execution not met")]
    NoticePeriodNotMet,
    #[msg("Proposal has already been executed")]
    ProposalAlreadyExecuted,
    #[msg("Withdrawal delay not met")]
    WithdrawalDelayNotMet,
    #[msg("Lock period not met")]
    LockPeriodNotMet,
    #[msg("Invalid proposal type")]
    InvalidProposalType,
    #[msg("Invalid proposal value count")]
    InvalidProposalValueCount,
    #[msg("Invalid tax rate")]
    InvalidTaxRate,
    #[msg("Invalid staking tiers")]
    InvalidStakingTiers,
    #[msg("Invalid tax allocation total")]
    InvalidTaxAllocationTotal,
    #[msg("Invalid reduction thresholds")]
    InvalidReductionThresholds,
    #[msg("Invalid reduction factor")]
    InvalidReductionFactor,
    #[msg("Invalid sell limit")]
    InvalidSellLimit,
    #[msg("Invalid transfer limit")]
    InvalidTransferLimit,
    #[msg("Invalid tax threshold")]
    InvalidTaxThreshold,
    #[msg("Invalid staking reward")]
    InvalidStakingReward,
    #[msg("Invalid accounts provided")]
    InvalidAccounts,
    #[msg("Arithmetic overflow occurred")]
    ArithmeticOverflow,
    #[msg("Vector overflow: Too many elements")]
    VectorOverflow,
    #[msg("Account not initialized")]
    UninitializedAccount,
    #[msg("Provided DEX program ID is not executable")]
    InvalidDexProgram,
    #[msg("Migration is not active")]
    MigrationNotActive,
    #[msg("Migration is active")]
    MigrationActive,
    #[msg("Presale is not active")]
    PresaleNotActive,
    #[msg("Presale supply exceeded")]
    PresaleSupplyExceeded,
    #[msg("Presale maximum per wallet exceeded")]
    PresaleMaxPerWalletExceeded,
    #[msg("Amount too small after tax")]
    AmountTooSmallAfterTax,
    #[msg("Description too long")]
    DescriptionTooLong,
    #[msg("Too many proposal values")]
    TooManyProposalValues,
    #[msg("Insufficient tier for voting")]
    InsufficientTierForVoting,
    #[msg("Invalid immediate amount")]
    InvalidImmediateAmount,
    #[msg("Invalid team member")]
    InvalidTeamMember,
    #[msg("Vesting canceled")]
    VestingCanceled,
    #[msg("Claim cooldown not met")]
    ClaimCooldownNotMet,
    #[msg("Exceeds vesting total")]
    ExceedsVestingTotal,
    #[msg("Too few owners in multisig")]
    TooFewOwners,
    #[msg("Duplicate owners in multisig")]
    DuplicateOwners,
    #[msg("Too many owners in multisig")]
    TooManyOwners,
    #[msg("Threshold exceeds number of owners")]
    ThresholdExceedsOwners,
    #[msg("Too many recipients")]
    TooManyRecipients,
    #[msg("Invalid recipient accounts")]
    InvalidRecipientAccounts,
    #[msg("Invalid recipient account")]
    InvalidRecipientAccount,
    #[msg("Invalid contract")]
    InvalidContract,
    #[msg("Too many whitelisted contracts")]
    TooManyWhitelistedContracts,
    #[msg("Contract not whitelisted")]
    ContractNotWhitelisted,
    #[msg("Already migrated")]
    AlreadyMigrated,
    #[msg("Invalid migration user")]
    InvalidMigrationUser,
    #[msg("Migration toggle cooldown active")]
    MigrationToggleCooldown,
    #[msg("Batch size too large")]
    BatchSizeTooLarge,
    #[msg("Whitelist delay not met")]
    WhitelistDelayNotMet,
    #[msg("Time lock requirement not met")]
    TimeLockNotMet,
    #[msg("Caller is not whitelisted")]
    CallerNotWhitelisted,
    #[msg("Invalid transfer destination")]
    InvalidDestination,
    #[msg("Version mismatch in whitelisted contract")]
    VersionMismatch,
    #[msg("KYC verification required for this purchase")]
    KYCRequired,
    /// Update
    #[msg("Invalid threshold")]
    InvalidThreshold,
    #[msg("Invalid vector size")]
    InvalidVectorSize,
    #[msg("Invalid proposal")]
    InvalidProposal,
    #[msg("Invalid buyer USDT owner")]
    InvalidBuyerUSDTOwner,
    #[msg("Invalid presale USDT receiver")]
    InvalidPresaleUSDTReceiver,
}
