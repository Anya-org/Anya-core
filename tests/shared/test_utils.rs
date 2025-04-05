use std::sync::Once;
use std::path::{Path, PathBuf};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::runtime::Runtime;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

static INIT: Once = Once::new();

/// Initialize the test environment
pub fn setup_test_env() {
    INIT.call_once(|| {
        // Initialize logging for tests
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::DEBUG)
            .with_test_writer()
            .init();

        // Set up test environment variables
        std::env::set_var("ENVIRONMENT", "test");
        std::env::set_var("DATABASE_URL", "sqlite::memory:");
    });
}

/// Get a Tokio runtime for async tests
pub fn get_test_runtime() -> Runtime {
    Runtime::new().expect("Failed to create Tokio runtime")
}

/// Create a temporary test directory
pub fn create_test_dir() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let test_dir = PathBuf::from(format!("test_workspace_{}", timestamp));
    fs::create_dir_all(&test_dir).expect("Failed to create test directory");
    test_dir
}

/// Clean up a test directory
pub fn cleanup_test_dir(dir: &Path) {
    if dir.exists() {
        fs::remove_dir_all(dir).expect("Failed to clean up test directory");
    }
}

/// Helper functions for simulating test data
pub mod test_data {
    use super::*;

    pub fn simulate_bitcoin_txid() -> String {
        format!("txid{:x}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs())
    }

    pub fn simulate_bitcoin_address() -> String {
        format!("bc1q{:x}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs())
    }

    pub fn simulate_did() -> String {
        format!("did:web5:{:x}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs())
    }

    pub fn simulate_rgb_asset_id() -> String {
        format!("rgb1{:x}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs())
    }
}

/// Performance testing utilities
pub mod performance {
    use super::*;
    use std::time::Instant;

    pub struct PerformanceTest {
        name: String,
        start_time: Option<Instant>,
    }

    impl PerformanceTest {
        pub fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                start_time: None,
            }
        }

        pub fn start(&mut self) {
            self.start_time = Some(Instant::now());
        }

        pub fn end(&self) -> f64 {
            if let Some(start) = self.start_time {
                start.elapsed().as_secs_f64()
            } else {
                0.0
            }
        }
    }
}