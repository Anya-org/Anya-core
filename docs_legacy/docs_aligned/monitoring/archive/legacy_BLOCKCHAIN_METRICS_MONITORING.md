# Blockchain Metrics Monitoring System

## Overview

The Anya Core Blockchain Metrics Monitoring System provides real-time monitoring and alerting for blockchain metrics including SegWit adoption, Taproot adoption, block propagation times, fee rates, and error rates. This system is essential for ensuring the health and performance of the Anya Core network, particularly when promoting new releases from Testnet to Mainnet.

## Architecture

The monitoring system consists of:

1. **Metrics Collection**: Collection of key blockchain metrics in real-time
2. **Metrics Storage**: Time-series storage of historical metrics
3. **Alerting System**: Real-time alerts for metrics outside acceptable thresholds
4. **API Endpoints**: HTTP API for querying metrics and alerts
5. **CLI Tools**: Command-line tools for interacting with the metrics system

## Key Metrics Monitored

The system monitors the following key metrics:

### Transaction Structure Metrics

- **SegWit Adoption Percentage**: Percentage of transactions using SegWit (Segregated Witness)
- **Taproot Adoption Percentage**: Percentage of transactions using Taproot

### Network Performance Metrics

- **Block Propagation Time**: Time in milliseconds for blocks to propagate across the network
- **Average Fee Rate**: Average transaction fee rate in sats/vB
- **Mempool Size**: Size of the mempool in bytes

### System Health Metrics

- **Error Rates**: Categorized error rates as percentages
- **UTXO Set Size**: Current size of the UTXO set
- **Network Hashrate**: Current hashrate of the network in EH/s

### Standards Compliance

- **BIP Compliance**: Compliance status for various Bitcoin Improvement Proposals

## API Endpoints

The monitoring system exposes the following API endpoints:

- `GET /metrics`: All metrics in JSON format
- `GET /metrics/prometheus`: Metrics in Prometheus format
- `GET /metrics/blockchain`: Blockchain-specific metrics
- `GET /metrics/blockchain/historical/{metric}`: Historical data for specific metrics
- `GET /metrics/alerts`: Active alerts
- `GET /metrics/alerts/history`: Alert history
- `POST /metrics/alerts/acknowledge/{alert_id}`: Acknowledge an alert

## CLI Tools

### Check Blockchain Metrics

The `check_blockchain_metrics.sh` script provides a simple way to check the current blockchain metrics:

```bash
./scripts/check_blockchain_metrics.sh [environment] [metric]
```

Parameters:

- `environment`: `mainnet` or `testnet` (default: `testnet`)
- `metric`: Specific metric to check (optional)

Example:

```bash
./scripts/check_blockchain_metrics.sh mainnet segwit_percentage
```

### Validate Testnet Metrics

The `validate_testnet_metrics.sh` script validates metrics before promoting a version from Testnet to Mainnet:

```bash
./scripts/validate_testnet_metrics.sh <version>
```

This script checks metrics against predefined thresholds and fails if any metrics are outside acceptable ranges.

## Configuration

The monitoring system can be configured using the following environment variables:

- `ANYA_METRICS_COLLECTION_INTERVAL_MS`: Interval in milliseconds for metrics collection (default: 10000)
- `ANYA_METRICS_PORT`: Port for the metrics API server (default: 9200)
- `ANYA_METRICS_HOST`: Host for the metrics API server (default: localhost)

## Integration with CI/CD Pipeline

The monitoring system integrates with the CI/CD pipeline in two primary ways:

1. **Testnet Validation**: Before promoting a build to Mainnet, the system validates that all metrics are within acceptable thresholds
2. **Post-Deployment Monitoring**: After deployment, the system monitors for any anomalies and alerts if needed

## Alert Thresholds

The system includes default thresholds for various metrics:

- **SegWit Adoption**: Must be above 80%
- **Taproot Adoption**: Must be above 10% 
- **Error Rates**: Must be below 0.5%
- **Block Propagation Time**: Must be below 1000ms
- **Fee Rate**: Warning if above 100 sats/vB

These thresholds can be adjusted as needed based on network conditions and requirements.

## Dashboard Access

Metrics dashboards are available at:

- Testnet: `https://testnet-metrics.anya-core.io/`
- Mainnet: `https://mainnet-metrics.anya-core.io/`

## Future Enhancements

Planned enhancements to the monitoring system include:

1. **Machine Learning-Based Anomaly Detection**: Automatic detection of unusual patterns
2. **Predictive Analytics**: Forecasting future network conditions based on historical data
3. **Enhanced Visualization**: More comprehensive dashboards for metrics visualization
4. **Integration with External Monitoring Systems**: Integration with Prometheus, Grafana, etc.
5. **Multi-Node Monitoring**: Distributed monitoring across multiple nodes

## Implementation Status

The blockchain metrics monitoring system is now functional with the following components completed:

- Core metrics collection
- Alerting system
- API endpoints
- CLI tools
- Integration with Testnet to Mainnet promotion pipeline

Ongoing work focuses on expanding the range of metrics collected and enhancing the alerting capabilities.
