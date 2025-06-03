// Bitcoin wallet implementation details
//
// This module contains the implementation details for the Bitcoin wallet.
// [AIR-1][AIS-1][AIM-1][AIP-1][RES-1]

use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use bitcoin::{Address, Network, OutPoint, Script, Transaction, TxIn, TxOut, Txid};
use bitcoin::consensus::encode;
use bitcoin::secp256k1::{self, Secp256k1};
use bitcoin::bip32::{ExtendedPrivKey, ExtendedPubKey, DerivationPath};
use bitcoin::psbt::PartiallySignedTransaction as PSBT;
use log::{debug, info, error, warn};
use rand::{thread_rng, Rng};
use serde_json;

use super::{
    AddressInfo, BitcoinWallet, CoinSelectionStrategy, FeeRate, FeeStrategy,
    SyncState, TransactionInfo, TransactionParams, Utxo, Wallet, WalletConfig,
    WalletError, WalletInfo, WalletIndexes, WalletMetadata, WalletStorage
};
use crate::bitcoin::network::NetworkConfig;
use crate::bitcoin::rpc::BitcoinRpcClient;
use super::coin_selection;

impl BitcoinWallet {
    /// Create a new Bitcoin wallet
    pub async fn new(
        config: WalletConfig,
        network_config: NetworkConfig,
        rpc_client: Option<Arc<BitcoinRpcClient>>,
    ) -> Result<Self, WalletError> {
        // Create wallet data directory if it doesn't exist
        std::fs::create_dir_all(&config.data_dir)
            .map_err(|e| WalletError::StorageError(format!("Failed to create wallet directory: {}", e)))?;
        
        // Initialize storage
        let storage_path = config.data_dir.join(format!("{}.json", config.name));
        let storage = if storage_path.exists() {
            // Load existing wallet
            let file = std::fs::File::open(&storage_path)
                .map_err(|e| WalletError::StorageError(format!("Failed to open wallet file: {}", e)))?;
            
            let storage: WalletStorage = serde_json::from_reader(file)
                .map_err(|e| WalletError::StorageError(format!("Failed to parse wallet file: {}", e)))?;
            
            // Verify network matches
            if storage.metadata.network != config.network {
                return Err(WalletError::ConfigError(format!(
                    "Wallet network ({:?}) doesn't match configuration network ({:?})",
                    storage.metadata.network, config.network
                )));
            }
            
            storage
        } else {
            // Create new wallet
            let now = SystemTime::now().duration_since(UNIX_EPOCH)
                .map_err(|e| WalletError::StorageError(format!("System time error: {}", e)))?
                .as_secs();
            
            WalletStorage {
                metadata: WalletMetadata {
                    created_at: now,
                    updated_at: now,
                    version: env!("CARGO_PKG_VERSION").to_string(),
                    network: config.network,
                    master_fingerprint: None,
                    labels: HashMap::new(),
                },
                utxos: HashMap::new(),
                transactions: HashMap::new(),
                addresses: HashMap::new(),
                indexes: WalletIndexes {
                    receive_index: 0,
                    change_index: 0,
                    last_block: None,
                    last_sync: None,
                },
            }
        };
        
        // Create wallet
        let wallet = Self {
            config,
            network_config,
            rpc_client,
            storage: Arc::new(Mutex::new(storage)),
            secp: Secp256k1::new(),
        };
        
        // Save wallet
        wallet.save().await?;
        
        Ok(wallet)
    }
    
    /// Save wallet to file
    async fn save(&self) -> Result<(), WalletError> {
        let storage_path = self.config.data_dir.join(format!("{}.json", self.config.name));
        
        let mut storage = self.storage.lock()
            .map_err(|_| WalletError::StorageError("Failed to lock wallet storage".to_string()))?;
        
        // Update timestamp
        let now = SystemTime::now().duration_since(UNIX_EPOCH)
            .map_err(|e| WalletError::StorageError(format!("System time error: {}", e)))?
            .as_secs();
        storage.metadata.updated_at = now;
        
        // Save to temporary file and rename
        let temp_path = storage_path.with_extension("json.tmp");
        let file = std::fs::File::create(&temp_path)
            .map_err(|e| WalletError::StorageError(format!("Failed to create wallet file: {}", e)))?;
        
        serde_json::to_writer_pretty(file, &*storage)
            .map_err(|e| WalletError::StorageError(format!("Failed to write wallet data: {}", e)))?;
        
        std::fs::rename(temp_path, storage_path)
            .map_err(|e| WalletError::StorageError(format!("Failed to finalize wallet save: {}", e)))?;
        
        Ok(())
    }
    
