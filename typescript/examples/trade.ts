import {
  setupWalletAndV3ApiClientAndRpcClient,
  deserializeTransactionSetBlockhashAndSignMessage,
  sendAndConfirmTransaction,
} from "../utils";

(async function main() {
  // See utils for more info on setup
  const { wallet, v3ApiClient, rpcClient } =
    setupWalletAndV3ApiClientAndRpcClient();

  // Setup trade inputs
  const marginAccountId = 0; // Margin account with id 0
  const marketId = 23; // SOL market
  const sizeDelta = -20; // 0.00002 SOL short
  const slippageToleranceBps = 200; // 2%

  // Fetch modify_position transaction and latest blockhash.
  const [response, { blockhash: latestBlockhash }] = await Promise.all([
    v3ApiClient.transactions.getModifyPositionTransaction({
      modifyPositionPayload: {
        owner: wallet.publicKey.toBase58(),
        marginAccountId: marginAccountId.toString(),
        marketId: marketId,
        sizeDelta: sizeDelta.toString(),
        slippageToleranceBps,
      },
    }),
    rpcClient.getLatestBlockhash(),
  ]);

  // Deserialize modify_position transaction into a versioned transaction. Set blockhash and sign transaction.
  const tx = deserializeTransactionSetBlockhashAndSignMessage(
    response.transaction,
    wallet,
    latestBlockhash
  );

  // Send modify_position transaction
  console.log("Sending modify_position transaction...");
  const signature = await sendAndConfirmTransaction(tx, rpcClient);
  console.log("Signature: ", signature);
})();
