#![feature(edition2021)]
// RGB Protocol Implementation for OPSource
// Implements client-side validated smart contracts on Bitcoin

use anyhow::{anyhow, Result};
use bitcoin::{Address, Network, OutPoint, Script, Transaction, TxIn, TxOut};
use rgb_core::{
    contract::{Contract, ContractId},
    schema::{Schema, SchemaId},
    validation::{Status as ValidationStatus, Validity},
};
use rgb_std::{
    persistence::{Inventory, Stash},
    Asset, AssetIface, AssetSchema, Disclosure, Invoice, Rgb, Transfer, seal::Revealed,
};
use secp256k1::{All, Secp256k1};
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

use crate::wallet::BitcoinWallet;

/// RGB asset manager for issuing and managing assets on Bitcoin
pub struct RgbManager {
    network: Network,
    inventory_path: PathBuf,
    secp: Secp256k1<All>,
}

/// Asset information structure
pub struct AssetInfo {
    pub contract_id: ContractId,
    pub name: String,
    pub ticker: Option<String>,
    pub description: Option<String>,
    pub total_supply: u64,
    pub issuer: Option<String>,
    pub balance: u64,
}

impl RgbManager {
    /// Create a new RGB manager
    pub fn new(network: Network, data_dir: &PathBuf) -> Result<Self> {
        let inventory_path = data_dir.join("rgb").join(network.to_string().to_lowercase());
        
        // Create directories if they don't exist
        std::fs::create_dir_all(&inventory_path)?;
        
        Ok(Self {
            network,
            inventory_path,
            secp: Secp256k1::new(),
        })
    }
    
    /// Initialize or open the RGB inventory
    fn open_inventory(&self) -> Result<Inventory> {
        Inventory::open(&self.inventory_path)
            .map_err(|e| anyhow!("Error opening RGB inventory: {}", e))
    }
    
    /// Issue a new fungible asset
    pub fn issue_asset(
        &self,
        wallet: &BitcoinWallet,
        name: &str,
        ticker: Option<&str>,
        amount: u64,
        description: Option<&str>,
    ) -> Result<AssetInfo> {
        let mut inventory = self.open_inventory()?;
        
        // Get a new address for the issuance
        let address = wallet.get_new_address()?;
        
        // Create the RGB runtime
        let mut rgb = Rgb::new(self.network)?;
        
        // Define asset schema
        let schema = AssetSchema::fungible_default();
        
        // Create asset based on the schema
        let ticker_str = ticker.unwrap_or("").to_string();
        let description_str = description.unwrap_or("").to_string();
        
        let mut asset = Asset::new(name, &ticker_str, &description_str)?;
        
        // Set the supply
        asset.set_supply(amount)?;
        
        // Issue the asset
        let contract = rgb.issue_asset(
            &self.secp,
            asset,
            &address.script_pubkey(),
            &mut inventory,
        )?;
        
        // Get the contract ID
        let contract_id = contract.contract_id();
        
        // Save to inventory
        inventory.save()?;
        
        Ok(AssetInfo {
            contract_id,
            name: name.to_string(),
            ticker: ticker.map(|t| t.to_string()),
            description: description.map(|d| d.to_string()),
            total_supply: amount,
            issuer: Some(address.to_string()),
            balance: amount,
        })
    }
    
    /// Get asset information
    pub fn get_asset_info(&self, contract_id: &str) -> Result<AssetInfo> {
        let contract_id = ContractId::from_str(contract_id)?;
        let inventory = self.open_inventory()?;
        
        let contract = inventory.get_contract(contract_id)?;
        
        // Extract asset details from the contract
        let name = contract.title().to_string();
        let data = contract.data();
        
        let ticker = data.get("ticker").map(|v| v.to_string());
        let description = data.get("description").map(|v| v.to_string());
        let total_supply = contract.total_supply()?.unwrap_or(0);
        let balance = contract.balance()?.unwrap_or(0);
        let issuer = contract.issuer_pubkey().map(|p| p.to_string());
        
        Ok(AssetInfo {
            contract_id,
            name,
            ticker,
            description,
            total_supply,
            issuer,
            balance,
        })
    }
    
    /// Transfer asset to a recipient
    pub fn transfer_asset(
        &self,
        wallet: &BitcoinWallet,
        contract_id: &str,
        recipient_address: &str,
        amount: u64,
    ) -> Result<String> {
        let contract_id = ContractId::from_str(contract_id)?;
        let mut inventory = self.open_inventory()?;
        
        // Parse recipient address
        let recipient = Address::from_str(recipient_address)?;
        
        // Create the RGB runtime
        let mut rgb = Rgb::new(self.network)?;
        
        // Create a transfer
        let invoice = Invoice::for_transfer(contract_id, amount, recipient.script_pubkey())?;
        
        // Execute the transfer
        let transfer = rgb.create_transfer(
            &self.secp,
            invoice,
            &mut inventory,
            |outpoint| wallet.get_utxo_for_outpoint(outpoint),
        )?;
        
        // Get the transaction ID
        let txid = transfer.txid.to_string();
        
        // Save to inventory
        inventory.save()?;
        
        Ok(txid)
    }
    
