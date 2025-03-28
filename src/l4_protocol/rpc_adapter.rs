#![feature(edition2021)]
// [BPC-3][AIS-3] Public RPC Integration
use crate::error::ProtocolError;
use reqwest::{Client, Url};
use serde_json::json;

pub struct PublicRPCAdapter {
    endpoints: Vec<Url>,
    client: Client,
    current_index: usize,
}

impl PublicRPCAdapter {
    pub fn new(network: Network) -> Self {
        let endpoints = match network {
            Network::Mainnet => vec![
                Url::parse("https://blockstream.info/api/").unwrap(),
                Url::parse("https://mempool.space/api/").unwrap(),
            ],
            Network::Testnet => vec![
                Url::parse("https://blockstream.info/testnet/api/").unwrap(),
                Url::parse("https://mempool.space/testnet/api/").unwrap(),
            ],
        };

        Self {
            endpoints,
            client: Client::new(),
            current_index: 0,
        }
    }

    /// Load-balanced RPC call with automatic failover
    pub async fn call(&mut self, method: &str, params: &[serde_json::Value]) -> Result<serde_json::Value, ProtocolError> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": "anya-l4",
            "method": method,
            "params": params
        });

        for _ in 0..self.endpoints.len() {
            let endpoint = &self.endpoints[self.current_index];
            self.current_index = (self.current_index + 1) % self.endpoints.len();
            
            match self.client.post(endpoint.clone())
                .json(&payload)
                .send()
                .await
            {
                Ok(response) if response.status().is_success() => {
                    return response.json().await.map_err(ProtocolError::from);
                },
                _ => continue,
            }
        }
        
        Err(ProtocolError::RpcConnectionError)
    }

    // [BIP-341] Taproot-specific methods
    pub async fn get_taproot_output(&self, txid: &str) -> Result<Value, ProtocolError> {
        self.call("gettxout", &[json!(txid), json!(0)]).await
    }
} 