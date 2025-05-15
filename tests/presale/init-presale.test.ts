import { BN } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { MockFactory } from "../../sdk/common/helper";

beforeAll(async () => {
  console.log("beforeAll");
  await MockFactory.create();
});

describe.skip("Init Presale Tests", () => {
  it("init presale", async () => {
    const { program, signer1 } = MockFactory.mockFactory;
    if (!program.provider) throw new Error("Provider not initialized");

    const [presalePurchasePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("presale_purchase"), signer1.payer.publicKey.toBuffer()],
      program.programId
    );

    const [presaleVestingPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("presale_vesting"), signer1.payer.publicKey.toBuffer()],
      program.programId
    );

    const tx = await (program.methods as any)
      .initPresale()
      .accounts({
        buyer: signer1.payer.publicKey,
      })
      .transaction();

    const txId = await program.provider.sendAndConfirm(tx, [signer1.payer]);
    console.log("Init presale tx:", txId);

    const presalePurchase = await program.account.presalePurchase.fetch(
      presalePurchasePda
    );
    console.log("presalePurchase", presalePurchase);
    expect(presalePurchase.wallet).toEqual(signer1.payer.publicKey);
    expect(presalePurchase.totalPurchased.toNumber()).toBe(0);
    expect(presalePurchase.kycVerified).toBe(true);

    const presaleVesting = await (program.account as any).presaleVesting.fetch(
      presaleVestingPda
    );
    expect(presaleVesting.totalAmount.toNumber()).toBe(0);
    expect(presaleVesting.claimedAmount.toNumber()).toBe(0);
  });

  it.skip("should fail if trying to initialize same account twice", async () => {
    const { program, signer1 } = MockFactory.mockFactory;
    if (!program.provider) throw new Error("Provider not initialized");

    const [presalePurchasePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("presale_purchase"), signer1.payer.publicKey.toBuffer()],
      program.programId
    );

    const [presaleVestingPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("presale_vesting"), signer1.payer.publicKey.toBuffer()],
      program.programId
    );

    try {
      const tx = await (program.methods as any)
        .initPresale()
        .accounts({
          buyer: signer1.payer.publicKey,
          presalePurchase: presalePurchasePda,
          presaleVesting: presaleVestingPda,
        })
        .transaction();

      await program.provider.sendAndConfirm(tx, [signer1.payer]);
      fail("Expected transaction to fail");
    } catch (err: any) {
      expect(err.message).toContain("already in use");
    }
  });
});
