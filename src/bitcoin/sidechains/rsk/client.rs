use std::error::Error;
// src/bitcoin/sidechains/rsk/client.rs

//! RSK client implementation
//!
//! Provides a client for interacting with the RSK blockchain.
//! Implements Ethereum-compatible JSON-RPC methods using web3 and ethers-rs.
//! [AIR-2][AIS-2][AIM-2][AIP-2][RES-2]

use serde::{Serialize, Deserialize};
use std::str::FromStr;
use std::fmt;
use std::sync::Arc;
use url::Url;
use thiserror::Error;
use async_trait::async_trait;
use web3::{
    Web3,
    transports::{Http, WebSocket},
    types::{H160, H256, U256, BlockNumber, Block, BlockId, Transaction, TransactionReceipt, CallRequest},
    contract::Contract,
    Error as Web3Error
};
use ethers::{
    core::{
        types::{Address, TransactionRequest, Bytes},
        abi::Abi,
    },
    providers::{Provider, Http as EthersHttp, Middleware},
    signers::{LocalWallet, Signer},
};
use tokio::sync::Mutex;

use crate::AnyaResult;

/// Errors that can occur during RSK client operations
#[derive(Error, Debug)]
pub enum ClientError {
    /// Network error
    #[error("Network error: {0}")]
    NetworkError(String),
    
    /// RPC error
    #[error("RPC error: {0}")]
    RpcError(String),
    
    /// Contract error
    #[error("Contract error: {0}")]
    ContractError(String),
    
    /// Transaction error
    #[error("Transaction error: {0}")]
    TransactionError(String),
    
    /// Encoding error
    #[error("Encoding error: {0}")]
    EncodingError(String),
    
    /// Wallet error
    #[error("Wallet error: {0}")]
    WalletError(String),
    
    /// Validation error
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

/// Network type for RSK
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkType {
    /// Mainnet
    Mainnet,
    
    /// Testnet
    Testnet,
    
    /// Regtest (local development)
    Regtest,
}

impl fmt::Display for NetworkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkType::Mainnet => write!(f, "mainnet"),
            NetworkType::Testnet => write!(f, "testnet"),
            NetworkType::Regtest => write!(f, "regtest"),
        }
    }
}

impl NetworkType {
    /// Get the chain ID for this network
    pub fn chain_id(&self) -> u64 {
        match self {
            NetworkType::Mainnet => 30, // RSK mainnet chain ID
            NetworkType::Testnet => 31, // RSK testnet chain ID
            NetworkType::Regtest => 33, // RSK regtest chain ID
        }
    }
    
    /// Get a default node URL for this network
    pub fn default_node_url(&self) -> &'static str {
        match self {
            NetworkType::Mainnet => "https://public-node.rsk.co",
            NetworkType::Testnet => "https://public-node.testnet.rsk.co",
            NetworkType::Regtest => "http://localhost:4444",
        }
    }
}

/// RSK client configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    /// Network type
    pub network: NetworkType,
    
    /// Node URL
    pub node_url: String,
    
    /// Gas price strategy
    pub gas_price_strategy: GasPriceStrategy,
    
    /// Transaction confirmation timeout (in seconds)
    pub tx_confirmation_timeout: u64,
    
    /// Transaction confirmation blocks
    pub tx_confirmation_blocks: u64,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            network: NetworkType::Testnet,
            node_url: NetworkType::Testnet.default_node_url().to_string(),
            gas_price_strategy: GasPriceStrategy::Standard,
            tx_confirmation_timeout: 300, // 5 minutes
            tx_confirmation_blocks: 6,
        }
    }
}

/// Gas price strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GasPriceStrategy {
    /// Use a fixed gas price
    Fixed(u64),
    
    /// Use the standard gas price (1x)
    Standard,
    
    /// Use a fast gas price (1.5x)
    Fast,
    
    /// Use a rapid gas price (2x)
    Rapid,
}

