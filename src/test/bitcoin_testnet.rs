use crate::bitcoin::endpoints::{BitcoinRpcEndpoints, DEFAULT_TESTNET_RPC};
use anyhow::Result;
use bitcoin::Network;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use std::error::Error;

/// Creates a testnet RPC client for testing Bitcoin functionality
pub fn get_testnet_client() -> Result<Client> {
    // Resolve using new centralized endpoints; allow BITCOIN_TESTNET_URL override (legacy)
    let legacy = std::env::var("BITCOIN_TESTNET_URL").ok();
    let eps = BitcoinRpcEndpoints::resolve(None, legacy.as_deref());
    let rpc_url = eps.testnet.clone();

    let rpc_auth = match std::env::var("BITCOIN_TESTNET_AUTH") {
        Ok(auth) if auth.contains(':') => {
            let mut it = auth.splitn(2, ':');
            Auth::UserPass(
                it.next().unwrap().to_string(),
                it.next().unwrap_or("").to_string(),
            )
        }
        _ => Auth::None,
    };

    Ok(Client::new(&rpc_url, rpc_auth)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_testnet_connection() -> Result<()> {
        if std::env::var("SKIP_BITCOIN_TESTS").is_ok() {
            println!("Skipping Bitcoin testnet tests");
            return Ok(());
        }

        let client = get_testnet_client()?;
        let blockchain_info = client.get_blockchain_info()?;
        assert_eq!(blockchain_info.chain, "test");
        Ok(())
    }
}
