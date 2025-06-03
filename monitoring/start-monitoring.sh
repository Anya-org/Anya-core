#!/bin/bash

# Start Monitoring Stack for Anya Core
# This script sets up and starts all monitoring components

# Set default environment variables
export GRAFANA_ADMIN_USER=${GRAFANA_ADMIN_USER:-admin}
export GRAFANA_ADMIN_PASSWORD=${GRAFANA_ADMIN_PASSWORD:-admin123}
export BITCOIN_RPC_USER=${BITCOIN_RPC_USER:-bitcoinrpc}
export BITCOIN_RPC_PASSWORD=${BITCOIN_RPC_PASSWORD:-password}

echo "Starting Anya Core Monitoring Stack..."
echo "Grafana Admin User: $GRAFANA_ADMIN_USER"
echo "Bitcoin RPC User: $BITCOIN_RPC_USER"

# Create necessary directories
echo "Creating directories..."
mkdir -p monitoring/grafana/provisioning/datasources
mkdir -p monitoring/grafana/provisioning/dashboards
mkdir -p monitoring/prometheus/alerts
mkdir -p monitoring/alertmanager/templates

# Set proper permissions
echo "Setting permissions..."
chmod -R 755 monitoring/
chmod 644 monitoring/grafana/provisioning/dashboards/*
chmod 644 monitoring/grafana/provisioning/datasources/*
chmod 644 monitoring/prometheus/*
chmod 644 monitoring/alertmanager/*

# Start Docker Compose
echo "Starting Docker Compose..."
docker-compose -f monitoring/docker-compose.yml up -d

# Wait for services to start
echo "Waiting for services to start..."
sleep 10

# Check if services are running
echo "Checking services..."
docker ps --format "table {{.Names}}\t{{.Status}}" | grep -E 'prometheus|grafana|node-exporter|cadvisor|alertmanager|blackbox-exporter|process-exporter|redis-exporter|redis|loki|promtail'

echo "\nMonitoring stack started successfully!"
echo "Grafana: http://localhost:3000 (admin/admin123)"
echo "Prometheus: http://localhost:9090"
echo "Alertmanager: http://localhost:9093"
echo "Node Exporter: http://localhost:9100/metrics"
echo "cAdvisor: http://localhost:8080"

echo "\nTo stop the monitoring stack, run: docker-compose -f monitoring/docker-compose.yml down"