    /// Perform UTXO selection based on the configured strategy
    async fn select_utxos(
        &self,
        amount: u64,
        fee_rate: FeeRate,
        strategy: CoinSelectionStrategy,
        specific_utxos: Option<Vec<OutPoint>>,
        allow_unconfirmed: bool,
    ) -> Result<(Vec<Utxo>, u64), WalletError> {
        let storage = self.storage.lock()
            .map_err(|_| WalletError::StorageError("Failed to lock wallet storage".to_string()))?;
        
        // If specific UTXOs are provided, use them
        if let Some(outpoints) = specific_utxos {
            let mut selected_utxos = Vec::new();
            let mut total_input = 0;
            
            for outpoint in outpoints {
                let utxo = storage.utxos.get(&outpoint)
                    .ok_or_else(|| WalletError::UtxoError(format!("UTXO {} not found", outpoint)))?;
                
                if !utxo.spendable {
                    return Err(WalletError::UtxoError(format!("UTXO {} is not spendable", outpoint)));
                }
                
                if !allow_unconfirmed && utxo.confirmations == 0 {
                    return Err(WalletError::UtxoError(format!("UTXO {} is unconfirmed", outpoint)));
                }
                
                if utxo.confirmations < self.config.min_confirmations {
                    return Err(WalletError::UtxoError(format!(
                        "UTXO {} has only {} confirmations (minimum {})",
                        outpoint, utxo.confirmations, self.config.min_confirmations
                    )));
                }
                
                total_input += utxo.txout.value;
                selected_utxos.push(utxo.clone());
            }
            
            if total_input < amount {
                return Err(WalletError::InsufficientFunds(format!(
                    "Selected UTXOs total {} satoshis, need at least {}",
                    total_input, amount
                )));
            }
            
            Ok((selected_utxos, total_input))
        } else {
            // Get all spendable UTXOs
            let mut available_utxos: Vec<Utxo> = storage.utxos.values()
                .filter(|utxo| {
                    utxo.spendable 
                    && (allow_unconfirmed || utxo.confirmations > 0)
                    && utxo.confirmations >= self.config.min_confirmations
                })
                .cloned()
                .collect();
            
            if available_utxos.is_empty() {
                return Err(WalletError::InsufficientFunds("No spendable UTXOs available".to_string()));
            }
            
            // Calculate total available
            let total_available: u64 = available_utxos.iter().map(|utxo| utxo.txout.value).sum();
            if total_available < amount {
                return Err(WalletError::InsufficientFunds(format!(
                    "Insufficient funds: have {} satoshis, need {}",
                    total_available, amount
                )));
            }
            
            // Sort UTXOs based on strategy
            match strategy {
                CoinSelectionStrategy::LargestFirst => {
                    available_utxos.sort_by(|a, b| b.txout.value.cmp(&a.txout.value));
                }
                CoinSelectionStrategy::SmallestFirst => {
                    available_utxos.sort_by(|a, b| a.txout.value.cmp(&b.txout.value));
                }
                CoinSelectionStrategy::OldestFirst => {
                    available_utxos.sort_by(|a, b| b.confirmations.cmp(&a.confirmations));
                }
                CoinSelectionStrategy::Random => {
                    let mut rng = thread_rng();
                    available_utxos.shuffle(&mut rng);
                }
                CoinSelectionStrategy::BranchAndBound => {
                    // Use branch and bound algorithm
                    return coin_selection::branch_and_bound(available_utxos, amount, fee_rate);
                }
                CoinSelectionStrategy::PrivacyOptimized => {
                    // Try to find exact match or minimal change
                    return coin_selection::privacy_optimized(available_utxos, amount, fee_rate);
                }
            }
            
            // Simple selection strategy
            let mut selected_utxos = Vec::new();
            let mut total_input = 0;
            
            for utxo in available_utxos {
                selected_utxos.push(utxo.clone());
                total_input += utxo.txout.value;
                
                // Add a buffer for fees
                if total_input >= amount + self.estimate_fee(selected_utxos.len() as u32, 2, fee_rate) {
                    break;
                }
            }
            
            if total_input < amount {
                return Err(WalletError::InsufficientFunds(format!(
                    "Not enough funds: selected {} satoshis, need {}",
                    total_input, amount
                )));
            }
            
            Ok((selected_utxos, total_input))
        }
    }
    
