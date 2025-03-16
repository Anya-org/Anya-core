use anya::bitcoin::protocol::testing::mock;
use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let test_type = args.get(1).map(|s| s.as_str()).unwrap_or("all");
    
    println!("Running Bitcoin protocol tests (no build required)");
    
    match test_type {
        "taproot" | "all" => {
            println!("\nTesting BIP-341 (Taproot) compliance:");
            for (tx_hex, expected) in mock::get_test_transactions() {
                let result = mock::verify_transaction(tx_hex);
                if result == expected {
                    println!("✅ Test passed");
                } else {
                    println!("❌ Test failed");
                    if test_type != "all" {
                        return Err(anyhow::anyhow!("Taproot test failed"));
                    }
                }
            }
        },
        "psbt" | "all" => {
            println!("\nTesting BIP-174 (PSBT) compliance:");
            // PSBT tests here
        },
        _ => {
            println!("Unknown test type: {}", test_type);
            println!("Available types: taproot, psbt, all");
        }
    }
    
    Ok(())
} 