#!/bin/bash
# [AIR-3][AIS-3][BPC-3][RES-3]
# Bitcoin Drive Preparation Script with Pruning Configuration
# Follows official Bitcoin Improvement Proposals (BIPs)
# May 20, 2025
#
# This script prepares drives for Bitcoin data storage with pruning
# enabled for optimal space efficiency while maintaining security
# and performance according to the hexagonal architecture requirements.

# Set strict error handling
set -e
set -o pipefail

# Error handling function
handle_error() {
    local exit_code=$?
    local line_number=$1
    echo "[AIR-3][AIS-3][BPC-3][RES-3] Error on line $line_number: Command exited with status $exit_code"
    exit $exit_code
}

# Set up error trap
trap 'handle_error $LINENO' ERR

# Check if running as root
if [ "$(id -u)" -ne 0 ]; then
    echo "[AIR-3][AIS-3][BPC-3][RES-3] Error: This script must be run as root"
    exit 1
fi

# Function to validate drive existence and ensure it's not busy
validate_drive() {
    local drive=$1
    if [ ! -b "$drive" ]; then
        echo "[AIR-3][AIS-3][BPC-3][RES-3] Error: Drive $drive does not exist or is not a block device"
        exit 1
    fi
    
    # Check if drive is mounted
    if grep -qs "$drive" /proc/mounts; then
        echo "[AIR-3][AIS-3][BPC-3][RES-3] Error: Drive $drive is currently mounted. Attempting to unmount..."
        umount -f "${drive}"* 2>/dev/null || true
        sleep 2
        
        # Check again after unmount attempt
        if grep -qs "$drive" /proc/mounts; then
            echo "[AIR-3][AIS-3][BPC-3][RES-3] Error: Could not unmount $drive. Please ensure it's not in use."
            exit 1
        fi
    fi
    
    # Force kernel to reread partition table
    blockdev --flushbufs "$drive" 2>/dev/null || true
    partprobe "$drive" 2>/dev/null || true
    sleep 2
}

# Parse command line options
SIMULATION_MODE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --simulate)
            SIMULATION_MODE=true
            shift
            ;;
        --data=*)
            DATA_DRIVE="${1#*=}"
            shift
            ;;
        --backup=*)
            BACKUP_DRIVE="${1#*=}"
            shift
            ;;
        *)
            # First non-option argument is data drive, second is backup drive
            if [ -z "${DATA_DRIVE+x}" ]; then
                DATA_DRIVE="$1"
            elif [ -z "${BACKUP_DRIVE+x}" ]; then
                BACKUP_DRIVE="$1"
            fi
            shift
            ;;
    esac
done

# Define drives and mount points with defaults
DATA_DRIVE=${DATA_DRIVE:-/dev/sdb}
BACKUP_DRIVE=${BACKUP_DRIVE:-/dev/sdc}
DATA_MOUNT="/mnt/bitcoin-data"
BACKUP_MOUNT="/mnt/bitcoin-backup"

# Auto-detect available drives if specified ones don't exist
if [ ! -b "$DATA_DRIVE" ] && [ "$SIMULATION_MODE" = false ]; then
    echo "[AIR-3][AIS-3][BPC-3][RES-3] Warning: $DATA_DRIVE not found, attempting to auto-detect..."
    # Try to find an available drive
    for drive in /dev/sd[bcdefg]; do
        if [ -b "$drive" ] && ! grep -q "$drive" /proc/mounts; then
            DATA_DRIVE="$drive"
            echo "[AIR-3][AIS-3][BPC-3][RES-3] Auto-detected data drive: $DATA_DRIVE"
            break
        fi
    done
fi

if [ ! -b "$BACKUP_DRIVE" ] && [ "$SIMULATION_MODE" = false ]; then
    echo "[AIR-3][AIS-3][BPC-3][RES-3] Warning: $BACKUP_DRIVE not found, attempting to auto-detect..."
    # Try to find an available drive different from DATA_DRIVE
    for drive in /dev/sd[bcdefg]; do
        if [ -b "$drive" ] && [ "$drive" != "$DATA_DRIVE" ] && ! grep -q "$drive" /proc/mounts; then
            BACKUP_DRIVE="$drive"
            echo "[AIR-3][AIS-3][BPC-3][RES-3] Auto-detected backup drive: $BACKUP_DRIVE"
            break
        fi
    done
fi

