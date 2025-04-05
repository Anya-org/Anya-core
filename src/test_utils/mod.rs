use tempfile::TempDir;
use bitcoin::Network;
use std::sync::Once;
use tracing_subscriber::FmtSubscriber;

static INIT: Once = Once::new();

pub struct TestContext {
    pub temp_dir: TempDir,
    pub network: Network,
}

impl TestContext {
    pub fn new() -> Self {
        setup_logging();
        Self {
            temp_dir: TempDir::new().unwrap(),
            network: Network::Regtest,
        }
    }
}

fn setup_logging() {
    INIT.call_once(|| {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(tracing::Level::INFO)
            .with_test_writer()
            .try_init()
            .expect("Failed to initialize logging");
    });
}

pub mod test_vectors;
pub mod mock_providers;

pub mod asset_test_utils {
    use crate::assets::{Asset, AssetMetadata, AssetType};
    use chrono::Utc;
    use std::collections::HashMap;

    pub fn create_test_metadata(name: &str, supply: u64) -> AssetMetadata {
        AssetMetadata {
            name: name.to_string(),
            supply,
            precision: 8,
            issuer: "test_issuer".to_string(),
            metadata: HashMap::new(),
            created_at: Utc::now(),
        }
    }

    pub async fn test_basic_asset_operations<A: Asset>(asset: &A) -> anyhow::Result<()> {
        let metadata = create_test_metadata("TestAsset", 1000000);
        let asset_id = asset.create(metadata.clone()).await?;
        
        // Test metadata retrieval
        let retrieved = asset.get_metadata(&asset_id).await?;
        assert_eq!(retrieved.name, metadata.name);
        assert_eq!(retrieved.supply, metadata.supply);

        // Test transfer
        asset.transfer(&asset_id, "recipient", 1000).await?;

        // Test validation
        assert!(asset.validate(&asset_id).await?);

        Ok(())
    }
}
