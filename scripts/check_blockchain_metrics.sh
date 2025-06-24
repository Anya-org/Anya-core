#!/bin/bash

# Script for checking blockchain metrics from the Anya Core monitoring system
# Usage: ./check_blockchain_metrics.sh [environment] [metric]
# environment: mainnet or testnet (default: testnet)
# metric: specific metric to check (optional)

set -eo pipefail

# Default values
ENVIRONMENT="testnet"
METRIC=""

# Terminal colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check arguments
if [ $# -ge 1 ]; then
    if [ "$1" = "mainnet" ] || [ "$1" = "testnet" ]; then
        ENVIRONMENT="$1"
    else
        echo -e "${RED}Error: Invalid environment. Use 'mainnet' or 'testnet'.${NC}"
        exit 1
    fi
fi

if [ $# -ge 2 ]; then
    METRIC="$2"
fi

# Get API base URL
if [ "$ENVIRONMENT" = "mainnet" ]; then
    API_BASE="https://mainnet-metrics.anya-core.io"
    echo -e "${YELLOW}Checking MAINNET blockchain metrics${NC}"
else
    API_BASE="https://testnet-metrics.anya-core.io"
    echo -e "${BLUE}Checking TESTNET blockchain metrics${NC}"
fi

# Function to check for curl and jq
check_dependencies() {
    if ! command -v curl &>/dev/null; then
        echo -e "${RED}Error: curl is required but not installed.${NC}"
        exit 1
    fi

    if ! command -v jq &>/dev/null; then
        echo -e "${RED}Error: jq is required but not installed.${NC}"
        exit 1
    fi
}

# Function to format a metric value
format_metric() {
    local name="$1"
    local value="$2"

    case "$name" in
    segwit_percentage | taproot_percentage | error_rate_*)
        printf "%.2f%%" "$value"
        ;;
    mempool_size)
        printf "%.2f MB" "$(echo "$value / 1048576" | bc -l)"
        ;;
    avg_block_size)
        printf "%.2f KB" "$(echo "$value / 1024" | bc -l)"
        ;;
    network_hashrate)
        printf "%.2f EH/s" "$value"
        ;;
    block_propagation_ms)
        printf "%d ms" "$value"
        ;;
    utxo_set_size)
        printf "%'d" "$value"
        ;;
    avg_fee_rate)
        printf "%.2f sats/vB" "$value"
        ;;
    *)
        echo "$value"
        ;;
    esac
}

# Function to check if a metric is in warning/error state
check_threshold() {
    local name="$1"
    local value="$2"

    case "$name" in
    segwit_percentage)
        if (($(echo "$value < 80" | bc -l))); then
            echo -e "${RED}WARNING: SegWit adoption below 80%${NC}"
        fi
        ;;
    taproot_percentage)
        if (($(echo "$value < 10" | bc -l))); then
            echo -e "${YELLOW}NOTICE: Taproot adoption below 10%${NC}"
        fi
        ;;
    error_rate_*)
        if (($(echo "$value > 0.05" | bc -l))); then
            echo -e "${RED}WARNING: High error rate detected!${NC}"
        elif (($(echo "$value > 0.01" | bc -l))); then
            echo -e "${YELLOW}NOTICE: Elevated error rate${NC}"
        fi
        ;;
    block_propagation_ms)
        if ((value > 1000)); then
            echo -e "${RED}WARNING: Slow block propagation!${NC}"
        elif ((value > 500)); then
            echo -e "${YELLOW}NOTICE: Block propagation above 500ms${NC}"
        fi
        ;;
    avg_fee_rate)
        if (($(echo "$value > 100" | bc -l))); then
            echo -e "${RED}WARNING: High fee rates!${NC}"
        elif (($(echo "$value > 50" | bc -l))); then
            echo -e "${YELLOW}NOTICE: Elevated fee rates${NC}"
        fi
        ;;
    esac
}

# Check dependencies
check_dependencies

# Get blockchain metrics
echo "Connecting to $API_BASE/metrics/blockchain..."
if ! METRICS=$(curl -s -f "$API_BASE/metrics/blockchain"); then
    echo -e "${RED}Error: Failed to retrieve metrics from $API_BASE/metrics/blockchain${NC}"
    exit 1
fi

