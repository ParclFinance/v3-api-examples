use anyhow::Result;
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

    // Fetch create_margin_account transaction and latest blockhash.
    let (create_margin_account_res, latest_blockhash) = {
        let (create_margin_account_res, latest_blockhash) = tokio::join!(
            // margin_account_id set to None. Api will find available margin_account_id
            v3_api_client.get_create_margin_account_transaction(wallet.pubkey(), None),
            rpc_client.get_latest_blockhash(),
        );
        (create_margin_account_res?, latest_blockhash?)
    };

    // Deserialize create_margin_account transaction into a versioned transaction. Set blockhash and sign transaction.
    let tx = deserialize_tx_set_recent_blockhash_and_sign_message(
        create_margin_account_res.transaction,
        &wallet,
        latest_blockhash,
    )?;

    // Send create_margin_account transaction
    let signature = send_transaction(&tx, rpc_client.clone()).await?;
    println!("Transaction successful: {signature:?}");

    Ok(())
}
