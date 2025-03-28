#!/bin/bash
# Layer 4 Architecture Setup for Anya Core
# [AIR-3][AIS-3][BPC-3][RES-3]
#
# This script converts an existing Anya Core installation to a proper
# Layer 4 Bitcoin protocol architecture with clear separation between layers.

set -e

# Terminal colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script variables
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
CONFIG_DIR="$PROJECT_ROOT/config"
INSTALL_PATH=${1:-"/opt/anya-core"}

# Function to print status messages
log() {
    local level=$1
    shift
    local message="$@"
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    
    case $level in
        "info")
            echo -e "${BLUE}[INFO]${NC} $message"
            ;;
        "success")
            echo -e "${GREEN}[SUCCESS]${NC} $message"
            ;;
        "warning")
            echo -e "${YELLOW}[WARNING]${NC} $message"
            ;;
        "error")
            echo -e "${RED}[ERROR]${NC} $message"
            ;;
        *)
            echo -e "$message"
            ;;
    esac
}

# Function to create Layer 4 directory structure
create_layer_structure() {
    log "info" "Creating Layer 4 architecture directory structure..."
    
    # Layer 1: Bitcoin Core
    mkdir -p "$INSTALL_PATH/layer1/bitcoin"
    
    # Layer 2: Lightning Network
    mkdir -p "$INSTALL_PATH/layer2/lightning/lnd"
    mkdir -p "$INSTALL_PATH/layer2/lightning/c-lightning"
    
    # Layer 3: Protocol Adapters
    mkdir -p "$INSTALL_PATH/layer3/rgb"
    mkdir -p "$INSTALL_PATH/layer3/dlc"
    mkdir -p "$INSTALL_PATH/layer3/taproot-assets"
    
    # Layer 4: Application Services
    mkdir -p "$INSTALL_PATH/layer4/dao"
    mkdir -p "$INSTALL_PATH/layer4/web5"
    mkdir -p "$INSTALL_PATH/layer4/ml"
    mkdir -p "$INSTALL_PATH/layer4/api"
    
    # Shared data and config
    mkdir -p "$INSTALL_PATH/data"
    mkdir -p "$INSTALL_PATH/config"
    mkdir -p "$INSTALL_PATH/logs"
    
    log "success" "Layer 4 directory structure created"
}

# Function to create Layer 1 configuration
create_layer1_config() {
    log "info" "Setting up Layer 1 (Bitcoin Core) configuration..."
    
    # Bitcoin config
    cat > "$INSTALL_PATH/layer1/bitcoin/bitcoin.conf" << EOF
# Bitcoin Layer 1 configuration for Anya Core
# Network: testnet
# Layer: 1 (Base Bitcoin Protocol)

# Network
testnet=1
server=1
listen=1

# RPC
rpcallowip=127.0.0.1
rpcport=18332
rpcuser=anyabitcoin
rpcpassword=anyapassword
rest=1

# ZMQ
zmqpubrawblock=tcp://127.0.0.1:28332
zmqpubrawtx=tcp://127.0.0.1:28333
zmqpubhashblock=tcp://127.0.0.1:28334

# Performance
dbcache=450
maxorphantx=10
maxmempool=50
maxconnections=40
maxuploadtarget=1000

# Layer 1 features
txindex=1
blockfilterindex=1
EOF

    # Create symbolic link for backward compatibility
    ln -sf "$INSTALL_PATH/layer1/bitcoin/bitcoin.conf" "$INSTALL_PATH/config/bitcoin.conf"
    
    log "success" "Layer 1 configuration created"
}

