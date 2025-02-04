import bs58 from "bs58";
import path from "path";
import { Connection, Keypair } from "@solana/web3.js";
import { ParclV3ApiClient } from "@parcl-oss/v3-api-client";
import * as dotenv from "dotenv";
dotenv.config({ path: path.resolve(__dirname, "../../.env") });

export function setupWalletAndV3ApiClientAndRpcClient(): {
  wallet: Keypair;
  v3ApiClient: ParclV3ApiClient;
  rpcClient: Connection;
} {
  // Setup wallet from private key
  if (process.env.WALLET_PRIVATE_KEY === undefined) {
    throw new Error("Missing env var: WALLET_PRIVATE_KEY");
  }
  const wallet = Keypair.fromSecretKey(
    bs58.decode(process.env.WALLET_PRIVATE_KEY)
  );

  // Setup v3 api client
  const v3ApiClient = new ParclV3ApiClient(
    process.env.API_URL
      ? {
          basePath: process.env.API_URL,
        }
      : undefined
  );

  // Setup rpc client
  const rpcClient = new Connection(
    process.env.RPC_URL ?? "https://api.mainnet-beta.solana.com"
  );
  return { wallet, v3ApiClient, rpcClient };
}
