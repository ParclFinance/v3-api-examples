use anyhow::Result;
use parcl_v3_api_client::{request::*, ParclV3ApiClient};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};
use std::{sync::Arc, time::Duration};
use tokio::time::interval;
use utils::{
    deserialize_tx_set_recent_blockhash_and_sign_message, send_transaction,
    setup_wallet_and_v3_api_client_and_rpc_client,
};

// Liquidator bot fetches unhealthy margin account addresses from api.
// If there are any unhealthy accounts, then bot attempts to liquidate the accounts.
#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::from_path("../.env").ok();

    // See utils for more info on setup
    let (liquidator, v3_api_client, rpc_client) =
        setup_wallet_and_v3_api_client_and_rpc_client(None)?;

    // Liquidator is paid the liquidator fee as a margin collateral deposited to the liquidator's margin account.
    // Make sure this margin account is initialized before running bot.
    let liquidator_margin_account_id = MarginAccountIdentifier::Id(0);

    // Loop fetches unhealthy margin account addresses from api and attempts liquidation transactions if any accounts are found.
    let mut interval = interval(Duration::from_secs(300));
    loop {
        interval.tick().await;
        // Fetch unhealthy accounts from api.
        let unhealthy_margin_account_addressses: Vec<Pubkey> =
            match v3_api_client.get_unhealthy_margin_accounts().await {
                Ok(unhealthy_margin_account_addressses) => {
                    println!(
                        "{} unhealthy accounts",
                        unhealthy_margin_account_addressses.len()
                    );
                    if unhealthy_margin_account_addressses.is_empty() {
                        continue;
                    }
                    unhealthy_margin_account_addressses
                }
                Err(err) => {
                    println!("Error fetching unhealthy accounts: {err:?}");
                    continue;
                }
            };
        // Attempt to liquidate the unhealthy accounts received from api.
        for margin_account_to_liquidate in unhealthy_margin_account_addressses {
            if let Err(err) = liquidate(
                margin_account_to_liquidate,
                &liquidator,
                liquidator_margin_account_id,
                v3_api_client.clone(),
                rpc_client.clone(),
            )
            .await
            {
                println!("Send liquidation transaction error: {err:?}");
            }
        }
    }
}

async fn liquidate(
    margin_account_to_liquidate: Pubkey,
    liquidator: &Keypair,
    liquidator_margin_account_id: MarginAccountIdentifier,
    v3_api_client: ParclV3ApiClient,
    rpc_client: Arc<RpcClient>,
) -> Result<()> {
    // Fetch liquidate transaction and latest blockhash.
    let (api_response, latest_blockhash) = {
        let (api_response, latest_blockhash) = tokio::join!(
            v3_api_client.get_liquidate_transaction(
                margin_account_to_liquidate,
                liquidator.pubkey(),
                liquidator_margin_account_id
            ),
            rpc_client.get_latest_blockhash(),
        );
        (api_response?, latest_blockhash?)
    };

    // Deserialize liquidate transaction into a versioned transaction. Set blockhash and sign transaction.
    let tx = deserialize_tx_set_recent_blockhash_and_sign_message(
        api_response.transaction,
        &liquidator,
        latest_blockhash,
    )?;

    // Send liquidate transaction
    let signature = send_transaction(&tx, rpc_client.clone()).await?;
    println!("Transaction successful: {signature:?}");
    Ok(())
}