# Function to create Layer 2 configuration
create_layer2_config() {
    log "info" "Setting up Layer 2 (Lightning Network) configuration..."
    
    # LND config
    cat > "$INSTALL_PATH/layer2/lightning/lnd/lnd.conf" << EOF
# LND configuration for Anya Core
# Layer: 2 (Lightning Network)
[Application Options]
debuglevel=info
maxpendingchannels=10
listen=0.0.0.0:9735
rpclisten=0.0.0.0:10009
restlisten=0.0.0.0:8080

[Bitcoin]
bitcoin.active=true
bitcoin.testnet=true
bitcoin.node=bitcoind

[Bitcoind]
bitcoind.rpcuser=anyabitcoin
bitcoind.rpcpass=anyapassword
bitcoind.rpchost=127.0.0.1:18332
bitcoind.zmqpubrawblock=tcp://127.0.0.1:28332
bitcoind.zmqpubrawtx=tcp://127.0.0.1:28333

[protocol]
protocol.wumbo-channels=true
protocol.option-scid-alias=true
protocol.zero-conf=true
EOF

    # C-Lightning config
    cat > "$INSTALL_PATH/layer2/lightning/c-lightning/config" << EOF
# C-Lightning configuration for Anya Core
# Layer: 2 (Lightning Network)
network=testnet
alias=anyacore-node
rgb=008000
bitcoin-rpcuser=anyabitcoin
bitcoin-rpcpassword=anyapassword
bitcoin-rpcport=18332
announce-addr=auto
log-level=info
log-file=lightning-debug.log
large-channels
EOF

    # Create symbolic links for backward compatibility
    ln -sf "$INSTALL_PATH/layer2/lightning/lnd/lnd.conf" "$INSTALL_PATH/config/lnd.conf"
    
    log "success" "Layer 2 configuration created"
}

# Function to create Layer 3 configuration
create_layer3_config() {
    log "info" "Setting up Layer 3 (Protocol Adapters) configuration..."
    
    # RGB config
    cat > "$INSTALL_PATH/layer3/rgb/rgb.yaml" << EOF
# RGB Layer 3 Protocol configuration
network:
  chain: bitcoin
  network: testnet
storage:
  contract_dir: "./data/contracts"
  data_dir: "./data/rgb"
rpc:
  bind: 127.0.0.1:7000
secrecy:
  encryption: true
  blinding: true
EOF

    # DLC config
    cat > "$INSTALL_PATH/layer3/dlc/dlc.yaml" << EOF
# DLC Layer 3 Protocol configuration
oracle:
  endpoint: "https://oracle.suredbits.com"
  public_key: ""
contract:
  storage_path: "./data/dlc"
  backup_path: "./data/dlc/backup"
network:
  p2p_bind: 127.0.0.1:9735
execution:
  auto_execute: true
  confirmation_target: 6
EOF

    # Taproot Assets config
    cat > "$INSTALL_PATH/layer3/taproot-assets/taproot-assets.yaml" << EOF
# Taproot Assets Layer 3 Protocol configuration
network:
  chain: bitcoin
  network: testnet
storage:
  db_path: "./data/taproot-assets/tap.db"
  assets_dir: "./data/taproot-assets/assets"
rpc:
  bind: 127.0.0.1:10029
  tls_cert_path: "./data/taproot-assets/tls.cert"
  macaroon_path: "./data/taproot-assets/admin.macaroon"
bitcoin:
  node_host: "127.0.0.1:18332"
  rpc_user: "anyabitcoin"
  rpc_password: "anyapassword"
EOF

    # Create symbolic links for backward compatibility
    ln -sf "$INSTALL_PATH/layer3/rgb/rgb.yaml" "$INSTALL_PATH/config/rgb.yaml"
    ln -sf "$INSTALL_PATH/layer3/dlc/dlc.yaml" "$INSTALL_PATH/config/dlc.yaml"
    ln -sf "$INSTALL_PATH/layer3/taproot-assets/taproot-assets.yaml" "$INSTALL_PATH/config/taproot-assets.yaml"
    
    log "success" "Layer 3 configuration created"
}

