use std::error::Error;
use std::path::{Path, PathBuf};
use std::fs;
use std::result::Result;
use crate::bitcoin::BitcoinConfig;

// BIP341 constant for silent leaf
const BIP341_SILENT_LEAF: bool = true;

// BIP Compliance information
#[derive(Debug)]
pub struct BIPCompliance {
    pub taproot_enabled: bool,
    pub schnorr_enabled: bool,
    pub psbt_version: u8,
}
pub struct ConfigManager {
    path: PathBuf,
}

impl ConfigManager {
    pub fn new(install_dir: &Path) -> Self {
        Self {
            path: install_dir.join("conf/bitcoin.conf"),
        }
    }

    pub fn generate(&self, config: &BitcoinConfig) -> Result<(), std::io::Error> {
        let content = format!(
            "network=mainnet\n\
            taproot=1\n\
            silent_leaf={}\n\
            psbt_version=2",
            BIP341_SILENT_LEAF
        );
        fs::write(&self.path, content)
    }

    pub fn validate_bips(&self) -> Result<BIPCompliance, std::io::Error> {
        let content = fs::read_to_string(&self.path)?;
        
        // Parse configuration and check for BIP compliance
        let taproot_enabled = content.contains("taproot=1");
        let schnorr_enabled = content.contains("schnorr=1") || taproot_enabled; // Taproot implies Schnorr
        
        // Extract PSBT version
        let psbt_version = if content.contains("psbt_version=") {
            let line = content.lines()
                .find(|line| line.starts_with("psbt_version="))
                .unwrap_or("psbt_version=0");
            
            line.split('=').nth(1)
                .and_then(|v| v.parse::<u8>().ok())
                .unwrap_or(0)
        } else {
            0 // Default version
        };
        
        Ok(BIPCompliance {
            taproot_enabled,
            schnorr_enabled,
            psbt_version,
        })
    }
} 
