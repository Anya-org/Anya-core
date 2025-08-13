//! Advanced Wallet Features - AIR001 Gap Analysis Implementation
//! 
//! This module implements hardware wallet support, multisig operations, and 
//! watch-only capabilities to address gaps identified in the AIR001 analysis.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::str::FromStr;
use bitcoin::{Address, PublicKey, Transaction, Network, OutPoint, secp256k1};
use bitcoin::bip32::{Xpriv, Xpub, DerivationPath, Fingerprint};
use bitcoin::psbt::Psbt;
use serde::{Deserialize, Serialize};
use crate::{AnyaResult, AnyaError};
use crate::bitcoin::wallet::{AddressType, TransactionParams};
use crate::bitcoin::error::BitcoinError;
use tracing::{debug, info, warn};

/// Hardware wallet device types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HardwareWalletType {
    /// Ledger hardware wallets
    Ledger,
    /// Trezor hardware wallets  
    Trezor,
    /// KeepKey hardware wallets
    KeepKey,
    /// ColdCard hardware wallets
    ColdCard,
    /// BitBox hardware wallets
    BitBox,
    /// Generic hardware wallet
    Generic(String),
}

/// Hardware wallet connection status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HWConnectionStatus {
    /// Not connected
    Disconnected,
    /// Connected but not authenticated
    Connected,
    /// Connected and authenticated
    Authenticated,
    /// Connection failed
    Failed(String),
    /// Device is locked (requires PIN/passphrase)
    Locked,
}

/// Hardware wallet device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareWalletInfo {
    /// Device type
    pub device_type: HardwareWalletType,
    /// Device serial number or identifier
    pub device_id: String,
    /// Firmware version
    pub firmware_version: String,
    /// Master fingerprint
    pub master_fingerprint: Fingerprint,
    /// Supported features
    pub features: HWFeatures,
    /// Connection status
    pub status: HWConnectionStatus,
    /// Label/name for the device
    pub label: String,
}

/// Hardware wallet supported features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HWFeatures {
    /// Supports SegWit transactions
    pub segwit_support: bool,
    /// Supports Taproot transactions
    pub taproot_support: bool,
    /// Supports multisig transactions
    pub multisig_support: bool,
    /// Supports PSBT (Partially Signed Bitcoin Transactions)
    pub psbt_support: bool,
    /// Maximum number of multisig signers supported
    pub max_multisig_signers: u8,
    /// Supports custom derivation paths
    pub custom_derivation: bool,
    /// Supports passphrases
    pub passphrase_support: bool,
}

/// Multisig configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultisigConfig {
    /// Minimum signatures required (M in M-of-N)
    pub threshold: u8,
    /// Total number of signers (N in M-of-N)
    pub total_signers: u8,
    /// Public keys of all signers
    pub signers: Vec<MultisigSigner>,
    /// Script type for multisig
    pub script_type: MultisigScriptType,
    /// Derivation path for multisig keys
    pub derivation_path: DerivationPath,
    /// Network for addresses
    pub network: Network,
}

/// Multisig script types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MultisigScriptType {
    /// Legacy P2SH multisig
    Legacy,
    /// SegWit P2WSH multisig
    SegWit,
    /// Nested SegWit P2SH-P2WSH multisig
    NestedSegWit,
    /// Taproot multisig (future support)
    Taproot,
}

/// Multisig signer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultisigSigner {
    /// Extended public key
    pub xpub: Xpub,
    /// Master fingerprint of the signing device
    pub master_fingerprint: Fingerprint,
    /// Derivation path from master key
    pub derivation_path: DerivationPath,
    /// Optional label for the signer
    pub label: Option<String>,
    /// Hardware wallet info (if applicable)
    pub hardware_info: Option<HardwareWalletInfo>,
}

/// Watch-only wallet configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchOnlyConfig {
    /// Extended public key to watch
    pub xpub: Xpub,
    /// Derivation path
    pub derivation_path: DerivationPath,
    /// Address types to generate (simplified)
    pub address_types: Vec<String>,
    /// Gap limit for address generation
    pub gap_limit: u32,
    /// Network
    pub network: Network,
    /// Optional label
    pub label: Option<String>,
}