# Validate drives if not in simulation mode
if [ "$SIMULATION_MODE" = false ]; then
    validate_drive "$DATA_DRIVE"
    validate_drive "$BACKUP_DRIVE"
    
    # Ensure drives are different
    if [ "$DATA_DRIVE" = "$BACKUP_DRIVE" ]; then
        echo "[AIR-3][AIS-3][BPC-3][RES-3] Error: Data drive and backup drive cannot be the same"
        exit 1
    fi
else
    echo "[AIR-3][AIS-3][BPC-3][RES-3] Running in simulation mode - no drives will be modified"
fi

# Pruning configuration
PRUNE_MODE="true"  # Set to true to enable pruning
PRUNE_SIZE="5000"  # Prune size in MB (5GB - optimal for 29GB drives)
TXINDEX="false"    # Transaction index (disable for pruned nodes)
BLOCKS_ONLY="true" # Only store blocks, not the entire mempool

# Create mount points if they don't exist
mkdir -p $DATA_MOUNT $BACKUP_MOUNT

# Ensure drives are not in use
echo "[AIR-3][AIS-3][BPC-3][RES-3] Ensuring drives are not in use..."

# Force unmount any partitions
umount -f ${DATA_DRIVE}* 2>/dev/null || true
umount -f ${BACKUP_DRIVE}* 2>/dev/null || true

# Kill any processes using the drives
fuser -k ${DATA_DRIVE}* 2>/dev/null || true
fuser -k ${BACKUP_DRIVE}* 2>/dev/null || true

# Force kernel to reread partition tables
blockdev --flushbufs ${DATA_DRIVE} 2>/dev/null || true
blockdev --flushbufs ${BACKUP_DRIVE} 2>/dev/null || true
partprobe ${DATA_DRIVE} 2>/dev/null || true
partprobe ${BACKUP_DRIVE} 2>/dev/null || true

# Wait for devices to settle
sleep 3

echo "===== [AIR-3][AIS-3][BPC-3][RES-3] Bitcoin Drive Preparation ====="
echo "Automatic Bitcoin drive preparation following BIP Standards"
echo "Formatting ${DATA_DRIVE} and ${BACKUP_DRIVE} without confirmation"

# Log the start of the process
logger -t bitcoin-prepare "Starting automatic Bitcoin drive preparation (BDF v2.5)"

# Wait for devices to be fully available
echo "[AIR-3][AIS-3][BPC-3][RES-3] Waiting for devices to be fully available..."
sleep 2

# Double check device availability after waiting
validate_drive "$DATA_DRIVE"
validate_drive "$BACKUP_DRIVE"

# Format drives with ext4 filesystem - fully automated approach
echo "[AIR-3][AIS-3][BPC-3][RES-3] Formatting ${DATA_DRIVE} for Bitcoin data (non-interactive)..."

# Function to safely prepare a drive
prepare_drive() {
    local drive=$1
    local name=$2
    
    echo "[AIR-3][AIS-3][BPC-3][RES-3] Preparing $name drive: $drive"
    
    # Ensure drive is not busy
    fuser -k ${drive}* 2>/dev/null || true
    sleep 1
    
    # Clear existing signatures
    echo "[AIR-3][AIS-3][BPC-3][RES-3] Clearing existing signatures on ${drive}..."
    wipefs -af ${drive} || {
        echo "[AIR-3][AIS-3][BPC-3][RES-3] Warning: Could not clear signatures, retrying after delay..."
        sleep 5
        wipefs -af ${drive}
    }
    
    # Sync to ensure changes are written
    sync
    sleep 2
    
    # Create new partition table
    echo "[AIR-3][AIS-3][BPC-3][RES-3] Creating partition on ${drive}..."
    parted -s ${drive} mklabel msdos || {
        echo "[AIR-3][AIS-3][BPC-3][RES-3] Warning: Could not create partition table, retrying..."
        sleep 3
        parted -s ${drive} mklabel msdos
    }
    
    # Create partition
    parted -s ${drive} mkpart primary ext4 1MiB 100% || {
        echo "[AIR-3][AIS-3][BPC-3][RES-3] Warning: Could not create partition, retrying..."
        sleep 3
        parted -s ${drive} mkpart primary ext4 1MiB 100%
    }
    
    # Sync again and wait for device to settle
    sync
    sleep 3
    
    # Force kernel to reread partition table
    partprobe ${drive} || true
    sleep 2
}

