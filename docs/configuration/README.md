# Configuration Reference

[AIR-3][AIS-3][BPC-3][RES-3]

**AI Labeling**: This documentation is AI-generated with technical review and validation.

*Last Updated: June 7, 2025*

## Overview

This document provides comprehensive configuration options for the Anya Core system, covering Bitcoin protocols, Web5 integration, ML systems, and enterprise features.

## Configuration Files

### Core Configuration

The main configuration file is located at `config/core.toml`:

```toml
[system]
name = "anya-core"
version = "3.1.0"
environment = "production"

[bitcoin]
network = "mainnet"  # mainnet, testnet, regtest
rpc_url = "http://127.0.0.1:8332"
rpc_user = "bitcoin"
rpc_password = "${BITCOIN_RPC_PASSWORD}"

[lightning]
enabled = true
network = "bitcoin"
port = 9735
```

### Web5 Configuration

```toml
[web5]
enabled = true
did_method = "did:web"
dwn_endpoint = "https://dwn.your-domain.com"

[web5.did]
resolver_cache_ttl = 3600
verification_methods = ["Ed25519", "Secp256k1"]

[web5.dwn]
storage_type = "filesystem"
storage_path = "./data/dwn"
```

### ML Configuration

```toml
[ml]
enabled = true
model_path = "./models"
inference_timeout = 30

[ml.agents]
enabled = true
max_agents = 10
health_check_interval = 60

[ml.federated]
enabled = false
coordinator_url = "https://federated.your-domain.com"
```

## Environment Variables

### Required Variables

- `BITCOIN_RPC_PASSWORD`: Bitcoin Core RPC password
- `DATABASE_URL`: PostgreSQL connection string
- `RUST_LOG`: Logging level (debug, info, warn, error)

### Optional Variables

- `WEB5_PRIVATE_KEY`: Web5 DID private key
- `ML_MODEL_API_KEY`: External ML service API key
- `MONITORING_ENDPOINT`: Prometheus endpoint

## Network Configuration

### Bitcoin Network

```toml
[bitcoin.network]
# Mainnet configuration
mainnet_peers = [
  "seed.bitcoin.sipa.be:8333",
  "dnsseed.bluematt.me:8333"
]

# Testnet configuration
testnet_peers = [
  "testnet-seed.bitcoin.jonasschnelli.ch:18333"
]
```

### Lightning Network

```toml
[lightning.network]
alias = "anya-lightning-node"
color = "#3399ff"
fee_base = 1000
fee_rate = 1
```

## Security Configuration

### Cryptographic Settings

```toml
[security.crypto]
key_derivation = "pbkdf2"
iterations = 100000
salt_length = 32

[security.tls]
cert_path = "./certs/server.crt"
key_path = "./certs/server.key"
min_version = "1.3"
```

### Access Control

```toml
[security.auth]
method = "jwt"
secret = "${JWT_SECRET}"
expiration = 3600

[security.rbac]
enabled = true
admin_role = "admin"
user_role = "user"
```

## Performance Configuration

### Resource Limits

```toml
[performance]
max_connections = 1000
thread_pool_size = 8
memory_limit = "4GB"

[performance.cache]
enabled = true
size = "1GB"
ttl = 3600
```

### Database Configuration

```toml
[database]
max_connections = 20
connection_timeout = 30
idle_timeout = 600

[database.pool]
min_size = 5
max_size = 20
```

## Monitoring Configuration

### Metrics

```toml
[monitoring.metrics]
enabled = true
endpoint = "0.0.0.0:9090"
interval = 10

[monitoring.alerts]
enabled = true
webhook_url = "${ALERT_WEBHOOK_URL}"
```

### Logging

```toml
[logging]
level = "info"
format = "json"
output = "stdout"

[logging.file]
enabled = true
path = "./logs/anya-core.log"
max_size = "100MB"
max_files = 10
```

## Enterprise Configuration

### Multi-tenancy

```toml
[enterprise.tenancy]
enabled = true
default_tenant = "default"
isolation_level = "strict"

[enterprise.compliance]
gdpr_enabled = true
audit_logging = true
data_retention_days = 2555  # 7 years
```

### High Availability

```toml
[enterprise.ha]
enabled = true
cluster_size = 3
consensus = "raft"

[enterprise.backup]
enabled = true
interval = "1h"
retention = "30d"
```

## Development Configuration

### Development Mode

```toml
[development]
enabled = false
hot_reload = true
debug_mode = true

[development.testing]
mock_external_services = true
test_data_path = "./test-data"
```

## Configuration Management

### Dynamic Configuration

Some configuration options can be updated at runtime:

```rust
use anya_core::config::ConfigManager;

let config_manager = ConfigManager::new();
config_manager.update_setting("ml.agents.max_agents", "15").await?;
```

### Configuration Validation

The system validates configuration on startup:

```bash
anya-core --validate-config
```

## Best Practices

### Security

1. **Environment Variables**: Store sensitive values in environment variables
2. **File Permissions**: Restrict config file permissions to 600
3. **Regular Rotation**: Rotate API keys and passwords regularly

### Performance

1. **Resource Monitoring**: Monitor resource usage and adjust limits
2. **Cache Tuning**: Tune cache settings based on usage patterns
3. **Connection Pooling**: Optimize database connection pool settings

### Maintenance

1. **Backup Configuration**: Regularly backup configuration files
2. **Version Control**: Keep configuration in version control
3. **Documentation**: Document all custom configuration changes

## Configuration Schema

For detailed configuration schema validation, see:

- [Core Schema](./schema/core.json)
- [Bitcoin Schema](./schema/bitcoin.json)
- [Web5 Schema](./schema/web5.json)
- [ML Schema](./schema/ml.json)

## Migration

When upgrading between versions, use the configuration migration tool:

```bash
anya-core migrate-config --from-version 3.0.0 --to-version 3.1.0
```

## Troubleshooting

### Common Issues

1. **Invalid Configuration**: Check syntax and required fields
2. **Permission Errors**: Verify file permissions and ownership
3. **Network Issues**: Check firewall and network connectivity

### Diagnostic Commands

```bash
# Validate configuration
anya-core config validate

# Show effective configuration
anya-core config show

# Test connectivity
anya-core config test-connections
```

## Support

For configuration support:

- [Configuration Examples](./examples/)
- [Troubleshooting Guide](../README.md)
- [Community Support](https://github.com/anya-core/community)

*This documentation follows the [AI Labeling Standards](../standards/AI_LABELING.md) based on official Bitcoin Improvement Proposals (BIPs).*
