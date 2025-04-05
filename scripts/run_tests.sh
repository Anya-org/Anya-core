#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Default values
COMPONENTS=""
REPORT_DIR="reports"
TEST_TYPE="all"
VERBOSE=""

# Print usage
function print_usage {
    echo "Usage: $0 [OPTIONS]"
    echo "Run the Anya-Core unified test suite"
    echo ""
    echo "Options:"
    echo "  -c, --components COMPONENTS   Comma-separated list of components to test"
    echo "  -r, --report-dir DIR         Directory for test reports"
    echo "  -t, --test-type TYPE         Test type (unit|integration|compliance|performance|all)"
    echo "  -v, --verbose                Enable verbose output"
    echo "  -h, --help                   Show this help message"
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -c|--components)
            COMPONENTS="$2"
            shift 2
            ;;
        -r|--report-dir)
            REPORT_DIR="$2"
            shift 2
            ;;
        -t|--test-type)
            TEST_TYPE="$2"
            shift 2
            ;;
        -v|--verbose)
            VERBOSE="true"
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

# Build test arguments
ARGS=()

if [ ! -z "$COMPONENTS" ]; then
    ARGS+=("--components" "$COMPONENTS")
fi

if [ ! -z "$REPORT_DIR" ]; then
    ARGS+=("--report-dir" "$REPORT_DIR")
fi

if [ ! -z "$TEST_TYPE" ]; then
    ARGS+=("--test-type" "$TEST_TYPE")
fi

if [ ! -z "$VERBOSE" ]; then
    ARGS+=("--verbose")
fi

# Run the unified test runner
echo -e "${YELLOW}Running Anya-Core unified test suite...${NC}"
if cargo run --bin anya-tester -- "${ARGS[@]}"; then
    echo -e "${GREEN}All tests completed successfully!${NC}"
    
    # Show report location if generated
    if [ -f "$REPORT_DIR/unified_test_report.md" ]; then
        echo -e "${GREEN}Test report generated at $REPORT_DIR/unified_test_report.md${NC}"
    fi
    
    exit 0
else
    echo -e "${RED}Some tests failed. Check the report for details.${NC}"
    exit 1
fi