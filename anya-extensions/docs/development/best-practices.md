# Best Practices

[AIR-3][AIS-3][AIT-3][RES-3][BPC-3]

Best practices for developing high-quality, secure, and maintainable extensions for the Anya Core platform.

*Last updated: June 7, 2025*

## Code Quality Standards

### Rust Best Practices

#### Error Handling

```rust
// ✅ Good: Use Result types with meaningful error messages
fn process_transaction(tx: &Transaction) -> Result<ProcessingResult, ExtensionError> {
    let validated = validate_transaction(tx)
        .map_err(|e| ExtensionError::Validation(format!("Invalid transaction: {}", e)))?;
    
    // Process the validated transaction
    Ok(ProcessingResult::Success(validated))
}

// ❌ Bad: Using unwrap() or expect() in production code
fn process_transaction_bad(tx: &Transaction) -> ProcessingResult {
    let validated = validate_transaction(tx).unwrap(); // Can panic!
    ProcessingResult::Success(validated)
}
```

#### Memory Management

```rust
// ✅ Good: Use Arc for shared ownership, minimize clones
use std::sync::Arc;

struct ExtensionState {
    config: Arc<ExtensionConfig>,
    cache: Arc<RwLock<HashMap<String, Value>>>,
}

// ✅ Good: Implement Drop for proper cleanup
impl Drop for MyExtension {
    fn drop(&mut self) {
        // Clean up resources
        self.close_connections();
        self.flush_caches();
    }
}
```

#### Concurrency

```rust
// ✅ Good: Use async/await for I/O operations
async fn fetch_bitcoin_data(&self, block_hash: &str) -> Result<Block, BitcoinError> {
    let response = self.client
        .get(&format!("/block/{}", block_hash))
        .send()
        .await?;
    
    let block: Block = response.json().await?;
    Ok(block)
}

// ✅ Good: Use proper synchronization primitives
use tokio::sync::RwLock;

struct CacheManager {
    cache: Arc<RwLock<HashMap<String, CachedValue>>>,
}

impl CacheManager {
    async fn get(&self, key: &str) -> Option<CachedValue> {
        let cache = self.cache.read().await;
        cache.get(key).cloned()
    }
    
    async fn set(&self, key: String, value: CachedValue) {
        let mut cache = self.cache.write().await;
        cache.insert(key, value);
    }
}
```

### Architecture Patterns

#### Hexagonal Architecture

```rust
// ✅ Good: Separate domain logic from infrastructure
pub struct TransactionProcessor {
    // Domain logic - no external dependencies
    rules: Vec<ValidationRule>,
}

impl TransactionProcessor {
    pub fn process(&self, tx: &Transaction) -> Result<ProcessedTransaction, ProcessingError> {
        // Pure domain logic
        for rule in &self.rules {
            rule.validate(tx)?;
        }
        Ok(ProcessedTransaction::from(tx))
    }
}

// Infrastructure adapters
pub struct BitcoinNetworkAdapter {
    client: BitcoinClient,
}

impl NetworkPort for BitcoinNetworkAdapter {
    async fn broadcast_transaction(&self, tx: &Transaction) -> Result<(), NetworkError> {
        self.client.send_transaction(tx).await
    }
}
```

#### Dependency Injection

```rust
// ✅ Good: Use dependency injection for testability
pub struct MyExtension<N: NetworkPort, S: StoragePort> {
    network: N,
    storage: S,
    processor: TransactionProcessor,
}

impl<N: NetworkPort, S: StoragePort> MyExtension<N, S> {
    pub fn new(network: N, storage: S) -> Self {
        Self {
            network,
            storage,
            processor: TransactionProcessor::new(),
        }
    }
}

// Easy to test with mocks
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_transaction_processing() {
        let mock_network = MockNetworkAdapter::new();
        let mock_storage = MockStorageAdapter::new();
        let extension = MyExtension::new(mock_network, mock_storage);
        
        // Test logic...
    }
}
```

## Security Best Practices

### Input Validation

