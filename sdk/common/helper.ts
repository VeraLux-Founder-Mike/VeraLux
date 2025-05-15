import {
  AnchorProvider,
  Program,
  Provider,
  setProvider,
  Wallet,
} from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { mplToolbox } from "@metaplex-foundation/mpl-toolbox";
import { generateSigner } from "@metaplex-foundation/umi";

import { Veralux } from "../idl/veralux";
import IDL from "../idl/veralux.json";
import { keypairIdentity } from "@metaplex-foundation/umi";
import {
  createFungible,
  mintV1,
  TokenStandard,
} from "@metaplex-foundation/mpl-token-metadata";
import {
  fromWeb3JsKeypair,
  fromWeb3JsPublicKey,
  toWeb3JsPublicKey,
} from "@metaplex-foundation/umi-web3js-adapters";

export class MockFactory {
  static mockFactory: MockFactory;

  constructor(
    public program: Program<Veralux>,
    public provider: Provider,
    public connection: Connection,
    public authority: Wallet,
    public signer1: Wallet,
    public teamWallet: Wallet,
    public charityWallet: Wallet,
    public tokenMint: PublicKey
  ) {
    MockFactory.mockFactory = this;
  }

  static async create(): Promise<MockFactory> {
    const authorityKeypair = getKeypairFromEnvSecret("AUTH_WALLET");
    const teamKeypair = getKeypairFromEnvSecret("TEAM_WALLET");
    const charityKeypair = getKeypairFromEnvSecret("CHARITY_WALLET");
    const signer1Keypair = getKeypairFromEnvSecret("SIGNER_1_SECRET_KEY");

    const authorityWallet = new Wallet(authorityKeypair);
    const teamWallet = new Wallet(teamKeypair);
    const charityWallet = new Wallet(charityKeypair);
    const signer1 = new Wallet(signer1Keypair);

    const connection = await getTestConnection();
    const provider = new AnchorProvider(connection, authorityWallet, {
      preflightCommitment: "confirmed",
    });
    setProvider(provider);

    const balance = await connection.getBalance(authorityWallet.publicKey);
    const signer1Balance = await connection.getBalance(signer1.publicKey);
    console.log("balance", balance, signer1Balance);

    const veraLuxInterface = JSON.parse(JSON.stringify(IDL));
    const program = new Program(veraLuxInterface, provider) as Program<Veralux>;

    let tokenMint: PublicKey;
    const existingMint = process.env.TOKEN_MINT;
    if (existingMint) {
      console.log("Using existing token mint:", existingMint);
      tokenMint = new PublicKey(existingMint);
    } else {
      tokenMint = await deployToken(
        "Veralux",
        "VRLX",
        "VRLX",
        connection,
        8,
        1000000000
      );
      console.log("New token mint:", tokenMint);
    }

    console.log("Program ID", program.programId);

    return new MockFactory(
      program,
      provider,
      connection,
      authorityWallet,
      signer1,
      teamWallet,
      charityWallet,
      tokenMint
    );
  }
}

export const getTestConnection = (): Connection => {
  const url =
    "https://devnet.helius-rpc.com/?api-key=c468ac4b-f75f-422d-b7c2-b965484d3eaf";
  return new Connection(url, "confirmed");
};

export const deployToken = (
  name: string,
  uri: string,
  symbol: string,
  connection: Connection,
  decimals: number = 9,
  initialSupply?: number
): Promise<PublicKey> => {
  try {
    const umi = createUmi(connection).use(mplToolbox());
    const authorityKeypair = process.env.AUTH_WALLET
      ? Keypair.fromSecretKey(
          Uint8Array.from(JSON.parse(process.env.AUTH_WALLET))
        )
      : Keypair.generate();
    umi.use(keypairIdentity(fromWeb3JsKeypair(authorityKeypair)));

    const mint = generateSigner(umi);

    let builder = createFungible(umi, {
      name,
      uri,
      symbol,
      sellerFeeBasisPoints: {
        basisPoints: BigInt(0),
        identifier: "%",
        decimals: 2,
      },
      decimals,
      mint,
    });

    if (initialSupply) {
      builder = builder.add(
        mintV1(umi, {
          mint: mint.publicKey,
          tokenStandard: TokenStandard.Fungible,
          tokenOwner: fromWeb3JsPublicKey(authorityKeypair.publicKey),
          amount: initialSupply * Math.pow(10, decimals),
        })
      );
    }

    builder.sendAndConfirm(umi, { confirm: { commitment: "finalized" } });

    return Promise.resolve(toWeb3JsPublicKey(mint.publicKey));
  } catch (error) {
    console.error(error);
    return Promise.reject(error);
  }
};

export const getKeypairFromEnvSecret = (
  envVarName: string,
  defaultKeypair?: Keypair
): Keypair => {
  const envValue = process.env[envVarName];
  if (envValue) {
    try {
      const secretKeyBytes = Uint8Array.from(JSON.parse(envValue));
      if (secretKeyBytes.length === 64) {
        return Keypair.fromSecretKey(secretKeyBytes);
      } else {
        console.warn(
          `Warning: Environment variable ${envVarName} does not contain a valid 64-byte secret key array. Length: ${secretKeyBytes.length}.`
        );
      }
    } catch (e) {
      console.warn(
        `Warning: Could not parse ${envVarName} as a secret key JSON array. Error: ${e}`
      );
    }
  }
  if (defaultKeypair) return defaultKeypair;
  console.warn(
    `Warning: Environment variable ${envVarName} not set or invalid. Generating new keypair.`
  );
  return Keypair.generate();
};
