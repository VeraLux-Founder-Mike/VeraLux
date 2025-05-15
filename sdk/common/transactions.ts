import { Wallet } from "@coral-xyz/anchor";
import {
  AddressLookupTableAccount,
  ConfirmOptions,
  Connection,
  PublicKey,
  sendAndConfirmRawTransaction,
  SendTransactionError,
  Signer,
  Transaction,
  TransactionMessage,
  VersionedTransaction,
} from "@solana/web3.js";

export const simulateTransaction = async (
  connection: Connection,
  tx: Transaction,
  wallet: Wallet,
  config?: {
    lookupTableIds?: PublicKey[];
    signers?: Signer[];
  }
) => {
  const blockhash = (await connection.getLatestBlockhash()).blockhash;
  console.log("blockhash", blockhash);
  const lookupTableAccounts = config?.lookupTableIds
    ? (
        await Promise.all(
          config.lookupTableIds.map((lookupTableId) =>
            connection.getAddressLookupTable(lookupTableId)
          ) ?? []
        )
      )
        .map((lut) => lut.value)
        .filter((x): x is AddressLookupTableAccount => x !== null)
    : [];

  const messageV0 = new TransactionMessage({
    payerKey: wallet.publicKey,
    recentBlockhash: blockhash,
    instructions: tx.instructions,
  }).compileToV0Message(lookupTableAccounts);
  let transactionV0 = new VersionedTransaction(messageV0);
  transactionV0 = await wallet.signTransaction(transactionV0);
  if (config?.signers) {
    transactionV0.sign(config.signers ?? []);
  }

  try {
    const simulation = await connection.simulateTransaction(transactionV0);
    return simulation;
  } catch (error) {
    console.error("Simulation error:", error);
    throw error;
  }
};

export async function executeTransaction(
  connection: Connection,
  tx: Transaction,
  wallet: Wallet,
  config?: {
    lookupTableIds?: PublicKey[];
    signers?: Signer[];
    silent?: boolean;
    confirmOptions?: ConfirmOptions;
  }
): Promise<string> {
  const blockhash = (await connection.getLatestBlockhash()).blockhash;
  const lookupTableAccounts = config?.lookupTableIds
    ? (
        await Promise.all(
          config?.lookupTableIds?.map((lookupTableId) =>
            connection.getAddressLookupTable(lookupTableId)
          ) ?? []
        )
      )
        .map((lut) => lut.value)
        .filter((x): x is AddressLookupTableAccount => x !== null)
    : [];

  const messageV0 = new TransactionMessage({
    payerKey: wallet.publicKey,
    recentBlockhash: blockhash,
    instructions: tx.instructions,
  }).compileToV0Message(lookupTableAccounts);
  let transactionV0 = new VersionedTransaction(messageV0);
  transactionV0 = await wallet.signTransaction(transactionV0);
  if (config?.signers) {
    transactionV0.sign(config?.signers ?? []);
  }
  try {
    const txid = await sendAndConfirmRawTransaction(
      connection,
      Buffer.from(transactionV0.serialize()),
      config?.confirmOptions
    );
    return txid;
  } catch (e) {
    if (!config?.silent) {
      logError(e);
    }
    throw e;
  }
}

export const logError = (e: unknown) => {
  const message = (e as SendTransactionError).message ?? "";
  const logs = (e as SendTransactionError).logs;
  if (logs) {
    console.log(logs, message);
  } else {
    console.log(e, message);
  }
};