impl GasPriceStrategy {
    /// Apply this strategy to a base gas price
    fn apply(&self, base_gas_price: U256) -> U256 {
        match self {
            GasPriceStrategy::Fixed(price) => U256::from(*price),
            GasPriceStrategy::Standard => base_gas_price,
            GasPriceStrategy::Fast => base_gas_price * U256::from(15) / U256::from(10),
            GasPriceStrategy::Rapid => base_gas_price * U256::from(2),
        }
    }
}

/// A transaction response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResponse {
    /// Transaction hash
    pub hash: String,
    
    /// Block hash (None if pending)
    pub block_hash: Option<String>,
    
    /// Block number (None if pending)
    pub block_number: Option<u64>,
    
    /// From address
    pub from: String,
    
    /// To address (None if contract creation)
    pub to: Option<String>,
    
    /// Contract address (None if not a contract creation)
    pub contract_address: Option<String>,
    
    /// Transaction value
    pub value: String,
    
    /// Gas price
    pub gas_price: String,
    
    /// Gas limit
    pub gas: String,
    
    /// Gas used (None if pending)
    pub gas_used: Option<String>,
    
    /// Transaction nonce
    pub nonce: u64,
    
    /// Transaction input data
    pub input: String,
    
    /// Transaction status (1 for success, 0 for failure, None if pending)
    pub status: Option<u64>,
}

/// A contract call request
#[derive(Debug, Clone)]
pub struct ContractCallRequest {
    /// Contract address
    pub to: String,
    
    /// Call data
    pub data: Vec<u8>,
    
    /// Value to send
    pub value: Option<u64>,
    
    /// From address
    pub from: Option<String>,
    
    /// Gas limit
    pub gas: Option<u64>,
    
    /// Gas price
    pub gas_price: Option<u64>,
}

/// RSK client trait
#[async_trait]
pub trait RskClientTrait {
    /// Get the current block number
    async fn get_block_number(&self) -> Result<u64, ClientError>;
    
    /// Get a block by number or hash
    async fn get_block(&self, block_id: &str) -> Result<Block<H256>, ClientError>;
    
    /// Get a transaction by hash
    async fn get_transaction(&self, tx_hash: &str) -> Result<Option<Transaction>, ClientError>;
    
    /// Get a transaction receipt
    async fn get_transaction_receipt(&self, tx_hash: &str) -> Result<Option<TransactionReceipt>, ClientError>;
    
    /// Get the balance of an address
    async fn get_balance(&self, address: &str, block: Option<BlockNumber>) -> Result<U256, ClientError>;
    
    /// Get the code at an address
    async fn get_code(&self, address: &str, block: Option<BlockNumber>) -> Result<Bytes, ClientError>;
    
    /// Get the transaction count of an address
    async fn get_transaction_count(&self, address: &str, block: Option<BlockNumber>) -> Result<U256, ClientError>;
    
    /// Send a raw transaction
    async fn send_raw_transaction(&self, _data: data: &[u8][u8]) -> Result<H256, ClientError>;
    
    /// Call a contract
    async fn call(&self, request: ContractCallRequest, block: Option<BlockNumber>) -> Result<Bytes, ClientError>;
    
    /// Estimate gas for a call
    async fn estimate_gas(&self, request: ContractCallRequest) -> Result<U256, ClientError>;
    
    /// Get the current gas price
    async fn get_gas_price(&self) -> Result<U256, ClientError>;
    
    /// Wait for a transaction to be confirmed
    async fn wait_for_transaction(&self, tx_hash: H256, timeout: Option<u64>, confirmations: Option<usize>) -> Result<Option<TransactionReceipt>, ClientError>;
}

/// RSK client implementation
pub struct RSKClient {
    /// Web3 instance
    web3: Web3<Http>,
    
    /// Client configuration
    config: ClientConfig,
    
    /// Ethers provider for additional functionality
    provider: Provider<EthersHttp>,
    
