---
title: "Upgrade"
description: "Documentation for Upgrade"
---

# Anya Core Upgrade Guide

## Table of Contents

- [Section 1](#section-1)
- [Section 2](#section-2)


[AIR-3][AIS-3][BPC-3][RES-3]

## Overview

This document provides guidelines for upgrading Anya Core between versions. Always review this guide before performing an upgrade to ensure a smooth transition.

## Version Compatibility

| From Version | To Version | Upgrade Path | Notes |
|-------------|-----------|--------------|-------|
| ≤ 0.2.0    | 0.3.0     | Full reinstall required | Major architectural changes |
| 0.3.0      | ≥ 0.3.1   | In-place upgrade | Follow these instructions |

## Pre-Upgrade Checklist

1. **Backup Important Data**

   ```bash
   # Create backup directory
   mkdir -p ~/anya_backup_$(date +%Y%m%d)
   
   # Backup configuration
   cp -r /etc/anya ~/anya_backup_$(date +%Y%m%d)/config
   
   # Backup wallet and chain data (if applicable)
   cp -r ~/.anya/{wallets,chaindata} ~/anya_backup_$(date +%Y%m%d)/
   ```

2. **Check System Requirements**
   - Verify disk space: `df -h /` (minimum 20GB free)
   - Check memory: `free -h` (minimum 4GB RAM)
   - Verify Docker: `docker --version` (≥ 20.10)
   - Verify Docker Compose: `docker-compose --version` (≥ 2.0)

3. **Review Release Notes**
   Always check the [CHANGELOG.md](../../scripts/enterprise/CHANGELOG.md) for breaking changes and new requirements.

## Upgrade Procedure

### Standard Upgrade (v0.3.0+)

1. **Stop Running Services**

   ```bash
   # Stop Anya Core service
   sudo systemctl stop anya-core
   
   # Stop monitoring stack (if running)
   cd monitoring
   docker-compose down
   cd ..
   ```

2. **Update Repository**

   ```bash
   # Fetch latest changes
   git fetch origin
   
   # Checkout the target version
   git checkout v0.3.0  # Replace with target version
   
   # Pull latest changes
   git pull
   ```

3. **Run Database Migrations (if any)**

   ```bash
   ./scripts/migrate.sh
   ```

4. **Update Configuration**

   ```bash
   # Backup current config
   cp config/config.toml config/config.toml.bak
   
   # Update configuration (preserve your settings)
   ./scripts/update-config.sh
   ```

5. **Restart Services**

   ```bash
   # Start Anya Core
   sudo systemctl start anya-core
   
   # Start monitoring (if enabled)
   cd monitoring
   ./start-monitoring.sh
   cd ..
   ```

### Monitoring Stack Upgrade

If upgrading the monitoring stack separately:

```bash
cd monitoring

# Pull latest container images
docker-compose pull

# Recreate containers with new images
docker-compose up -d --force-recreate

# Verify all services are running
docker-compose ps
```

## Post-Upgrade Verification

1. **Check Service Status**

   ```bash
   # Check Anya Core
   systemctl status anya-core
   
   # Check monitoring stack
   docker-compose -f monitoring/docker-compose.yml ps
   ```

2. **Verify Data Integrity**
   - Check logs for errors: `journalctl -u anya-core -n 50`
   - Verify metrics are being collected in Grafana
   - Test alert notifications

3. **Update Documentation**
   - Review and update any local documentation
   - Note any configuration changes in your runbook

## Rollback Procedure

If you encounter issues after upgrade:

1. **Stop Services**

   ```bash
   sudo systemctl stop anya-core
   cd monitoring
   docker-compose down
   cd ..
   ```

2. **Restore Backup**

   ```bash
   # Restore configuration
   cp -r ~/anya_backup_$(date +%Y%m%d)/config /etc/anya/
   
   # Restore data (if needed)
   cp -r ~/anya_backup_$(date +%Y%m%d)/* ~/.anya/
   ```

3. **Revert Code**

   ```bash
   git checkout <previous-version-tag>
   ```

4. **Restart Services**

   ```bash
   sudo systemctl start anya-core
   cd monitoring
   ./start-monitoring.sh
   ```

## Troubleshooting

### Common Issues

1. **Version Mismatch**
   - Symptom: Services fail to start after upgrade
   - Solution: Ensure all components are at compatible versions
   
2. **Database Migration Failures**
   - Symptom: Errors during migration
   - Solution: Restore from backup and check migration scripts
   
3. **Permission Issues**
   - Symptom: Permission denied errors
   - Solution: `sudo chown -R anya:anya /var/lib/anya`

### Getting Help

If you encounter issues:

1. Check logs: `journalctl -u anya-core -n 100`
2. Review [Troubleshooting Guide](../installation/troubleshooting.md)
3. Open an issue on [GitHub](https://github.com/your-org/anya-core/issues)
4. Email support: <botshelomokoka@gmail.com>

## Security Considerations

- Always verify checksums of downloaded packages
- Use secure channels for all file transfers
- Rotate credentials after upgrade
- Review and update firewall rules if needed

## AI Labeling

- [AIR-3] - Automated upgrade process with validation
- [AIS-3] - Secure upgrade procedures with rollback
- [BPC-3] - Follows Bitcoin node upgrade best practices
- [RES-3] - Resilient upgrade process with verification

## See Also

- [Related Document](#related-document)

