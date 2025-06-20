use anya_core::enterprise::{NostrClient, NostrConfig, NostrUserProfile};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Subscribe with existing key
    let profile = NostrUserProfile::subscribe_with_key(
        "nsec1...", // Replace with your private key
        None,       // Use default relays
    )
    .await?;

    // Initialize Nostr client
    let config = NostrConfig {
        private_key: profile.to_nsec()?,
        relays: vec![
            "wss://relay.damus.io".to_string(),
            "wss://relay.nostr.info".to_string(),
            "wss://nostr-pub.wellorder.net".to_string(),
        ],
        metadata: None,
        // default_kind: 1,
        // pow_difficulty: 0,
    };

    let client = NostrClient::new(config).await?;

    // Send encrypted message
    client
        .send_message("Hello, this is an encrypted message!", Some("recipient_pubkey"))
        .await?;

    // Publish public note (commented out - API not yet implemented)
    // let event = client.create_text_note("Hello Nostr world!")?;
    // client.publish_event_to_best_relays(event).await?;

    // Monitor relay health (commented out - API not yet implemented)
    // for relay in client.get_healthy_relays().await? {
    //     println!("Healthy relay: {}", relay);
    //     println!(
    //         "Health score: {}",
    //         client.get_relay_health_score(&relay).await?
    //     );
    // }
    
    println!("Nostr client example completed successfully!");

    Ok(())
}
