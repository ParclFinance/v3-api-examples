use anyhow::Result;
use parcl_v3_api_client::request::*;
use solana_sdk::signer::Signer;
use utils::{
    deserialize_tx_set_recent_blockhash_and_sign_message, send_transaction,
    setup_wallet_and_v3_api_client_and_rpc_client,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::from_path("../.env").ok();

    // See utils for more info on setup
    let (wallet, v3_api_client, rpc_client) = setup_wallet_and_v3_api_client_and_rpc_client(None)?;

    // Setup deposit_margin inputs
    let margin_account_id = MarginAccountIdentifier::Id(0); // Margin account with id 0
    let margin_to_deposit = 1_000_000; // 1 usdc to deposit

    // Fetch deposit_margin transaction and latest blockhash.
    let (api_response, latest_blockhash) = {
        let (api_response, latest_blockhash) = tokio::join!(
            v3_api_client.get_deposit_margin_transaction(
                wallet.pubkey(),
                margin_account_id,
                margin_to_deposit,
            ),
            rpc_client.get_latest_blockhash(),
        );
        (api_response?, latest_blockhash?)
    };

    // Deserialize deposit_margin transaction into a versioned transaction. Set blockhash and sign transaction.
    let tx = deserialize_tx_set_recent_blockhash_and_sign_message(
        api_response.transaction,
        &wallet,
        latest_blockhash,
    )?;

    // Send deposit_margin transaction
    let signature = send_transaction(&tx, rpc_client.clone()).await?;
    println!("Transaction successful: {signature:?}");
    Ok(())
}