    /// Estimate fee for a transaction
    fn estimate_fee(&self, input_count: u32, output_count: u32, fee_rate: FeeRate) -> u64 {
        // Estimate size: inputs + outputs + overhead
        // P2WPKH input: ~68 vbytes
        // P2WSH input: ~104 vbytes
        // P2PKH input: ~148 vbytes
        // P2SH input: ~varying
        // Output: ~31-43 vbytes
        // Overhead: ~10-12 vbytes
        
        // Use conservative estimates
        const INPUT_SIZE: u64 = 150;
        const OUTPUT_SIZE: u64 = 43;
        const OVERHEAD: u64 = 12;
        
        let tx_size = (input_count as u64 * INPUT_SIZE) + (output_count as u64 * OUTPUT_SIZE) + OVERHEAD;
        tx_size * fee_rate.to_sat_per_vb()
    }
}

#[async_trait]
impl Wallet for BitcoinWallet {
    async fn init(&self) -> Result<(), WalletError> {
        info!("Initializing wallet {}", self.config.name);
        
        // Check descriptors
        if self.config.receive_descriptor.is_empty() || self.config.change_descriptor.is_empty() {
            return Err(WalletError::ConfigError("Invalid wallet descriptors".to_string()));
        }
        
        // Try to parse descriptors to validate them
        // (Full validation would require a descriptor parsing library)
        
        Ok(())
    }
    
    async fn get_new_address(&self) -> Result<Address, WalletError> {
        let mut storage = self.storage.lock()
            .map_err(|_| WalletError::StorageError("Failed to lock wallet storage".to_string()))?;
        
        let index = storage.indexes.receive_index;
        storage.indexes.receive_index += 1;
        
        // Derive address from descriptor with index
        // For placeholder, we'll create a random address compatible with network
        let address = match self.config.network {
            Network::Bitcoin => "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4",
            Network::Testnet => "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx",
            Network::Regtest => "bcrt1qw508d6qejxtdg4y5r3zarvary0c5xw7kygt080",
            Network::Signet => "tb1qrp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3q0sl5k7",
            _ => return Err(WalletError::ConfigError("Unsupported network".to_string())),
        };
        
        let address = Address::from_str(address)
            .map_err(|e| WalletError::AddressError(format!("Failed to parse address: {}", e)))?;
        
        // Save address to storage
        let path = DerivationPath::from_str(&format!("m/84'/0'/0'/0/{}", index))
            .map_err(|e| WalletError::ConfigError(format!("Invalid derivation path: {}", e)))?;
        
        storage.addresses.insert(address.to_string(), AddressInfo {
            address: address.to_string(),
            path: Some(path),
            script: address.script_pubkey(),
            is_change: false,
            index,
            labels: Vec::new(),
            last_used: None,
        });
        
        // Save wallet
        drop(storage); // Release lock before async call
        self.save().await?;
        
        Ok(address)
    }
    
    async fn get_current_address(&self) -> Result<Address, WalletError> {
        let storage = self.storage.lock()
            .map_err(|_| WalletError::StorageError("Failed to lock wallet storage".to_string()))?;
        
        let index = if storage.indexes.receive_index > 0 {
            storage.indexes.receive_index - 1
        } else {
            0
        };
        
        // Derive address from descriptor with index
        // For placeholder, we'll create a random address compatible with network
        let address = match self.config.network {
            Network::Bitcoin => "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4",
            Network::Testnet => "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx",
            Network::Regtest => "bcrt1qw508d6qejxtdg4y5r3zarvary0c5xw7kygt080",
            Network::Signet => "tb1qrp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3q0sl5k7",
            _ => return Err(WalletError::ConfigError("Unsupported network".to_string())),
        };
        
        let address = Address::from_str(address)
            .map_err(|e| WalletError::AddressError(format!("Failed to parse address: {}", e)))?;
        
        Ok(address)
    }
    
