#!/bin/bash
# [AIR-3][AIS-3][BPC-3][AIT-3] BIP Health Check Script
# Runs the BIP health checker and creates a report

set -e

# Script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( cd "$SCRIPT_DIR/../.." && pwd )"

# Create reports directory if it doesn't exist
REPORTS_DIR="$PROJECT_ROOT/reports/bip"
mkdir -p "$REPORTS_DIR"

# Default timestamp format
TIMESTAMP=$(date +"%Y%m%d%H%M%S")

# Parse arguments
VERBOSE=""
FORMAT="markdown"
MODE="report"
OUTPUT="$REPORTS_DIR/bip-health-$TIMESTAMP.md"

print_usage() {
    echo "Usage: $0 [options]"
    echo "Options:"
    echo "  -v, --verbose            Enable verbose output"
    echo "  -f, --format FORMAT      Output format (markdown, json, text)"
    echo "  -o, --output FILE        Output file path"
    echo "  -m, --mode MODE          Mode (check, report, monitor)"
    echo "  -h, --help               Print this help message"
}

while [[ $# -gt 0 ]]; do
    case $1 in
        -v|--verbose)
            VERBOSE="--verbose"
            shift
            ;;
        -f|--format)
            FORMAT="$2"
            shift 2
            ;;
        -o|--output)
            OUTPUT="$2"
            shift 2
            ;;
        -m|--mode)
            MODE="$2"
            shift 2
            ;;
        -h|--help)
            print_usage
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            print_usage
            exit 1
            ;;
    esac
done

# Build the binary if needed
echo "Building BIP health checker..."
cd "$PROJECT_ROOT"
cargo build --bin bip_health

# Run the health checker
echo "Running BIP health check in $MODE mode..."

BINARY="$PROJECT_ROOT/target/debug/bip_health"

case $MODE in
    check)
        $BINARY $VERBOSE check --format "$FORMAT"
        ;;
    report)
        $BINARY $VERBOSE report --format "$FORMAT" --output "$OUTPUT"
        echo "Report saved to: $OUTPUT"
        ;;
    monitor)
        $BINARY $VERBOSE monitor --output-dir "$REPORTS_DIR"
        ;;
    *)
        echo "Invalid mode: $MODE"
        print_usage
        exit 1
        ;;
esac

echo "BIP health check completed successfully!" 