# Check for specific metric if provided
if [ -n "$METRIC" ]; then
    VALUE=$(echo "$METRICS" | jq -r ".data.$METRIC")

    if [ "$VALUE" = "null" ]; then
        echo -e "${RED}Error: Metric '$METRIC' not found${NC}"
        echo "Available metrics:"
        echo "$METRICS" | jq -r '.data | keys[]' | sort | sed 's/^/  - /'
        exit 1
    fi

    echo -e "${BLUE}$METRIC:${NC} $(format_metric "$METRIC" "$VALUE")"
    check_threshold "$METRIC" "$VALUE"
    exit 0
fi

# Display all metrics
echo -e "${GREEN}Current blockchain metrics:${NC}"
echo -e "${BLUE}Block height:${NC} $(echo "$METRICS" | jq -r '.data.block_height')"
echo -e "${BLUE}SegWit adoption:${NC} $(format_metric "segwit_percentage" "$(echo "$METRICS" | jq -r '.data.segwit_percentage')")"
check_threshold "segwit_percentage" "$(echo "$METRICS" | jq -r '.data.segwit_percentage')"

echo -e "${BLUE}Taproot adoption:${NC} $(format_metric "taproot_percentage" "$(echo "$METRICS" | jq -r '.data.taproot_percentage')")"
check_threshold "taproot_percentage" "$(echo "$METRICS" | jq -r '.data.taproot_percentage')"

echo -e "${BLUE}UTXO set size:${NC} $(format_metric "utxo_set_size" "$(echo "$METRICS" | jq -r '.data.utxo_set_size')")"
echo -e "${BLUE}Mempool size:${NC} $(format_metric "mempool_size" "$(echo "$METRICS" | jq -r '.data.mempool_size')")"
echo -e "${BLUE}Average block size:${NC} $(format_metric "avg_block_size" "$(echo "$METRICS" | jq -r '.data.avg_block_size')")"
echo -e "${BLUE}Average fee rate:${NC} $(format_metric "avg_fee_rate" "$(echo "$METRICS" | jq -r '.data.avg_fee_rate')")"
check_threshold "avg_fee_rate" "$(echo "$METRICS" | jq -r '.data.avg_fee_rate')"

echo -e "${BLUE}Network hashrate:${NC} $(format_metric "network_hashrate" "$(echo "$METRICS" | jq -r '.data.network_hashrate')")"

# Get the error rates
echo ""
echo -e "${GREEN}Error rates:${NC}"
ERROR_RATES=$(echo "$METRICS" | jq -r '.data.error_rates')
for key in $(echo "$ERROR_RATES" | jq -r 'keys[]'); do
    value=$(echo "$ERROR_RATES" | jq -r ".[\"$key\"]")
    formatted=$(format_metric "error_rate_$key" "$value")
    echo -e "${BLUE}$key:${NC} $formatted"
    check_threshold "error_rate_$key" "$value"
done

# Get BIP compliance
echo ""
echo -e "${GREEN}BIP Compliance:${NC}"
COMPLIANCE=$(echo "$METRICS" | jq -r '.data.bip_compliance')
for key in $(echo "$COMPLIANCE" | jq -r 'keys[]' | sort -n); do
    value=$(echo "$COMPLIANCE" | jq -r ".[\"$key\"]")
    if [ "$value" = "true" ]; then
        echo -e "${BLUE}BIP-$key:${NC} ${GREEN}Compliant✓${NC}"
    else
        echo -e "${BLUE}BIP-$key:${NC} ${RED}Not Compliant✗${NC}"
    fi
done

# Check for alerts
echo ""
echo "Checking for active alerts..."
if ! ALERTS=$(curl -s -f "$API_BASE/metrics/alerts"); then
    echo -e "${YELLOW}Warning: Failed to retrieve alerts${NC}"
else
    ALERT_COUNT=$(echo "$ALERTS" | jq -r '.count')

    if [ "$ALERT_COUNT" -gt 0 ]; then
        echo -e "${RED}$ALERT_COUNT active alerts:${NC}"

        echo "$ALERTS" | jq -r '.data[] | "\(.severity): \(.description) [ID: \(.id)]"' | while read -r line; do
            echo -e " - ${RED}$line${NC}"
        done
    else
        echo -e "${GREEN}No active alerts${NC}"
    fi
fi

echo ""
echo -e "${GREEN}Metrics check completed successfully${NC}"
