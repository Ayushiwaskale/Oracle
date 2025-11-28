use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use anyhow::Result;

pub struct SwitchboardClient {
    rpc: RpcClient,
}

impl SwitchboardClient {
    pub fn new(rpc_url: &str) -> Self {
        Self { rpc: RpcClient::new(rpc_url.to_string()) }
    }

    pub async fn get_price(&self, _aggregator: &Pubkey) -> Result<(f64,f64,i64)> {
        Err(anyhow::anyhow!("Switchboard client not implemented"))
    }
}