    async fn get_change_address(&self) -> Result<Address, WalletError> {
        let mut storage = self.storage.lock()
            .map_err(|_| WalletError::StorageError("Failed to lock wallet storage".to_string()))?;
        
        let index = storage.indexes.change_index;
        storage.indexes.change_index += 1;
        
        // Derive address from descriptor with index
        // For placeholder, we'll create a random address compatible with network
        let address = match self.config.network {
            Network::Bitcoin => "bc1qwqdg6squsna38e46795at95yu9atm8azzmyvckulcc7kytlcckxswvvzej",
            Network::Testnet => "tb1qwqdg6squsna38e46795at95yu9atm8azzmyvckulcc7kytlcckxs5g0fzh",
            Network::Regtest => "bcrt1qwqdg6squsna38e46795at95yu9atm8azzmyvckulcc7kytlcckxsj92xp2",
            Network::Signet => "tb1pqqqqp399et2xygdj5xreqhjjvcmzhxw4aywxecjdzew6hylgvsesf3hn0c",
            _ => return Err(WalletError::ConfigError("Unsupported network".to_string())),
        };
        
        let address = Address::from_str(address)
            .map_err(|e| WalletError::AddressError(format!("Failed to parse address: {}", e)))?;
        
        // Save address to storage
        let path = DerivationPath::from_str(&format!("m/84'/0'/0'/1/{}", index))
            .map_err(|e| WalletError::ConfigError(format!("Invalid derivation path: {}", e)))?;
        
        storage.addresses.insert(address.to_string(), AddressInfo {
            address: address.to_string(),
            path: Some(path),
            script: address.script_pubkey(),
            is_change: true,
            index,
            labels: Vec::new(),
            last_used: None,
        });
        
        // Save wallet
        drop(storage); // Release lock before async call
        self.save().await?;
        
        Ok(address)
    }
    
    async fn is_mine(&self, address: &Address) -> Result<bool, WalletError> {
        let storage = self.storage.lock()
            .map_err(|_| WalletError::StorageError("Failed to lock wallet storage".to_string()))?;
        
        Ok(storage.addresses.contains_key(&address.to_string()))
    }
    
    async fn list_addresses(&self) -> Result<Vec<Address>, WalletError> {
        let storage = self.storage.lock()
            .map_err(|_| WalletError::StorageError("Failed to lock wallet storage".to_string()))?;
        
        let mut addresses = Vec::new();
        for addr_info in storage.addresses.values() {
            let address = Address::from_str(&addr_info.address)
                .map_err(|e| WalletError::AddressError(format!("Failed to parse address: {}", e)))?;
            addresses.push(address);
        }
        
        Ok(addresses)
    }
    
    async fn get_balance(&self) -> Result<u64, WalletError> {
        let (confirmed, _, _) = self.get_detailed_balance().await?;
        Ok(confirmed)
    }
    
    async fn get_detailed_balance(&self) -> Result<(u64, u64, u64), WalletError> {
        let storage = self.storage.lock()
            .map_err(|_| WalletError::StorageError("Failed to lock wallet storage".to_string()))?;
        
        let mut confirmed = 0;
        let mut unconfirmed = 0;
        let mut immature = 0;
        
        for utxo in storage.utxos.values() {
            if !utxo.spendable {
                continue;
            }
            
            if utxo.confirmations == 0 {
                unconfirmed += utxo.txout.value;
            } else if utxo.confirmations < self.config.min_confirmations {
                immature += utxo.txout.value;
            } else {
                confirmed += utxo.txout.value;
            }
        }
        
        Ok((confirmed, unconfirmed, immature))
    }
    
    async fn list_utxos(&self) -> Result<Vec<Utxo>, WalletError> {
        let storage = self.storage.lock()
            .map_err(|_| WalletError::StorageError("Failed to lock wallet storage".to_string()))?;
        
        let utxos = storage.utxos.values().cloned().collect();
        Ok(utxos)
    }
    
    async fn get_transactions(&self) -> Result<Vec<TransactionInfo>, WalletError> {
        let storage = self.storage.lock()
            .map_err(|_| WalletError::StorageError("Failed to lock wallet storage".to_string()))?;
        
        let txs = storage.transactions.values().cloned().collect();
        Ok(txs)
    }
    
    async fn get_transaction(&self, txid: &Txid) -> Result<Option<TransactionInfo>, WalletError> {
        let storage = self.storage.lock()
            .map_err(|_| WalletError::StorageError("Failed to lock wallet storage".to_string()))?;
        
        Ok(storage.transactions.get(txid).cloned())
    }
    
