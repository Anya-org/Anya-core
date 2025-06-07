# Core Extensions

[AIR-3][AIS-3][AIT-3][RES-3] Essential extensions that provide fundamental Bitcoin, Web5, and ML functionality in the Anya ecosystem.

*Last updated: June 7, 2025*

## Table of Contents

- [Overview](#overview)
- [Bitcoin Core Extensions](#bitcoin-core-extensions)
- [Web5 Core Extensions](#web5-core-extensions)
- [ML Core Extensions](#ml-core-extensions)
- [Security Core Extensions](#security-core-extensions)
- [System Core Extensions](#system-core-extensions)
- [Installation and Configuration](#installation-and-configuration)
- [Usage Examples](#usage-examples)
- [Development Guide](#development-guide)

## Overview

Core extensions are the fundamental building blocks of the Anya platform, providing essential functionality for Bitcoin operations, Web5 protocols, machine learning, security, and system management. These extensions are maintained by the core team and are required for most Anya deployments.

### Extension Categories

```rust
use anya_core::{Extension, ExtensionCategory};

/// Core extension categories
#[derive(Debug, Clone)]
pub enum CoreExtensionCategory {
    Bitcoin,      // Bitcoin protocol operations
    Web5,         // Web5 identity and data protocols
    ML,           // Machine learning and AI
    Security,     // Security and cryptography
    System,       // System management and utilities
}

/// Core extension registry
pub struct CoreExtensionRegistry {
    extensions: HashMap<String, CoreExtension>,
    categories: HashMap<CoreExtensionCategory, Vec<String>>,
}
```

## Bitcoin Core Extensions

### Bitcoin Client Extension

The primary interface for Bitcoin network operations.

#### Installation

```bash
# Install Bitcoin core extension
anya extension install bitcoin-client

# Configure for testnet
anya config set bitcoin.network testnet
anya config set bitcoin.rpc_url "http://localhost:18332"
```

#### Features

```rust
use anya_bitcoin::{BitcoinClient, Network, Transaction, Address};

/// Bitcoin client extension capabilities
pub struct BitcoinClientExtension {
    client: BitcoinClient,
    network: Network,
    wallet: Option<WalletManager>,
}

impl BitcoinClientExtension {
    /// Create and broadcast transactions
    pub async fn create_transaction(
        &self,
        inputs: Vec<TxInput>,
        outputs: Vec<TxOutput>,
    ) -> Result<Transaction> {
        let tx = Transaction::new(inputs, outputs)?;
        
        // Sign transaction
        let signed_tx = self.wallet
            .as_ref()
            .ok_or(Error::WalletNotLoaded)?
            .sign_transaction(tx)?;
        
        // Broadcast to network
        let txid = self.client.broadcast_transaction(signed_tx.clone()).await?;
        
        info!("Transaction broadcast: {}", txid);
        Ok(signed_tx)
    }
    
    /// Monitor address for transactions
    pub async fn monitor_address(&self, address: &str) -> Result<AddressMonitor> {
        let monitor = AddressMonitor::new(address, self.client.clone());
        monitor.start_monitoring().await?;
        Ok(monitor)
    }
    
    /// Get blockchain information
    pub async fn get_blockchain_info(&self) -> Result<BlockchainInfo> {
        self.client.get_blockchain_info().await
    }
}
```

#### Configuration

```toml
[bitcoin]
network = "testnet"  # mainnet, testnet, regtest
rpc_url = "http://localhost:18332"
rpc_username = "bitcoin"
rpc_password = "password"
wallet_name = "anya_wallet"

[bitcoin.monitoring]
confirmation_threshold = 6
block_polling_interval = "10s"
mempool_monitoring = true

[bitcoin.security]
hardware_wallet = true
multisig_threshold = 2
backup_encryption = true
```

### Bitcoin Lightning Extension

Lightning Network functionality for instant payments.

#### Features

```rust
use anya_lightning::{LightningNode, Invoice, Payment};

pub struct LightningExtension {
    node: LightningNode,
    channels: ChannelManager,
    router: PaymentRouter,
}

impl LightningExtension {
    /// Create Lightning invoice
    pub async fn create_invoice(&self, amount_sats: u64, description: &str) -> Result<Invoice> {
        let invoice = Invoice::new(amount_sats, description)?;
        self.node.create_invoice(invoice).await
    }
    
    /// Send Lightning payment
    pub async fn send_payment(&self, invoice: &str) -> Result<Payment> {
        let decoded = self.node.decode_invoice(invoice)?;
        let route = self.router.find_route(&decoded.destination, decoded.amount).await?;
        
        self.node.send_payment(route).await
    }
    
    /// Open Lightning channel
    pub async fn open_channel(&self, peer: &str, amount_sats: u64) -> Result<ChannelId> {
        self.channels.open_channel(peer, amount_sats).await
    }
}
```

### Bitcoin Wallet Extension

Hierarchical deterministic wallet management.

#### Features

```rust
use anya_wallet::{HDWallet, Mnemonic, ExtendedPrivateKey};

pub struct WalletExtension {
    wallets: HashMap<String, HDWallet>,
    security: SecurityManager,
}

impl WalletExtension {
    /// Create new HD wallet
    pub async fn create_wallet(&mut self, name: &str) -> Result<Mnemonic> {
        let mnemonic = Mnemonic::generate()?;
        let wallet = HDWallet::from_mnemonic(&mnemonic)?;
        
        // Encrypt and store
        let encrypted_wallet = self.security.encrypt_wallet(wallet)?;
        self.wallets.insert(name.to_string(), encrypted_wallet);
        
        Ok(mnemonic)
    }
    
    /// Derive addresses
    pub fn derive_address(&self, wallet: &str, path: &str) -> Result<Address> {
        let wallet = self.wallets.get(wallet)
            .ok_or(Error::WalletNotFound)?;
        
        wallet.derive_address(path)
    }
    
    /// Sign transaction
    pub fn sign_transaction(&self, wallet: &str, tx: Transaction) -> Result<Transaction> {
        let wallet = self.wallets.get(wallet)
            .ok_or(Error::WalletNotFound)?;
        
        wallet.sign_transaction(tx)
    }
}
```

## Web5 Core Extensions

### Web5 Identity Extension

Decentralized identity management using DIDs.

#### Features

```rust
use anya_web5::{DID, Document, VerifiableCredential};

pub struct IdentityExtension {
    did_resolver: DIDResolver,
    credential_manager: CredentialManager,
    key_manager: KeyManager,
}

impl IdentityExtension {
    /// Create new DID
    pub async fn create_did(&self, method: &str) -> Result<DID> {
        let key_pair = self.key_manager.generate_key_pair()?;
        let did = DID::new(method, &key_pair.public_key)?;
        
        // Create DID document
        let document = Document::new(did.clone(), key_pair)?;
        
        // Publish to network
        self.did_resolver.publish_document(document).await?;
        
        Ok(did)
    }
    
    /// Resolve DID to document
    pub async fn resolve_did(&self, did: &str) -> Result<Document> {
        self.did_resolver.resolve(did).await
    }
    
    /// Issue verifiable credential
    pub async fn issue_credential(
        &self,
        issuer_did: &str,
        subject_did: &str,
        claims: serde_json::Value,
    ) -> Result<VerifiableCredential> {
        let credential = VerifiableCredential::new(
            issuer_did,
            subject_did,
            claims,
        )?;
        
        // Sign with issuer's key
        let signed_credential = self.key_manager
            .sign_credential(issuer_did, credential)?;
        
        Ok(signed_credential)
    }
}
```

### Web5 Data Extension

Decentralized data storage and synchronization.

#### Features

```rust
use anya_web5_data::{DataStore, Protocol, Record};

pub struct DataExtension {
    store: DataStore,
    protocols: ProtocolManager,
    sync: SyncEngine,
}

impl DataExtension {
    /// Store data with protocol
    pub async fn store_data(
        &self,
        protocol: &str,
        data: Vec<u8>,
        metadata: RecordMetadata,
    ) -> Result<RecordId> {
        let record = Record::new(protocol, data, metadata)?;
        let record_id = self.store.store(record).await?;
        
        // Sync with network
        self.sync.sync_record(&record_id).await?;
        
        Ok(record_id)
    }
    
    /// Query data by protocol
    pub async fn query_data(
        &self,
        protocol: &str,
        filter: QueryFilter,
    ) -> Result<Vec<Record>> {
        self.store.query(protocol, filter).await
    }
    
    /// Define new protocol
    pub async fn define_protocol(&self, definition: ProtocolDefinition) -> Result<()> {
        self.protocols.define_protocol(definition).await
    }
}
```

## ML Core Extensions

### ML Inference Extension

Machine learning model inference and serving.

#### Features

```rust
use anya_ml::{Model, Tensor, InferenceEngine};

pub struct InferenceExtension {
    engine: InferenceEngine,
    models: ModelRegistry,
    cache: InferenceCache,
}

impl InferenceExtension {
    /// Load ML model
    pub async fn load_model(&mut self, model_path: &str) -> Result<ModelHandle> {
        let model = Model::load_from_file(model_path)?;
        let handle = self.models.register(model)?;
        
        info!("Loaded model: {}", handle.id());
        Ok(handle)
    }
    
    /// Run inference
    pub async fn predict(
        &self,
        model_handle: &ModelHandle,
        input: Tensor,
    ) -> Result<Tensor> {
        // Check cache first
        let cache_key = self.cache.generate_key(model_handle, &input);
        if let Some(cached_result) = self.cache.get(&cache_key).await? {
            return Ok(cached_result);
        }
        
        // Run inference
        let result = self.engine.predict(model_handle, input).await?;
        
        // Cache result
        self.cache.set(&cache_key, &result).await?;
        
        Ok(result)
    }
    
    /// Batch inference
    pub async fn batch_predict(
        &self,
        model_handle: &ModelHandle,
        inputs: Vec<Tensor>,
    ) -> Result<Vec<Tensor>> {
        self.engine.batch_predict(model_handle, inputs).await
    }
}
```

### ML Training Extension

Model training and fine-tuning capabilities.

#### Features

```rust
use anya_ml_training::{Trainer, Dataset, TrainingConfig};

pub struct TrainingExtension {
    trainer: Trainer,
    datasets: DatasetManager,
    metrics: MetricsCollector,
}

impl TrainingExtension {
    /// Train new model
    pub async fn train_model(
        &self,
        dataset: &Dataset,
        config: TrainingConfig,
    ) -> Result<ModelHandle> {
        let training_session = self.trainer.start_training(dataset, config)?;
        
        // Monitor training progress
        let model = self.monitor_training(training_session).await?;
        
        // Evaluate model
        let metrics = self.evaluate_model(&model, dataset).await?;
        self.metrics.record_training_metrics(metrics)?;
        
        Ok(model)
    }
    
    /// Fine-tune existing model
    pub async fn fine_tune_model(
        &self,
        base_model: &ModelHandle,
        dataset: &Dataset,
        config: FineTuningConfig,
    ) -> Result<ModelHandle> {
        self.trainer.fine_tune(base_model, dataset, config).await
    }
}
```

## Security Core Extensions

### Cryptography Extension

Core cryptographic operations and key management.

#### Features

```rust
use anya_crypto::{KeyManager, Cipher, Hash, Signature};

pub struct CryptographyExtension {
    key_manager: KeyManager,
    cipher: Cipher,
    hasher: Hash,
}

impl CryptographyExtension {
    /// Generate cryptographic keys
    pub async fn generate_keypair(&self, algorithm: &str) -> Result<KeyPair> {
        match algorithm {
            "secp256k1" => self.key_manager.generate_secp256k1(),
            "ed25519" => self.key_manager.generate_ed25519(),
            "rsa" => self.key_manager.generate_rsa(2048),
            _ => Err(Error::UnsupportedAlgorithm(algorithm.to_string())),
        }
    }
    
    /// Encrypt data
    pub async fn encrypt(&self, data: &[u8], key: &PublicKey) -> Result<Vec<u8>> {
        self.cipher.encrypt(data, key)
    }
    
    /// Decrypt data
    pub async fn decrypt(&self, data: &[u8], key: &PrivateKey) -> Result<Vec<u8>> {
        self.cipher.decrypt(data, key)
    }
    
    /// Hash data
    pub fn hash(&self, data: &[u8], algorithm: &str) -> Result<Vec<u8>> {
        match algorithm {
            "sha256" => Ok(self.hasher.sha256(data)),
            "blake2b" => Ok(self.hasher.blake2b(data)),
            "keccak256" => Ok(self.hasher.keccak256(data)),
            _ => Err(Error::UnsupportedHashAlgorithm(algorithm.to_string())),
        }
    }
}
```

### Authentication Extension

Multi-factor authentication and session management.

#### Features

```rust
use anya_auth::{Authenticator, Session, MFAProvider};

pub struct AuthenticationExtension {
    authenticator: Authenticator,
    session_manager: SessionManager,
    mfa_provider: MFAProvider,
}

impl AuthenticationExtension {
    /// Authenticate user
    pub async fn authenticate(
        &self,
        username: &str,
        password: &str,
    ) -> Result<AuthenticationResult> {
        let user = self.authenticator.verify_credentials(username, password)?;
        
        // Check if MFA is required
        if user.mfa_enabled {
            return Ok(AuthenticationResult::MFARequired(user.id));
        }
        
        // Create session
        let session = self.session_manager.create_session(user)?;
        Ok(AuthenticationResult::Success(session))
    }
    
    /// Verify MFA token
    pub async fn verify_mfa(&self, user_id: &str, token: &str) -> Result<Session> {
        self.mfa_provider.verify_token(user_id, token)?;
        let user = self.authenticator.get_user(user_id)?;
        self.session_manager.create_session(user)
    }
}
```

## System Core Extensions

### Monitoring Extension

System monitoring and observability.

#### Features

```rust
use anya_monitoring::{MetricsCollector, AlertManager, Dashboard};

pub struct MonitoringExtension {
    metrics: MetricsCollector,
    alerts: AlertManager,
    dashboard: Dashboard,
}

impl MonitoringExtension {
    /// Record metric
    pub async fn record_metric(&self, name: &str, value: f64, tags: Tags) -> Result<()> {
        self.metrics.record(name, value, tags).await
    }
    
    /// Create alert rule
    pub async fn create_alert(&self, rule: AlertRule) -> Result<AlertId> {
        self.alerts.create_rule(rule).await
    }
    
    /// Get system health
    pub async fn get_health(&self) -> Result<HealthStatus> {
        let cpu_usage = self.metrics.get_cpu_usage().await?;
        let memory_usage = self.metrics.get_memory_usage().await?;
        let disk_usage = self.metrics.get_disk_usage().await?;
        
        Ok(HealthStatus {
            cpu_usage,
            memory_usage,
            disk_usage,
            status: if cpu_usage < 80.0 && memory_usage < 80.0 && disk_usage < 80.0 {
                Status::Healthy
            } else {
                Status::Degraded
            },
        })
    }
}
```

### Configuration Extension

Dynamic configuration management.

#### Features

```rust
use anya_config::{ConfigManager, ConfigSource, ConfigValue};

pub struct ConfigurationExtension {
    manager: ConfigManager,
    sources: Vec<Box<dyn ConfigSource>>,
    watchers: Vec<ConfigWatcher>,
}

impl ConfigurationExtension {
    /// Get configuration value
    pub async fn get<T>(&self, key: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let value = self.manager.get(key).await?;
        serde_json::from_value(value)
            .map_err(|e| Error::ConfigDeserialization(e.to_string()))
    }
    
    /// Set configuration value
    pub async fn set<T>(&self, key: &str, value: T) -> Result<()>
    where
        T: serde::Serialize,
    {
        let json_value = serde_json::to_value(value)
            .map_err(|e| Error::ConfigSerialization(e.to_string()))?;
        
        self.manager.set(key, json_value).await
    }
    
    /// Watch for configuration changes
    pub async fn watch(&self, key: &str) -> Result<ConfigWatcher> {
        self.manager.watch(key).await
    }
}
```

## Installation and Configuration

### Global Installation

```bash
# Install all core extensions
anya extension install-core-bundle

# Or install individually
anya extension install bitcoin-client
anya extension install web5-identity
anya extension install ml-inference
anya extension install security-crypto
anya extension install system-monitoring
```

### Configuration File

```toml
# anya-core.toml
[extensions.core]
enabled = ["bitcoin-client", "web5-identity", "ml-inference", "security-crypto"]

[extensions.bitcoin-client]
network = "testnet"
rpc_url = "http://localhost:18332"

[extensions.web5-identity]
did_method = "ion"
resolver_url = "https://ion.network"

[extensions.ml-inference]
device = "cuda"  # cpu, cuda, metal
cache_size = "1GB"

[extensions.security-crypto]
default_algorithm = "secp256k1"
key_derivation = "pbkdf2"

[extensions.system-monitoring]
metrics_retention = "30d"
alert_webhook = "https://alerts.example.com/webhook"
```

### Environment Variables

```bash
# Bitcoin configuration
export ANYA_BITCOIN_NETWORK=testnet
export ANYA_BITCOIN_RPC_URL=http://localhost:18332

# Web5 configuration
export ANYA_WEB5_DID_METHOD=ion
export ANYA_WEB5_RESOLVER_URL=https://ion.network

# ML configuration
export ANYA_ML_DEVICE=cuda
export ANYA_ML_CACHE_SIZE=1GB

# Security configuration
export ANYA_CRYPTO_ALGORITHM=secp256k1
export ANYA_KEY_DERIVATION=pbkdf2
```

## Usage Examples

### Bitcoin Transaction Example

```rust
use anya_extensions::{CoreExtensions, bitcoin::BitcoinClientExtension};

#[tokio::main]
async fn main() -> Result<()> {
    let extensions = CoreExtensions::new().await?;
    let bitcoin = extensions.bitcoin_client();
    
    // Create transaction
    let inputs = vec![TxInput::new("prev_txid", 0, "script_sig")];
    let outputs = vec![TxOutput::new(50000, "recipient_address")];
    
    let tx = bitcoin.create_transaction(inputs, outputs).await?;
    println!("Transaction created: {}", tx.txid());
    
    Ok(())
}
```

### Web5 Identity Example

```rust
use anya_extensions::{CoreExtensions, web5::IdentityExtension};

#[tokio::main]
async fn main() -> Result<()> {
    let extensions = CoreExtensions::new().await?;
    let identity = extensions.web5_identity();
    
    // Create DID
    let did = identity.create_did("ion").await?;
    println!("Created DID: {}", did);
    
    // Issue credential
    let claims = json!({
        "name": "Alice",
        "email": "alice@example.com"
    });
    
    let credential = identity.issue_credential(
        &did.to_string(),
        "did:ion:recipient",
        claims,
    ).await?;
    
    println!("Issued credential: {}", credential.id());
    
    Ok(())
}
```

### ML Inference Example

```rust
use anya_extensions::{CoreExtensions, ml::InferenceExtension};

#[tokio::main]
async fn main() -> Result<()> {
    let extensions = CoreExtensions::new().await?;
    let ml = extensions.ml_inference();
    
    // Load model
    let model = ml.load_model("models/sentiment.onnx").await?;
    
    // Run inference
    let input = Tensor::from_vec(vec![1.0, 2.0, 3.0], vec![1, 3]);
    let output = ml.predict(&model, input).await?;
    
    println!("Prediction: {:?}", output);
    
    Ok(())
}
```

## Development Guide

### Creating Custom Core Extensions

```rust
use anya_core::{Extension, ExtensionMetadata, ExtensionRequest, ExtensionResponse};

pub struct CustomCoreExtension {
    config: CustomConfig,
}

#[async_trait]
impl Extension for CustomCoreExtension {
    fn metadata(&self) -> ExtensionMetadata {
        ExtensionMetadata {
            name: "custom-core".to_string(),
            version: "1.0.0".to_string(),
            category: ExtensionCategory::Core,
            capabilities: vec![
                Capability::Bitcoin,
                Capability::Web5,
            ],
            dependencies: vec![
                Dependency::new("bitcoin-client", "^1.0.0"),
                Dependency::new("web5-identity", "^1.0.0"),
            ],
        }
    }
    
    async fn initialize(&mut self, context: &ExtensionContext) -> Result<()> {
        // Initialize extension with context
        self.config = CustomConfig::from_context(context)?;
        Ok(())
    }
    
    async fn execute(&self, request: ExtensionRequest) -> Result<ExtensionResponse> {
        match request.operation.as_str() {
            "custom_operation" => self.handle_custom_operation(request).await,
            _ => Err(Error::UnsupportedOperation(request.operation)),
        }
    }
    
    async fn shutdown(&mut self) -> Result<()> {
        // Cleanup resources
        Ok(())
    }
}
```

### Extension Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use anya_testing::{TestContext, MockBitcoinClient};
    
    #[tokio::test]
    async fn test_core_extension() {
        let mut context = TestContext::new();
        context.add_mock_service("bitcoin", MockBitcoinClient::new());
        
        let mut extension = CustomCoreExtension::new();
        extension.initialize(&context.as_extension_context()).await.unwrap();
        
        let request = ExtensionRequest::new("custom_operation", json!({}));
        let response = extension.execute(request).await.unwrap();
        
        assert!(response.success);
    }
}
```

---

## Related Documentation

- [Community Extensions](community-extensions.md) - Community-developed extensions
- [Enterprise Extensions](enterprise-extensions.md) - Enterprise-grade extensions
- [Extension Development](../development/README.md) - Development guide
- [API Reference](../development/api-reference.md) - Complete API documentation
- [Integration Patterns](../integration/README.md) - System integration guide

## Community and Support

- **Documentation**: [https://docs.anya-ai.org](https://docs.anya-ai.org)
- **Extension Registry**: [https://extensions.anya-ai.org](https://extensions.anya-ai.org)
- **Community Forum**: [https://community.anya-ai.org](https://community.anya-ai.org)
- **GitHub Issues**: [https://github.com/anya-ai/core/issues](https://github.com/anya-ai/core/issues)
