import {
  setupWalletAndV3ApiClientAndRpcClient,
  deserializeTransactionSetBlockhashAndSignMessage,
  sendAndConfirmTransaction,
} from "../utils";
import { Keypair, Connection } from "@solana/web3.js";
import { ParclV3ApiClient } from "@parcl-oss/v3-api-client";

(async function main() {
  // See utils for more info on setup
  const { wallet, v3ApiClient, rpcClient } =
    setupWalletAndV3ApiClientAndRpcClient();

  // Setup liquidate inputs
  const liquidatorMarginAccountId = 0; // Margin account with id 0

  // First run without waiting for interval
  await tryLiquidate(liquidatorMarginAccountId, wallet, v3ApiClient, rpcClient);

  setInterval(async () => {
    await tryLiquidate(
      liquidatorMarginAccountId,
      wallet,
      v3ApiClient,
      rpcClient
    );
  }, 300 * 1000); // 300 second interval
})();

async function tryLiquidate(
  liquidatorMarginAccountId: number,
  wallet: Keypair,
  v3ApiClient: ParclV3ApiClient,
  rpcClient: Connection
) {
  const unhealthyMarginAccounts =
    await v3ApiClient.accounts.getUnhealthyMarginAccounts();
  console.log(`${unhealthyMarginAccounts.length} unhealthy accounts`);
  if (unhealthyMarginAccounts.length === 0) {
    return;
  }
  for (const marginAccountToLiquidate of unhealthyMarginAccounts) {
    // Fetch liquidate transaction and latest blockhash.
    const [response, { blockhash: latestBlockhash }] = await Promise.all([
      v3ApiClient.transactions.getLiquidateTransaction({
        liquidatePayload: {
          marginAccountToLiquidate,
          liquidator: wallet.publicKey.toBase58(),
          liquidatorMarginAccountId: liquidatorMarginAccountId.toString(),
        },
      }),
      rpcClient.getLatestBlockhash(),
    ]);

    // Deserialize liquidate transaction into a versioned transaction. Set blockhash and sign transaction.
    const tx = deserializeTransactionSetBlockhashAndSignMessage(
      response.transaction,
      wallet,
      latestBlockhash
    );

    // Send liquidate transaction
    console.log("Sending liquidate transaction...");
    const signature = await sendAndConfirmTransaction(tx, rpcClient);
    console.log("Signature: ", signature);
  }
}
