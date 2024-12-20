import {
  setupWalletAndV3ApiClientAndRpcClient,
  deserializeTransactionSetBlockhashAndSignMessage,
  sendAndConfirmTransaction,
} from "../utils";

(async function main() {
  // See utils for more info on setup
  const { wallet, v3ApiClient, rpcClient } =
    setupWalletAndV3ApiClientAndRpcClient();

  // Setup deposit_margin inputs
  const marginAccountId = 0; // Margin account with id 0
  const marginToDeposit = 1_000_000; // 1 usdc to deposit

  // Fetch deposit_margin transaction and latest blockhash.
  const [response, { blockhash: latestBlockhash }] = await Promise.all([
    v3ApiClient.transactions.getDepositMarginTransaction({
      depositMarginPayload: {
        owner: wallet.publicKey.toBase58(),
        marginAccountId: marginAccountId.toString(),
        margin: marginToDeposit,
      },
    }),
    rpcClient.getLatestBlockhash(),
  ]);

  // Deserialize deposit_margin transaction into a versioned transaction. Set blockhash and sign transaction.
  const tx = deserializeTransactionSetBlockhashAndSignMessage(
    response.transaction,
    wallet,
    latestBlockhash
  );

  // Send deposit_margin transaction
  console.log("Sending deposit_margin transaction...");
  const signature = await sendAndConfirmTransaction(tx, rpcClient);
  console.log("Signature: ", signature);
})();