    /// Contract cache
    contract_cache: Arc<Mutex<std::collections::HashMap<String, Contract<Http>>>>,
}

impl RSKClient {
    /// Create a new RSK client
    pub async fn new(config: ClientConfig) -> Result<Self, ClientError> {
        // Validate the node URL
        let url = Url::parse(&config.node_url)
            .map_err(|e| ClientError::ConfigurationError(format!("Invalid node URL: {}", e)))?;
            
        // Create Web3 transport
        let transport = Http::new(&config.node_url)
            .map_err(|e| ClientError::NetworkError(format!("Failed to create HTTP transport: {}", e)))?;
            
        // Create Web3 instance
        let web3 = Web3::new(transport);
        
        // Create Ethers provider
        let provider = Provider::<EthersHttp>::try_from(config.node_url.clone())
            .map_err(|e| ClientError::ConfigurationError(format!("Failed to create Ethers provider: {}", e)))?;
            
        // Test connection
        let _block_number = web3.eth().block_number().await
            .map_err(|e| ClientError::NetworkError(format!("Failed to connect to node: {}", e)))?;
            
        Ok(Self {
            web3,
            config,
            provider,
            contract_cache: Arc::new(Mutex::new(std::collections::HashMap::new())),
        })
    }
    
    /// Create a new contract instance
    pub async fn contract(&self, address: &str, abi: &[u8]) -> Result<Contract<Http>, ClientError> {
        // Check cache first
        let mut cache = self.contract_cache.lock().await;
        if let Some(contract) = cache.get(address) {
            return Ok(contract.clone());
        }
        
        // Parse address
        let address = H160::from_str(address)
            .map_err(|e| ClientError::ValidationError(format!("Invalid contract address: {}", e)))?;
            
        // Create contract
        let contract = Contract::from_json(self.web3.eth(), address, abi)
            .map_err(|e| ClientError::ContractError(format!("Failed to create contract: {}", e)))?;
            
        // Cache it
        cache.insert(address.to_string(), contract.clone());
        
        Ok(contract)
    }
    
    /// Create a new wallet
    pub fn create_wallet(&self, private_key: &str) -> Result<LocalWallet, ClientError> {
        // Parse private key
        let wallet = private_key.parse::<LocalWallet>()
            .map_err(|e| ClientError::WalletError(format!("Invalid private key: {}", e)))?;
            
        // Set the chain ID
        let wallet = wallet.with_chain_id(self.config.network.chain_id());
        
        Ok(wallet)
    }
    
    /// Send a transaction
    pub async fn send_transaction<S: Signer>(
        &self,
        from: &S,
        to: &str,
        value: Option<U256>,
        data: Option<Vec<u8>>,
        gas_limit: Option<U256>,
        nonce: Option<U256>,
    ) -> Result<H256, ClientError> {
        // Parse to address
        let to_address = if to.is_empty() {
            None // Contract creation
        } else {
            Some(H160::from_str(to)
                .map_err(|e| ClientError::ValidationError(format!("Invalid to address: {}", e)))?)
        };
        
        // Get gas price according to strategy
        let base_gas_price = self.web3.eth().gas_price().await
            .map_err(|e| ClientError::RpcError(format!("Failed to get gas price: {}", e)))?;
            
        let gas_price = self.config.gas_price_strategy.apply(base_gas_price);
        
        // Get nonce if not provided
        let nonce = if let Some(n) = nonce {
            n
        } else {
            let from_address = H160::from_slice(from.address().as_bytes());
            self.web3.eth().transaction_count(from_address, None).await
                .map_err(|e| ClientError::RpcError(format!("Failed to get nonce: {}", e)))?
        };
        
        // Create transaction request
        let tx_request = TransactionRequest::new()
            .from(from.address())
            .to(to_address.map(|addr| Address::from_slice(addr.as_bytes())))
            .gas_price(gas_price.as_u128())
            .nonce(nonce.as_u64());
            
        let tx_request = if let Some(v) = value {
            tx_request.value(v.as_u128())
        } else {
            tx_request
        };
        
        let tx_request = if let Some(d) = data {
            tx_request.data(d)
        } else {
            tx_request
        };
        
        let tx_request = if let Some(g) = gas_limit {
            tx_request.gas(g.as_u64())
        } else {
            // Estimate gas
            let gas = self.provider.estimate_gas(&tx_request, None).await
                .map_err(|e| ClientError::RpcError(format!("Failed to estimate gas: {}", e)))?;
                
            // Add a buffer (20%)
            let gas = gas * 12 / 10;
            
            tx_request.gas(gas)
        };
        
        // Sign the transaction
        let signed_tx = from.sign_transaction(tx_request).await
            .map_err(|e| ClientError::WalletError(format!("Failed to sign transaction: {}", e)))?;
            
        // Send the transaction
        let tx_hash = self.provider.send_raw_transaction(signed_tx).await
            .map_err(|e| ClientError::TransactionError(format!("Failed to send transaction: {}", e)))?;
            
        Ok(H256::from_slice(tx_hash.as_bytes()))
    }
    
