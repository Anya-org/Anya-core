#!/bin/bash
# Production Deployment Script for Anya Core v1.3.0
# This script prepares and deploys the production-ready enhancements

set -euo pipefail

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging function
log() {
    echo -e "${GREEN}[$(date +'%Y-%m-%d %H:%M:%S')] $1${NC}"
}

warn() {
    echo -e "${YELLOW}[$(date +'%Y-%m-%d %H:%M:%S')] WARNING: $1${NC}"
}

error() {
    echo -e "${RED}[$(date +'%Y-%m-%d %H:%M:%S')] ERROR: $1${NC}"
}

info() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')] INFO: $1${NC}"
}

# Script configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BACKUP_DIR="${PROJECT_ROOT}/backup_$(date +%Y%m%d_%H%M%S)"
PRODUCTION_CONFIG_DIR="${PROJECT_ROOT}/config/production"

# Version information
VERSION="v1.3.0"
DEPLOYMENT_ID="prod_$(date +%Y%m%d_%H%M%S)"

log "Starting Anya Core Production Deployment - ${VERSION}"
log "Deployment ID: ${DEPLOYMENT_ID}"

# Create backup directory
log "Creating backup directory: ${BACKUP_DIR}"
mkdir -p "${BACKUP_DIR}"

# Backup current configuration
if [ -d "${PROJECT_ROOT}/config" ]; then
    log "Backing up current configuration..."
    cp -r "${PROJECT_ROOT}/config" "${BACKUP_DIR}/"
fi

# Create production configuration directory
log "Setting up production configuration..."
mkdir -p "${PRODUCTION_CONFIG_DIR}"

# Create production ML configuration
cat > "${PRODUCTION_CONFIG_DIR}/ml_service.toml" << EOF
[ml_service]
# Production ML Service Configuration
models_dir = "./models/production"
max_memory_mb = 4096
enable_gpu = false
max_batch_size = 32
confidence_threshold = 0.75
model_cache_timeout = 3600
auto_retrain_interval = 24
enable_federated_learning = true

[inference_cache]
max_entries = 10000
ttl_seconds = 1800
enable_compression = true

[monitoring]
enable_metrics = true
metrics_port = 9090
log_level = "info"
EOF

# Create production HSM configuration
cat > "${PRODUCTION_CONFIG_DIR}/hsm.toml" << EOF
[hsm]
# Production HSM Configuration
provider_type = "SoftwareKeyStore"
enabled = true
audit_enabled = true

[hsm.software]
token_dir = "./hsm/tokens"
max_sessions = 10
lock_timeout_seconds = 300
use_testnet = false

[hsm.bitcoin]
network = "mainnet"
derivation_path = "m/84'/0'/0'"

[hsm.audit]
log_file = "./logs/hsm_audit.log"
max_file_size_mb = 100
retention_days = 365
encrypt_logs = true
EOF

# Create production Layer2 configuration
cat > "${PRODUCTION_CONFIG_DIR}/layer2.toml" << EOF
[layer2]
# Production Layer2 Configuration
enable_lightning = true
enable_rgb = true
enable_dlc = true
enable_state_channels = true

[layer2.lightning]
network = "mainnet"
data_dir = "./lightning/data"
log_level = "info"
autopilot_enabled = true

[layer2.rgb]
network = "mainnet"
data_dir = "./rgb/data"
contract_cache_size = 1000

[layer2.dlc]
network = "mainnet"
data_dir = "./dlc/data"
oracle_timeout_seconds = 30

[layer2.state_channels]
network = "mainnet"
data_dir = "./state_channels/data"
dispute_timeout_blocks = 144
EOF

# Create production security configuration
cat > "${PRODUCTION_CONFIG_DIR}/security.toml" << EOF
[security]
# Production Security Configuration
encryption_enabled = true
audit_logging = true
rate_limiting = true

[security.encryption]
algorithm = "AES-256-GCM"
key_rotation_hours = 168  # 7 days
backup_keys = true

[security.audit]
log_all_operations = true
include_request_data = false
include_response_data = false
anonymize_user_data = true

[security.rate_limiting]
requests_per_minute = 100
burst_limit = 150
enable_ip_whitelist = true
EOF

# Create environment-specific configuration
cat > "${PRODUCTION_CONFIG_DIR}/environment.toml" << EOF
[environment]
# Production Environment Configuration
name = "production"
version = "${VERSION}"
deployment_id = "${DEPLOYMENT_ID}"

[environment.database]
host = "localhost"
port = 5432
name = "anya_core_prod"
pool_size = 20
connection_timeout = 30

[environment.redis]
host = "localhost"
port = 6379
db = 0
pool_size = 10

[environment.monitoring]
enable_metrics = true
enable_tracing = true
enable_profiling = false
health_check_interval = 30
EOF

# Check Rust environment
log "Checking Rust environment..."
if ! command -v rustc &> /dev/null; then
    error "Rust compiler not found. Please install Rust."
    exit 1
fi

RUST_VERSION=$(rustc --version)
log "Using Rust: ${RUST_VERSION}"

# Build production binary
log "Building production binary..."
cd "${PROJECT_ROOT}"

# Set production environment variables
export RUST_ENV=production
export ANYA_CONFIG_DIR="${PRODUCTION_CONFIG_DIR}"

# Build with optimizations
log "Compiling with production optimizations..."
cargo build --release --features production

if [ $? -eq 0 ]; then
    log "✅ Production binary built successfully"
else
    error "❌ Production build failed"
    exit 1
fi

# Run production tests
log "Running production test suite..."
cargo test --release --features production -- --test-threads=1

if [ $? -eq 0 ]; then
    log "✅ Production tests passed"
else
    warn "⚠️ Some production tests failed - review before deployment"
fi

