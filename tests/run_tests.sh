#!/bin/bash

# Set error handling
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Print header
echo -e "${YELLOW}Anya Core Protocol Test Suite${NC}"
echo "====================================="
echo "Starting test run at $(date)"
echo

# Create test reports directory if it doesn't exist
mkdir -p test_reports

# Run protocol tests
echo -e "${YELLOW}Running protocol tests...${NC}"
cargo run --bin run_protocol_tests

# Check if tests passed
if [ $? -eq 0 ]; then
    echo -e "${GREEN}All protocol tests passed!${NC}"
else
    echo -e "${RED}Some protocol tests failed. Check the report for details.${NC}"
fi

# Generate test summary
echo
echo -e "${YELLOW}Generating test summary...${NC}"
latest_report=$(ls -t test_reports/protocol_test_report_*.json | head -n1)
if [ -n "$latest_report" ]; then
    echo "Latest report: $latest_report"
    
    # Parse and display summary
    echo
    echo -e "${YELLOW}Test Summary:${NC}"
    total_protocols=$(jq '.total_protocols' "$latest_report")
    successful_protocols=$(jq '.successful_protocols' "$latest_report")
    failed_protocols=$(jq '.failed_protocols' "$latest_report")
    total_time=$(jq '.total_time' "$latest_report")
    
    echo "Total Protocols: $total_protocols"
    echo "Successful: $successful_protocols"
    echo "Failed: $failed_protocols"
    echo "Total Time: ${total_time}s"
    
    # Display protocol-specific results
    echo
    echo -e "${YELLOW}Protocol Results:${NC}"
    jq -r '.protocol_results[] | "\(.name): \(.status) (\(.completion_time)s)"' "$latest_report"
    
    # Check for failed protocols
    if [ "$failed_protocols" -gt 0 ]; then
        echo
        echo -e "${RED}Failed Protocols:${NC}"
        jq -r '.protocol_results[] | select(.status == "Failed") | "\(.name): \(.error)"' "$latest_report"
    fi
else
    echo -e "${RED}No test reports found!${NC}"
fi

# Generate milestone report
echo
echo -e "${YELLOW}Generating milestone report...${NC}"
if [ -n "$latest_report" ]; then
    milestone_report="test_reports/milestone_report_$(date +%Y%m%d_%H%M%S).md"
    
    # Create milestone report
    cat > "$milestone_report" << EOF
# Protocol Test Milestone Report
Generated on: $(date)

## Overview
- Total Protocols: $total_protocols
- Successful: $successful_protocols
- Failed: $failed_protocols
- Total Time: ${total_time}s

## Protocol Results
EOF
    
    # Add protocol results
    jq -r '.protocol_results[] | "### \(.name)\n- Status: \(.status)\n- Completion Time: \(.completion_time)s\n\n#### Milestones:"' "$latest_report" >> "$milestone_report"
    
    # Add milestone details
    jq -r '.protocol_results[] | "### \(.name)\n\n#### Milestones:\n\(.milestones[] | "- \(.name): \(.status) (\(.completion_time)s)")"' "$latest_report" >> "$milestone_report"
    
    echo "Milestone report generated: $milestone_report"
fi

# Check if all tests passed
if [ "$failed_protocols" -gt 0 ]; then
    echo -e "\n${RED}Some tests failed. Please check the reports for details.${NC}"
    exit 1
else
    echo -e "\n${GREEN}All tests passed successfully!${NC}"
    exit 0
fi 