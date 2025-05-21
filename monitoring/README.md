# Anya Core Monitoring Stack

This directory contains the configuration and setup for monitoring the Anya Core Bitcoin node and system metrics using Prometheus, Grafana, and related tools.

## Components

- **Prometheus**: Metrics collection and storage
- **Grafana**: Visualization and dashboards
- **Node Exporter**: System metrics
- **cAdvisor**: Container metrics
- **Alertmanager**: Alert management
- **Loki**: Log aggregation
- **Promtail**: Log collection
- **Redis**: Caching and rate limiting
- **Blackbox Exporter**: Network probing
- **Process Exporter**: Process monitoring

## Prerequisites

- Docker and Docker Compose
- At least 4GB of RAM
- At least 10GB of free disk space

## Getting Started

1. **Set environment variables** (optional, edit `start-monitoring.sh` for defaults):
   ```bash
   export GRAFANA_ADMIN_USER=admin
   export GRAFANA_ADMIN_PASSWORD=securepassword
   export BITCOIN_RPC_USER=bitcoinrpc
   export BITCOIN_RPC_PASSWORD=securepassword
   ```

2. **Start the monitoring stack**:
   ```bash
   ./monitoring/start-monitoring.sh
   ```

3. **Access the services**:
   - Grafana: http://localhost:3000
   - Prometheus: http://localhost:9090
   - Alertmanager: http://localhost:9093

## Dashboards

### Bitcoin Node Dashboard
- Block height and synchronization status
- Mempool metrics
- Network connections and traffic
- Peer information
- RPC method call rates

### System Dashboard
- CPU, memory, and disk usage
- Network I/O
- Container metrics
- Process metrics
- Disk I/O and filesystem usage

## Alerting

Alerts are configured in `prometheus/alerts/` and managed by Alertmanager. Default alerts include:

- Bitcoin node down
- High CPU usage
- High memory usage
- Disk space running low
- High network latency
- Service restarts

## Configuration

### Prometheus
- Main config: `prometheus/prometheus.yml`
- Alert rules: `prometheus/alerts/*.yml`
- Service discovery: `prometheus/file_sd/`

### Alertmanager
- Config: `alertmanager/config.yml`
- Templates: `alertmanager/templates/`

### Grafana
- Dashboards: `grafana/provisioning/dashboards/`
- Data sources: `grafana/provisioning/datasources/`

## Maintenance

### Updating Dashboards
1. Make changes in the Grafana UI
2. Export the dashboard JSON
3. Save to `grafana/provisioning/dashboards/`

### Adding New Alerts
1. Create or edit YAML files in `prometheus/alerts/`
2. Update `prometheus/prometheus.yml` if needed
3. Reload Prometheus: `curl -X POST http://localhost:9090/-/reload`

## Troubleshooting

### Check Service Logs
```bash
docker-compose -f monitoring/docker-compose.yml logs -f
```

### Check Service Status
```bash
docker-compose -f monitoring/docker-compose.yml ps
```

### Restart Services
```bash
docker-compose -f monitoring/docker-compose.yml restart
```

## Security Considerations

- Change default credentials
- Use HTTPS in production
- Restrict access to monitoring endpoints
- Regularly update container images
- Monitor resource usage

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