    /// Complete RGB asset functionality to enable transferring assets with metadata
    pub fn transfer_asset_with_metadata(
        &self,
        wallet: &BitcoinWallet,
        contract_id: &str,
        recipient_address: &str,
        amount: u64,
        metadata: HashMap<String, String>,
    ) -> Result<String> {
        let contract_id = ContractId::from_str(contract_id)?;
        let mut inventory = self.open_inventory()?;
        
        // Parse recipient address
        let recipient = Address::from_str(recipient_address)?;
        
        // Create the RGB runtime
        let mut rgb = Rgb::new(self.network)?;
        
        // Create a transfer with metadata
        let invoice = Invoice::for_transfer(contract_id, amount, recipient.script_pubkey())?;
        
        // Add metadata to the transfer
        let mut transfer_builder = rgb.transfer_builder(invoice);
        for (key, value) in metadata {
            transfer_builder.add_metadata(key, value);
        }
        
        // Execute the transfer
        let transfer = transfer_builder.finalize(
            &self.secp,
            &mut inventory,
        )?;
        
        // Create and sign transaction
        let txid = wallet.broadcast_rgb_transaction(transfer)?;
        
        // Update the RGB state
        rgb.update_state(&mut inventory)?;
        
        Ok(txid.to_string())
    }
    
    /// Batch transfer multiple assets in a single transaction
    pub fn batch_transfer_assets(
        &self,
        wallet: &BitcoinWallet,
        transfers: Vec<(String, String, u64)>, // (contract_id, recipient, amount)
    ) -> Result<String> {
        if transfers.is_empty() {
            return Err(anyhow!("No transfers provided"));
        }
        
        let mut inventory = self.open_inventory()?;
        let mut rgb = Rgb::new(self.network)?;
        
        // Create batch transfer
        let mut batch_builder = rgb.batch_transfer_builder();
        
        // Add all transfers to the batch
        for (contract_id_str, recipient_address, amount) in transfers {
            let contract_id = ContractId::from_str(&contract_id_str)?;
            let recipient = Address::from_str(&recipient_address)?;
            
            let invoice = Invoice::for_transfer(
                contract_id, 
                amount, 
                recipient.script_pubkey()
            )?;
            
            batch_builder.add_transfer(invoice)?;
        }
        
        // Finalize the batch transfer
        let batch_transfer = batch_builder.finalize(
            &self.secp,
            &mut inventory,
        )?;
        
        // Create and sign transaction
        let txid = wallet.broadcast_rgb_transaction(batch_transfer)?;
        
        // Update the RGB state
        rgb.update_state(&mut inventory)?;
        
        Ok(txid.to_string())
    }
    
    /// Verify asset ownership based on the RGB state
    pub fn verify_asset_ownership(
        &self,
        contract_id: &str,
        address: &str,
    ) -> Result<bool> {
        let contract_id = ContractId::from_str(contract_id)?;
        let address = Address::from_str(address)?;
        
        let inventory = self.open_inventory()?;
        let contract = inventory.get_contract(contract_id)?;
        
        // Check if the address owns any of this asset
        let ownership = contract.verify_ownership(address.script_pubkey())?;
        
        Ok(ownership > 0)
    }
    
    /// Validate contract status
    pub fn validate_contract(&self, contract_id: &str) -> Result<ValidationStatus> {
        let contract_id = ContractId::from_str(contract_id)?;
        let inventory = self.open_inventory()?;
        
        // Create the RGB runtime
        let rgb = Rgb::new(self.network)?;
        
        // Validate the contract
        let status = rgb.validate_contract(contract_id, &inventory)?;
        
        Ok(status)
    }
    
    /// List all assets in the inventory
    pub fn list_assets(&self) -> Result<Vec<AssetInfo>> {
        let inventory = self.open_inventory()?;
        let contracts = inventory.contracts()?;
        
        let mut assets = Vec::new();
        
        for contract in contracts {
            let contract_id = contract.contract_id();
            
            // Extract asset details from the contract
            let name = contract.title().to_string();
            let data = contract.data();
            
            let ticker = data.get("ticker").map(|v| v.to_string());
            let description = data.get("description").map(|v| v.to_string());
            let total_supply = contract.total_supply()?.unwrap_or(0);
            let balance = contract.balance()?.unwrap_or(0);
            let issuer = contract.issuer_pubkey().map(|p| p.to_string());
            
            assets.push(AssetInfo {
                contract_id,
                name,
                ticker,
                description,
                total_supply,
                issuer,
                balance,
            });
        }
        
        Ok(assets)
    }
    
    /// Test RGB asset issuance and transfer
    pub fn test(&self, wallet: &BitcoinWallet) -> Result<()> {
        println!("Testing RGB asset issuance and transfer...");
        
        // Issue a new test asset
        let asset_info = self.issue_asset(
            wallet,
            "Test Asset",
            Some("TST"),
            1000,
            Some("A test asset for RGB functionality validation")
        )?;
        
        println!("✓ Successfully issued asset: {}", asset_info.name);
        println!("  Contract ID: {}", asset_info.contract_id);
        println!("  Supply: {}", asset_info.total_supply);
        
        // Get a new address for testing transfer
        let recipient = wallet.get_new_address()?;
        
        // Transfer some of the asset
        let txid = self.transfer_asset(
            wallet,
            &asset_info.contract_id.to_string(),
            &recipient.to_string(),
            100
        )?;
        
        println!("✓ Successfully transferred asset");
        println!("  Transaction ID: {}", txid);
        
        // Validate the contract
        let status = self.validate_contract(&asset_info.contract_id.to_string())?;
        
        println!("✓ Contract validation: {:?}", status);
        
        // Get updated asset info
        let updated_info = self.get_asset_info(&asset_info.contract_id.to_string())?;
        
        println!("✓ Updated balance: {}", updated_info.balance);
        
        Ok(())
    }
}

// Implementation traits
impl AssetIface for RgbManager {
    fn network(&self) -> Network {
        self.network
    }
    
    fn inventory(&self) -> Result<Inventory> {
        self.open_inventory()
    }
}