    /// Call a contract method
    pub async fn call_contract<S: Signer>(
        &self,
        from: &S,
        contract: &Contract<Http>,
        method: &str,
        params: Vec<ethers::core::abi::Token>,
        value: Option<U256>,
    ) -> Result<Vec<ethers::core::abi::Token>, ClientError> {
        // Encode the call
        let data = contract.function(method)
            .map_err(|e| ClientError::ContractError(format!("Function not found: {}", e)))?
            .encode_input(&params)
            .map_err(|e| ClientError::EncodingError(format!("Failed to encode parameters: {}", e)))?;
            
        // Create call request
        let request = CallRequest {
            from: Some(H160::from_slice(from.address().as_bytes())),
            to: Some(contract.address()),
            gas: None,
            gas_price: None,
            value,
            data: Some(web3::types::Bytes(data.clone())),
            transaction_type: None,
            access_list: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
        };
        
        // Execute call
        let result = self.web3.eth().call(request, None).await
            .map_err(|e| ClientError::RpcError(format!("Call failed: {}", e)))?;
            
        // Decode result
        let tokens = contract.function(method)
            .map_err(|e| ClientError::ContractError(format!("Function not found: {}", e)))?
            .decode_output(&result.0)
            .map_err(|e| ClientError::EncodingError(format!("Failed to decode output: {}", e)))?;
            
        Ok(tokens)
    }
    
    /// Deploy a contract
    pub async fn deploy_contract<S: Signer>(
        &self,
        from: &S,
        bytecode: Vec<u8>,
        abi: &[u8],
        constructor_args: Vec<ethers::core::abi::Token>,
        value: Option<U256>,
        gas_limit: Option<U256>,
    ) -> Result<(H256, H160), ClientError> {
        // Parse the ABI
        let abi: Abi = serde_json::from_slice(abi)
            .map_err(|e| ClientError::ContractError(format!("Failed to parse ABI: {}", e)))?;
            
        // Encode constructor arguments
        let mut data = bytecode;
        if !constructor_args.is_empty() {
            if let Some(constructor) = abi.constructor() {
                let encoded = constructor.encode_input(data.as_slice(), &constructor_args)
                    .map_err(|e| ClientError::EncodingError(format!("Failed to encode constructor arguments: {}", e)))?;
                data = encoded;
            }
        }
        
        // Send the deployment transaction
        let tx_hash = self.send_transaction(
            from,
            "", // Empty to address for contract creation
            value,
            Some(data),
            gas_limit,
            None,
        ).await?;
        
        // Wait for the transaction to be mined
        let receipt = self.wait_for_transaction(
            tx_hash,
            Some(self.config.tx_confirmation_timeout),
            Some(self.config.tx_confirmation_blocks as usize),
        ).await?
            .ok_or_else(|| ClientError::TransactionError("Transaction not found after timeout".to_string()))?;
            
        // Get the contract address
        let contract_address = receipt.contract_address
            .ok_or_else(|| ClientError::ContractError("Contract address not found in receipt".to_string()))?;
            
        Ok((tx_hash, contract_address))
    }
}