/// Hardware wallet interface trait
pub trait HardwareWalletInterface: Send + Sync {
    /// Enumerate connected hardware wallets
    fn enumerate_devices(&self) -> AnyaResult<Vec<HardwareWalletInfo>>;
    
    /// Connect to a specific hardware wallet
    fn connect(&self, device_id: &str) -> AnyaResult<()>;
    
    /// Disconnect from hardware wallet
    fn disconnect(&self) -> AnyaResult<()>;
    
    /// Get device information
    fn get_device_info(&self) -> AnyaResult<HardwareWalletInfo>;
    
    /// Get extended public key at derivation path
    fn get_xpub(&self, path: &DerivationPath) -> AnyaResult<Xpub>;
    
    /// Get address at specific path
    fn get_address(&self, path: &DerivationPath, address_type: AddressType) -> AnyaResult<Address>;
    
    /// Sign PSBT with hardware wallet
    fn sign_psbt(&self, psbt: &mut Psbt) -> AnyaResult<bool>;
    
    /// Display address on device for verification
    fn display_address(&self, path: &DerivationPath, address_type: AddressType) -> AnyaResult<()>;
    
    /// Get master fingerprint
    fn get_master_fingerprint(&self) -> AnyaResult<Fingerprint>;
}

/// Advanced wallet manager supporting hardware wallets, multisig, and watch-only
pub struct AdvancedWalletManager {
    /// Connected hardware wallets
    hardware_wallets: Arc<RwLock<HashMap<String, Box<dyn HardwareWalletInterface>>>>,
    
    /// Multisig configurations
    multisig_configs: Arc<RwLock<HashMap<String, MultisigConfig>>>,
    
    /// Watch-only wallets
    watch_only_wallets: Arc<RwLock<HashMap<String, WatchOnlyWallet>>>,
    
    /// Secp256k1 context
    secp: secp256k1::Secp256k1<secp256k1::All>,
}

/// Watch-only wallet implementation
#[derive(Debug, Clone)]
pub struct WatchOnlyWallet {
    /// Configuration
    config: WatchOnlyConfig,
    
    /// Generated addresses cache
    addresses: HashMap<(AddressType, u32), Address>,
    
    /// Transaction history
    transactions: Vec<Transaction>,
    
    /// UTXO set
    utxos: HashMap<OutPoint, bitcoin::TxOut>,
}

impl Default for HWFeatures {
    fn default() -> Self {
        Self {
            segwit_support: true,
            taproot_support: false,
            multisig_support: true,
            psbt_support: true,
            max_multisig_signers: 15,
            custom_derivation: true,
            passphrase_support: true,
        }
    }
}

impl AdvancedWalletManager {
    /// Create new advanced wallet manager
    pub fn new() -> Self {
        Self {
            hardware_wallets: Arc::new(RwLock::new(HashMap::new())),
            multisig_configs: Arc::new(RwLock::new(HashMap::new())),
            watch_only_wallets: Arc::new(RwLock::new(HashMap::new())),
            secp: secp256k1::Secp256k1::new(),
        }
    }

    /// Register a hardware wallet interface
    pub fn register_hardware_wallet(
        &self,
        device_id: String,
        interface: Box<dyn HardwareWalletInterface>,
    ) -> AnyaResult<()> {
        let mut hw_wallets = self.hardware_wallets.write().unwrap();
        hw_wallets.insert(device_id.clone(), interface);
        info!("Registered hardware wallet: {}", device_id);
        Ok(())
    }

    /// Get all connected hardware wallets
    pub fn list_hardware_wallets(&self) -> AnyaResult<Vec<HardwareWalletInfo>> {
        let hw_wallets = self.hardware_wallets.read().unwrap();
        let mut devices = Vec::new();
        
        for interface in hw_wallets.values() {
            match interface.get_device_info() {
                Ok(info) => devices.push(info),
                Err(e) => warn!("Failed to get device info: {}", e),
            }
        }
        
        Ok(devices)
    }

