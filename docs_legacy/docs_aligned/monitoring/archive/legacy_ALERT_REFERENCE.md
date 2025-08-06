---
title: "Alert Reference"
description: "Reference for configured alerts in Anya Core monitoring"
---

# Anya Core Alert Reference

## Overview

This document provides a reference for all active alerts configured in the Anya Core monitoring stack. Alerts are categorized by severity and component for easy reference.

## Alert Severity Levels

| Level      | Description                                 | Response Time   | Notification Channel         |
|----------- |---------------------------------------------|-----------------|------------------------------|
| **Critical** | Immediate attention required, service impact | < 15 minutes    | Email, SMS, PagerDuty        |
| **Warning**  | Attention needed soon, potential issues      | < 1 hour        | Email, Slack                 |
| **Info**     | Informational messages, no immediate action  | N/A             | Email (digest)               |

## Core Alerts

### Node Health

| Alert Name      | Severity   | Condition                                                        | Description                        | Resolution                        |
|----------------|----------- |------------------------------------------------------------------|------------------------------------|------------------------------------|
| `NodeDown`     | Critical   | `up == 0`                                                        | Node is not responding to metrics collection | Check node status, restart if needed |
| `NodeHighCPU`  | Warning    | `rate(node_cpu_seconds_total{mode!="idle"}[5m]) > 0.9`          | CPU usage is very high             | Investigate high CPU processes      |
| `NodeHighMemory` | Warning  | `(node_memory_MemTotal_bytes - node_memory_MemAvailable_bytes) / node_memory_MemTotal_bytes > 0.9` | Memory usage is very high | Check for memory leaks, add more RAM |

### Disk & Storage

| Alert Name      | Severity   | Condition                                                        | Description                        | Resolution                        |
|----------------|----------- |------------------------------------------------------------------|------------------------------------|------------------------------------|
| `LowDiskSpace` | Warning    | `(node_filesystem_avail_bytes{mountpoint="/"} * 100) / node_filesystem_size_bytes{mountpoint="/"} < 10` | Disk space is running low | Clean up disk space or expand storage |

### Network

| Alert Name           | Severity   | Condition                                                        | Description                        | Resolution                        |
|---------------------|----------- |------------------------------------------------------------------|------------------------------------|------------------------------------|
| `HighNetworkTraffic`| Warning    | `rate(node_network_receive_bytes_total[5m]) > 100000000`         | High network receive rate          | Investigate traffic source         |
| `NetworkErrors`     | Warning    | `rate(node_network_receive_errs_total[5m]) > 0`                  | Network interface errors detected  | Check network hardware and connections |

## Bitcoin-Specific Alerts

### Blockchain

| Alert Name         | Severity   | Condition                                                        | Description                        | Resolution                        |
|-------------------|----------- |------------------------------------------------------------------|------------------------------------|------------------------------------|
| `BitcoinNodeDown` | Critical   | `up{job="bitcoin-node"} == 0`                                   | Bitcoin node has been down for more than 5 minutes | Check bitcoind status |
| `BitcoinOutOfSync`| Warning    | `(time() - bitcoin_blocks_time) > 600`                           | Bitcoin node is more than 10 minutes behind the network | Check sync status |
| `BitcoinIBD`      | Warning    | `bitcoin_ibd == 1`                                               | Node is in Initial Block Download  | Monitor progress                   |
| `BitcoinMempoolFull` | Warning | `bitcoin_mempool_size > 100000`                                  | Mempool size is very large         | Check for network congestion       |

### P2P Network

| Alert Name         | Severity   | Condition                                                        | Description                        | Resolution                        |
|-------------------|----------- |------------------------------------------------------------------|------------------------------------|------------------------------------|
| `LowPeerCount`    | Warning    | `bitcoin_peers < 8`                                              | Low number of peer connections     | Check network connectivity         |
| `HighPingTime`    | Warning    | `bitcoin_ping_time > 5`                                          | High ping time to peers            | Check network latency              |

## Custom Alert Rules

To add new alerts, edit the appropriate rule file in `monitoring/prometheus/alerts/` and follow the Prometheus format:

```yaml
- alert: AlertName
  expr: alert_condition
  for: 5m
  labels:
    severity: warning|critical
  annotations:
    summary: "Short alert summary"
    description: "Detailed description"
```


## Notification Channels

Alerts are routed to notification channels as configured in `config/production.yaml`:

- Email: <botshelomokoka+anya-core-monitoring@gmail.com>
- Slack: via webhook
- PagerDuty: for critical alerts

## Best Practices

- Set meaningful alert thresholds and durations
- Regularly review and tune alert rules
- Document all alerts and escalation policies
- Test alerts in staging before production rollout

## AI Labeling


*Complies with [AI Labeling Standards](../standards/AI_LABELING.md) — AIR-3, AIS-3, BPC-3, RES-3*