#[async_trait]
impl RskClientTrait for RSKClient {
    async fn get_block_number(&self) -> Result<u64, ClientError> {
        self.web3.eth().block_number().await
            .map(|n| n.as_u64())
            .map_err(|e| ClientError::RpcError(format!("Failed to get block number: {}", e)))
    }
    
    async fn get_block(&self, block_id: &str) -> Result<Block<H256>, ClientError> {
        let block_id = if block_id.starts_with("0x") {
            // It's a hash
            let hash = H256::from_str(block_id)
                .map_err(|e| ClientError::ValidationError(format!("Invalid block hash: {}", e)))?;
            BlockId::Hash(hash)
        } else {
            // It's a number
            let num = block_id.parse::<u64>()
                .map_err(|e| ClientError::ValidationError(format!("Invalid block number: {}", e)))?;
            BlockId::Number(BlockNumber::Number(num.into()))
        };
        
        self.web3.eth().block(block_id).await
            .map_err(|e| ClientError::RpcError(format!("Failed to get block: {}", e)))?
            .ok_or_else(|| ClientError::ValidationError(format!("Block not found: {}", block_id)))
    }
    
    async fn get_transaction(&self, tx_hash: &str) -> Result<Option<Transaction>, ClientError> {
        let hash = H256::from_str(tx_hash)
            .map_err(|e| ClientError::ValidationError(format!("Invalid transaction hash: {}", e)))?;
            
        self.web3.eth().transaction(web3::types::TransactionId::Hash(hash)).await
            .map_err(|e| ClientError::RpcError(format!("Failed to get transaction: {}", e)))
    }
    
    async fn get_transaction_receipt(&self, tx_hash: &str) -> Result<Option<TransactionReceipt>, ClientError> {
        let hash = H256::from_str(tx_hash)
            .map_err(|e| ClientError::ValidationError(format!("Invalid transaction hash: {}", e)))?;
            
        self.web3.eth().transaction_receipt(hash).await
            .map_err(|e| ClientError::RpcError(format!("Failed to get transaction receipt: {}", e)))
    }
    
    async fn get_balance(&self, address: &str, block: Option<BlockNumber>) -> Result<U256, ClientError> {
        let address = H160::from_str(address)
            .map_err(|e| ClientError::ValidationError(format!("Invalid address: {}", e)))?;
            
        self.web3.eth().balance(address, block).await
            .map_err(|e| ClientError::RpcError(format!("Failed to get balance: {}", e)))
    }
    
    async fn get_code(&self, address: &str, block: Option<BlockNumber>) -> Result<Bytes, ClientError> {
        let address = H160::from_str(address)
            .map_err(|e| ClientError::ValidationError(format!("Invalid address: {}", e)))?;
            
        let code = self.web3.eth().code(address, block).await
            .map_err(|e| ClientError::RpcError(format!("Failed to get code: {}", e)))?;
            
        Ok(Bytes::from(code.0))
    }
    
    async fn get_transaction_count(&self, address: &str, block: Option<BlockNumber>) -> Result<U256, ClientError> {
        let address = H160::from_str(address)
            .map_err(|e| ClientError::ValidationError(format!("Invalid address: {}", e)))?;
            
        self.web3.eth().transaction_count(address, block).await
            .map_err(|e| ClientError::RpcError(format!("Failed to get transaction count: {}", e)))
    }
    