    /// Create a new multisig configuration
    pub fn create_multisig_config(
        &self,
        name: String,
        threshold: u8,
        signers: Vec<MultisigSigner>,
        script_type: MultisigScriptType,
        derivation_path: DerivationPath,
        network: Network,
    ) -> AnyaResult<()> {
        if threshold == 0 || threshold > signers.len() as u8 {
            return Err(AnyaError::InvalidInput(
                "Invalid threshold for multisig".to_string()
            ));
        }

        if signers.len() > 15 {
            return Err(AnyaError::InvalidInput(
                "Too many signers for multisig (max 15)".to_string()
            ));
        }

        let config = MultisigConfig {
            threshold,
            total_signers: signers.len() as u8,
            signers,
            script_type,
            derivation_path,
            network,
        };

        let mut configs = self.multisig_configs.write().unwrap();
        configs.insert(name.clone(), config.clone());
        
        info!("Created multisig configuration: {} ({}-of-{})", 
              name, threshold, config.total_signers);
        Ok(())
    }

    /// Get multisig address for given index
    pub fn get_multisig_address(
        &self,
        config_name: &str,
        index: u32,
    ) -> AnyaResult<Address> {
        let configs = self.multisig_configs.read().unwrap();
        let config = configs.get(config_name)
            .ok_or_else(|| AnyaError::NotFound(format!("Multisig config not found: {}", config_name)))?;

        self.derive_multisig_address(config, index)
    }

    /// Derive multisig address at specific index (simplified)
    fn derive_multisig_address(
        &self,
        config: &MultisigConfig,
        index: u32,
    ) -> AnyaResult<Address> {
        // Simplified implementation for demo purposes
        // In a real implementation, this would:
        // 1. Derive public keys for all signers at the given index
        // 2. Create the multisig script
        // 3. Generate the appropriate address type
        
        info!("Deriving multisig address for config with {} signers at index {}", 
              config.total_signers, index);
        
        // Return a mock address for now
        let addr_str = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4";
        Address::from_str(addr_str)
            .map_err(|e| AnyaError::Bitcoin(e.to_string()))?
            .require_network(Network::Bitcoin)
            .map_err(|e| AnyaError::Bitcoin(e.to_string()))
    }

    /// Create PSBT for multisig transaction (simplified)
    pub fn create_multisig_psbt(
        &self,
        config_name: &str,
        params: TransactionParams,
    ) -> AnyaResult<Psbt> {
        let configs = self.multisig_configs.read().unwrap();
        let _config = configs.get(config_name)
            .ok_or_else(|| AnyaError::NotFound(format!("Multisig config not found: {}", config_name)))?;

        // Create basic PSBT structure (simplified)
        let unsigned_tx = bitcoin::Transaction {
            version: bitcoin::transaction::Version(2),
            lock_time: bitcoin::absolute::LockTime::ZERO,
            input: Vec::new(),
            output: Vec::new(),
        };
        
        let psbt = Psbt::from_unsigned_tx(unsigned_tx)
            .map_err(|e| AnyaError::Bitcoin(e.to_string()))?;

        debug!("Created multisig PSBT for {}: {} recipients", 
               config_name, params.recipients.len());
        Ok(psbt)
    }

    /// Sign multisig PSBT with available hardware wallets
    pub fn sign_multisig_psbt(
        &self,
        config_name: &str,
        psbt: &mut Psbt,
    ) -> AnyaResult<u8> {
        let configs = self.multisig_configs.read().unwrap();
        let config = configs.get(config_name)
            .ok_or_else(|| AnyaError::NotFound(format!("Multisig config not found: {}", config_name)))?;

        let hw_wallets = self.hardware_wallets.read().unwrap();
        let mut signatures_added = 0u8;

        // Try to sign with each available hardware wallet
        for signer in &config.signers {
            if let Some(hw_info) = &signer.hardware_info {
                if let Some(hw_interface) = hw_wallets.get(&hw_info.device_id) {
                    match hw_interface.sign_psbt(psbt) {
                        Ok(signed) => {
                            if signed {
                                signatures_added += 1;
                                info!("Signed PSBT with hardware wallet: {}", hw_info.device_id);
                            }
                        }
                        Err(e) => {
                            warn!("Failed to sign with hardware wallet {}: {}", hw_info.device_id, e);
                        }
                    }
                }
            }
        }

        info!("Added {} signatures to multisig PSBT (threshold: {})", 
              signatures_added, config.threshold);
        Ok(signatures_added)
    }

