#!/bin/bash
# Enhance unified installer with better error handling
set -e

INSTALLER_FILE="src/bin/unified_installer.rs"

if [ ! -f "$INSTALLER_FILE" ]; then
    echo "Error: Unified installer file not found at $INSTALLER_FILE"
    exit 1
fi

# Add improved error handling to the installer
sed -i 's/fn main() -> Result<()> {/fn main() -> Result<()> {\n    // Set up improved error handling\n    if std::env::var("RUST_LOG").is_err() {\n        std::env::set_var("RUST_LOG", "info");\n    }\n    env_logger::init();\n    \n    // Handle panics gracefully\n    std::panic::set_hook(Box::new(|panic_info| {\n        error!("CRITICAL ERROR: {}", panic_info);\n        error!("Installation aborted due to unrecoverable error.");\n    }));\n/' "$INSTALLER_FILE"

# Add retry logic for network operations
sed -i '/fn install_components(&self) -> Result<()> {/a \        // Add retry logic for network-dependent operations\n        let max_retries = 3;\n        let mut retry_count = 0;\n        let mut last_error = None;\n\n        while retry_count < max_retries {\n            match self.try_install_components() {\n                Ok(_) => return Ok(()),\n                Err(e) => {\n                    retry_count += 1;\n                    warn!("Install attempt {} failed: {}", retry_count, e);\n                    last_error = Some(e);\n                    if retry_count < max_retries {\n                        let delay = std::time::Duration::from_secs(2 * retry_count as u64);\n                        warn!("Retrying in {} seconds...", delay.as_secs());\n                        std::thread::sleep(delay);\n                    }\n                }\n            }\n        }\n\n        Err(last_error.unwrap_or_else(|| anyhow!("Installation failed after {} attempts", max_retries)))\n    }\n\n    fn try_install_components(&self) -> Result<()> {' "$INSTALLER_FILE"

# Add safety check for dependencies
sed -i '/fn install_dependencies(&self) -> Result<()> {/a \        // Add safety verification of installed dependencies\n        info!("Verifying system integrity before continuing...");\n        let integrity_check = self.verify_system_integrity()?;\n        if !integrity_check {\n            return Err(anyhow!("System integrity check failed. Please run with --verify_only to diagnose."));\n        }' "$INSTALLER_FILE"

echo "Enhanced unified installer with better error handling and retry logic"
