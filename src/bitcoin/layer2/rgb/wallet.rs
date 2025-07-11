// RGB Wallet implementation
// This file provides wallet functionality for RGB assets

use std::collections::HashMap;

/// RGB Asset Balance
#[derive(Debug, Clone)]
pub struct AssetBalance {
    pub asset_id: String,
    pub amount: u64,
    pub confirmed: u64,
    pub pending: u64,
}

/// RGB Wallet
#[derive(Debug)]
pub struct RGBWallet {
    address: String,
    assets: HashMap<String, u64>, // asset_id -> amount
}

impl RGBWallet {
    /// Create a new RGB wallet
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
            assets: HashMap::new(),
        }
    }

    /// Get wallet address
    pub fn address(&self) -> &str {
        &self.address
    }

    /// Add asset to wallet
    pub fn add_asset(&mut self, asset_id: &str, amount: u64) {
        let current_amount = self.assets.get(asset_id).cloned().unwrap_or(0);
        self.assets
            .insert(asset_id.to_string(), current_amount + amount);
    }

    /// Transfer asset from wallet
    pub fn transfer_asset(&mut self, asset_id: &str, amount: u64) -> Result<(), &'static str> {
        let current_amount = self.assets.get(asset_id).cloned().unwrap_or(0);

        if current_amount < amount {
            return Err("Insufficient funds");
        }

        self.assets
            .insert(asset_id.to_string(), current_amount - amount);
        Ok(())
    }

    /// Get asset balance
    pub fn get_balance(&self, asset_id: &str) -> Result<u64, &'static str> {
        Ok(self.assets.get(asset_id).cloned().unwrap_or(0))
    }

    /// Get all asset balances
    pub fn get_balances(&self) -> Vec<AssetBalance> {
        self.assets
            .iter()
            .map(|(asset_id, amount)| AssetBalance {
                asset_id: asset_id.clone(),
                amount: *amount,
                confirmed: *amount, // For simplicity, all amount is confirmed
                pending: 0,
            })
            .collect()
    }
}
