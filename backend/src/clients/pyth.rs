use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use anyhow::{Result, anyhow};

pub struct PythClient {
    rpc: RpcClient,
}

impl PythClient {
    pub fn new(rpc_url: &str) -> Self {
        Self { rpc: RpcClient::new(rpc_url.to_string()) }
    }

    pub async fn get_price_with_confidence(&self, _price_feed: &Pubkey) -> Result<(f64,f64,i64)> {
        Err(anyhow!("pyth client not implemented in this dev build"))
    }
}
