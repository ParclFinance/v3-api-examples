use anyhow::Result;
use parcl_v3_api_client::request::*;
use solana_sdk::{commitment_config::CommitmentConfig, signer::Signer};
use utils::{
    deserialize_tx_set_recent_blockhash_and_sign_message, send_transaction,
    setup_wallet_and_v3_api_client_and_rpc_client,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::from_path("../.env").ok();

    // See utils for more info on setup
    // Use finalized commitment because need margin account to be init before sending deposit_margin transaction.
    let (wallet, v3_api_client, rpc_client) =
        setup_wallet_and_v3_api_client_and_rpc_client(Some(CommitmentConfig::finalized()))?;

    // Fetch create_margin_account transaction and latest blockhash.
    let (create_margin_account_res, latest_blockhash) = {
        let (create_margin_account_res, latest_blockhash) = tokio::join!(
            // margin_account_id set to None so api will find available margin_account_id
            v3_api_client.get_create_margin_account_transaction(wallet.pubkey(), None),
            rpc_client.get_latest_blockhash(),
        );
        (create_margin_account_res?, latest_blockhash?)
    };

    // Deserialize create_margin_account transaction into a versioned transaction. Set blockhash and sign transaction.
    let create_margin_account_tx = deserialize_tx_set_recent_blockhash_and_sign_message(
        create_margin_account_res.transaction,
        &wallet,
        latest_blockhash,
    )?;

    // Send create_margin_account transaction
    let signature = send_transaction(&create_margin_account_tx, rpc_client.clone()).await?;
    println!("Transaction successful: {signature:?}");

    // Setup deposit_margin inputs
    let margin_to_deposit = 100_000; // 0.1 of usdc collateral

    // Fetch deposit_margin transaction and latest blockhash
    let (deposit_margin_res, latest_blockhash) = {
        let (deposit_margin_res, latest_blockhash) = tokio::join!(
            v3_api_client.get_deposit_margin_transaction(
                wallet.pubkey(),
                MarginAccountIdentifier::Id(create_margin_account_res.margin_account_id),
                margin_to_deposit,
            ),
            rpc_client.get_latest_blockhash(),
        );
        (deposit_margin_res?, latest_blockhash?)
    };

    // Deserialize deposit_margin transaction into a versioned transaction. Set blockhash and sign transaction.
    let deposit_margin_tx = deserialize_tx_set_recent_blockhash_and_sign_message(
        deposit_margin_res.transaction,
        &wallet,
        latest_blockhash,
    )?;

    // Send deposit_margin transaction
    let signature = send_transaction(&deposit_margin_tx, rpc_client.clone()).await?;
    println!("Transaction successful: {signature:?}");
    Ok(())
}
