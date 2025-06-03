#!/bin/bash
# [AIR-3][AIS-3][BPC-3][RES-3]

# Define colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Print usage
function print_usage {
    echo "Usage: $0 [OPTIONS]"
    echo "Run the Anya-Core unified test suite"
    echo ""
    echo "Options:"
    echo "  -c, --components COMPONENTS   Comma-separated list of components to test"
    echo "  -e, --endpoint URL            Bitcoin RPC endpoint URL"
    echo "  -r, --report-dir DIR          Directory for test reports"
    echo "  -n, --no-reports              Disable report generation"
    echo "  -v, --verbose                 Enable verbose output"
    echo "  -h, --help                    Show this help message"
    echo ""
    echo "Available components: bitcoin, dao, web5, ml, system, compliance"
    echo "Available RPC endpoints:"
    echo "  Mainnet: https://bitcoin-rpc.publicnode.com"
    echo "  Testnet: https://bitcoin-testnet-rpc.publicnode.com"
}

# Parse command line arguments
COMPONENTS=""
RPC_ENDPOINT=""
REPORT_DIR="reports"
NO_REPORTS=""
VERBOSE=""

while [[ $# -gt 0 ]]; do
    case $1 in
        -c|--components)
            COMPONENTS="$2"
            shift 2
            ;;
        -e|--endpoint)
            RPC_ENDPOINT="$2"
            shift 2
            ;;
        -r|--report-dir)
            REPORT_DIR="$2"
            shift 2
            ;;
        -n|--no-reports)
            NO_REPORTS="--no-reports"
            shift
            ;;
        -v|--verbose)
            VERBOSE="--verbose"
            shift
            ;;
        -h|--help)
            print_usage
            exit 0
            ;;
        *)
            echo -e "${RED}Error: Unknown option: $1${NC}"
            print_usage
            exit 1
            ;;
    esac
done

# Build the command
CMD="cargo run --bin anya-tester -- unified"

if [ ! -z "$COMPONENTS" ]; then
    CMD="$CMD --components $COMPONENTS"
fi

if [ ! -z "$RPC_ENDPOINT" ]; then
    CMD="$CMD --rpc-endpoint $RPC_ENDPOINT"
fi

if [ ! -z "$REPORT_DIR" ]; then
    CMD="$CMD --report-dir $REPORT_DIR"
fi

if [ ! -z "$NO_REPORTS" ]; then
    CMD="$CMD $NO_REPORTS"
fi

if [ ! -z "$VERBOSE" ]; then
    CMD="$CMD $VERBOSE"
fi

# Run the tests
echo -e "${YELLOW}Running Anya-Core unified tests...${NC}"
echo "Command: $CMD"
echo ""

eval $CMD

# Check the exit code
EXIT_CODE=$?
if [ $EXIT_CODE -eq 0 ]; then
    echo -e "${GREEN}All tests passed!${NC}"
    
    # Check if report was generated
    if [ -z "$NO_REPORTS" ] && [ -f "$REPORT_DIR/unified_test_report.md" ]; then
        echo -e "${GREEN}Test report generated at $REPORT_DIR/unified_test_report.md${NC}"
    fi
else
    echo -e "${RED}Some tests failed. See the report for details.${NC}"
    
    # Check if report was generated
    if [ -z "$NO_REPORTS" ] && [ -f "$REPORT_DIR/unified_test_report.md" ]; then
        echo -e "${YELLOW}Test report generated at $REPORT_DIR/unified_test_report.md${NC}"
    fi
fi

exit $EXIT_CODE 