# Anya Core Monitoring Guide

[AIR-3][AIS-3][BPC-3][RES-3]

## Overview

The Anya Core monitoring stack provides comprehensive observability for your node with the following components:

- **Prometheus**: Metrics collection and storage
- **Grafana**: Visualization and dashboards
- **Alertmanager**: Alert management and routing
- **Node Exporter**: System metrics
- **cAdvisor**: Container metrics
- **Loki**: Log aggregation
- **Promtail**: Log collection

## Getting Started

### Prerequisites

- Docker and Docker Compose installed
- Minimum 2GB RAM (4GB recommended)
- 10GB free disk space for metrics storage

### Installation

Monitoring can be installed in two ways:

1. **During Initial Installation**

   ```bash
   ./scripts/install/main_installer.sh --with-monitoring
   ```

2. **Adding to Existing Installation**

   ```bash
   cd /path/to/anya-core
   ./monitoring/start-monitoring.sh
   ```

## Accessing the Dashboards

### Grafana

- **URL**: <http://localhost:3000>
- **Default Credentials**:
  - Username: `admin`
  - Password: `admin123` (change on first login)

### Prometheus

- **URL**: <http://localhost:9090>
- **Metrics Endpoint**: <http://localhost:9090/metrics>

### Alertmanager

- **URL**: <http://localhost:9093>

## Configuration

### Email Notifications

To configure email notifications, edit the `.env` file in the monitoring directory:

```bash
# Monitoring/.env
SMTP_FROM=botshelomokoka@gmail.com
SMTP_SMARTHOST=smtp.gmail.com:587
SMTP_AUTH_USERNAME=botshelomokoka@gmail.com
SMTP_AUTH_PASSWORD=your-gmail-app-password
SMTP_HELO=gmail.com

# Alert Recipients
ALERT_EMAIL_RECIPIENT=botshelomokoka@gmail.com
MAINNET_ALERT_RECIPIENT=mainnet-alerts@anyacore.org
```

> **Note**: For Gmail, you'll need to generate an App Password if 2FA is enabled.

### Alert Rules

Alert rules are defined in `monitoring/prometheus/alerts/`. The default rules include:

- Node down
- High CPU usage
- High memory usage
- Disk space warnings
- Service restarts

## Dashboards

### Available Dashboards

1. **Anya Core Overview**
   - Node status
   - Sync status
   - Network connections
   - Resource usage

2. **Bitcoin Node**
   - Block height
   - Mempool size
   - Peer connections
   - RPC metrics

3. **System**
   - CPU/Memory/Disk usage
   - Network I/O
   - Container metrics

## Troubleshooting

### Common Issues

1. **Grafana Login Issues**
   - Default credentials: admin/admin123
   - Reset password: `docker-compose -f monitoring/docker-compose.yml exec grafana grafana-cli admin reset-admin-password newpassword`

2. **Prometheus Targets Down**
   - Check if services are running: `docker ps`
   - View logs: `docker-compose -f monitoring/docker-compose.yml logs prometheus`

3. **Email Notifications Not Working**
   - Verify SMTP settings in `.env`
   - Check Alertmanager logs: `docker-compose -f monitoring/docker-compose.yml logs alertmanager`

## Backup and Restore

### Backup Monitoring Data

```bash
# Create backup directory
mkdir -p /backup/monitoring

# Backup Prometheus data
docker run --rm -v monitoring_prometheus_data:/source -v /backup/monitoring:/backup alpine tar czf /backup/prometheus-$(date +%Y%m%d).tar.gz -C /source .

# Backup Grafana data
docker run --rm -v monitoring_grafana_data:/source -v /backup/monitoring:/backup alpine tar czf /backup/grafana-$(date +%Y%m%d).tar.gz -C /source .
```

### Restore Monitoring Data

```bash
# Stop monitoring services
cd monitoring
docker-compose down

# Restore Prometheus data
docker run --rm -v monitoring_prometheus_data:/target -v /backup/monitoring:/backup alpine sh -c "rm -rf /target/* && tar xzf /backup/prometheus-20230521.tar.gz -C /target"

# Restore Grafana data
docker run --rm -v monitoring_grafana_data:/target -v /backup/monitoring:/backup alpine sh -c "rm -rf /target/* && tar xzf /backup/grafana-20230521.tar.gz -C /target"

# Start services
docker-compose up -d
```

## Security Considerations

1. **Change Default Credentials**
   - Change Grafana admin password immediately
   - Use strong passwords for all services

2. **Network Security**
   - Restrict access to monitoring ports (3000, 9090, 9093)
   - Use a reverse proxy with HTTPS
   - Enable authentication for all services

3. **Data Retention**
   - Configure retention policies in Prometheus
   - Monitor disk usage for metrics storage

## Support

For assistance with monitoring:

- Email: <botshelomokoka@gmail.com>
- GitHub Issues: <https://github.com/your-org/anya-core/issues>