# Function to create Layer 4 configuration
create_layer4_config() {
    log "info" "Setting up Layer 4 (Application Services) configuration..."
    
    # Main Anya Core Layer 4 configuration
    cat > "$INSTALL_PATH/config/anya.conf" << EOF
# Anya Core Layer 4 Bitcoin Network Configuration
# Version: 2.5.0
# Layer Architecture: Layer 4 Bitcoin Protocol Application

[network]
network_type = "testnet"

# Layer 1: Bitcoin Core
[network.layer1]
enabled = true
implementation = "bitcoin-core"
mode = "full"
bitcoin_mainnet_rpc_url = "https://bitcoin-rpc.publicnode.com"
bitcoin_testnet_rpc_url = "https://bitcoin-testnet-rpc.publicnode.com"
bitcoin_custom_rpc_url = ""
connection_timeout_sec = 30

# Layer 2: Lightning Network
[network.layer2]
enabled = true
implementation = "lnd"
channel_backup_path = "data/lightning/channel.backup"
max_channel_size_btc = 0.5
min_channel_size_btc = 0.001

# Layer 3: Protocol Adapters
[network.layer3]
enabled = true
rgb_enabled = true
dlc_enabled = true
taproot_assets = true
tap_protocol_version = "0.2.0"

# Layer 4: Application Services
[service.layer4]
enabled = true
dao_enabled = true
web5_enabled = true
ml_enabled = false
api_enabled = true

[wallet]
enable_taproot = true
bip370_support = true
coin_selection_strategy = "efficient"

[dao]
quadratic_voting = true
dao_level = "DAO4"
proposal_threshold = 100
voting_period_days = 7
execution_delay_hours = 24

[web5]
did_method = "ion"
dwn_endpoint = "http://localhost:3000"
storage_location = "data/web5"

[system_awareness]
mempool_alert_threshold_kb = 100
fee_spike_threshold = 200.0
attack_threshold = 60.0

[performance]
cache_size_mb = 20
batch_size = 100
use_prepared_statements = true
EOF

    # DAO configuration
    cat > "$INSTALL_PATH/layer4/dao/dao-config.json" << EOF
{
  "dao": {
    "name": "Anya DAO",
    "description": "Decentralized governance for Anya Core",
    "proposal_threshold": 100,
    "voting_periods": {
      "standard": 7,
      "emergency": 1,
      "protocol_change": 14
    },
    "execution_delay": 24,
    "governance_token": {
      "name": "Anya Governance Token",
      "symbol": "AGT",
      "total_supply": 21000000000,
      "initial_distribution": {
        "treasury": 35,
        "liquidity": 25,
        "team": 20,
        "community": 15,
        "strategic": 5
      }
    }
  }
}
EOF

    # Web5 configuration
    cat > "$INSTALL_PATH/layer4/web5/web5-config.json" << EOF
{
  "web5": {
    "did": {
      "method": "ion",
      "resolution_endpoint": "https://beta.discover.did.microsoft.com/1.0/identifiers"
    },
    "dwn": {
      "endpoint": "http://localhost:3000",
      "storage_location": "data/web5/storage",
      "message_limit": 1000,
      "protocols": [
        "https://anya.ai/protocol"
      ]
    },
    "protocols": {
      "anya": {
        "types": {
          "proposal": {
            "schema": "https://anya.ai/schemas/proposal",
            "dataFormats": ["application/json"]
          },
          "vote": {
            "schema": "https://anya.ai/schemas/vote",
            "dataFormats": ["application/json"]
          }
        }
      }
    }
  }
}
EOF

    # API Gateway configuration
    cat > "$INSTALL_PATH/layer4/api/api-config.json" << EOF
{
  "api": {
    "bind": "0.0.0.0:3000",
    "cors": {
      "allowed_origins": ["*"],
      "allowed_methods": ["GET", "POST", "PUT", "DELETE"],
      "allowed_headers": ["Content-Type", "Authorization"],
      "max_age": 86400
    },
    "auth": {
      "jwt_secret": "CHANGE_THIS_TO_A_SECURE_SECRET",
      "token_expiry": 86400
    },
    "rate_limit": {
      "requests_per_minute": 60,
      "burst": 10
    },
    "endpoints": {
      "layer1": true,
      "layer2": true,
      "layer3": true,
      "layer4": true
    }
  }
}
EOF

    # Create symbolic links for backward compatibility
    ln -sf "$INSTALL_PATH/layer4/dao/dao-config.json" "$INSTALL_PATH/config/dao-config.json" 
    ln -sf "$INSTALL_PATH/layer4/web5/web5-config.json" "$INSTALL_PATH/config/web5-config.json"
    ln -sf "$INSTALL_PATH/layer4/api/api-config.json" "$INSTALL_PATH/config/api-config.json"
    
    log "success" "Layer 4 configuration created"
}

