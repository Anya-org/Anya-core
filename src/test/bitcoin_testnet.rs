use std::error::Error;
use bitcoin::Network;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use anyhow::Result;

/// Creates a testnet RPC client for testing Bitcoin functionality
pub fn get_testnet_client() -> Result<Client> {
    // Use public testnet nodes instead of local Bitcoin Core
    let rpc_url = std::env::var("BITCOIN_TESTNET_URL")
        .unwrap_or_else(|_| "https://testnet-btc.getblock.io/".to_string());
    
    let rpc_auth = match std::env::var("BITCOIN_TESTNET_AUTH") {
        Ok(auth) => Auth::UserPass(auth.split(':').next().unwrap_or("").to_string(), 
                                   auth.split(':').nth(1).unwrap_or("").to_string()),
        Err(_) => Auth::None
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
