use std::error::Error;
use anyhow::{Context, Result};
use crate::network::validation::{NetworkValidator, NetworkValidationResult};
use crate::install::{modes::*, cluster::EnterpriseClusterManager};
use bitcoin_protocol::BIP341Validator;
use std::path::PathBuf;

/// Main installer implementation with BIP compliance
pub struct AnyaInstaller {
    network_validator: NetworkValidator,
    installation_source: InstallationSource,
    bitcoin_config: BitcoinConfig,
}

impl AnyaInstaller {
    pub fn new(
        network_config: NetworkValidationConfig,
        install_source: InstallationSource,
        bitcoin_config: BitcoinConfig
    ) -> Result<Self> {
        // Validate BIP requirements before initialization
        BIP341Validator::check_environment()?;
        
        Ok(Self {
            network_validator: NetworkValidator::new(network_config),
            installation_source: install_source,
            bitcoin_config,
        })
    }

    /// Execute full installation process
    pub async fn install(&self, target_dir: PathBuf) -> Result<()> {
        // Phase 1: Network validation
        let network_status = self.network_validator.validate_network().await?;
        self.validate_network_compliance(&network_status)?;

        // Phase 2: Source validation
        self.installation_source.validate()?;

        // Phase 3: Installation execution
        let handler = InstallationHandler::new(
            self.installation_source.clone(),
            self.bitcoin_config.clone()
        )?;
        
        handler.install(&target_dir).await?;

        // Phase 4: Post-installation cluster setup
        if let InstallationSource::EnterpriseCluster { license_key, cluster_url, psbt_contract } = &self.installation_source {
            let cluster_manager = EnterpriseClusterManager::new(
                license_key.clone(),
                cluster_url.clone(),
                psbt_contract.clone(),
                self.bitcoin_config.clone()
            )?;
            
            cluster_manager.connect().await?;
        }

        Ok(())
    }

    fn validate_network_compliance(&self, status: &NetworkValidationResult) -> Result<()> {
        // Check required BIP support based on network status
        if self.bitcoin_config.taproot_enabled && !status.connectivity.endpoints_reachable.iter().any(|(e,_)| e.contains("taproot")) {
            anyhow::bail!("Taproot-enabled network requires access to Taproot endpoints");
        }

        if status.ports.closed_ports.iter().any(|p| [8333, 18333, 8433].contains(p)) {
            anyhow::bail!("Essential Bitcoin ports blocked");
        }

        Ok(())
    }
}

/// Protocol version checker
pub mod protocol {
    use super::*;
    
    pub fn verify_bip_support(required_bips: &[u32], installed_bips: &[u32]) -> Result<()> {
        let missing: Vec<_> = required_bips
            .iter()
            .filter(|bip| !installed_bips.contains(bip))
            .collect();

        if !missing.is_empty() {
            anyhow::bail!("Missing required BIPs: {:?}", missing);
        }
        
        Ok(())
    }

    pub fn check_taproot_activation(height: u64) -> Result<()> {
        const TAPROOT_ACTIVATION_HEIGHT: u64 = 709632;  // Mainnet activation
        
        if height < TAPROOT_ACTIVATION_HEIGHT {
            anyhow::bail!("Taproot not active until block {}", TAPROOT_ACTIVATION_HEIGHT);
        }
        
        Ok(())
    }
} 
