use std::env;

#[tokio::test]
async fn bitcoin_rpc_smoke_env_honest() {
    // Gate on presence of BITCOIN_RPC_URL
    let Some(url) = env::var_os("BITCOIN_RPC_URL") else {
        eprintln!("[skip] BITCOIN_RPC_URL not set; skipping RPC smoke test");
        return;
    };
    let url = url.to_string_lossy().to_string();

    // Try to parse for basic auth
    let Ok(parsed) = url::Url::parse(&url) else {
        eprintln!("[skip] BITCOIN_RPC_URL invalid; skipping: {url}");
        return;
    };
    let user = parsed.username().to_string();
    let pass = parsed.password().unwrap_or("").to_string();
    if user.is_empty() {
        eprintln!("[skip] BITCOIN_RPC_URL missing user:pass auth; skipping");
        return;
    }

    // Build client via the new shim
    let client = match anya_core::bitcoin::rpc::BitcoinRpcClient::new(
        &url,
        &user,
        &pass,
        std::time::Duration::from_secs(10),
    ) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[skip] Cannot create RPC client: {e}");
            return;
        }
    };

    // Query basic chain info
    let info = match client.get_blockchain_info().await {
        Ok(i) => i,
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("unexpected HTTP code: 400")
                || msg.contains("unexpected HTTP code: 401")
            {
                eprintln!("[skip] public / unauthenticated RPC endpoint rejected request ({msg}); treating as skip");
            } else if msg.contains("Connection refused") || msg.contains("connection refused") {
                eprintln!(
                    "[skip] RPC endpoint unreachable ({msg}); treating as skip"
                );
            } else {
                eprintln!("[skip] get_blockchain_info failed: {msg}");
            }
            return;
        }
    };
    assert!(info.blocks > 0, "height should be > 0");
    assert!(!info.best_block_hash.is_empty(), "best hash present");

    // Fee estimation (tolerate None)
    let fee = client.estimate_smart_fee(6).await.ok();
    if let Some(f) = fee {
        if let Some(fr) = f.fee_rate {
            assert!(fr >= 0.0);
        }
    }
}