```rust
// ✅ Good: Validate all inputs
fn process_user_input(input: &str) -> Result<ProcessedInput, ValidationError> {
    // Length validation
    if input.len() > MAX_INPUT_LENGTH {
        return Err(ValidationError::InputTooLong);
    }
    
    // Content validation
    if !input.chars().all(|c| c.is_alphanumeric() || c.is_whitespace()) {
        return Err(ValidationError::InvalidCharacters);
    }
    
    // Business logic validation
    let parsed = parse_input(input)?;
    validate_business_rules(&parsed)?;
    
    Ok(ProcessedInput::new(parsed))
}
```

### Cryptographic Operations

```rust
// ✅ Good: Use secure random number generation
use rand::rngs::OsRng;

fn generate_nonce() -> [u8; 32] {
    let mut nonce = [0u8; 32];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

// ✅ Good: Use constant-time comparisons for sensitive data
use subtle::ConstantTimeEq;

fn verify_signature(signature: &[u8], expected: &[u8]) -> bool {
    signature.ct_eq(expected).into()
}

// ✅ Good: Clear sensitive data from memory
use zeroize::Zeroize;

struct PrivateKey([u8; 32]);

impl Drop for PrivateKey {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}
```

### Permission Management

```rust
// ✅ Good: Implement principle of least privilege
impl ExtensionTrait for MyExtension {
    fn metadata(&self) -> ExtensionMetadata {
        ExtensionMetadata {
            permissions: vec![
                Permission::ReadBitcoinData, // Only request what's needed
                Permission::NetworkAccess(vec!["api.bitcoin.org".to_string()]),
            ],
            // Don't request unnecessary permissions
        }
    }
}

// ✅ Good: Check permissions before operations
fn access_bitcoin_data(&self) -> Result<BitcoinData, SecurityError> {
    if !self.has_permission(&Permission::ReadBitcoinData) {
        return Err(SecurityError::PermissionDenied);
    }
    
    // Proceed with operation
    self.fetch_bitcoin_data()
}
```

## Performance Best Practices

### Efficient Data Structures

```rust
// ✅ Good: Use appropriate data structures
use std::collections::HashMap;
use indexmap::IndexMap;

struct TransactionCache {
    // For fast lookups
    by_id: HashMap<TxId, Transaction>,
    // For maintaining insertion order
    ordered: IndexMap<TxId, Transaction>,
}

// ✅ Good: Implement efficient serialization
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct EfficientData {
    #[serde(with = "serde_bytes")]
    binary_data: Vec<u8>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    optional_field: Option<String>,
}
```

### Caching Strategies

```rust
// ✅ Good: Implement LRU cache with TTL
use lru::LruCache;
use std::time::{Duration, Instant};

struct CachedValue<T> {
    value: T,
    expires_at: Instant,
}

struct TimedLruCache<T> {
    cache: LruCache<String, CachedValue<T>>,
    ttl: Duration,
}

impl<T> TimedLruCache<T> {
    fn get(&mut self, key: &str) -> Option<&T> {
        if let Some(cached) = self.cache.get(key) {
            if cached.expires_at > Instant::now() {
                return Some(&cached.value);
            } else {
                self.cache.pop(key);
            }
        }
        None
    }
    
    fn put(&mut self, key: String, value: T) {
        let cached = CachedValue {
            value,
            expires_at: Instant::now() + self.ttl,
        };
        self.cache.put(key, cached);
    }
}
```

### Resource Management

```rust
// ✅ Good: Use connection pooling
use deadpool_postgres::{Config, Pool};

struct DatabaseManager {
    pool: Pool,
}

impl DatabaseManager {
    async fn execute_query(&self, query: &str) -> Result<Vec<Row>, DatabaseError> {
        let client = self.pool.get().await?;
        let rows = client.query(query, &[]).await?;
        Ok(rows)
    }
}

// ✅ Good: Implement backpressure for high-throughput scenarios
use tokio::sync::Semaphore;

struct RateLimitedProcessor {
    semaphore: Arc<Semaphore>,
    max_concurrent: usize,
}

impl RateLimitedProcessor {
    async fn process_transaction(&self, tx: Transaction) -> Result<(), ProcessingError> {
        let _permit = self.semaphore.acquire().await?;
        // Process transaction with rate limiting
        self.do_process(tx).await
    }
}
```

## Testing Best Practices