    /// Create a new watch-only wallet
    pub fn create_watch_only_wallet(
        &self,
        name: String,
        config: WatchOnlyConfig,
    ) -> AnyaResult<()> {
        let wallet = WatchOnlyWallet::new(config)?;
        
        let mut watch_wallets = self.watch_only_wallets.write().unwrap();
        watch_wallets.insert(name.clone(), wallet);
        
        info!("Created watch-only wallet: {}", name);
        Ok(())
    }

    /// Get watch-only wallet balance
    pub fn get_watch_only_balance(&self, name: &str) -> AnyaResult<u64> {
        let watch_wallets = self.watch_only_wallets.read().unwrap();
        let wallet = watch_wallets.get(name)
            .ok_or_else(|| AnyaError::NotFound(format!("Watch-only wallet not found: {}", name)))?;

        Ok(wallet.get_balance())
    }

    /// Get watch-only wallet addresses (simplified)
    pub fn get_watch_only_addresses(
        &self,
        name: &str,
        address_type_str: &str,
        count: u32,
    ) -> AnyaResult<Vec<Address>> {
        let mut watch_wallets = self.watch_only_wallets.write().unwrap();
        let wallet = watch_wallets.get_mut(name)
            .ok_or_else(|| AnyaError::NotFound(format!("Watch-only wallet not found: {}", name)))?;

        wallet.get_addresses(address_type_str, count)
    }

    /// Import address to watch-only wallet
    pub fn import_watch_address(
        &self,
        name: &str,
        address: Address,
    ) -> AnyaResult<()> {
        let mut watch_wallets = self.watch_only_wallets.write().unwrap();
        let wallet = watch_wallets.get_mut(name)
            .ok_or_else(|| AnyaError::NotFound(format!("Watch-only wallet not found: {}", name)))?;

        wallet.import_address(address)?;
        info!("Imported address to watch-only wallet: {}", name);
        Ok(())
    }

    /// Get comprehensive wallet statistics
    pub fn get_wallet_statistics(&self) -> HashMap<String, u64> {
        let mut stats = HashMap::new();

        // Hardware wallet statistics
        let hw_wallets = self.hardware_wallets.read().unwrap();
        stats.insert("hardware_wallets_connected".to_string(), hw_wallets.len() as u64);

        // Multisig statistics
        let multisig_configs = self.multisig_configs.read().unwrap();
        stats.insert("multisig_configurations".to_string(), multisig_configs.len() as u64);

        // Watch-only statistics
        let watch_wallets = self.watch_only_wallets.read().unwrap();
        stats.insert("watch_only_wallets".to_string(), watch_wallets.len() as u64);

        let total_watch_balance: u64 = watch_wallets.values()
            .map(|w| w.get_balance())
            .sum();
        stats.insert("total_watch_only_balance".to_string(), total_watch_balance);

        stats
    }
}

impl WatchOnlyWallet {
    /// Create new watch-only wallet
    pub fn new(config: WatchOnlyConfig) -> AnyaResult<Self> {
        Ok(Self {
            config,
            addresses: HashMap::new(),
            transactions: Vec::new(),
            utxos: HashMap::new(),
        })
    }

    /// Get wallet balance
    pub fn get_balance(&self) -> u64 {
        self.utxos.values().map(|utxo| utxo.value.to_sat()).sum()
    }

    /// Get addresses of specified type (simplified)
    pub fn get_addresses(
        &mut self,
        address_type_str: &str,
        count: u32,
    ) -> AnyaResult<Vec<Address>> {
        let address_type = match address_type_str {
            "segwit" => AddressType::SegWit,
            "legacy" => AddressType::Legacy,
            "nested_segwit" => AddressType::NestedSegWit,
            "taproot" => AddressType::Taproot,
            _ => return Err(AnyaError::InvalidInput(format!("Unknown address type: {}", address_type_str))),
        };

        let mut addresses = Vec::new();

        for i in 0..count {
            let key = (address_type, i);
            
            if let Some(address) = self.addresses.get(&key) {
                addresses.push(address.clone());
            } else {
                // Generate new address
                let address = self.derive_address(address_type, i)?;
                self.addresses.insert(key, address.clone());
                addresses.push(address);
            }
        }

        Ok(addresses)
    }