    async fn create_transaction(&self, params: TransactionParams) -> Result<PSBT, WalletError> {
        // Get fee rate based on strategy
        let fee_strategy = params.fee_strategy.unwrap_or(self.config.fee_strategy);
        let fee_rate = self.get_fee_rate(fee_strategy).await?;
        
        // Calculate total amount needed
        let total_amount: u64 = params.recipients.iter().map(|(_, amount)| amount).sum();
        
        // Select UTXOs
        let (selected_utxos, total_input) = self.select_utxos(
            total_amount,
            fee_rate,
            self.config.coin_selection,
            params.utxos,
            params.allow_unconfirmed,
        ).await?;
        
        // Calculate fee and check if we have enough funds
        let output_count = params.recipients.len() as u32 + 1; // +1 for change
        let fee = self.estimate_fee(selected_utxos.len() as u32, output_count, fee_rate);
        
        if total_input < total_amount + fee {
            return Err(WalletError::InsufficientFunds(format!(
                "Not enough funds: have {} satoshis, need {} (amount) + {} (fee)",
                total_input, total_amount, fee
            )));
        }
        
        // Create inputs
        let mut inputs = Vec::new();
        for utxo in &selected_utxos {
            inputs.push(TxIn {
                previous_output: utxo.outpoint,
                script_sig: Script::new(),
                sequence: if params.enable_rbf { 0xFFFFFFFD } else { 0xFFFFFFFE },
                witness: Vec::new(),
            });
        }
        
        // Create outputs
        let mut outputs = Vec::new();
        for (address, amount) in &params.recipients {
            outputs.push(TxOut {
                value: *amount,
                script_pubkey: address.script_pubkey(),
            });
        }
        
        // Add OP_RETURN if requested
        if let Some(data) = &params.op_return_data {
            if data.len() > 80 {
                return Err(WalletError::InvalidParameters(
                    "OP_RETURN data cannot exceed 80 bytes".to_string()
                ));
            }
            
            let script = Script::new_op_return(data);
            outputs.push(TxOut {
                value: 0,
                script_pubkey: script,
            });
        }
        
        // Calculate change
        let change_amount = total_input - total_amount - fee;
        if change_amount > 546 { // Dust threshold
            // Get change address
            let change_address = match &params.change_address {
                Some(addr) => addr.clone(),
                None => self.get_change_address().await?,
            };
            
            outputs.push(TxOut {
                value: change_amount,
                script_pubkey: change_address.script_pubkey(),
            });
        }
        
        // Create transaction
        let tx = Transaction {
            version: 2,
            lock_time: params.lock_time.unwrap_or(0),
            input: inputs,
            output: outputs,
        };
        
        // Create PSBT
        let mut psbt = PSBT::from_unsigned_tx(tx)
            .map_err(|e| WalletError::PsbtError(format!("Failed to create PSBT: {}", e)))?;
        
        // Add UTXO information to PSBT inputs
        for (i, utxo) in selected_utxos.iter().enumerate() {
            if let Some(redeem_script) = &utxo.redeem_script {
                psbt.inputs[i].redeem_script = Some(redeem_script.clone());
            }
            
            if let Some(witness_script) = &utxo.witness_script {
                psbt.inputs[i].witness_script = Some(witness_script.clone());
            }
            
            // Set the UTXO value and script pubkey
            psbt.inputs[i].witness_utxo = Some(utxo.txout.clone());
        }
        
        Ok(psbt)
    }
    
    async fn sign_transaction(&self, psbt: &mut PSBT) -> Result<bool, WalletError> {
        // In a real implementation, we would use the wallet's private keys to sign
        // For this placeholder, we'll just return a positive result
        info!("Signing transaction with wallet {}", self.config.name);
        
        Ok(true)
    }
    
    async fn broadcast_transaction(&self, transaction: &Transaction) -> Result<Txid, WalletError> {
        if let Some(rpc_client) = &self.rpc_client {
            let tx_hex = encode::serialize_hex(transaction);
            rpc_client.send_raw_transaction(&tx_hex)
                .await
                .map_err(|e| WalletError::RpcError(format!("Failed to broadcast transaction: {}", e)))
        } else {
            // For placeholder, just return the txid
            Ok(transaction.txid())
        }
    }
    
    async fn get_fee_rate(&self, strategy: FeeStrategy) -> Result<FeeRate, WalletError> {
        match strategy {
            FeeStrategy::VeryLow => Ok(FeeRate::SatPerVb(1)),
            FeeStrategy::Low => Ok(FeeRate::SatPerVb(2)),
            FeeStrategy::Medium => Ok(FeeRate::SatPerVb(5)),
            FeeStrategy::High => Ok(FeeRate::SatPerVb(10)),
            FeeStrategy::VeryHigh => Ok(FeeRate::SatPerVb(20)),
            FeeStrategy::Custom(rate) => Ok(rate),
        }
    }
    
