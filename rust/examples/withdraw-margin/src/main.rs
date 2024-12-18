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

    // Setup withdraw_margin inputs
    let margin_account_id = MarginAccountIdentifier::Id(0); // Margin account with id 0
    let margin_to_withdraw = 10_000; // 0.01 usdc to withdraw

    // Fetch withdraw_margin transaction and latest blockhash.
    let (api_response, latest_blockhash) = {
        let (api_response, latest_blockhash) = tokio::join!(
            v3_api_client.get_withdraw_margin_transaction(
                wallet.pubkey(),
                margin_account_id,
                margin_to_withdraw,
                None,
                None
            ),
            rpc_client.get_latest_blockhash(),
        );
        (api_response?, latest_blockhash?)
    };

    // Deserialize withdraw_margin transaction into a versioned transaction. Set blockhash and sign transaction.
    let tx = deserialize_tx_set_recent_blockhash_and_sign_message(
        api_response.transaction,
        &wallet,
        latest_blockhash,
    )?;

    // Send withdraw_margin transaction
    let signature = send_transaction(&tx, rpc_client.clone()).await?;
    println!("Transaction successful: {signature:?}");
    Ok(())
}