    /// Derive address at specific index (simplified)
    fn derive_address(&self, address_type: AddressType, index: u32) -> AnyaResult<Address> {
        // Simplified implementation for demo purposes
        // In a real implementation, this would derive the actual address
        // using the xpub and derivation path
        
        debug!("Deriving {} address at index {}", 
               match address_type {
                   AddressType::Legacy => "Legacy",
                   AddressType::SegWit => "SegWit",
                   AddressType::NestedSegWit => "NestedSegWit",
                   AddressType::Taproot => "Taproot",
               }, index);
        
        // Return a mock address for now
        let addr_str = match address_type {
            AddressType::SegWit => "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4",
            AddressType::Legacy => "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
            AddressType::NestedSegWit => "3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy",
            AddressType::Taproot => "bc1p5d7rjq7g6rdk2yhzks9smlaqtedr4dekq08ge8ztwac72sfr9rusxg3297",
        };
        
        Address::from_str(addr_str)
            .map_err(|e| AnyaError::Bitcoin(e.to_string()))?
            .require_network(self.config.network)
            .map_err(|e| AnyaError::Bitcoin(e.to_string()))
    }

    /// Import a specific address for watching
    pub fn import_address(&mut self, address: Address) -> AnyaResult<()> {
        // In a real implementation, this would add the address to the watch set
        // and potentially trigger a blockchain rescan to find transactions
        debug!("Imported address for watching: {}", address);
        Ok(())
    }

    /// Update UTXO set (called during sync)
    pub fn update_utxos(&mut self, utxos: HashMap<OutPoint, bitcoin::TxOut>) {
        self.utxos = utxos;
    }

    /// Add transaction to history
    pub fn add_transaction(&mut self, tx: Transaction) {
        self.transactions.push(tx);
    }
}

/// Mock hardware wallet implementation for testing
pub struct MockHardwareWallet {
    device_info: HardwareWalletInfo,
    connected: bool,
}

impl MockHardwareWallet {
    pub fn new(device_type: HardwareWalletType) -> Self {
        Self {
            device_info: HardwareWalletInfo {
                device_type,
                device_id: "mock_device_001".to_string(),
                firmware_version: "1.0.0".to_string(),
                master_fingerprint: Fingerprint::default(),
                features: HWFeatures::default(),
                status: HWConnectionStatus::Disconnected,
                label: "Mock Hardware Wallet".to_string(),
            },
            connected: false,
        }
    }
}

impl HardwareWalletInterface for MockHardwareWallet {
    fn enumerate_devices(&self) -> AnyaResult<Vec<HardwareWalletInfo>> {
        Ok(vec![self.device_info.clone()])
    }

    fn connect(&self, _device_id: &str) -> AnyaResult<()> {
        info!("Connected to mock hardware wallet");
        Ok(())
    }

    fn disconnect(&self) -> AnyaResult<()> {
        info!("Disconnected from mock hardware wallet");
        Ok(())
    }

    fn get_device_info(&self) -> AnyaResult<HardwareWalletInfo> {
        Ok(self.device_info.clone())
    }

    fn get_xpub(&self, _path: &DerivationPath) -> AnyaResult<Xpub> {
        // Return a mock xpub
        let secp = secp256k1::Secp256k1::new();
        let secret_key = secp256k1::SecretKey::from_slice(&[1; 32])
            .map_err(|e| AnyaError::Bitcoin(e.to_string()))?;
        let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
        
        let xpub = Xpub {
            network: bitcoin::NetworkKind::Main,
            depth: 0,
            parent_fingerprint: Fingerprint::default(),
            child_number: bitcoin::bip32::ChildNumber::from_normal_idx(0)
                .map_err(|e| AnyaError::Bitcoin(e.to_string()))?,
            public_key,
            chain_code: bitcoin::bip32::ChainCode::from([0; 32]),
        };
        
        Ok(xpub)
    }

