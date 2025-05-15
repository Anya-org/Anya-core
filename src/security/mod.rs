// AIE-001: Security Module Integration
// Exports system hardening functionality

// System hardening module
pub mod system_hardening;

// Re-exports for convenience
pub use system_hardening::SystemHardening;
pub use system_hardening::SecurityLevel;
pub use system_hardening::ConfigStatus;
pub use system_hardening::HardeningConfig;

/// Helper function to create a system hardening manager with default auto-save frequency (20)
pub fn create_system_hardening() -> SystemHardening {
    SystemHardening::new(20)
}

/// Helper function to create a basic security configuration for a component
pub fn create_basic_security_config(component_name: &str) -> std::collections::HashMap<String, String> {
    let mut settings = std::collections::HashMap::new();
    
    // Basic security settings
    settings.insert("firewall".to_string(), "enabled".to_string());
    settings.insert("encryption".to_string(), "enabled".to_string());
    settings.insert("access_control".to_string(), "strict".to_string());
    settings.insert("audit_logging".to_string(), "enabled".to_string());
    settings.insert("intrusion_detection".to_string(), "enabled".to_string());
    
    // Component-specific settings
    match component_name {
        "network" => {
            settings.insert("port_scanning_protection".to_string(), "enabled".to_string());
            settings.insert("ddos_protection".to_string(), "enabled".to_string());
        },
        "database" => {
            settings.insert("query_sanitization".to_string(), "strict".to_string());
            settings.insert("data_encryption".to_string(), "aes-256".to_string());
        },
        "api" => {
            settings.insert("rate_limiting".to_string(), "enabled".to_string());
            settings.insert("input_validation".to_string(), "strict".to_string());
        },
        _ => {
            // Generic settings for other components
            settings.insert("default_deny".to_string(), "enabled".to_string());
        }
    }
    
    settings
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_security_config() {
        let network_config = create_basic_security_config("network");
        let db_config = create_basic_security_config("database");
        
        // Check common settings
        assert_eq!(network_config.get("firewall"), Some(&"enabled".to_string()));
        assert_eq!(db_config.get("firewall"), Some(&"enabled".to_string()));
        
        // Check component-specific settings
        assert_eq!(network_config.get("ddos_protection"), Some(&"enabled".to_string()));
        assert_eq!(db_config.get("data_encryption"), Some(&"aes-256".to_string()));
    }
}

// Security module
// Implements security features for Bitcoin operations
// as per Bitcoin Development Framework v2.5 requirements

pub mod validation;

// Re-export key types
pub use validation::ValidationResult;

// Security module for Anya Core
// [AIR-3][AIS-3][AIT-3][AIP-3][RES-3]

use log::info;
use crate::security::hsm::provider::PublicKeyInfo;
use crate::security::hsm::audit::AuditFilter;

// Re-export HSM module
pub mod hsm;
pub mod constant_time;
pub mod crypto;
pub mod auth;

// Export HSM manager and related types
pub use hsm::{
    HsmManager,
    HsmStatus,
    config::HsmConfig,
    provider::{
        KeyGenParams,
        KeyType,
        KeyUsage,
        KeyInfo,
        SigningAlgorithm,
        EncryptionAlgorithm,
    },
    error::HsmError,
    audit::{
        AuditEvent,
        AuditLoggerConfig,
        AuditStorageType,
    },
    bitcoin::{
        BitcoinHsmProvider,
        BitcoinHsmConfig,
        BitcoinKeyInfo,
        BitcoinKeyType,
        BitcoinNetwork,
        BitcoinSignatureType,
        TaprootOutputInfo,
        TaprootScriptTree,
        BitcoinScriptDetails,
        BitcoinScriptType,
        BitcoinSpvProof,
        DlcInfo,
        DlcParams,
        create_dlc,
    },
};

// Other security modules - to be implemented
// pub mod authentication;
// pub mod authorization;
// pub mod compliance;
// pub mod crypto;
// pub mod secrets;
// pub mod validation;

/// Initialize the security subsystem
/// 
/// This function initializes the security subsystem, including the HSM manager
/// if configured. It follows the security requirements specified in the
/// Bitcoin Development Framework v2.5.
/// 
/// # Returns
/// `Ok(())` on success, `Err` on failure
pub async fn initialize() -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing security subsystem");
    
    // Initialize HSM if configured
    // This is just placeholder code - actual initialization would be handled by the application
    
    // let hsm_config = HsmConfig::development();
    // let hsm_manager = HsmManager::new(hsm_config);
    // hsm_manager.initialize().await?;
    
    info!("Security subsystem initialized");
    Ok(())
}

/// Create a Bitcoin HSM provider with default configuration
/// 
/// This function creates a Bitcoin HSM provider with default configuration,
/// using the specified HSM provider as the base provider.
/// 
/// # Arguments
/// * `base_provider` - Base HSM provider to use
/// 
/// # Returns
/// BitcoinHsmProvider configured for Bitcoin operations
pub fn create_bitcoin_hsm_provider(base_provider: std::sync::Arc<dyn hsm::provider::HsmProvider>) -> BitcoinHsmProvider {
    let config = BitcoinHsmConfig {
        base_provider,
        network: BitcoinNetwork::Testnet, // Default to testnet for safety
        ..Default::default()
    };
    
    BitcoinHsmProvider::new(config)
}

/// Verify a Bitcoin payment using SPV proof
/// 
/// This function verifies a Bitcoin payment using SPV proof, as described
/// in the Bitcoin Development Framework v2.5 requirements.
/// 
/// # Arguments
/// * `bitcoin_provider` - Bitcoin HSM provider
/// * `proof` - SPV proof of the payment
/// 
/// # Returns
/// `Ok(true)` if payment is valid, `Ok(false)` if not, `Err` on failure
pub async fn verify_bitcoin_payment(
    bitcoin_provider: &BitcoinHsmProvider,
    proof: BitcoinSpvProof,
) -> Result<bool, HsmError> {
    bitcoin_provider.verify_bitcoin_spv_proof(proof).await
}

/// Create a Taproot asset
/// 
/// This function creates a Taproot asset as specified in the Bitcoin Development
/// Framework v2.5 requirements, using the provided metadata.
/// 
/// # Arguments
/// * `bitcoin_provider` - Bitcoin HSM provider
/// * `metadata` - Asset metadata
/// * `supply` - Asset supply
/// 
/// # Returns
/// `Ok(asset_id)` on success, `Err` on failure
pub async fn create_taproot_asset(
    bitcoin_provider: &BitcoinHsmProvider,
    metadata: &str,
    supply: u64,
) -> Result<String, HsmError> {
    // Generate a key for the asset
    let asset_key = bitcoin_provider.generate_bitcoin_key(
        "asset",
        Some(BitcoinKeyType::Taproot),
        None,
    ).await?;
    
    // Create a simple script tree for this asset
    let script_tree = TaprootScriptTree {
        root: hsm::bitcoin::TaprootScriptNode::Leaf {
            script: format!("asset_metadata_{}", metadata),
            version: 0xc0, // Asset version
        },
    };
    
    // Create the Taproot output
    let output = bitcoin_provider.create_taproot_output(
        &asset_key.key_id,
        Some(script_tree),
    ).await?;
    
    // In a real implementation, this would create an actual Taproot Asset
    // using the RGB protocol. For now, just return the output key ID as the asset ID.
    Ok(output.output_key_id)
} 
