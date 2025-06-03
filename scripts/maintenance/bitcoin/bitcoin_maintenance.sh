#!/bin/bash
# [AIR-3][AIS-3][BPC-3][RES-3]
# Bitcoin Data Maintenance Script
# Following official Bitcoin Improvement Proposals (BIPs)

DATA_MOUNT="/mnt/bitcoin-data"
BACKUP_MOUNT="/mnt/bitcoin-backup"

# Prune old debug logs
find $DATA_MOUNT/bitcoin/blockchain/debug.log -size +100M -exec rm {} \;

# Backup wallet files
BACKUP_DIR="$BACKUP_MOUNT/bitcoin/backups/$(date +%Y-%m-%d)"
mkdir -p $BACKUP_DIR
cp -r $DATA_MOUNT/bitcoin/wallets/* $BACKUP_DIR/

# Clean up old backups (keep last 7 days)
find $BACKUP_MOUNT/bitcoin/backups -type d -mtime +7 -exec rm -rf {} \; 2>/dev/null || true

# Optimize database
if [ -f $DATA_MOUNT/bitcoin/blockchain/bitcoind.pid ]; then
  bitcoin-cli -datadir=$DATA_MOUNT/bitcoin/blockchain vacuumdb
fi

# Report disk usage
echo "===== Bitcoin Storage Report ====="
du -sh $DATA_MOUNT/bitcoin/*
df -h $DATA_MOUNT $BACKUP_MOUNT
