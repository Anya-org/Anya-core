use std::error::Error;
use std::collections::HashMap;
use crate::bitcoin::wallet::AddressType;

/// Options for transaction creation
#[derive(Debug, Clone)]
pub struct TxOptions {
    /// Coin selection strategy
    pub coin_selection: CoinSelectionStrategy,
    
    /// Replace-by-fee enabled
    pub rbf: bool,
    
    /// Lock time for the transaction
    pub lock_time: Option<u32>,
    
    /// Change address type
    pub change_address_type: AddressType,
    
    /// Change address index (if None, a new address will be generated)
    pub change_address_index: Option<u32>,
    
    /// If true, only confirmed UTXOs will be used
    pub confirmed_only: bool,
    
    /// Custom inputs to use (if None, inputs will be selected automatically)
    pub custom_inputs: Option<Vec<Utxo>>,
    
    /// Custom extra outputs to add to the transaction
    pub extra_outputs: Vec<(String, u64)>,
    
    /// OP_RETURN data to include in the transaction
    pub op_return_data: Option<Vec<u8>>,
    
    /// If true, subtract fee from outputs (required amount)
    pub subtract_fee_from_amount: bool,
    
    /// Signature type to use (Schnorr for Taproot, ECDSA for others)
    pub signature_type: SignatureType,
    
    /// Custom metadata to associate with the transaction
    pub metadata: HashMap<String, String>,
}

impl Default for TxOptions {
    fn default() -> Self {
        Self {
            coin_selection: CoinSelectionStrategy::default(),
            rbf: true,
            lock_time: None,
            change_address_type: AddressType::SegWit,
            change_address_index: None,
            confirmed_only: true,
            custom_inputs: None,
            extra_outputs: vec![],
            op_return_data: None,
            subtract_fee_from_amount: false,
            signature_type: SignatureType::default(),
            metadata: HashMap::new(),
        }
    }
}

/// Coin selection strategy
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoinSelectionStrategy {
    /// Branch and bound algorithm (try to find exact match)
    BranchAndBound,
    
    /// FIFO - First in, first out
    FIFO,
    
    /// Largest first
    LargestFirst,
    
    /// Smallest first
    SmallestFirst,
    
    /// Random selection
    Random,
    
    /// Select all available UTXOs
    SelectAll,
}

impl Default for CoinSelectionStrategy {
    fn default() -> Self {
        Self::BranchAndBound
    }
}

/// Signature type for transaction signing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignatureType {
    /// ECDSA (legacy, segwit)
    ECDSA,
    
    /// Schnorr (taproot)
    Schnorr,
}

impl Default for SignatureType {
    fn default() -> Self {
        Self::ECDSA
    }
}

/// Unspent transaction output (UTXO)
#[derive(Debug, Clone)]
pub struct Utxo {
    /// Transaction ID
    pub txid: String,
    
    /// Output index
    pub vout: u32,
    
    /// Amount in satoshis
    pub amount: u64,
    
    /// Address associated with the UTXO
    pub address: String,
    
    /// Script pubkey
    pub script_pubkey: Vec<u8>,
    
    /// Derivation path used to create this UTXO (if known)
    pub derivation_path: Option<String>,
    
    /// Confirmation status
    pub confirmed: bool,
    
    /// Block height when confirmed (if confirmed)
    pub confirmation_height: Option<u32>,
}

/// Implements methods to analyze transaction-related data
pub struct TransactionAnalyzer;

impl TransactionAnalyzer {
    /// Calculate the fee for a transaction given its size and fee rate
    pub fn calculate_fee(tx_size: usize, fee_rate: f64) -> u64 {
        (tx_size as f64 * fee_rate).ceil() as u64
    }
    
    /// Estimate the size of a transaction with the given number of inputs and outputs
    pub fn estimate_tx_size(input_count: usize, output_count: usize) -> usize {
        // Simple estimation for P2WPKH inputs and outputs
        let header_size = 10; // 4 bytes version, 1 byte input count, 1 byte output count, 4 bytes locktime
        let input_size = 148; // Approximate P2WPKH input size with signature
        let output_size = 34; // Approximate P2WPKH output size
        
        header_size + (input_count * input_size) + (output_count * output_size)
    }
    
    /// Estimate the vsize of a transaction with the given number of inputs and outputs
    pub fn estimate_tx_vsize(input_count: usize, output_count: usize) -> usize {
        // Simple estimation for P2WPKH inputs and outputs
        let header_vsize = 10.0; // 4 bytes version, 1 byte input count, 1 byte output count, 4 bytes locktime
        let input_vsize = 68.0;  // Approximate P2WPKH input vsize with signature
        let output_vsize = 31.0; // Approximate P2WPKH output vsize
        
        (header_vsize + (input_count as f64 * input_vsize) + (output_count as f64 * output_vsize)).ceil() as usize
    }
    
    /// Calculate the fee rate from transaction size and fee
    pub fn calculate_fee_rate(tx_size: usize, fee: u64) -> f64 {
        fee as f64 / tx_size as f64
    }
} 
