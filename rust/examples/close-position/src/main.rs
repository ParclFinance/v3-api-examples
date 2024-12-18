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

    // Setup close_position inputs
    let margin_account_id = MarginAccountIdentifier::Id(0); // Margin account with id 0
    let market_id = 23; // SOL market
    let slippage_setting = SlippageSetting::SlippageToleranceBps(200); // 2%

    // Fetch close_position transaction and latest blockhash.
    let (api_response, latest_blockhash) = {
        let (api_response, latest_blockhash) = tokio::join!(
            v3_api_client.get_close_position_transaction(
                wallet.pubkey(),
                margin_account_id,
                market_id,
                slippage_setting,
            ),
            rpc_client.get_latest_blockhash(),
        );
        (api_response?, latest_blockhash?)
    };

    // Deserialize close_position transaction into a versioned transaction. Set blockhash and sign transaction.
    let tx = deserialize_tx_set_recent_blockhash_and_sign_message(
        api_response.transaction,
        &wallet,
        latest_blockhash,
    )?;

    // Send close_position transaction
    let signature = send_transaction(&tx, rpc_client.clone()).await?;
    println!("Transaction successful: {signature:?}");
    Ok(())
}
