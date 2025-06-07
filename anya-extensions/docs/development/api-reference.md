# API Reference

[AIR-3][AIS-3][AIT-3][BPC-3][RES-3]

Complete API reference for Anya Core extension development, covering all interfaces, traits, and utilities available to extension developers.

*Last updated: June 7, 2025*

## Core Extension Traits

### ExtensionTrait
Base trait that all extensions must implement:

```rust
pub trait ExtensionTrait: Send + Sync {
    /// Initialize the extension with provided configuration
    fn initialize(&mut self, config: ExtensionConfig) -> Result<(), ExtensionError>;
    
    /// Execute extension logic within the provided context
    fn execute(&self, context: &ExecutionContext) -> Result<ExtensionResult, ExtensionError>;
    
    /// Clean up resources when extension is shutting down
    fn shutdown(&mut self) -> Result<(), ExtensionError>;
    
    /// Get extension metadata
    fn metadata(&self) -> ExtensionMetadata;
    
    /// Health check for the extension
    fn health_check(&self) -> HealthStatus;
}
```

### BitcoinExtensionTrait
Specialized trait for Bitcoin protocol extensions:

```rust
pub trait BitcoinExtensionTrait: ExtensionTrait {
    /// Process Bitcoin transactions
    fn process_transaction(&self, tx: &Transaction) -> Result<ProcessingResult, BitcoinError>;
    
    /// Validate transaction against extension rules
    fn validate_transaction(&self, tx: &Transaction) -> Result<bool, ValidationError>;
    
    /// Handle BIP-specific functionality
    fn handle_bip(&self, bip_type: BipType, data: &[u8]) -> Result<BipResult, BipError>;
}
```

## Core APIs

### Bitcoin Integration
```rust
// Transaction management
use anya_core::bitcoin::{Transaction, TransactionManager};

let tx_manager = TransactionManager::new(network_config)?;
let transaction = tx_manager.create_transaction(inputs, outputs)?;

// PSBT operations
use anya_core::bitcoin::psbt::{Psbt, PsbtManager};

let psbt_manager = PsbtManager::new();
let psbt = psbt_manager.create_psbt(transaction)?;
```

### Web5 Integration
```rust
// Decentralized Identity
use anya_core::web5::{Did, DidManager};

let did_manager = DidManager::new();
let did = did_manager.create_did(key_pair)?;

// Decentralized Web Node
use anya_core::web5::{Dwn, DwnMessage};

let dwn = Dwn::new(config);
let message = DwnMessage::new(data, signature);
dwn.store_message(message)?;
```

### ML/AI Integration
```rust
// Agent System
use anya_core::ml::{Agent, AgentChecker, SystemStage};

let agent_checker = AgentChecker::new();
let stage = agent_checker.determine_stage(&system_metrics)?;

// Model Management
use anya_core::ml::{MlModel, ModelManager};

let model_manager = ModelManager::new();
let model = model_manager.load_model("bitcoin_analytics")?;
let prediction = model.predict(&input_data)?;
```

## Configuration API

### ExtensionConfig
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionConfig {
    pub name: String,
    pub version: String,
    pub enabled: bool,
    pub settings: HashMap<String, serde_json::Value>,
    pub dependencies: Vec<String>,
    pub permissions: Vec<Permission>,
}

impl ExtensionConfig {
    pub fn new(name: impl Into<String>) -> Self;
    pub fn with_setting<T: Serialize>(mut self, key: &str, value: T) -> Self;
    pub fn get_setting<T: DeserializeOwned>(&self, key: &str) -> Result<T, ConfigError>;
}
```

## Error Handling

### ExtensionError
```rust
#[derive(Debug, Error)]
pub enum ExtensionError {
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Initialization failed: {0}")]
    Initialization(String),
    
    #[error("Execution error: {0}")]
    Execution(String),
    