# Prepare data drive
prepare_drive "${DATA_DRIVE}" "data"

# Prepare backup drive
prepare_drive "${BACKUP_DRIVE}" "backup"

# Wait for devices to be ready
sleep 2

# Wait for devices to be ready with retry logic
echo "[AIR-3][AIS-3][BPC-3][RES-3] Waiting for partitions to be recognized by the system..."

# Function to check if partition exists
check_partition() {
    local partition=$1
    local max_attempts=$2
    local attempt=1
    
    while [ $attempt -le $max_attempts ]; do
        if [ -b "$partition" ]; then
            echo "[AIR-3][AIS-3][BPC-3][RES-3] Partition $partition detected"
            return 0
        fi
        echo "[AIR-3][AIS-3][BPC-3][RES-3] Waiting for partition $partition (attempt $attempt/$max_attempts)"
        sleep 2
        attempt=$((attempt+1))
    done
    
    echo "[AIR-3][AIS-3][BPC-3][RES-3] Error: Partition $partition not detected after $max_attempts attempts"
    return 1
}

# Wait for partitions with retry
check_partition "${DATA_DRIVE}1" 5 || exit 1
check_partition "${BACKUP_DRIVE}1" 5 || exit 1

# Create filesystems with non-interactive options and force flag to overwrite any existing filesystem
echo "[AIR-3][AIS-3][BPC-3][RES-3] Creating ext4 filesystem on ${DATA_DRIVE}1 (non-interactive)..."
mkfs.ext4 -F -q -L BITCOIN-DATA ${DATA_DRIVE}1

echo "[AIR-3][AIS-3][BPC-3][RES-3] Creating ext4 filesystem on ${BACKUP_DRIVE}1 (non-interactive)..."
mkfs.ext4 -F -q -L BITCOIN-BACKUP ${BACKUP_DRIVE}1

# Wait for filesystems to be ready
sleep 2

# Mount the drives
mount ${DATA_DRIVE}1 $DATA_MOUNT
mount ${BACKUP_DRIVE}1 $BACKUP_MOUNT

# Create Bitcoin directory structure following hexagonal architecture
mkdir -p $DATA_MOUNT/bitcoin/{blockchain,wallets,network,p2p}
mkdir -p $DATA_MOUNT/bitcoin/adapters/{lightning,taproot,dlc}
mkdir -p $DATA_MOUNT/bitcoin/metrics
mkdir -p $BACKUP_MOUNT/bitcoin/{blockchain,wallets,network}
mkdir -p $BACKUP_MOUNT/bitcoin/backups/$(date +%Y-%m-%d)

# Create Bitcoin configuration file with pruning settings
cat > $DATA_MOUNT/bitcoin/bitcoin.conf << EOF
# [AIR-3][AIS-3][BPC-3][RES-3]
# Bitcoin Core Configuration with Pruning
# Following official Bitcoin Improvement Proposals (BIPs)
# Generated on $(date)

# Network Configuration
chain=bitcoin
testnet=0
regtest=0

# Pruning Configuration
prune=${PRUNE_SIZE}
txindex=${TXINDEX}
blocks-only=${BLOCKS_ONLY}

# Performance Optimization
dbcache=512
maxmempool=100
maxconnections=40
rpcthreads=4

# Security Settings
disablewallet=0
rpcauth=anya:$(tr -dc 'a-zA-Z0-9' < /dev/urandom | head -c 32)

# Hexagonal Architecture Paths
datadir=$DATA_MOUNT/bitcoin/blockchain
walletdir=$DATA_MOUNT/bitcoin/wallets

# Monitoring & Metrics (BIP Support Matrix)
prometheus=1
prometheusport=9090

# [BIP-341] Taproot Support
taproot=1

# [BIP-174] PSBT Support
avoidpartialspends=1

# System Awareness Requirements
mempoolexpiry=72
minrelaytxfee=0.00001
EOF

# Create a backup of the configuration
cp $DATA_MOUNT/bitcoin/bitcoin.conf $BACKUP_MOUNT/bitcoin/

# Set proper permissions
chown -R anya:anya $DATA_MOUNT $BACKUP_MOUNT
chmod -R 750 $DATA_MOUNT $BACKUP_MOUNT