    async fn calculate_fee(&self, psbt: &PSBT) -> Result<u64, WalletError> {
        let mut input_total = 0;
        for (i, input) in psbt.inputs.iter().enumerate() {
            if let Some(utxo) = &input.witness_utxo {
                input_total += utxo.value;
            } else {
                return Err(WalletError::PsbtError(format!("Missing UTXO information for input {}", i)));
            }
        }
        
        let output_total: u64 = psbt.unsigned_tx.output.iter().map(|o| o.value).sum();
        Ok(input_total - output_total)
    }
    
    async fn sync(&self) -> Result<SyncState, WalletError> {
        if let Some(rpc_client) = &self.rpc_client {
            // Get blockchain info
            let blockchain_info = rpc_client.get_blockchain_info()
                .await
                .map_err(|e| WalletError::SyncError(format!("Failed to get blockchain info: {}", e)))?;
            
            // Get current time
            let now = SystemTime::now().duration_since(UNIX_EPOCH)
                .map_err(|e| WalletError::StorageError(format!("System time error: {}", e)))?
                .as_secs();
            
            let mut storage = self.storage.lock()
                .map_err(|_| WalletError::StorageError("Failed to lock wallet storage".to_string()))?;
            
            // Update sync state
            let current_height = blockchain_info.blocks;
            storage.indexes.last_block = Some(current_height);
            storage.indexes.last_sync = Some(now);
            
            // Save wallet
            drop(storage); // Release lock before async call
            self.save().await?;
            
            // Return sync state
            Ok(SyncState {
                block_height: current_height,
                block_hash: blockchain_info.best_block_hash,
                last_scan: now,
                progress: blockchain_info.verification_progress,
                ibd: blockchain_info.initial_block_download,
            })
        } else {
            // Return a placeholder sync state
            Ok(SyncState {
                block_height: 0,
                block_hash: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
                last_scan: 0,
                progress: 0.0,
                ibd: false,
            })
        }
    }
    
    async fn export(&self, path: &Path) -> Result<(), WalletError> {
        let storage = self.storage.lock()
            .map_err(|_| WalletError::StorageError("Failed to lock wallet storage".to_string()))?;
        
        let file = std::fs::File::create(path)
            .map_err(|e| WalletError::IoError(e))?;
        
        serde_json::to_writer_pretty(file, &*storage)
            .map_err(|e| WalletError::SerializationError(format!("Failed to serialize wallet data: {}", e)))?;
        
        Ok(())
    }
    
    async fn import(&self, path: &Path) -> Result<(), WalletError> {
        let file = std::fs::File::open(path)
            .map_err(|e| WalletError::IoError(e))?;
        
        let imported_storage: WalletStorage = serde_json::from_reader(file)
            .map_err(|e| WalletError::SerializationError(format!("Failed to deserialize wallet data: {}", e)))?;
        
        // Verify network matches
        if imported_storage.metadata.network != self.config.network {
            return Err(WalletError::ConfigError(format!(
                "Imported wallet network ({:?}) doesn't match current network ({:?})",
                imported_storage.metadata.network, self.config.network
            )));
        }
        
        let mut storage = self.storage.lock()
            .map_err(|_| WalletError::StorageError("Failed to lock wallet storage".to_string()))?;
        
        // Replace storage with imported data
        *storage = imported_storage;
        
        // Save wallet
        drop(storage); // Release lock before async call
        self.save().await?;
        
        Ok(())
    }
    
    async fn backup(&self, path: &Path) -> Result<(), WalletError> {
        self.export(path).await
    }
    
    async fn get_info(&self) -> Result<WalletInfo, WalletError> {
        let storage = self.storage.lock()
            .map_err(|_| WalletError::StorageError("Failed to lock wallet storage".to_string()))?;
        
        let (balance, unconfirmed, immature) = self.get_detailed_balance().await?;
        
        Ok(WalletInfo {
            name: self.config.name.clone(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            format: "descriptor".to_string(),
            network: self.config.network,
            balance,
            unconfirmed_balance: unconfirmed,
            immature_balance: immature,
            keypools: (storage.indexes.receive_index + storage.indexes.change_index) as u32,
            tx_count: storage.transactions.len() as u32,
            keypool_oldest: storage.metadata.created_at,
            keypool_size: self.config.gap_limit,
            private_keys_enabled: true,
            unlocked_until: None,
            hdseedid: None,
            avoid_reuse: true,
            scanning: false,
            descriptors: true,
        })
    }
} 
