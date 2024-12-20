import {
  setupWalletAndV3ApiClientAndRpcClient,
  deserializeTransactionSetBlockhashAndSignMessage,
  sendAndConfirmTransaction,
} from "../utils";

(async function main() {
  // See utils for more info on setup
  const { wallet, v3ApiClient, rpcClient } =
    setupWalletAndV3ApiClientAndRpcClient();

  // Fetch close_margin_account transaction and latest blockhash.
  const [response, { blockhash: latestBlockhash }] = await Promise.all([
    v3ApiClient.transactions.getCloseMarginAccountTransaction({
      closeMarginAccountPayload: {
        owner: wallet.publicKey.toBase58(),
        marginAccountId: "1",
      },
    }),
    rpcClient.getLatestBlockhash(),
  ]);

  // Deserialize close_margin_account transaction into a versioned transaction. Set blockhash and sign transaction.
  const tx = deserializeTransactionSetBlockhashAndSignMessage(
    response.transaction,
    wallet,
    latestBlockhash
  );

  // Send close_margin_account transaction
  console.log("Sending close_margin_account transaction...");
  const signature = await sendAndConfirmTransaction(tx, rpcClient);
  console.log("Signature: ", signature);
})();