    async fn send_raw_transaction(&self, _data: data: &[u8][u8]) -> Result<H256, ClientError> {
        self.web3.eth().send_raw_transaction(web3::types::Bytes(data.to_vec())).await
            .map_err(|e| ClientError::TransactionError(format!("Failed to send raw transaction: {}", e)))
    }
    
    async fn call(&self, request: ContractCallRequest, block: Option<BlockNumber>) -> Result<Bytes, ClientError> {
        let to = H160::from_str(&request.to)
            .map_err(|e| ClientError::ValidationError(format!("Invalid to address: {}", e)))?;
            
        let from = if let Some(from) = request.from {
            Some(H160::from_str(&from)
                .map_err(|e| ClientError::ValidationError(format!("Invalid from address: {}", e)))?)
        } else {
            None
        };
        
        let call_request = CallRequest {
            from,
            to: Some(to),
            gas: request.gas.map(U256::from),
            gas_price: request.gas_price.map(U256::from),
            value: request.value.map(U256::from),
            data: Some(web3::types::Bytes(request.data)),
            transaction_type: None,
            access_list: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
        };
        
        let result = self.web3.eth().call(call_request, block).await
            .map_err(|e| ClientError::RpcError(format!("Call failed: {}", e)))?;
            
        Ok(Bytes::from(result.0))
    }
    
    async fn estimate_gas(&self, request: ContractCallRequest) -> Result<U256, ClientError> {
        let to = if request.to.is_empty() {
            None // Contract creation
        } else {
            Some(H160::from_str(&request.to)
                .map_err(|e| ClientError::ValidationError(format!("Invalid to address: {}", e)))?)
        };
        
        let from = if let Some(from) = request.from {
            Some(H160::from_str(&from)
                .map_err(|e| ClientError::ValidationError(format!("Invalid from address: {}", e)))?)
        } else {
            None
        };
        
        let call_request = CallRequest {
            from,
            to,
            gas: None,
            gas_price: request.gas_price.map(U256::from),
            value: request.value.map(U256::from),
            data: Some(web3::types::Bytes(request.data)),
            transaction_type: None,
            access_list: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
        };
        
        self.web3.eth().estimate_gas(call_request, None).await
            .map_err(|e| ClientError::RpcError(format!("Failed to estimate gas: {}", e)))
    }
    
    async fn get_gas_price(&self) -> Result<U256, ClientError> {
        self.web3.eth().gas_price().await
            .map_err(|e| ClientError::RpcError(format!("Failed to get gas price: {}", e)))
    }
    
    async fn wait_for_transaction(&self, tx_hash: H256, timeout: Option<u64>, confirmations: Option<usize>) -> Result<Option<TransactionReceipt>, ClientError> {
        let timeout = timeout.unwrap_or(self.config.tx_confirmation_timeout);
        let confirmations = confirmations.unwrap_or(self.config.tx_confirmation_blocks as usize);
        
        let start = std::time::Instant::now();
        let timeout_duration = std::time::Duration::from_secs(timeout);
        
        loop {
            if start.elapsed() > timeout_duration {
                return Err(ClientError::TransactionError(format!(
                    "Timeout waiting for transaction confirmation: {}",
                    tx_hash
                )));
            }
            
            // Get current receipt
            match self.web3.eth().transaction_receipt(tx_hash).await {
                Ok(Some(receipt)) => {
                    // Check confirmations
                    if let Some(block_number) = receipt.block_number {
                        let current_block = self.web3.eth().block_number().await
                            .map_err(|e| ClientError::RpcError(format!("Failed to get block number: {}", e)))?;
                            
                        let conf = current_block.as_u64().saturating_sub(block_number.as_u64());
                        if conf >= confirmations as u64 {
                            return Ok(Some(receipt));
                        }
                    }
                },
                Ok(None) => {
                    // Transaction not yet mined
                },
                Err(e) => {
                    return Err(ClientError::RpcError(format!("Failed to get transaction receipt: {}", e)));
                }
            }
            
            // Wait a bit before checking again
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }
} 