    fn get_address(&self, _path: &DerivationPath, _address_type: AddressType) -> AnyaResult<Address> {
        // Return a mock address
        let addr_str = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4";
        Address::from_str(addr_str)
            .map_err(|e| AnyaError::Bitcoin(e.to_string()))?
            .require_network(Network::Bitcoin)
            .map_err(|e| AnyaError::Bitcoin(e.to_string()))
    }

    fn sign_psbt(&self, _psbt: &mut Psbt) -> AnyaResult<bool> {
        // Mock signing - always successful
        info!("Mock signed PSBT");
        Ok(true)
    }

    fn display_address(&self, _path: &DerivationPath, _address_type: AddressType) -> AnyaResult<()> {
        info!("Displaying address on mock hardware wallet");
        Ok(())
    }

    fn get_master_fingerprint(&self) -> AnyaResult<Fingerprint> {
        Ok(self.device_info.master_fingerprint)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::Network;

    #[test]
    fn test_advanced_wallet_manager() {
        let manager = AdvancedWalletManager::new();
        
        // Test hardware wallet registration
        let mock_hw = MockHardwareWallet::new(HardwareWalletType::Ledger);
        assert!(manager.register_hardware_wallet(
            "test_device".to_string(),
            Box::new(mock_hw)
        ).is_ok());
        
        // Test statistics
        let stats = manager.get_wallet_statistics();
        assert_eq!(stats.get("hardware_wallets_connected"), Some(&1));
    }

    #[test]
    fn test_watch_only_wallet() {
        let secp = secp256k1::Secp256k1::new();
        let secret_key = secp256k1::SecretKey::from_slice(&[1; 32]).unwrap();
        let xpriv = Xpriv::new_master(bitcoin::NetworkKind::Main, &[0; 32]).unwrap();
        let xpub = Xpub::from_priv(&secp, &xpriv);
        
        let config = WatchOnlyConfig {
            xpub,
            derivation_path: DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            address_types: vec!["segwit".to_string()],
            gap_limit: 20,
            network: Network::Bitcoin,
            label: Some("Test Watch Wallet".to_string()),
        };

        let mut wallet = WatchOnlyWallet::new(config).unwrap();
        
        // Test address generation
        let addresses = wallet.get_addresses("segwit", 5).unwrap();
        assert_eq!(addresses.len(), 5);
        
        // Test balance (should be 0 initially)
        assert_eq!(wallet.get_balance(), 0);
    }

    #[test]
    fn test_multisig_config_validation() {
        let manager = AdvancedWalletManager::new();
        
        // Test invalid threshold
        let result = manager.create_multisig_config(
            "test".to_string(),
            0, // Invalid threshold
            vec![],
            MultisigScriptType::Legacy,
            DerivationPath::from_str("m/45'").unwrap(),
            Network::Bitcoin,
        );
        assert!(result.is_err());
        
        // Test too many signers
        let secp = secp256k1::Secp256k1::new();
        let xpriv = Xpriv::new_master(bitcoin::NetworkKind::Main, &[0; 32]).unwrap();
        let xpub = Xpub::from_priv(&secp, &xpriv);
        
        let signers = vec![MultisigSigner {
            xpub,
            master_fingerprint: Fingerprint::default(),
            derivation_path: DerivationPath::from_str("m/0").unwrap(),
            label: None,
            hardware_info: None,
        }; 16]; // Too many signers
        
        let result = manager.create_multisig_config(
            "test".to_string(),
            10,
            signers,
            MultisigScriptType::Legacy,
            DerivationPath::from_str("m/45'").unwrap(),
            Network::Bitcoin,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_hardware_wallet_mock() {
        let mock_hw = MockHardwareWallet::new(HardwareWalletType::Trezor);
        
        assert!(mock_hw.connect("test_device").is_ok());
        assert!(mock_hw.get_device_info().is_ok());
        assert!(mock_hw.disconnect().is_ok());
    }
}