### Unit Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    
    mock! {
        NetworkAdapter {}
        
        #[async_trait]
        impl NetworkPort for NetworkAdapter {
            async fn send_transaction(&self, tx: &Transaction) -> Result<(), NetworkError>;
        }
    }
    
    #[tokio::test]
    async fn test_transaction_processing() {
        // ✅ Good: Use mocks for external dependencies
        let mut mock_network = MockNetworkAdapter::new();
        mock_network
            .expect_send_transaction()
            .times(1)
            .returning(|_| Ok(()));
        
        let processor = TransactionProcessor::new(mock_network);
        let tx = create_test_transaction();
        
        let result = processor.process(tx).await;
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validation_rules() {
        // ✅ Good: Test edge cases and error conditions
        let validator = TransactionValidator::new();
        
        // Test valid transaction
        let valid_tx = create_valid_transaction();
        assert!(validator.validate(&valid_tx).is_ok());
        
        // Test invalid transactions
        let invalid_tx = create_invalid_transaction();
        assert!(validator.validate(&invalid_tx).is_err());
        
        // Test edge cases
        let zero_value_tx = create_zero_value_transaction();
        assert!(validator.validate(&zero_value_tx).is_err());
    }
}
```

### Integration Testing

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use testcontainers::*;
    
    #[tokio::test]
    async fn test_bitcoin_integration() {
        // ✅ Good: Use testcontainers for integration tests
        let docker = clients::Cli::default();
        let bitcoin_node = docker.run(images::bitcoin::Bitcoin::default());
        
        let config = BitcoinConfig {
            rpc_url: format!("http://localhost:{}", bitcoin_node.get_host_port(18443)),
            // ... other config
        };
        
        let client = BitcoinClient::new(config);
        
        // Test real Bitcoin integration
        let block_count = client.get_block_count().await?;
        assert!(block_count >= 0);
    }
}
```

## Documentation Best Practices

### Code Documentation

```rust
/// Processes Bitcoin transactions according to BIP-341 Taproot specifications.
/// 
/// # Arguments
/// 
/// * `transaction` - The Bitcoin transaction to process
/// * `context` - Processing context including network parameters
/// 
/// # Returns
/// 
/// Returns `Ok(ProcessingResult)` on successful processing, or an error if:
/// - Transaction validation fails
/// - Network communication errors occur
/// - Insufficient permissions
/// 
/// # Examples
/// 
/// ```rust
/// use anya_core::bitcoin::Transaction;
/// 
/// let processor = TransactionProcessor::new();
/// let tx = Transaction::from_hex("01000000...")?;
/// let context = ProcessingContext::mainnet();
/// 
/// match processor.process_transaction(&tx, &context).await {
///     Ok(result) => println!("Processed: {:?}", result),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
/// 
/// # Compliance
/// 
/// This function implements:
/// - BIP-341: Taproot validation rules
/// - BIP-342: Tapscript execution
/// - BIP-174: PSBT compatibility
pub async fn process_transaction(
    &self,
    transaction: &Transaction,
    context: &ProcessingContext,
) -> Result<ProcessingResult, ProcessingError> {
    // Implementation...
}
```

### Error Messages

```rust
// ✅ Good: Provide helpful error messages with context
#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Transaction input {input_index} references unknown UTXO {txid}:{vout}")]
    UnknownUtxo {
        input_index: usize,
        txid: String,
        vout: u32,
    },
    
    #[error("Insufficient funds: required {required} satoshis, available {available} satoshis")]
    InsufficientFunds {
        required: u64,
        available: u64,
    },
    
    #[error("Invalid signature for input {input_index}: {reason}")]
    InvalidSignature {
        input_index: usize,
        reason: String,
    },
}
```

## Configuration Management

### Environment-Specific Settings

```rust
// ✅ Good: Use configuration files with environment overrides
#[derive(Debug, Deserialize)]
pub struct ExtensionConfig {
    #[serde(default)]
    pub network: NetworkConfig,
    
    #[serde(default)]
    pub security: SecurityConfig,
    
    #[serde(default)]
    pub performance: PerformanceConfig,
}

impl Default for ExtensionConfig {
    fn default() -> Self {
        Self {
            network: NetworkConfig::mainnet(),
            security: SecurityConfig::production(),
            performance: PerformanceConfig::default(),
        }
    }
}

// Load configuration with environment overrides
let config = ConfigBuilder::new()
    .add_source(config::File::with_name("config/default"))
    .add_source(config::File::with_name(&format!("config/{}", env)).required(false))
    .add_source(config::Environment::with_prefix("ANYA").separator("_"))
    .build()?
    .try_deserialize::<ExtensionConfig>()?;