    #[error("Bitcoin protocol error: {0}")]
    Bitcoin(#[from] BitcoinError),
    
    #[error("Web5 error: {0}")]
    Web5(#[from] Web5Error),
    
    #[error("ML error: {0}")]
    Ml(#[from] MlError),
}
```

## Event System

### Event Publishing
```rust
use anya_core::events::{Event, EventBus, EventHandler};

// Publishing events
let event = Event::new("transaction.confirmed", transaction_data);
event_bus.publish(event).await?;

// Subscribing to events
struct MyEventHandler;

impl EventHandler for MyEventHandler {
    async fn handle(&self, event: Event) -> Result<(), EventError> {
        match event.event_type.as_str() {
            "transaction.confirmed" => {
                // Handle transaction confirmation
                Ok(())
            },
            _ => Ok(()),
        }
    }
}

event_bus.subscribe("transaction.*", Box::new(MyEventHandler)).await?;
```

## Security APIs

### Permission System
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Permission {
    ReadBitcoinData,
    WriteBitcoinData,
    AccessWeb5Storage,
    ExecuteMlModels,
    NetworkAccess(Vec<String>),
    FileSystemAccess(PathBuf),
}

// Check permissions
fn check_permission(extension: &Extension, permission: &Permission) -> bool {
    extension.metadata().permissions.contains(permission)
}
```

### Cryptographic Operations
```rust
use anya_core::crypto::{KeyPair, Signature, Hash};

// Key generation
let key_pair = KeyPair::generate()?;

// Signing
let message = b"Hello, Bitcoin!";
let signature = key_pair.sign(message)?;

// Verification
let is_valid = key_pair.public_key().verify(message, &signature)?;
```

## Testing APIs

### Test Utilities
```rust
use anya_core::testing::{TestEnvironment, MockBitcoinNetwork, TestConfig};

// Set up test environment
let test_env = TestEnvironment::new()
    .with_mock_bitcoin_network()
    .with_test_config(test_config);

// Extension testing
let mut extension = MyExtension::new();
extension.initialize(test_env.config())?;

let result = extension.execute(&test_env.context())?;
assert!(result.is_success());
```

## Performance Monitoring

### Metrics Collection
```rust
use anya_core::metrics::{Counter, Histogram, Gauge};

// Define metrics
let transaction_counter = Counter::new("extension_transactions_total")?;
let processing_time = Histogram::new("extension_processing_duration_seconds")?;
let active_connections = Gauge::new("extension_active_connections")?;

// Record metrics
transaction_counter.increment();
let timer = processing_time.start_timer();
// ... do work ...
timer.observe_duration();
```

## Lifecycle Management

### Extension Manager
```rust
use anya_core::extensions::{ExtensionManager, ExtensionStatus};

let manager = ExtensionManager::new();

// Load extension
manager.load_extension("my_extension", config).await?;

// Check status
let status = manager.get_status("my_extension")?;
assert_eq!(status, ExtensionStatus::Running);

// Reload extension
manager.reload_extension("my_extension").await?;

// Unload extension
manager.unload_extension("my_extension").await?;
```

## Examples

### Simple Bitcoin Extension
```rust
use anya_core::prelude::*;

#[derive(Extension)]
pub struct TransactionLogger {
    log_file: PathBuf,
}

impl ExtensionTrait for TransactionLogger {
    fn initialize(&mut self, config: ExtensionConfig) -> Result<(), ExtensionError> {
        self.log_file = config.get_setting("log_file")?;
        Ok(())
    }
    
    fn execute(&self, context: &ExecutionContext) -> Result<ExtensionResult, ExtensionError> {
        if let Some(tx) = context.get_transaction() {
            self.log_transaction(tx)?;
        }
        Ok(ExtensionResult::Success)
    }
    
    fn shutdown(&mut self) -> Result<(), ExtensionError> {
        // Cleanup resources
        Ok(())
    }
    
    fn metadata(&self) -> ExtensionMetadata {
        ExtensionMetadata {
            name: "TransactionLogger".to_string(),
            version: "1.0.0".to_string(),
            description: "Logs Bitcoin transactions".to_string(),
            permissions: vec![Permission::ReadBitcoinData],
        }
    }
    
    fn health_check(&self) -> HealthStatus {
        if self.log_file.exists() {
            HealthStatus::Healthy
        } else {
            HealthStatus::Unhealthy("Log file missing".to_string())
        }
    }
}

impl TransactionLogger {
    fn log_transaction(&self, tx: &Transaction) -> Result<(), ExtensionError> {
        let log_entry = format!("{}: {}\n", 
            chrono::Utc::now().to_rfc3339(), 
            tx.txid()
        );
        std::fs::write(&self.log_file, log_entry)?;
        Ok(())
    }
}
```

## Integration Examples

See the [Integration Guide](../integration/README.md) for complete examples of:
- Bitcoin protocol extensions
- Web5 service integrations
- ML model deployment
- Security plugin development

## Resources

- [Extension Development Guide](README.md)
- [Best Practices](best-practices.md)  
- [Security Guidelines](../integration/security-guidelines.md)
- [Community Extensions](../extensions/community-extensions.md)
