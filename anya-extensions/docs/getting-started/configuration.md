# Extension Configuration Guide

[AIR-3][AIS-3][AIT-3][RES-3] **Comprehensive configuration guide for Anya Core extensions with Bitcoin, Web5, and ML system integration.**

*Last updated: May 30, 2025*

## Table of Contents

- [Configuration Overview](#configuration-overview)
- [Core Configuration](#core-configuration)
- [Bitcoin Configuration](#bitcoin-configuration)
- [Web5 Configuration](#web5-configuration)
- [ML Configuration](#ml-configuration)
- [Extension Configuration](#extension-configuration)
- [Security Configuration](#security-configuration)
- [Performance Tuning](#performance-tuning)
- [Environment-Specific Setup](#environment-specific-setup)

## Configuration Overview

Anya Core uses a hierarchical configuration system supporting multiple formats:

- **TOML**: Primary configuration format (recommended)
- **JSON**: Alternative for programmatic configuration
- **YAML**: Supported for Docker/Kubernetes deployments
- **Environment Variables**: Runtime overrides and secrets

### Configuration Hierarchy

```
1. Environment variables (highest priority)
2. Command-line arguments
3. User configuration (~/.anya/config.toml)
4. Project configuration (./anya.toml)
5. System configuration (/etc/anya/config.toml)
6. Default values (lowest priority)
```

## Core Configuration

### Basic Configuration File

Create `~/.anya/config.toml`:

```toml
# Anya Core Configuration
[core]
# Core system settings
log_level = "info"  # trace, debug, info, warn, error
data_dir = "/home/user/.anya"
plugin_dir = "/home/user/.anya/extensions"
temp_dir = "/tmp/anya"

# Networking
bind_address = "127.0.0.1"
port = 8080
max_connections = 1000
connection_timeout = 30

# Resource limits
max_memory = "8GB"
max_disk_usage = "100GB"
max_cpu_usage = 80  # percentage

[logging]
# Logging configuration
format = "json"  # json, text, structured
output = "file"  # file, stdout, syslog
file_path = "/var/log/anya/anya.log"
max_file_size = "100MB"
max_files = 10
compress_old_files = true

# Component-specific log levels
[logging.components]
bitcoin = "debug"
web5 = "info"
ml = "warn"
extensions = "info"
```

### Advanced Core Settings

```toml
[core.advanced]
# Threading configuration
worker_threads = 8
blocking_threads = 16
thread_stack_size = "2MB"

# Memory management
garbage_collection_interval = 300  # seconds
memory_pressure_threshold = 0.8
swap_usage_threshold = 0.5

# Cache settings
enable_cache = true
cache_size = "1GB"
cache_ttl = 3600  # seconds
cache_compression = "lz4"

[core.storage]
# Storage backend configuration
backend = "rocksdb"  # rocksdb, sled, sqlite
compression = "snappy"
block_cache_size = "256MB"
write_buffer_size = "64MB"
max_write_buffer_number = 3
```

## Bitcoin Configuration

### Bitcoin Core Integration

```toml
[bitcoin]
# Network configuration
network = "mainnet"  # mainnet, testnet, regtest, signet
chain = "main"

# RPC connection settings
rpc_host = "127.0.0.1"
rpc_port = 8332
rpc_user = "bitcoinrpc"
rpc_password_file = "/home/user/.anya/bitcoin_rpc_password"
rpc_timeout = 60
rpc_max_retries = 3

# Data directory and files
data_dir = "/home/user/.bitcoin"
wallet_dir = "/home/user/.bitcoin/wallets"
blocks_dir = "/home/user/.bitcoin/blocks"

# Performance settings
rpc_work_queue = 32
rpc_threads = 16
max_mempool_size = 300  # MB
mempool_expiry = 336    # hours

[bitcoin.validation]
# Transaction validation
validate_transactions = true
validate_scripts = true
validate_signatures = true
validate_witness = true

# Block validation
validate_blocks = true
assume_valid = "000000000000000000052d314a259755ca65944e18d2e0fb35c047ae3f8a11a5e"
checkpoint_verification = true

[bitcoin.fees]
# Fee estimation
fee_estimation = "economical"  # economical, conservative
target_confirmations = 6
min_relay_fee = 1000  # satoshis per kB
max_fee_rate = 100000  # satoshis per kB

[bitcoin.wallet]
# Wallet settings
default_wallet = "anya_main"
wallet_broadcast = true
wallet_rbf = true
address_type = "bech32"
change_type = "bech32"
keypool_size = 1000
```

### Lightning Network Configuration

```toml
[lightning]
# LND integration
enabled = true
lnd_host = "127.0.0.1"
lnd_port = 10009
tls_cert_path = "/home/user/.lnd/tls.cert"
macaroon_path = "/home/user/.lnd/data/chain/bitcoin/mainnet/admin.macaroon"

# Channel management
auto_channel_open = false
min_channel_size = 20000    # satoshis
max_channel_size = 16777215 # satoshis
channel_fee_rate = 0.001    # percentage

# Payment settings
payment_timeout = 60        # seconds
max_payment_attempts = 3
max_htlc_msat = 990000000  # millisatoshis

[lightning.watchtower]
# Watchtower configuration
enabled = true
watchtower_host = "127.0.0.1"
watchtower_port = 9911
backup_interval = 3600      # seconds
```

## Web5 Configuration

### DID Configuration

```toml
[web5.dids]
# Default DID method
default_method = "did:ion"

# DID resolvers
[web5.dids.resolvers]
ion = { endpoint = "https://beta.ion.msidentity.com/api/v1.0/identifiers/", cache_ttl = 3600 }
key = { local = true }
web = { timeout = 5000, max_redirects = 3 }
jwk = { local = true }

# DID document cache
[web5.dids.cache]
enabled = true
size = "100MB"
ttl = 3600          # seconds
persistent = true

[web5.identity]
# Identity wallet configuration
wallet_path = "/home/user/.anya/web5/wallet"
encryption_key_file = "/home/user/.anya/web5/wallet.key"
backup_enabled = true
backup_interval = 86400  # seconds
```

### Verifiable Credentials Configuration

```toml
[web5.credentials]
# Credential formats
supported_formats = ["jwt", "jsonld", "cbor"]
default_format = "jwt"

# Signature suites
supported_suites = ["Ed25519Signature2020", "JsonWebSignature2020"]
default_suite = "Ed25519Signature2020"

# Credential validation
validate_schema = true
validate_signatures = true
validate_expiration = true
validate_revocation = true

[web5.credentials.issuance]
# Credential issuance
default_issuer_did = "did:ion:EiClkZMDxPKqC9c-umQfTkR8vvZ9JPhl_xLDI9Nfk38w5w"
default_validity_period = 31536000  # seconds (1 year)
include_proof = true
include_metadata = true

[web5.credentials.verification]
# Verification settings
require_proof = true
allow_expired = false
check_revocation = true
trusted_issuers = []
```

### Protocol Configuration

```toml
[web5.protocols]
# Protocol definitions
protocol_path = "/home/user/.anya/web5/protocols"
auto_install = true
update_check_interval = 86400  # seconds

# Default protocols
[web5.protocols.definitions]
social = "https://areweweb5yet.com/protocols/social"
chat = "https://areweweb5yet.com/protocols/chat"
marketplace = "https://areweweb5yet.com/protocols/marketplace"

[web5.dwn]
# Decentralized Web Node configuration
enabled = true
endpoints = [
    "https://dwn.tbddev.org/dwn0",
    "https://dwn.tbddev.org/dwn3",
    "https://dwn.tbddev.org/dwn5"
]
sync_interval = 300         # seconds
conflict_resolution = "last_write_wins"
```

## ML Configuration

### Inference Configuration

```toml
[ml.inference]
# Runtime configuration
backend = "onnx"           # onnx, tensorflow, pytorch, tflite
device = "cpu"             # cpu, cuda, metal, opencl
num_threads = 4
batch_size = 32
max_sequence_length = 512

# Model management
model_repository = "/home/user/.anya/models"
cache_size = "10GB"
auto_download = true
update_check_interval = 86400  # seconds

# Performance optimization
enable_gpu_memory_growth = true
gpu_memory_limit = "4GB"
enable_mixed_precision = true
optimize_for_inference = true

[ml.models]
# Pre-configured models
[ml.models.text_classification]
name = "anya-text-classifier"
version = "1.0.0"
file = "text_classifier.onnx"
input_size = [1, 512]
output_size = [1, 10]

[ml.models.sentiment_analysis]
name = "anya-sentiment"
version = "2.1.0"
file = "sentiment_model.onnx"
preprocessing = "bert_tokenizer"
postprocessing = "softmax"

[ml.training]
# Training configuration (if enabled)
enabled = false
output_dir = "/home/user/.anya/training"
checkpoint_interval = 1000
validation_split = 0.2
early_stopping_patience = 5

# Resource allocation for training
max_memory = "16GB"
max_gpu_memory = "8GB"
distributed_training = false
```

### Model Serving Configuration

```toml
[ml.serving]
# Model serving API
enabled = true
host = "127.0.0.1"
port = 8081
max_concurrent_requests = 100
request_timeout = 30       # seconds

# Load balancing
model_replicas = 2
load_balancing_strategy = "round_robin"
health_check_interval = 30  # seconds

[ml.monitoring]
# Performance monitoring
enabled = true
metrics_port = 8082
log_predictions = false    # Privacy consideration
log_performance = true
alert_on_degradation = true
```

## Extension Configuration

### Extension Management

```toml
[extensions]
# Extension system settings
enabled_extensions = [
    "bitcoin-core",
    "web5-dids", 
    "ml-inference",
    "security-tools"
]
extension_path = "/home/user/.anya/extensions"
auto_update = false
update_check_interval = 86400  # seconds

# Extension sandboxing
enable_sandbox = true
max_memory_per_extension = "1GB"
max_cpu_per_extension = 50     # percentage
network_isolation = true

[extensions.repositories]
# Extension repositories
official = "https://extensions.anya.org"
community = "https://community.anya.org/extensions"
enterprise = "https://enterprise.anya.org/extensions"

# Repository authentication
[extensions.auth]
official_token_file = "/home/user/.anya/official_token"
enterprise_license_file = "/home/user/.anya/enterprise.license"
```

### Per-Extension Configuration

```toml
# Bitcoin extension configuration
[extensions.bitcoin-core]
priority = 10
memory_limit = "2GB"
cpu_limit = 25
network_access = ["bitcoin_network"]

# Web5 extension configuration
[extensions.web5-dids]
priority = 8
memory_limit = "1GB"
cpu_limit = 15
network_access = ["web5_resolvers", "dwn_endpoints"]

# ML extension configuration
[extensions.ml-inference]
priority = 5
memory_limit = "4GB"
cpu_limit = 40
gpu_access = true
```

## Security Configuration

### Cryptography Settings

```toml
[security.crypto]
# Cryptographic preferences
default_key_type = "ed25519"
default_hash_algorithm = "sha256"
secure_random_source = "/dev/urandom"

# Key management
key_derivation_function = "scrypt"
key_derivation_iterations = 32768
password_min_length = 12
password_require_special = true

[security.encryption]
# Data encryption
encrypt_at_rest = true
encryption_algorithm = "aes-256-gcm"
key_rotation_interval = 2592000  # seconds (30 days)

# Transport encryption
require_tls = true
min_tls_version = "1.3"
cipher_suites = ["TLS_AES_256_GCM_SHA384", "TLS_CHACHA20_POLY1305_SHA256"]

[security.access_control]
# Access control
enable_rbac = true
default_permissions = "read"
session_timeout = 3600     # seconds
max_failed_attempts = 5
lockout_duration = 1800    # seconds
```

### Network Security

```toml
[security.network]
# Firewall settings
allowed_ips = ["127.0.0.1", "::1"]
blocked_ips = []
enable_rate_limiting = true
rate_limit_requests = 100  # per minute
rate_limit_window = 60     # seconds

# DDoS protection
enable_ddos_protection = true
max_connections_per_ip = 10
connection_rate_limit = 5  # per second

[security.audit]
# Security auditing
enable_audit_log = true
audit_log_path = "/var/log/anya/audit.log"
log_successful_auth = true
log_failed_auth = true
log_privilege_escalation = true
```

## Performance Tuning

### System Optimization

```toml
[performance]
# Performance profiles
profile = "balanced"       # balanced, performance, efficiency

# CPU optimization
cpu_affinity = [0, 1, 2, 3]
enable_numa_optimization = true
scheduler_policy = "normal"  # normal, batch, idle, fifo, rr

# Memory optimization
memory_allocator = "jemalloc"  # system, jemalloc, tcmalloc
enable_memory_mapping = true
prefault_pages = true
huge_pages = "transparent"

[performance.caching]
# Cache configuration
l1_cache_size = "32KB"
l2_cache_size = "256KB"
l3_cache_size = "8MB"
cache_line_size = 64       # bytes
prefetch_distance = 4
```

### Database Optimization

```toml
[performance.database]
# RocksDB optimization
write_buffer_size = "64MB"
max_write_buffer_number = 3
target_file_size_base = "64MB"
max_bytes_for_level_base = "256MB"
compression_type = "snappy"
bloom_filter_bits_per_key = 10
cache_index_and_filter_blocks = true
```

## Environment-Specific Setup

### Development Environment

```toml
[environments.development]
log_level = "debug"
enable_debug_endpoints = true
hot_reload = true
disable_auth = false       # Keep auth even in dev
mock_external_services = true

[environments.development.bitcoin]
network = "regtest"
generate_blocks = true
fund_addresses = true
```

### Testing Environment

```toml
[environments.testing]
log_level = "warn"
enable_test_mode = true
reset_data_on_startup = true
use_in_memory_storage = true

[environments.testing.overrides]
bitcoin_network = "testnet"
web5_test_mode = true
ml_models = "test_models"
```

### Production Environment

```toml
[environments.production]
log_level = "info"
enable_monitoring = true
enable_metrics = true
health_check_enabled = true

[environments.production.security]
enforce_https = true
enable_audit_logging = true
require_authentication = true
rate_limiting_strict = true

[environments.production.performance]
profile = "performance"
enable_all_optimizations = true
monitoring_interval = 60   # seconds
```

### Docker Configuration

```toml
[environments.docker]
# Docker-specific settings
use_host_network = false
mount_host_bitcoin_data = true
shared_volume_path = "/anya-data"

[environments.docker.resources]
memory_limit = "8GB"
cpu_limit = "4.0"
storage_limit = "100GB"
```

## Configuration Validation

### Validation Commands

```bash
# Validate configuration file
anya config validate

# Check configuration against schema
anya config validate --schema

# Test configuration with dry run
anya config test --dry-run

# Show effective configuration
anya config show --effective

# Compare configurations
anya config diff production.toml staging.toml
```

### Configuration Templates

```bash
# Generate configuration template
anya config template --output config.toml

# Generate environment-specific template
anya config template --env production --output production.toml

# Generate minimal configuration
anya config template --minimal --output minimal.toml
```

## Environment Variables

### Core Variables

```bash
# Core settings
export ANYA_LOG_LEVEL=info
export ANYA_DATA_DIR=/home/user/.anya
export ANYA_CONFIG_FILE=/home/user/.anya/config.toml

# Network settings
export ANYA_BIND_ADDRESS=127.0.0.1
export ANYA_PORT=8080

# Security settings
export ANYA_ENCRYPT_AT_REST=true
export ANYA_REQUIRE_AUTH=true
```

### Bitcoin Variables

```bash
# Bitcoin configuration
export BITCOIN_NETWORK=mainnet
export BITCOIN_RPC_HOST=127.0.0.1
export BITCOIN_RPC_PORT=8332
export BITCOIN_RPC_USER=bitcoinrpc
export BITCOIN_RPC_PASSWORD=your_secure_password
```

### Web5 Variables

```bash
# Web5 configuration
export WEB5_DID_METHOD=did:ion
export WEB5_DWN_ENDPOINTS=https://dwn.tbddev.org/dwn0,https://dwn.tbddev.org/dwn3
export WEB5_CACHE_ENABLED=true
```

### ML Variables

```bash
# ML configuration
export ML_BACKEND=onnx
export ML_DEVICE=cpu
export ML_MODEL_REPOSITORY=/home/user/.anya/models
export ML_CACHE_SIZE=10GB
```

## Troubleshooting Configuration

### Common Issues

#### Invalid Configuration Format
```bash
# Check TOML syntax
anya config validate --syntax-only

# Show parsing errors
anya config parse --debug
```

#### Permission Issues
```bash
# Fix file permissions
chmod 600 ~/.anya/config.toml
chown $USER:$USER ~/.anya/config.toml

# Fix directory permissions
chmod 755 ~/.anya
```

#### Environment Variable Conflicts
```bash
# Show environment variables
anya config env

# Clear conflicting variables
unset ANYA_LOG_LEVEL
unset BITCOIN_NETWORK
```

For detailed troubleshooting, see the [Installation Guide](./installation.md#troubleshooting) and [Best Practices](../development/best-practices.md).
