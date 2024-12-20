import bs58 from "bs58";
import {
  Connection,
  VersionedTransaction,
  Keypair,
  Blockhash,
  sendAndConfirmRawTransaction,
} from "@solana/web3.js";

export function deserializeTransactionSetBlockhashAndSignMessage(
  rawTx: String,
  wallet: Keypair,
  latestBlockhash: Blockhash
): VersionedTransaction {
  const tx = VersionedTransaction.deserialize(Buffer.from(rawTx, "base64"));
  tx.message.recentBlockhash = latestBlockhash;
  tx.sign([wallet]); // overwrites any dummy sigs
  return tx;
}

export async function sendAndConfirmTransaction(
  tx: VersionedTransaction,
  rpcClient: Connection
): Promise<string> {
  const serializedTransaction = tx.serialize();
  const signature = bs58.encode(tx.signatures[0]);
  await sendAndConfirmRawTransaction(
    rpcClient,
    Buffer.from(serializedTransaction),
    {
      skipPreflight: true,
      maxRetries: 0,
    }
  );
  return signature;
}
