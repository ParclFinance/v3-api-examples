pub mod transaction;

pub use transaction::*;

use anyhow::{anyhow, Result};
use parcl_v3_api_client::{
    constants::DEFAULT_V3_API_URL, ParclV3ApiClient, ParclV3ApiClientConfig,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, signature::Keypair};
use std::sync::Arc;

pub fn setup_wallet_and_v3_api_client_and_rpc_client(
    commitment_config: Option<CommitmentConfig>,
) -> Result<(Keypair, ParclV3ApiClient, Arc<RpcClient>)> {
    // Setup wallet keypair
    let wallet = {
        let private_key = std::env::var("WALLET_PRIVATE_KEY")
            .map_err(|_| anyhow!("Failed to find env var: WALLET_PRIVATE_KEY"))?;
        // Specifically looking for bs58 encoded pvt key
        Keypair::from_base58_string(&private_key)
    };

    // Setup parcl-v3 api client
    let v3_api_client = {
        let base_url = std::env::var("API_URL")
            .map_err(|_| anyhow!("Failed to find env var: API_URL"))
            .unwrap_or(DEFAULT_V3_API_URL.to_string());
        ParclV3ApiClient::new(ParclV3ApiClientConfig {
            base_url,
            exchange_id: None,
            priority_fee_percentile: None,
        })
    };

    // Setup rpc api client
    let rpc_client = {
        let url = std::env::var("RPC_URL")
            .map_err(|_| anyhow!("Failed to find env var: RPC_URL"))
            .unwrap_or("https://api.mainnet-beta.solana.com".to_string());
        Arc::new(RpcClient::new_with_commitment(
            url,
            commitment_config.unwrap_or(CommitmentConfig::confirmed()),
        ))
    };
    Ok((wallet, v3_api_client, rpc_client))
}
