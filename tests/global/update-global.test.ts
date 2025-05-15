import { BN } from "bn.js";
import { MockFactory } from "../../sdk/common/helper";
import { AnchorProvider } from "@coral-xyz/anchor";

beforeAll(async () => {
  await MockFactory.create();
});

describe.skip("Update Global Tests", () => {
  test("update global", async () => {
    const { program, teamWallet, charityWallet, authority, signer1 } =
      MockFactory.mockFactory;

    const tx = await program.methods
      .updateGlobal({
        presaleUsdtReceiver: authority.publicKey,
        launchTimestamp: new BN(Math.floor(Date.now() / 1000)),
        teamWallet: teamWallet.publicKey,
        charityWallet: charityWallet.publicKey,
      })
      .accounts({
        signer: authority.publicKey,
      })
      .transaction();

    const provider = program.provider as AnchorProvider;
    const txId = await provider.sendAndConfirm(tx, [signer1.payer]);

    console.log("txId", txId);
  });
});
