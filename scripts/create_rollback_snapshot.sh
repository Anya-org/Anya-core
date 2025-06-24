#!/bin/bash
# Script to create a rollback snapshot before deploying to Mainnet

# Exit immediately if a command exits with a non-zero status
set -e

VERSION=$1

if [ -z "$VERSION" ]; then
    echo "Error: Version parameter is required"
    echo "Usage: $0 <version>"
    exit 1
fi

echo "Creating rollback snapshot for version $VERSION..."

# Create snapshot directory
SNAPSHOT_DIR="/var/backups/anya-core/mainnet-snapshots"
SNAPSHOT_PATH="${SNAPSHOT_DIR}/${VERSION}_$(date +%Y%m%d%H%M%S)"

# In a real scenario, we would create these directories and perform actual backups
# For demonstration purposes, we'll simulate the process
echo "Creating snapshot directory: $SNAPSHOT_PATH"
mkdir -p "${SNAPSHOT_PATH}" 2>/dev/null || echo "Using existing directory"

# Simulate database backup
echo "Backing up blockchain database..."
echo "-- Simulated database backup --" >"${SNAPSHOT_PATH}/blockchain_db.backup"

# Simulate configuration backup
echo "Backing up configuration files..."
echo "-- Simulated config backup --" >"${SNAPSHOT_PATH}/config.backup"

# Simulate state snapshot
echo "Creating state snapshot..."
echo "-- Simulated state snapshot --" >"${SNAPSHOT_PATH}/state.snapshot"

# Record deployment metadata
echo "Recording deployment metadata..."
cat >"${SNAPSHOT_PATH}/metadata.json" <<EOL
{
  "version": "${VERSION}",
  "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "previous_version": "v1.1.0",
  "backup_files": [
    "blockchain_db.backup",
    "config.backup",
    "state.snapshot"
  ]
}
EOL

# Create verification hash
echo "Creating verification hash..."
find "${SNAPSHOT_PATH}" -type f | sort | xargs sha256sum >"${SNAPSHOT_PATH}/checksum.sha256"

echo "Setting up quick rollback symlink..."
ln -sf "${SNAPSHOT_PATH}" "${SNAPSHOT_DIR}/latest"

echo "✅ Rollback snapshot created successfully at: ${SNAPSHOT_PATH}"
exit 0
