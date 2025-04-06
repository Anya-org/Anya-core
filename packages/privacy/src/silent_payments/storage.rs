use crate::{Error, Result};
use bitcoin::Network;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// Data structure for persisting Silent Payment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SilentPaymentStorage {
    /// Version of the storage format
    pub version: u8,
    
    /// Network (mainnet, testnet, etc)
    pub network: Network,
    
    /// Scan pubkey (hex encoded)
    pub scan_pubkey: String,
    
    /// Spend pubkey (hex encoded)
    pub spend_pubkey: String,
    
    /// Detected payments (serialized)
    pub payments: Vec<SerializedPaymentInfo>,
}

/// Serializable payment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedPaymentInfo {
    /// Transaction ID (hex encoded)
    pub txid: String,
    
    /// Output index
    pub vout: u32,
    
    /// Amount in satoshis
    pub amount: u64,
    
    /// Block height (if confirmed)
    pub block_height: Option<u32>,
    
    /// Whether the payment has been spent
    pub spent: bool,
}

impl SilentPaymentStorage {
    /// Save storage to file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)
                .map_err(|e| Error::FilesystemError(format!("Failed to create directory: {}", e)))?;
        }
        
        // Serialize to JSON
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| Error::SerializationError(format!("Failed to serialize data: {}", e)))?;
        
        // Write to file
        let mut file = File::create(path)
            .map_err(|e| Error::FilesystemError(format!("Failed to create file: {}", e)))?;
            
        file.write_all(json.as_bytes())
            .map_err(|e| Error::FilesystemError(format!("Failed to write to file: {}", e)))?;
        
        Ok(())
    }
    
    /// Load storage from file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        // Read file contents
        let mut file = File::open(&path)
            .map_err(|e| Error::FilesystemError(format!("Failed to open file: {}", e)))?;
            
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| Error::FilesystemError(format!("Failed to read file: {}", e)))?;
        
        // Deserialize from JSON
        let storage = serde_json::from_str(&contents)
            .map_err(|e| Error::DeserializationError(format!("Failed to deserialize data: {}", e)))?;
        
        Ok(storage)
    }
    
    /// Get default storage path
    pub fn default_storage_path() -> PathBuf {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        
        #[cfg(target_os = "windows")]
        {
            home.join(".anya").join("silent_payments").join("storage.json")
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            home.join(".anya").join("silent_payments").join("storage.json")
        }
    }
}