```

### Secrets Management

```rust
// ✅ Good: Use proper secrets management
use anya_core::security::SecretManager;

async fn load_api_key() -> Result<String, SecurityError> {
    let secret_manager = SecretManager::new();
    
    // Try environment variable first
    if let Ok(key) = std::env::var("API_KEY") {
        return Ok(key);
    }
    
    // Fall back to secure storage
    secret_manager.get_secret("bitcoin_api_key").await
}
```

## Monitoring and Observability

### Metrics and Logging

```rust
use tracing::{info, warn, error, instrument};
use anya_core::metrics::Counter;

static TRANSACTION_COUNTER: Counter = Counter::new("extension_transactions_processed_total");

#[instrument(skip(self, transaction))]
pub async fn process_transaction(&self, transaction: &Transaction) -> Result<(), ProcessingError> {
    info!(
        tx_id = %transaction.txid(),
        input_count = transaction.inputs().len(),
        output_count = transaction.outputs().len(),
        "Processing transaction"
    );
    
    match self.validate_transaction(transaction).await {
        Ok(_) => {
            TRANSACTION_COUNTER.increment();
            info!("Transaction processed successfully");
            Ok(())
        }
        Err(e) => {
            warn!(error = %e, "Transaction validation failed");
            Err(e)
        }
    }
}
```

### Health Checks

```rust
impl ExtensionTrait for MyExtension {
    fn health_check(&self) -> HealthStatus {
        let mut checks = Vec::new();
        
        // Check database connectivity
        if let Err(e) = self.database.ping() {
            checks.push(HealthCheck::unhealthy("database", e.to_string()));
        } else {
            checks.push(HealthCheck::healthy("database"));
        }
        
        // Check external API connectivity
        if let Err(e) = self.api_client.health_check() {
            checks.push(HealthCheck::unhealthy("external_api", e.to_string()));
        } else {
            checks.push(HealthCheck::healthy("external_api"));
        }
        
        HealthStatus::from_checks(checks)
    }
}
```

## Deployment Best Practices

### Container Configuration

```dockerfile
# ✅ Good: Multi-stage build for smaller images
FROM rust:1.70 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/my-extension /usr/local/bin/

# ✅ Good: Run as non-root user
RUN useradd -r -s /bin/false anya
USER anya

EXPOSE 8080
CMD ["my-extension"]
```

### Resource Limits

```yaml
# ✅ Good: Set appropriate resource limits
apiVersion: v1
kind: Pod
spec:
  containers:
  - name: my-extension
    image: my-extension:latest
    resources:
      requests:
        memory: "64Mi"
        cpu: "250m"
      limits:
        memory: "128Mi"
        cpu: "500m"
    livenessProbe:
      httpGet:
        path: /health
        port: 8080
      initialDelaySeconds: 30
      periodSeconds: 10
    readinessProbe:
      httpGet:
        path: /ready
        port: 8080
      initialDelaySeconds: 5
      periodSeconds: 5
```

## Common Anti-Patterns to Avoid

### ❌ Bad Practices

```rust
// Don't use global mutable state
static mut GLOBAL_CONFIG: Option<Config> = None;

// Don't ignore errors
let result = risky_operation(); // Missing error handling

// Don't use blocking operations in async code
async fn bad_async_function() {
    std::thread::sleep(Duration::from_secs(1)); // Blocks executor!
}

// Don't hardcode configuration values
const API_URL: &str = "https://api.bitcoin.org"; // Should be configurable

// Don't use unwrap() in production code
let value = might_fail().unwrap(); // Can panic!
```

### ✅ Good Alternatives

```rust
// Use dependency injection for configuration
struct MyExtension {
    config: Arc<Config>,
}

// Always handle errors appropriately
let result = risky_operation()
    .map_err(|e| ProcessingError::from(e))?;

// Use async sleep in async code
async fn good_async_function() {
    tokio::time::sleep(Duration::from_secs(1)).await;
}

// Make configuration flexible
let api_url = config.api_url.as_deref()
    .unwrap_or("https://api.bitcoin.org");

// Use proper error handling
let value = might_fail()
    .map_err(|e| format!("Operation failed: {}", e))?;
```
