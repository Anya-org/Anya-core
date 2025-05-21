---
title: "Alert_reference"
description: "Documentation for Alert_reference"
---

# Anya Core Alert Reference

## Table of Contents

- [Section 1](#section-1)
- [Section 2](#section-2)


[AIR-3][AIS-3][BPC-3][RES-3]

## Overview

This document provides a comprehensive reference for all alerts configured in the Anya Core monitoring stack. Alerts are categorized by severity and component for easy reference.

## Alert Severity Levels

| Level | Description | Response Time | Notification Channel |
|-------|-------------|----------------|----------------------|
| **Critical** | Immediate attention required, service impact | < 15 minutes | Email, SMS, PagerDuty |
| **Warning** | Attention needed soon, potential issues | < 1 hour | Email, Slack |
| **Info** | Informational messages, no immediate action | N/A | Email (digest) |

## Core Alerts

### Node Health

| Alert Name | Severity | Condition | Description | Resolution |
|------------|----------|-----------|-------------|-------------|
| `NodeDown` | Critical | `up == 0` | Node is not responding to metrics collection | Check node status, restart if needed |
| `NodeHighCPU` | Warning | `rate(node_cpu_seconds_total{mode!="idle"}[5m]) > 0.9` | CPU usage is very high | Investigate high CPU processes |
| `NodeHighMemory` | Warning | `(node_memory_MemTotal_bytes - node_memory_MemAvailable_bytes) / node_memory_MemTotal_bytes > 0.9` | Memory usage is very high | Check for memory leaks, add more RAM |

### Disk & Storage

| Alert Name | Severity | Condition | Description | Resolution |
|------------|----------|-----------|-------------|-------------|
| `LowDiskSpace` | Warning | `node_filesystem_avail_bytes{mountpoint="/"} / node_filesystem_size_bytes{mountpoint="/"} < 0.2` | Disk space is running low | Clean up disk space or expand storage |
| `HighDiskIO` | Warning | `rate(node_disk_io_time_seconds_total[5m]) > 0.9` | High disk I/O utilization | Check for disk bottlenecks |

### Network

| Alert Name | Severity | Condition | Description | Resolution |
|------------|----------|-----------|-------------|-------------|
| `HighNetworkTraffic` | Warning | `rate(node_network_receive_bytes_total[5m]) > 100000000` | High network receive rate | Investigate traffic source |
| `NetworkErrors` | Warning | `rate(node_network_receive_errs_total[5m]) > 0` | Network interface errors detected | Check network hardware and connections |

## Bitcoin-Specific Alerts

### Blockchain

| Alert Name | Severity | Condition | Description | Resolution |
|------------|----------|-----------|-------------|-------------|
| `BitcoinNodeDown` | Critical | `bitcoin_blocks < (time() - bitcoin_latest_block_time) / 600 > 3` | Bitcoin node is not syncing | Check bitcoind status |
| `BitcoinIBD` | Warning | `bitcoin_ibd == 1` | Node is in Initial Block Download | Monitor progress |
| `BitcoinMempoolFull` | Warning | `bitcoin_mempool_size > 100000` | Mempool size is very large | Check for network congestion |

### P2P Network

| Alert Name | Severity | Condition | Description | Resolution |
|------------|----------|-----------|-------------|-------------|
| `LowPeerCount` | Warning | `bitcoin_peers < 8` | Low number of peer connections | Check network connectivity |
| `HighPingTime` | Warning | `bitcoin_ping_time > 5` | High ping time to peers | Check network latency |

## Custom Alert Rules

### Adding New Alerts

1. Edit the appropriate rule file in `monitoring/prometheus/rules/`
2. Follow the format:

   ```yaml
   - alert: AlertName
     expr: alert_condition
     for: 5m
     labels:
       severity: warning|critical
     annotations:
       description: "Detailed description"
       summary: "Short alert summary"
   ```

### Alert Routing

Alerts are routed based on severity and component:

```yaml
routes:
  - match:
      severity: 'critical'
    receiver: 'critical-alerts'
  - match:
      severity: 'warning'
    receiver: 'warning-alerts'
  - match:
      alertname: 'NodeDown'
    receiver: 'pagerduty'
```

## Notification Templates

### Email Template

```text
{{ define "email.default.html" }}
{{- if gt (len .Alerts.Firing) 0 -}}
{{ range .Alerts.Firing }}
[FIRING] {{ .Labels.alertname }}
Severity: {{ .Labels.severity }}
Summary: {{ .Annotations.summary }}
Description: {{ .Annotations.description }}
{{ end }}
{{- end }}
{{- if gt (len .Alerts.Resolved) 0 -}}
{{ range .Alerts.Resolved }}
[RESOLVED] {{ .Labels.alertname }}
Resolved at: {{ .StartsAt }}
{{ end }}
{{- end }}
{{- end }}
```

## Testing Alerts

### Manual Testing

1. Use the Alertmanager UI to silence an alert
2. Use `amtool` to test alert configurations:

   ```bash
   amtool alert --alertmanager.url=http://localhost:9093 --alertname=NodeDown
   ```

### Integration Testing

1. Deploy to staging environment
2. Trigger test alerts using the Alertmanager API:

   ```bash
   curl -X POST http://localhost:9093/api/v2/alerts -d '
   [
     {
       "status": "firing",
       "labels": {
         "alertname": "TestAlert",
         "severity": "warning"
       },
       "annotations": {
         "summary": "Test alert",
         "description": "This is a test alert"
       }
     }
   ]'
   ```

## Alert Suppression

### During Maintenance

1. Create a maintenance window in Alertmanager:

   ```bash
   curl -X POST http://localhost:9093/api/v2/silences \
     -H "Content-Type: application/json" \
     -d '{
       "matchers": [
         {"name": "alertname", "value": ".+", "isRegex": true}
       ],
       "startsAt": "2025-01-01T00:00:00Z",
       "endsAt": "2025-01-01T02:00:00Z",
       "createdBy": "maintenance",
       "comment": "Planned maintenance window"
     }'
   ```

## Best Practices

1. **Alert Fatigue Prevention**
   - Set appropriate thresholds
   - Use alert grouping
   - Implement alert inhibition rules

2. **Alert Documentation**
   - Document all alerts
   - Include runbooks
   - Define escalation policies

3. **Alert Tuning**
   - Regularly review alert thresholds
   - Remove unused alerts
   - Adjust for seasonality

## Support

For alert-related issues:

- Email: <botshelomokoka+alerts@gmail.com>
- GitHub Issues: <https://github.com/your-org/anya-core/issues>
- Documentation: [Monitoring Guide](../installation/MONITORING.md)

## AI Labeling

- [AIR-3] - Automated alert management
- [AIS-3] - Secure alert handling
- [BPC-3] - Bitcoin monitoring best practices
- [RES-3] - Comprehensive alert coverage

## See Also

- [Related Document](#related-document)

