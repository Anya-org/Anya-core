---
title: "Readme"
description: "Documentation for Readme"
last_updated: 2025-05-30
---

# Anya Core Installation Guide

## Overview

Add a brief overview of this document here.


[AIR-3][AIS-3][BPC-3][RES-3]

## Table of Contents

- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
- [Installation Options](#installation-options)
  - [Standard Installation](#standard-installation)
  - [With Monitoring](#with-monitoring)
  - [Custom Configuration](#custom-configuration)
- [Verification](#verification)
- [Troubleshooting](#troubleshooting)
- [Upgrading](#upgrading)

## Prerequisites

- Linux (Ubuntu 20.04+ recommended)
- Docker and Docker Compose
- 4GB+ RAM (8GB recommended for production)
- 20GB+ free disk space
- Python 3.8+
- Rust (latest stable)

## Quick Start

```bash
# Clone the repository
git clone https://github.com/your-org/anya-core.git
cd anya-core

# Run the installer with default options
./scripts/install/main_installer.sh
```

## Installation Options

### Standard Installation

```bash
./scripts/install/main_installer.sh --type=standard --network=testnet
```

### With Monitoring

To install with comprehensive monitoring:

```bash
./scripts/install/main_installer.sh --with-monitoring
```

This will set up:

- Prometheus for metrics collection
- Grafana for visualization
- Alertmanager for notifications
- Node Exporter for system metrics
- cAdvisor for container metrics

### Custom Configuration

```bash
# View all available options
./scripts/install/main_installer.sh --help

# Example: Install with custom configuration
./scripts/install/main_installer.sh \
    --type=complete \
    --network=mainnet \
    --with-monitoring
```

## Verification

After installation, verify the services are running:

```bash
# Check Anya Core service
systemctl status anya-core

# Check monitoring services (if installed with --with-monitoring)
docker-compose -f monitoring/docker-compose.yml ps
```

Access the monitoring dashboards:

- Grafana: <http://localhost:3000>
- Prometheus: <http://localhost:9090>
- Alertmanager: <http://localhost:9093>

## Troubleshooting

### Common Issues

1. **Port Conflicts**
   - Check for other services using ports 3000, 9090, 9093
   - Update port mappings in `monitoring/docker-compose.yml` if needed

2. **Permission Issues**
   - Ensure your user has docker permissions
   - Run with `sudo` if necessary

3. **Monitoring Not Starting**
   - Check Docker logs: `docker-compose -f monitoring/docker-compose.yml logs`
   - Verify Docker is running: `systemctl status docker`

## Upgrading

To upgrade an existing installation:

```bash
# Pull the latest changes
git pull

# Run the installer again
./scripts/install/main_installer.sh --upgrade

# Restart services
systemctl restart anya-core
docker-compose -f monitoring/docker-compose.yml up -d
```

## Support

For additional help:

1. Check the [troubleshooting guide](troubleshooting.md)
2. Open an issue on [GitHub](https://github.com/your-org/anya-core/issues)
3. Email support: <botshelomokoka@gmail.com>

## See Also

- [Related Document 1](./related1.md)
- [Related Document 2](./related2.md)
