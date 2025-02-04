import {
  setupWalletAndV3ApiClientAndRpcClient,
  deserializeTransactionSetBlockhashAndSignMessage,
  sendAndConfirmTransaction,
} from "../utils";

(async function main() {
  // See utils for more info on setup
  const { wallet, v3ApiClient, rpcClient } =
    setupWalletAndV3ApiClientAndRpcClient();

  // Setup close_position inputs
  const marginAccountId = 0; // Margin account with id 0
  const marketId = 23; // SOL market
  const slippageToleranceBps = 200; // 2%

  // Fetch close_position transaction and latest blockhash.
  const [response, { blockhash: latestBlockhash }] = await Promise.all([
    v3ApiClient.transactions.getClosePositionTransaction({
      closePositionPayload: {
        owner: wallet.publicKey.toBase58(),
        marginAccountId: marginAccountId.toString(),
        marketId: marketId,
        slippageToleranceBps,
      },
    }),
    rpcClient.getLatestBlockhash(),
  ]);

  // Deserialize close_position transaction into a versioned transaction. Set blockhash and sign transaction.
  const tx = deserializeTransactionSetBlockhashAndSignMessage(
    response.transaction,
    wallet,
    latestBlockhash
  );

  // Send close_position transaction
  console.log("Sending close_position transaction...");
  const signature = await sendAndConfirmTransaction(tx, rpcClient);
  console.log("Signature: ", signature);
})();
