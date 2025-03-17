# Anya Core Testnet Installer

## Overview

This branch (`installer-dev`) contains updates to the Anya Core installation and verification scripts to use Bitcoin testnet and a public RPC endpoint instead of setting up a local Bitcoin node. This approach offers several advantages:

1. **Faster setup**: No need to download and sync a full Bitcoin node
2. **Reduced resource requirements**: Lower disk space and CPU usage on the server
3. **Immediate development**: Start building on Bitcoin testnet without waiting for blockchain sync

## Key Changes

### 1. Server Setup Script (`scripts/server_setup.sh`)

- Removed local Bitcoin Core installation
- Added configuration for public Bitcoin testnet RPC endpoint (testnet.getblock.io)
- Added connection testing to verify Bitcoin RPC access
- Updated Web5 DWN configuration to use the public endpoint
- Optimized firewall rules for testnet-only operation
- Implemented proper directory structure for Hexagonal Architecture

### 2. Verification Script (`scripts/verify_installation.sh`)

- Updated to check for testnet configuration
- Added RPC connection testing to the public endpoint
- Added BIP 341/342 (Taproot) verification through the public RPC
- Added Web5 DWN configuration verification
- Added Hexagonal Architecture directory structure verification

### 3. Deployment Tools

- Created `deploy_to_server.ps1` for direct deployment to a remote server
- Created `push_to_server.ps1` for simply pushing files to a server
- Created `Final-Validation.ps1` to verify all scripts and configurations
- Created `Test-AnyaCore.ps1` to validate scripts before deployment

### 4. Documentation

- Added `DEPLOYMENT.md` with detailed deployment instructions
- Updated verification criteria to focus on testnet operation

## Configuration Details

### Bitcoin RPC Details

- **Endpoint**: testnet.getblock.io:3333
- **Username**: mainnet_8731
- **Password**: c8f7af8a-c33c-4f49-954c-a997a50b9a22
- **Network**: Bitcoin Testnet

### Service Structure

- **Web5 DWN**: Port 3000
- **Prometheus**: Port 9090

## Usage

### Local Testing

1. Run the validation script:
   ```powershell
   .\Final-Validation.ps1 -Verbose
   ```

2. Check script compatibility:
   ```powershell
   .\Test-AnyaCore.ps1
   ```

### Deployment

1. Deploy to a remote server:
   ```powershell
   .\deploy_to_server.ps1 -ServerHost "your-server-ip" -ServerUser "anya"
   ```

2. Or use GitHub Actions by configuring the required secrets and triggering the workflow.

## Next Steps

1. Test deployment on a fresh Ubuntu server
2. Implement additional monitoring for Bitcoin testnet
3. Add a fallback mechanism if the public RPC endpoint is unavailable
4. Expand test coverage for all Bitcoin-related operations 