# Function to create systemd service file
create_systemd_service() {
    log "info" "Creating Layer 4 systemd service file..."
    
    cat > "$INSTALL_PATH/anya-core.service" << EOF
[Unit]
Description=Anya Core Layer 4 Bitcoin Network Service
After=network.target bitcoind.service lnd.service
Wants=bitcoind.service lnd.service

[Service]
Type=simple
User=$(whoami)
WorkingDirectory=$INSTALL_PATH
ExecStart=$INSTALL_PATH/bin/anya-core
Restart=on-failure
RestartSec=10
TimeoutStartSec=120

# Layer 4 service environment
Environment=ANYA_LAYER4_SERVICE=1
Environment=ANYA_LOG_LEVEL=info

[Install]
WantedBy=multi-user.target
EOF

    log "info" "To install the service, run: sudo cp $INSTALL_PATH/anya-core.service /etc/systemd/system/ && sudo systemctl daemon-reload"
    log "success" "Layer 4 service file created successfully"
}

# Function to create layer documentation
create_layer_docs() {
    log "info" "Creating Layer 4 architecture documentation..."
    
    mkdir -p "$INSTALL_PATH/docs"
    
    cat > "$INSTALL_PATH/docs/LAYER_ARCHITECTURE.md" << 'EOF'
# Anya Core Layer 4 Architecture

Anya Core implements a comprehensive Bitcoin network stack with clear separation between protocol layers:

## Layer 1: Bitcoin Core (Base Protocol)

The foundation layer provides access to the Bitcoin blockchain:

- Bitcoin Core implementation
- Full node with transaction indexing
- Blockchain state management
- UTXO set maintenance
- P2P network communication

**Integration Points:**
- RPC API
- ZMQ notifications
- REST API

## Layer 2: Lightning Network

The payment channel network layer enables fast, low-fee transactions:

- Lightning Network implementation (LND/c-lightning)
- Channel management
- Payment routing
- Liquidity management
- Backup and recovery

**Integration Points:**
- gRPC API
- REST API
- Macaroon-based authentication

## Layer 3: Protocol Adapters

The protocol extension layer enables advanced Bitcoin functionality:

- RGB protocol for asset issuance
- DLC (Discreet Log Contracts) for smart contracts
- Taproot Assets for token issuance and management

**Integration Points:**
- Protocol-specific APIs
- Transaction construction
- Asset lifecycle management

## Layer 4: Application Services

The application layer provides user-facing functionality:

- DAO governance system
- Web5 decentralized identity and data
- Machine learning components
- Unified API gateway

**Integration Points:**
- RESTful API
- WebSocket for real-time updates
- Web interface
- Mobile SDK

## Layer Separation Benefits

This clear separation of concerns provides several advantages:

1. **Modularity**: Each layer can be updated independently
2. **Resilience**: Failures in higher layers don't affect lower layers
3. **Security**: Clear boundaries minimize attack surfaces
4. **Flexibility**: Components within each layer can be swapped
5. **Scalability**: Resources can be allocated by layer based on needs

## Layer Communication

Inter-layer communication follows strict protocols:

- Layer N can only call Layer N-1 through defined interfaces
- Layer N should handle failures in Layer N-1 gracefully
- Each layer provides observability (metrics, logs) for the layers above
- Configuration flows from top to bottom

## Deployment Considerations

For production environments:
- Layer 1 and 2 may be deployed on dedicated hardware
- Layer 3 protocols can be enabled selectively based on needs
- Layer 4 services can scale horizontally
EOF

    log "success" "Layer 4 architecture documentation created"
}

# Main function
main() {
    log "info" "Starting Anya Core Layer 4 architecture setup..."
    
    # Create directory structure
    create_layer_structure
    
    # Set up layer-specific configurations
    create_layer1_config
    create_layer2_config
    create_layer3_config
    create_layer4_config
    
    # Create systemd service
    create_systemd_service
    
    # Create documentation
    create_layer_docs
    
    log "success" "Anya Core is now configured as a Layer 4 Bitcoin network application"
    log "info" "Layer 4 installation path: $INSTALL_PATH"
    log "info" "To install and start the service, run:"
    log "info" "  sudo cp $INSTALL_PATH/anya-core.service /etc/systemd/system/"
    log "info" "  sudo systemctl daemon-reload"
    log "info" "  sudo systemctl enable anya-core.service"
    log "info" "  sudo systemctl start anya-core.service"
}

# Run main function
main "$@" 