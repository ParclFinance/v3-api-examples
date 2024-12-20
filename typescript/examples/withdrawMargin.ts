import {
  setupWalletAndV3ApiClientAndRpcClient,
  deserializeTransactionSetBlockhashAndSignMessage,
  sendAndConfirmTransaction,
} from "../utils";

(async function main() {
  // See utils for more info on setup
  const { wallet, v3ApiClient, rpcClient } =
    setupWalletAndV3ApiClientAndRpcClient();

  // Setup withdraw_margin inputs
  const marginAccountId = 0; // Margin account with id 0
  const marginToWithdraw = 1_000_000; // 1 usdc to withdraw

  // Fetch withdraw_margin transaction and latest blockhash.
  const [response, { blockhash: latestBlockhash }] = await Promise.all([
    v3ApiClient.transactions.getWithdrawMarginTransaction({
      withdrawMarginPayload: {
        owner: wallet.publicKey.toBase58(),
        marginAccountId: marginAccountId.toString(),
        margin: marginToWithdraw,
      },
    }),
    rpcClient.getLatestBlockhash(),
  ]);

  // Deserialize withdraw_margin transaction into a versioned transaction. Set blockhash and sign transaction.
  const tx = deserializeTransactionSetBlockhashAndSignMessage(
    response.transaction,
    wallet,
    latestBlockhash
  );

  // Send withdraw_margin transaction
  console.log("Sending withdraw_margin transaction...");
  const signature = await sendAndConfirmTransaction(tx, rpcClient);
  console.log("Signature: ", signature);
})();