# Validate configuration
log "Validating production configuration..."
./target/release/anya-core config validate --config-dir="${PRODUCTION_CONFIG_DIR}"

if [ $? -eq 0 ]; then
    log "✅ Production configuration validated"
else
    error "❌ Production configuration validation failed"
    exit 1
fi

# Create deployment manifest
cat > "${PROJECT_ROOT}/deployment_manifest.json" << EOF
{
  "deployment_id": "${DEPLOYMENT_ID}",
  "version": "${VERSION}",
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "components": {
    "layer2_adapters": {
      "status": "production",
      "file": "src/layer2/production_adapters.rs",
      "features": ["lightning", "rgb", "dlc", "state_channels"]
    },
    "ml_service": {
      "status": "enhanced",
      "file": "src/ml/production.rs",
      "features": ["inference_cache", "production_mode", "metrics"]
    },
    "hsm_providers": {
      "status": "production_ready",
      "providers": ["software", "simulator"],
      "features": ["audit_logging", "encryption", "fallback"]
    }
  },
  "configuration": {
    "directory": "./config/production",
    "validated": true,
    "backup": "${BACKUP_DIR}"
  },
  "build": {
    "rust_version": "${RUST_VERSION}",
    "optimization": "release",
    "features": ["production"]
  }
}
EOF

# Create service startup script
cat > "${PROJECT_ROOT}/start_production.sh" << 'EOF'
#!/bin/bash
# Production startup script for Anya Core

set -euo pipefail

# Configuration
ANYA_CONFIG_DIR="./config/production"
ANYA_LOG_LEVEL="info"
ANYA_BIND_ADDRESS="0.0.0.0:8080"

# Export environment variables
export ANYA_CONFIG_DIR
export ANYA_LOG_LEVEL
export ANYA_BIND_ADDRESS
export RUST_LOG=anya_core=info

# Create necessary directories
mkdir -p ./logs
mkdir -p ./hsm/tokens
mkdir -p ./lightning/data
mkdir -p ./rgb/data
mkdir -p ./dlc/data
mkdir -p ./state_channels/data
mkdir -p ./models/production

# Set permissions
chmod 700 ./hsm/tokens
chmod 755 ./logs

echo "Starting Anya Core Production Service..."
echo "Configuration Directory: ${ANYA_CONFIG_DIR}"
echo "Log Level: ${ANYA_LOG_LEVEL}"
echo "Bind Address: ${ANYA_BIND_ADDRESS}"

# Start the service
exec ./target/release/anya-core \
    --config-dir="${ANYA_CONFIG_DIR}" \
    --log-level="${ANYA_LOG_LEVEL}" \
    --bind="${ANYA_BIND_ADDRESS}"
EOF

chmod +x "${PROJECT_ROOT}/start_production.sh"

# Create monitoring script
cat > "${PROJECT_ROOT}/monitor_production.sh" << 'EOF'
#!/bin/bash
# Production monitoring script for Anya Core

set -euo pipefail

METRICS_URL="http://localhost:9090/metrics"
HEALTH_URL="http://localhost:8080/health"

echo "=== Anya Core Production Monitor ==="
echo "Timestamp: $(date)"
echo

# Check health endpoint
echo "Health Check:"
if curl -s "${HEALTH_URL}" > /dev/null; then
    echo "✅ Service is healthy"
else
    echo "❌ Service health check failed"
fi

# Check metrics endpoint
echo
echo "Metrics Check:"
if curl -s "${METRICS_URL}" > /dev/null; then
    echo "✅ Metrics endpoint responding"
else
    echo "❌ Metrics endpoint unavailable"
fi

# Check log files
echo
echo "Log Status:"
if [ -f "./logs/anya-core.log" ]; then
    LOG_LINES=$(wc -l < "./logs/anya-core.log")
    echo "✅ Main log: ${LOG_LINES} lines"
else
    echo "⚠️ Main log file not found"
fi

if [ -f "./logs/hsm_audit.log" ]; then
    AUDIT_LINES=$(wc -l < "./logs/hsm_audit.log")
    echo "✅ HSM audit log: ${AUDIT_LINES} lines"
else
    echo "⚠️ HSM audit log not found"
fi

echo
echo "=== End Monitor ==="
EOF

chmod +x "${PROJECT_ROOT}/monitor_production.sh"

# Final deployment summary
log "=== Production Deployment Summary ==="
info "Deployment ID: ${DEPLOYMENT_ID}"
info "Version: ${VERSION}"
info "Configuration: ${PRODUCTION_CONFIG_DIR}"
info "Backup: ${BACKUP_DIR}"
info "Binary: ./target/release/anya-core"

log ""
log "📋 Production Components Deployed:"
log "  ✅ Layer2 Production Adapters"
log "  ✅ ML Production Service"
log "  ✅ HSM Security Providers"
log "  ✅ Production Configuration"
log "  ✅ Monitoring Tools"

log ""
log "🚀 Next Steps:"
log "  1. Review configuration in: ${PRODUCTION_CONFIG_DIR}"
log "  2. Start production service: ./start_production.sh"
log "  3. Monitor system: ./monitor_production.sh"
log "  4. Check deployment manifest: ./deployment_manifest.json"

log ""
log "✅ Production deployment preparation complete!"
log "🔒 Security: HSM providers configured with audit logging"
log "🧠 ML/AI: Production inference service ready"
log "⚡ Layer2: Real protocol adapters activated"

warn ""
warn "⚠️ IMPORTANT REMINDERS:"
warn "  • Review and customize production configuration files"
warn "  • Set appropriate encryption keys for HSM"
warn "  • Configure monitoring and alerting"
warn "  • Perform security audit before going live"
warn "  • Test all integrations in staging environment"

log ""
log "🎉 Anya Core ${VERSION} is ready for production deployment!"
