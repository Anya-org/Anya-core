# Anya-Core Configuration Guide

## Bitcoin RPC Configuration

Anya-Core uses public RPC endpoints by default:

- Mainnet: `https://bitcoin-rpc.publicnode.com`
- Testnet: `https://bitcoin-testnet-rpc.publicnode.com`

These endpoints are provided by [PublicNode.com](https://bitcoin-rpc.publicnode.com) which offers free, privacy-focused RPC services.

### Using Custom RPC Endpoints

If you prefer to use your own Bitcoin node or a different RPC provider, you can configure this in two ways:

1. **During installation**:

   ```bash
   anya-installer --rpc-endpoint https://your-custom-endpoint.com
   ```

2. **In configuration file** (`config/anya.conf`):

   ```toml
   [network]
   bitcoin_custom_rpc_url = "https://your-custom-endpoint.com"
   ```

When `bitcoin_custom_rpc_url` is set, it takes precedence over the default endpoints.

### Switching Networks

To switch between mainnet and testnet:

1. **During installation**:

   ```bash
   anya-installer --network mainnet
   # or
   anya-installer --network testnet
   ```

2. **In configuration file** (`config/anya.conf`):

   ```toml
   [network]
   network_type = "mainnet"  # or "testnet"
   ```

The system will automatically use the appropriate default RPC endpoint based on the selected network, unless overridden by `bitcoin_custom_rpc_url`.