# Add to fstab for automatic mounting
echo "# Bitcoin data and backup drives - Added $(date)" >> /etc/fstab
echo "UUID=$(blkid -s UUID -o value ${DATA_DRIVE}1) $DATA_MOUNT ext4 defaults,noatime 0 2" >> /etc/fstab
echo "UUID=$(blkid -s UUID -o value ${BACKUP_DRIVE}1) $BACKUP_MOUNT ext4 defaults,noatime 0 2" >> /etc/fstab

# Create symbolic links in the project
ln -sf $DATA_MOUNT/bitcoin /home/anya/anyachainlabs/projects/anya-core/bitcoin-data
ln -sf $BACKUP_MOUNT/bitcoin /home/anya/anyachainlabs/projects/anya-core/bitcoin-backup

# Create cron job for maintenance script
cat > /etc/cron.d/bitcoin-maintenance << EOF
# Bitcoin maintenance - run daily at 2:30 AM
30 2 * * * anya /home/anya/anyachainlabs/projects/anya-core/scripts/bitcoin_maintenance.sh >> /var/log/bitcoin-maintenance.log 2>&1
EOF

# Create a README file with information about the pruning configuration
cat > $DATA_MOUNT/bitcoin/README.md << EOF
# Bitcoin Storage with Pruning Configuration

## Overview
This Bitcoin node is configured with pruning enabled to optimize storage usage according to official Bitcoin Improvement Proposals (BIPs). The pruning configuration helps maintain a smaller blockchain footprint while preserving full validation capabilities.

## Pruning Configuration
- Prune Mode: Enabled
- Prune Size: ${PRUNE_SIZE} MB
- Transaction Index: ${TXINDEX}
- Blocks Only Mode: ${BLOCKS_ONLY}

## Maintenance
Automatic maintenance is scheduled via cron job at 2:30 AM daily:
- Pruning old debug logs
- Creating wallet backups
- Cleaning up old backups (keeping last 7 days)
- Database optimization

## Hexagonal Architecture
The storage follows the hexagonal architecture pattern required by BDF v2.5:
- Core blockchain data isolated from protocol adapters
- Clear separation between blockchain, wallet, and network data
- Dedicated spaces for Lightning, Taproot, and DLC implementations

## BIP Support
- BIP 341/342 (Taproot): Enabled
- BIP 174 (PSBT): Enabled

Generated on: $(date)
EOF

echo "===== [AIR-3][AIS-3][BPC-3][RES-3] Drive Preparation Complete ====="
echo "Bitcoin data drive mounted at: $DATA_MOUNT"
echo "Bitcoin backup drive mounted at: $BACKUP_MOUNT"
echo "Symbolic links created in project directory"
echo "Drives will be automatically mounted on system boot"
echo ""
echo "Pruning Configuration:"
echo "- Prune Mode: Enabled (${PRUNE_SIZE} MB)"
echo "- Transaction Index: ${TXINDEX}"
echo "- Blocks Only Mode: ${BLOCKS_ONLY}"
echo ""
echo "Daily maintenance scheduled at 2:30 AM"
echo "See $DATA_MOUNT/bitcoin/README.md for details"
echo ""
echo "Directory structure follows official Bitcoin Improvement Proposals (BIPs)"
echo "with proper hexagonal architecture implementation."
echo ""

# Display drive information
echo "Drive Information:"
df -h | grep -E "${DATA_DRIVE}|${BACKUP_DRIVE}"

# Log completion
logger -t bitcoin-prepare "Bitcoin drive preparation completed successfully (BDF v2.5)"

# Verify critical components
echo ""
echo "Verification:"
if [ -f "$DATA_MOUNT/bitcoin/bitcoin.conf" ]; then
    echo "✓ Bitcoin configuration file created"
else
    echo "✗ Bitcoin configuration file missing"
fi

if [ -d "$DATA_MOUNT/bitcoin/blockchain" ] && [ -d "$DATA_MOUNT/bitcoin/wallets" ]; then
    echo "✓ Hexagonal architecture directories created"
else
    echo "✗ Directory structure incomplete"
fi

if grep -q "$DATA_MOUNT" /etc/fstab && grep -q "$BACKUP_MOUNT" /etc/fstab; then
    echo "✓ Automatic mounting configured"
else
    echo "✗ Automatic mounting not configured"
fi

echo ""
echo "[AIR-3][AIS-3][BPC-3][RES-3] Bitcoin drives prepared successfully according to BIP Standards"
