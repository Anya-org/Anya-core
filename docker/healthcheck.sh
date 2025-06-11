#!/bin/sh
set -e

# Healthcheck script for Anya decentralized core
# This script verifies the health of the Anya core service

ANYA_PORT=${ANYA_PORT:-8080}
TIMEOUT=${TIMEOUT:-10}

echo "Checking Anya core health on port $ANYA_PORT..."

# Check if the service is responding to HTTP requests
if curl -f --connect-timeout $TIMEOUT --max-time $TIMEOUT "http://localhost:$ANYA_PORT/health" >/dev/null 2>&1; then
    echo "✓ Anya core is healthy"
    exit 0
else
    echo "✗ Anya core health check failed"
    exit 1
fi