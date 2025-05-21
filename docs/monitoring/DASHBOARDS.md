---
title: "Dashboards"
description: "Documentation for Dashboards"
---

# Anya Core Monitoring Dashboards

## Table of Contents

- [Section 1](#section-1)
- [Section 2](#section-2)


[AIR-3][AIS-3][BPC-3][RES-3]

## Overview

This document provides detailed information about the monitoring dashboards available in the Anya Core monitoring stack. These dashboards are designed to provide real-time visibility into the health and performance of your Anya Core node and its components.

## Dashboard Index

### 1. Anya Core Overview

- **Purpose**: High-level view of node health and status
- **Access**: `http://<grafana-host>:3000/d/anya-overview`
- **Refresh Rate**: 15s
- **Retention**: 30 days

### 2. Bitcoin Node Metrics

- **Purpose**: Detailed Bitcoin node metrics and performance
- **Access**: `http://<grafana-host>:3000/d/bitcoin-node`
- **Refresh Rate**: 15s
- **Retention**: 30 days

### 3. System Resources

- **Purpose**: Host system resource utilization
- **Access**: `http://<grafana-host>:3000/d/system`
- **Refresh Rate**: 15s
- **Retention**: 7 days

### 4. Network Monitoring

- **Purpose**: Network I/O and connectivity
- **Access**: `http://<grafana-host>:3000/d/network`
- **Refresh Rate**: 15s
- **Retention**: 7 days

### 5. Alert Dashboard

- **Purpose**: View and manage active alerts
- **Access**: `http://<grafana-host>:3000/d/alerts`
- **Refresh Rate**: 30s
- **Retention**: 90 days

## Dashboard Details

### Anya Core Overview

#### Panels

1. **Node Status**
   - Uptime
   - Sync status
   - Version information
   - Network (mainnet/testnet/regtest)

2. **Performance**
   - Transactions per second
   - Mempool size
   - Block processing time
   - Peer connections

3. **Resource Usage**
   - CPU/Memory/Disk usage
   - I/O operations
   - Network traffic

### Bitcoin Node Metrics

#### Panels

1. **Blockchain**
   - Block height
   - Headers
   - Verification progress
   - IBD status

2. **Mempool**
   - Transaction count
   - Size in MB
   - Fee rates
   - Orphan transactions

3. **P2P Network**
   - Peer count
   - Banned peers
   - Bytes sent/received
   - Ping time

## Customizing Dashboards

### Adding New Panels

1. Log in to Grafana
2. Navigate to the desired dashboard
3. Click "Add Panel" > "Add new panel"
4. Configure the panel with PromQL queries
5. Set appropriate thresholds and alerts

### Importing Dashboards

1. Download dashboard JSON from source
2. In Grafana, click "+" > "Import"
3. Upload the JSON file
4. Select the data source (Prometheus)
5. Click "Import"

## Alerting

### Pre-configured Alerts

| Alert Name | Severity | Condition | Description |
|------------|----------|-----------|-------------|
| NodeDown | Critical | `up == 0` | Node is down |
| HighCPU | Warning | `rate(node_cpu_seconds_total{mode!="idle"}[5m]) > 0.9` | High CPU usage |
| LowDiskSpace | Warning | `node_filesystem_avail_bytes{mountpoint="/"} / node_filesystem_size_bytes{mountpoint="/"} < 0.2` | Low disk space |
| HighMemUsage | Warning | `(node_memory_MemTotal_bytes - node_memory_MemAvailable_bytes) / node_memory_MemTotal_bytes > 0.9` | High memory usage |

### Creating Custom Alerts

1. Navigate to Alerting > Alert rules
2. Click "New alert rule"
3. Define the alert conditions using PromQL
4. Set alert labels and annotations
5. Configure notification policies

## Troubleshooting

### Common Issues

1. **No Data in Panels**
   - Verify Prometheus is running
   - Check service discovery configuration
   - Verify network connectivity between Prometheus and targets

2. **High Load on Grafana**
   - Increase dashboard refresh interval
   - Reduce time range
   - Use time-based retention policies

3. **Alert Notifications Not Working**
   - Verify Alertmanager configuration
   - Check SMTP settings
   - Review notification policies

## Best Practices

1. **Dashboard Design**
   - Group related metrics
   - Use consistent color schemes
   - Add descriptive titles and units
   - Set appropriate Y-axis ranges

2. **Alerting**
   - Set meaningful alert thresholds
   - Use alert grouping
   - Configure proper notification channels
   - Test alerts regularly

3. **Performance**
   - Limit dashboard refresh rate
   - Use recording rules for expensive queries
   - Monitor Grafana resource usage

## Security Considerations

- Restrict dashboard access using Grafana roles
- Use read-only users for shared dashboards
- Regularly rotate credentials
- Monitor access logs

## Support

For assistance with monitoring:

- Email: <botshelomokoka+monitoring@gmail.com>
- GitHub Issues: <https://github.com/your-org/anya-core/issues>
- Documentation: [Monitoring Guide](../installation/MONITORING.md)

## AI Labeling

- [AIR-3] - Automated monitoring and visualization
- [AIS-3] - Secure dashboard access and configuration
- [BPC-3] - Bitcoin monitoring best practices
- [RES-3] - Comprehensive monitoring coverage

## See Also

- [Related Document](#related-document)

