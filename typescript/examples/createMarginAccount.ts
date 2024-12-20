import {
  setupWalletAndV3ApiClientAndRpcClient,
  deserializeTransactionSetBlockhashAndSignMessage,
  sendAndConfirmTransaction,
} from "../utils";

(async function main() {
  // See utils for more info on setup
  const { wallet, v3ApiClient, rpcClient } =
    setupWalletAndV3ApiClientAndRpcClient();

  // Fetch create_margin_account transaction and latest blockhash.
  const [response, { blockhash: latestBlockhash }] = await Promise.all([
    v3ApiClient.transactions.getCreateMarginAccountTransaction({
      createMarginAccountPayload: {
        owner: wallet.publicKey.toBase58(),
      },
    }),
    rpcClient.getLatestBlockhash(),
  ]);

  // Deserialize create_margin_account transaction into a versioned transaction. Set blockhash and sign transaction.
  const tx = deserializeTransactionSetBlockhashAndSignMessage(
    response.transaction,
    wallet,
    latestBlockhash
  );

  // Send create_margin_account transaction
  console.log("Sending create_margin_account transaction...");
  const signature = await sendAndConfirmTransaction(tx, rpcClient);
  console.log("Signature: ", signature);
})();
