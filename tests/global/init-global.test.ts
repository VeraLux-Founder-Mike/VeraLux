import { Transaction, Keypair, PublicKey } from "@solana/web3.js";
import { getKeypairFromEnvSecret, MockFactory } from "../../sdk/common/helper";
import { BN } from "@coral-xyz/anchor";
import { executeTransaction } from "../../sdk/common/transactions";

beforeAll(async () => {
  await MockFactory.create();
});

describe.skip("Init Global Tests", () => {
  test("init global", async () => {
    const { program, teamWallet, charityWallet, signer1, authority } =
      MockFactory.mockFactory;

    const [statePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("contract_state")],
      program.programId
    );
    const [treasuryPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("treasury")],
      program.programId
    );
    const [multisigPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("multisig")],
      program.programId
    );

    const owner1Keypair = getKeypairFromEnvSecret("OWNER_1_SECRET_KEY");
    const owner2Keypair = getKeypairFromEnvSecret("OWNER_2_SECRET_KEY");
    const owner3Keypair = getKeypairFromEnvSecret("OWNER_3_SECRET_KEY");

    const initialOwners: PublicKey[] = [
      owner1Keypair.publicKey,
      owner2Keypair.publicKey,
      owner3Keypair.publicKey,
    ];

    const initialDexPrograms = [
      new PublicKey("11111111111111111111111111111111"), // System Program as example
    ];

    const remainingAccountsMeta = initialDexPrograms.map((pubkey) => ({
      pubkey,
      isSigner: false,
      isWritable: false,
    }));

    const tx = await program.methods
      .initGlobal({
        charityWallet: charityWallet.publicKey,
        teamWallet: teamWallet.publicKey,
        liquidityPool: Keypair.generate().publicKey,
        launchTimestamp: new BN(Math.floor(Date.now() / 1000)),
        presaleUsdtReceiver: authority.publicKey,
        initialOwners,
        initialDexPrograms,
        threshold: 2,
      })
      .accounts({
        signer1: signer1.publicKey,
      })
      .remainingAccounts(remainingAccountsMeta)
      .transaction();

    console.log("Transaction created, attempting to send:", tx);

    const txId = await program.provider?.sendAndConfirm(tx, [signer1.payer]);

    console.log("txId", txId);
    // const tx = new Transaction().add(ix);
    // try {
    //   const txId = await executeTransaction(connection, tx, signer1, {
    //     silent: true,
    //     signers: [signer1.payer],
    //   });
    //   console.log("Initialization Transaction ID:", txId);
    // } catch (error) {
    //   console.error("Transaction failed:", error);
    //   if (error instanceof SendTransactionError) {
    //     console.error("Transaction Logs:", error.logs);
    //   } else if (error && typeof error === "object" && "message" in error) {
    //     // Handle cases where error might have a message property but isn't a SendTransactionError
    //     console.error("Error message:", (error as { message: string }).message);
    //   }
    //   throw error;
    // }

    const state = await program.account.contractState.fetch(statePda);
    expect(state.admin).toEqual(multisigPda);
    expect(state.treasury).toEqual(treasuryPda);
    expect(state.charityWallet).toEqual(charityWallet.publicKey);
    expect(state.teamWallet).toEqual(teamWallet.publicKey);
    expect(state.paused).toBeFalsy();
    expect(state.presaleActive).toBeTruthy();
  });

  test.skip("Too few owners", async () => {
    const { program, authority, connection, teamWallet, charityWallet } =
      MockFactory.mockFactory;

    const owner1Keypair = getKeypairFromEnvSecret("OWNER_1_SECRET_KEY");
    const initialOwners = [owner1Keypair.publicKey];

    const ix = await program.methods
      .initGlobal({
        charityWallet: charityWallet.publicKey,
        teamWallet: teamWallet.publicKey,
        liquidityPool: Keypair.generate().publicKey,
        launchTimestamp: new BN(Math.floor(Date.now() / 1000)),
        presaleUsdtReceiver: Keypair.generate().publicKey,
        initialOwners,
        initialDexPrograms: [],
        threshold: 2,
      })
      .accounts({
        signer1: authority.publicKey,
      })
      .instruction();

    const tx = new Transaction().add(ix);

    await expect(
      executeTransaction(connection, tx, authority)
    ).rejects.toThrow();
  });
});
