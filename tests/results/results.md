# VeraLux Smart Contract Test Results

This document records the test results for the VeraLux smart contract, submitted to the Colosseum Solana Breakout Hackathon 2025. Each test case includes a description, a screen recording of the test execution, and console logs with transaction IDs and events, providing verifiable proof of functionality. Screen recordings are stored in `Docs/recordings/` as MP4 files, and logs include commands, outputs, and errors.

**Instructions for Developer**:
- For each test case, replace placeholders (e.g., `(Add description)`) with specific details.
- Upload screen recordings to `results/recordings/` (e.g., `./results/recordings/normal_transfer.mp4`).
- Paste console logs in the `Logs` section, including the command run, transaction ID (if applicable), emitted events, and error messages (for failures).
- Add new test cases under each component as needed to cover additional scenarios.
- Commit changes to this file after each test session with messages like “Added transfer test results”.

---

## 1. Contract Initialization

### 1.1 Successful Initialization
**Description**: (Add description, e.g., Initialize contract with 3 multisig owners and threshold of 2)  
**Screen Recording**: (Link to `./Docs/recordings/init_success.mp4`)  
**Logs**:
```text
(Add command, e.g., anchor test -- --test initialization)
(Add output, e.g., ✔ Initialization test passed, Transaction ID: <tx_id>, Event: InitializeEvent { ... })

**1.2 Too Few Owners**
Description: (Test initialization with fewer than 2 owners, expecting TooFewOwners error)
Screen Recording: (Link to ./Docs/recordings/init_too_few_owners.mp4)
Logs:
text

(Add command and error, e.g., Error: TooFewOwners)

**1.3 Too Many Owners**
Description: (Test initialization with more than 5 owners, expecting TooManyOwners error)
Screen Recording: (Link to ./Docs/recordings/init_too_many_owners.mp4)
Logs:
text

(Add command and error)

****2. Presale Operations****
**2.1 Successful Purchase**
Description: (Test USDT purchase, e.g., 1000 USDT for 625K LUX)
Screen Recording: (Link to ./Docs/recordings/presale_purchase_success.mp4)
Logs:
text

(Add command, transaction ID, PresalePurchaseEvent, updated PresalePurchase.total_purchased)

**2.2 KYC Verification Required**
Description: (Test purchase ≥ $1000 USDT without KYC, expecting KYCRequired error)
Screen Recording: (Link to ./Docs/recordings/presale_kyc_required.mp4)
Logs:
text

(Add command and error)

**2.3 Exceed Max Per Wallet**
Description: (Test purchase exceeding 2M tokens, expecting PresaleMaxPerWalletExceeded error)
Screen Recording: (Link to ./Docs/recordings/presale_max_exceeded.mp4)
Logs:
text

(Add command and error)

**2.4 Claim Presale Tokens**
Description: (Test claiming tokens after vesting period)
Screen Recording: (Link to ./Docs/recordings/presale_claim_success.mp4)
Logs:
text

(Add command, transaction ID, updated PresaleVesting.claimed_amount)

3. Token Transfers
3.1 Normal Transfer
Description: (Test transfer of 1000 LUX with 5% tax, verify tax distribution)
Screen Recording: (Link to ./Docs/recordings/normal_transfer.mp4)
Logs:
text

(Add command, transaction ID, TransferEvent { amount: 1000, tax: 50, burn: 10, ... }, recipient balance)

3.2 Progressive Tax
Description: (Test transfer >0.5% of supply, verify tripled tax rate)
Screen Recording: (Link to ./Docs/recordings/progressive_tax.mp4)
Logs:
text

(Add command, transaction ID, TransferEvent with tripled tax)

3.3 Exceed Sell Limit
Description: (Test transfer exceeding MAX_SELL_TXN_LIMIT, expecting MaxSellTxnLimitExceeded error)
Screen Recording: (Link to ./Docs/recordings/exceed_sell_limit.mp4)
Logs:
text

(Add command and error)

3.4 Whitelisted Transfer
Description: (Test transfer by whitelisted contract with reduced tax)
Screen Recording: (Link to ./Docs/recordings/whitelisted_transfer.mp4)
Logs:
text

(Add command, transaction ID, TransferEvent with reduced tax)

4. Staking Operations
4.1 Successful Staking
Description: (Test staking 100K LUX in Tier 1, verify Staker.tier)
Screen Recording: (Link to ./Docs/recordings/stake_success.mp4)
Logs:
text

(Add command, transaction ID, StakeEvent, updated Staker.amount)

4.2 Unstaking After Lock
Description: (Test unstaking after 14-day lock period)
Screen Recording: (Link to ./Docs/recordings/unstake_success.mp4)
Logs:
text

(Add command, transaction ID, UnstakeEvent)

4.3 Claim Rewards
Description: (Test claiming staking rewards for Tier 1)
Screen Recording: (Link to ./Docs/recordings/claim_rewards_success.mp4)
Logs:
text

(Add command, transaction ID, ClaimRewardsEvent, updated Staker.last_claim)

4.4 LP Staking
Description: (Test LP token staking)
Screen Recording: (Link to ./Docs/recordings/lp_stake_success.mp4)
Logs:
text

(Add command, transaction ID, StakeLPEvent)

5. Governance Operations
5.1 Submit Proposal
Description: (Test submitting a tax rate change proposal)
Screen Recording: (Link to ./Docs/recordings/proposal_submit_success.mp4)
Logs:
text

(Add command, transaction ID, ProposalSubmittedEvent)

5.2 Successful Voting
Description: (Test voting by Tier 1 staker)
Screen Recording: (Link to ./Docs/recordings/vote_success.mp4)
Logs:
text

(Add command, transaction ID, VoteEvent, updated Proposal.votes_for)

5.3 Execute Proposal
Description: (Test executing proposal after voting and notice period)
Screen Recording: (Link to ./Docs/recordings/proposal_execute_success.mp4)
Logs:
text

(Add command, transaction ID, ProposalExecutedEvent, updated ContractState.tax_rate)

5.4 Insufficient Tier for Voting
Description: (Test voting with insufficient tier, expecting InsufficientTierForVoting error)
Screen Recording: (Link to ./Docs/recordings/vote_insufficient_tier.mp4)
Logs:
text

(Add command and error)

6. Airdrop and LP Incentives
6.1 Successful Airdrop
Description: (Test airdrop to 50 recipients)
Screen Recording: (Link to ./Docs/recordings/airdrop_success.mp4)
Logs:
text

(Add command, transaction ID, AirdropEvent, updated Treasury.airdrop_pool)

6.2 LP Incentive Distribution
Description: (Test distributing LP incentives to 10 recipients)
Screen Recording: (Link to ./Docs/recordings/lp_incentive_success.mp4)
Logs:
text

(Add command, transaction ID, LPIncentivesDistributedEvent)

6.3 Too Many Recipients
Description: (Test airdrop to >100 recipients, expecting TooManyRecipients error)
Screen Recording: (Link to ./Docs/recordings/airdrop_too_many.mp4)
Logs:
text

(Add command and error)

7. Pause and Resume
7.1 Successful Pause
Description: (Test initiating and confirming pause after 24-hour timelock)
Screen Recording: (Link to ./Docs/recordings/pause_success.mp4)
Logs:
text

(Add command, transaction ID, PauseEvent, updated ContractState.paused)

7.2 Successful Resume
Description: (Test initiating and confirming resume after 24-hour timelock)
Screen Recording: (Link to ./Docs/recordings/resume_success.mp4)
Logs:
text

(Add command, transaction ID, ResumeEvent)

7.3 Timelock Not Met
Description: (Test confirming pause before 24 hours, expecting TimeLockNotMet error)
Screen Recording: (Link to ./Docs/recordings/pause_timelock.mp4)
Logs:
text

(Add command and error)

8. Team and Freelancer Vesting
8.1 Team Vesting Claim
Description: (Test claiming team vesting after 3-month cliff)
Screen Recording: (Link to ./Docs/recordings/team_vesting_claim.mp4)
Logs:
text

(Add command, transaction ID, TeamVestingClaimedEvent)

8.2 Cancel Team Vesting
Description: (Test canceling team vesting)
Screen Recording: (Link to ./Docs/recordings/team_vesting_cancel.mp4)
Logs:
text

(Add command, transaction ID, TeamVestingCanceledEvent)

8.3 Freelancer Vesting Claim
Description: (Test claiming freelancer vesting after milestone release)
Screen Recording: (Link to ./Docs/recordings/freelancer_vesting_claim.mp4)
Logs:
text

(Add command, transaction ID, FreelancerVestingClaimedEvent)

9. Treasury Withdrawals
9.1 Successful Withdrawal
Description: (Test initiating and completing withdrawal after 48-hour delay)
Screen Recording: (Link to ./Docs/recordings/withdrawal_success.mp4)
Logs:
text

(Add command, transaction ID, WithdrawalCompletedEvent)

9.2 Delay Not Met
Description: (Test completing withdrawal before delay, expecting WithdrawalDelayNotMet error)
Screen Recording: (Link to ./Docs/recordings/withdrawal_delay.mp4)
Logs:
text

(Add command and error)

10. Multisig Management
10.1 Update Multisig
Description: (Test updating multisig owners and threshold)
Screen Recording: (Link to ./Docs/recordings/multisig_update.mp4)
Logs:
text

(Add command, transaction ID, MultisigUpdatedEvent)

10.2 Invalid Owners
Description: (Test updating with too few owners, expecting TooFewOwners error)
Screen Recording: (Link to ./Docs/recordings/multisig_too_few.mp4)
Logs:
text

(Add command and error)

11. Whitelist Management
11.1 Add Whitelisted Contract
Description: (Test adding a whitelisted contract)
Screen Recording: (Link to ./Docs/recordings/whitelist_add.mp4)
Logs:
text

(Add command, transaction ID, WhitelistedContractAddedEvent)

11.2 Remove Whitelisted Contract
Description: (Test removing a whitelisted contract)
Screen Recording: (Link to ./Docs/recordings/whitelist_remove.mp4)
Logs:
text

(Add command, transaction ID, WhitelistedContractRemovedEvent)

12. DEX Program Updates
12.1 Update DEX Programs
Description: (Test updating DEX program IDs)
Screen Recording: (Link to ./Docs/recordings/dex_update.mp4)
Logs:
text

(Add command, transaction ID, DexProgramsUpdatedEvent)

12.2 Invalid DEX Program
Description: (Test updating with invalid DEX program, expecting InvalidDexProgram error)
Screen Recording: (Link to ./Docs/recordings/dex_invalid.mp4)
Logs:
text

(Add command and error)

13. Token Migration
13.1 Lock Tokens
Description: (Test locking tokens for migration)
Screen Recording: (Link to ./Docs/recordings/migration_lock.mp4)
Logs:
text

(Add command, transaction ID, TokensLockedForMigrationEvent)

13.2 Unlock Tokens
Description: (Test unlocking tokens when migration inactive)
Screen Recording: (Link to ./Docs/recordings/migration_unlock.mp4)
Logs:
text

(Add command, transaction ID, TokensUnlockedForMigrationEvent)

13.3 Burn Locked Tokens
Description: (Test burning locked tokens)
Screen Recording: (Link to ./Docs/recordings/migration_burn.mp4)
Logs:
text

(Add command, transaction ID, LockedTokensBurnedEvent)

14. Treasury Pool Transfers
14.1 Successful Pool Transfer
Description: (Test transferring between staking and airdrop pools)
Screen Recording: (Link to ./Docs/recordings/pool_transfer_success.mp4)
Logs:
text

(Add command, transaction ID, TreasuryPoolAdjusted)

14.2 Insufficient Funds
Description: (Test transfer with insufficient funds, expecting ArithmeticOverflow error)
Screen Recording: (Link to ./Docs/recordings/pool_transfer_insufficient.mp4)
Logs:
text

(Add command and error)

15. Query Functions
15.1 Query Pending Rewards
Description: (Test querying rewards for a staker)
Screen Recording: (Link to ./Docs/recordings/query_rewards.mp4)
Logs:
text

(Add command, output with reward amount)

15.2 Query Contract State
Description: (Test querying ContractState fields)
Screen Recording: (Link to ./Docs/recordings/query_state.mp4)
Logs:
text

(Add command, output with ContractState fields)

Notes
Recording Tips: Ensure screen recordings are clear, showing the terminal with commands, outputs, and transaction IDs. Use tools like OBS Studio or QuickTime Player.

File Uploads: Upload recordings to Docs/recordings/ using GitHub’s Upload files feature. Name files descriptively (e.g., normal_transfer.mp4).

Completeness: Add test cases as needed to cover edge cases (e.g., unauthorized access, arithmetic overflows).

Commits: Commit changes to this file regularly to track progress, e.g., “Added initialization test results”.

