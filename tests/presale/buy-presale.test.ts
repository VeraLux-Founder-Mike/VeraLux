import { BN } from "bn.js";
import { MockFactory } from "../../sdk/common/helper";
import {
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import {
  getAssociatedTokenAddress,
  createAssociatedTokenAccountInstruction,
  getOrCreateAssociatedTokenAccount,
  getAccount,
  createInitializeAccountInstruction,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { utils } from "@coral-xyz/anchor";

beforeAll(async () => {
  await MockFactory.create();
});

describe("Buy Presale Tests", () => {
  it("should buy presale", async () => {
    const { program, signer1, authority } = MockFactory.mockFactory;
    if (!program.provider) throw new Error("Provider not initialized");
    const provider = program.provider;

    const buyAmount = 1000;

    const usdtMint = new PublicKey(
      "HgcnJ8m979XRp7CFou5GunrTc1VJ7KjLoikJpxbNVdDX"
    );

    const [statePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("contract_state")],
      program.programId
    );

    const [presalePurchasePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("presale_purchase"), signer1.payer.publicKey.toBuffer()],
      program.programId
    );

    const [presaleUsdtReceiver] = PublicKey.findProgramAddressSync(
      [Buffer.from("presale_usdt_receiver")],
      program.programId
    );

    const presaleUsdtAccountInfo = await getAccount(
      provider.connection,
      presaleUsdtReceiver
    ).catch(() => null);

    if (!presaleUsdtAccountInfo) {
      // Create empty token account owned by PDA
      const presaleUsdtAccount = Keypair.generate();

      const lamportsForAccount =
        await provider.connection.getMinimumBalanceForRentExemption(165); // size of TokenAccount

      const tx = new Transaction();

      tx.add(
        SystemProgram.createAccount({
          fromPubkey: signer1.payer.publicKey,
          newAccountPubkey: presaleUsdtAccount.publicKey,
          space: 165,
          lamports: lamportsForAccount,
          programId: TOKEN_PROGRAM_ID,
        })
      );

      tx.add(
        createInitializeAccountInstruction(
          presaleUsdtAccount.publicKey,
          usdtMint,
          presaleUsdtReceiver // Owner PDA
        )
      );

      await provider.sendAndConfirm(tx, [presaleUsdtAccount]);
    }

    const buyerUsdtAccount = (
      await getOrCreateAssociatedTokenAccount(
        provider.connection,
        authority.payer,
        usdtMint,
        signer1.payer.publicKey
      )
    ).address;

    const presaleUsdtAccount = (
      await getOrCreateAssociatedTokenAccount(
        provider.connection,
        authority.payer,
        usdtMint,
        authority.publicKey,
        true // allow owner off curve for PDA
      )
    ).address;

    // Get state to find presale USDT receiver
    const state = await (program.account as any).contractState.fetch(statePda);
    console.log("presaleUsdtAccount", state.presaleUsdtReceiver.toBase58());

    const tx = await (program.methods as any)
      .buyPresale(new BN(buyAmount))
      .accounts({
        buyer: signer1.payer.publicKey,
        state: statePda,
        buyerUsdtAccount,
        presaleUsdtAccount,
      })
      .transaction();

    const txId = await provider.sendAndConfirm(tx, [signer1.payer]);
    console.log("Buy presale tx:", txId);

    const presalePurchase = await (
      program.account as any
    ).presalePurchase.fetch(presalePurchasePda);
    expect(presalePurchase.totalPurchased).toEqual(new BN(buyAmount));
  });
});
