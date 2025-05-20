#!/bin/bash
# [AIR-3][AIS-3][BPC-3][RES-3]
# [AIR-3][AIS-3][AIT-3][BPC-3]
# Anya Core Compliance Report Generator
# This script analyzes the codebase for BDF v2.5 compliance and AI labeling

set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
REPORT_DIR="${PROJECT_ROOT}/reports"
REPORT_FILE="${REPORT_DIR}/compliance_report_$(date +%Y%m%d-%H%M%S).md"
LOG_FILE="${REPORT_DIR}/compliance_scan.log"

# Create reports directory if it doesn't exist
mkdir -p "${REPORT_DIR}"

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Log function
log_message() {
  local level=$1
  local message=$2
  echo -e "[$(date '+%Y-%m-%d %H:%M:%S')] [${level}] ${message}" | tee -a "$LOG_FILE"
}

# Initialize the report
initialize_report() {
  log_message "INFO" "Initializing compliance report"
  
  cat > "${REPORT_FILE}" << EOF
# Anya Core Compliance Report
**Generated:** $(date '+%Y-%m-%d %H:%M:%S')

## Bitcoin Development Framework v2.5 Compliance

This report analyzes the Anya Core codebase for compliance with Bitcoin Development Framework v2.5
requirements and proper AI labeling standards.

## Executive Summary

EOF
}

# Check AI labeling compliance
check_ai_labeling() {
  log_message "INFO" "Analyzing AI labeling compliance"
  
  local total_files=$(find "${PROJECT_ROOT}/src" -name "*.rs" | wc -l)
  local labeled_files=$(find "${PROJECT_ROOT}/src" -name "*.rs" | xargs grep -l "\[AIR-[0-9]\]" | wc -l)
  local compliance_percentage=$(( 100 * labeled_files / total_files ))
  
  cat >> "${REPORT_FILE}" << EOF
### AI Labeling Compliance

- **Total Rust Files:** ${total_files}
- **Files with AI Labeling:** ${labeled_files}
- **Compliance Rate:** ${compliance_percentage}%

EOF
  
  if [ "$compliance_percentage" -lt 80 ]; then
    cat >> "${REPORT_FILE}" << EOF
⚠️ **Compliance Warning:** AI labeling coverage is below 80% threshold.
Consider running \`./scripts/enforce_ai_labels.sh --auto-fix\` to automatically apply missing labels.

EOF
  fi
  
  log_message "SUCCESS" "AI labeling analysis completed"
}

# Check BDF v2.5 feature compliance
check_bdf_compliance() {
  log_message "INFO" "Analyzing BDF v2.5 feature compliance"
  
  # Define required BDF v2.5 features
  local features=(
    "Taproot (BIP-341/342)"
    "Layer 2 Protocol Support"
    "DLC (Discrete Log Contracts)"
    "RSK Smart Contract Verification"
    "Hexagonal Architecture"
    "System Awareness"
    "Monitoring Metrics"
  )
  
  # Check for each feature
  cat >> "${REPORT_FILE}" << EOF
### BDF v2.5 Feature Compliance

| Feature | Status | Implementation Path |
|---------|--------|---------------------|
EOF
  
  # Taproot implementation
  if grep -q "struct Taproot" "${PROJECT_ROOT}/src" -r || 
     grep -q "TaprootImplementation" "${PROJECT_ROOT}/src" -r; then
    echo "| Taproot (BIP-341/342) | ✅ Implemented | src/bitcoin/taproot |" >> "${REPORT_FILE}"
  else
    echo "| Taproot (BIP-341/342) | ❌ Not Found | - |" >> "${REPORT_FILE}"
  fi
  
  # Layer 2 Protocol
  if [ -d "${PROJECT_ROOT}/src/layer2" ] || [ -d "${PROJECT_ROOT}/src/bitcoin/layer2" ]; then
    echo "| Layer 2 Protocol Support | ✅ Implemented | src/layer2, src/bitcoin/layer2 |" >> "${REPORT_FILE}"
  else
    echo "| Layer 2 Protocol Support | ❌ Not Found | - |" >> "${REPORT_FILE}"
  fi
  
  # DLC implementation
  if grep -q "DLC" "${PROJECT_ROOT}/src" -r || 
     [ -d "${PROJECT_ROOT}/src/bitcoin/dlc" ]; then
    echo "| DLC (Discrete Log Contracts) | ✅ Implemented | src/bitcoin/dlc |" >> "${REPORT_FILE}"
  else
    echo "| DLC (Discrete Log Contracts) | ❌ Not Found | - |" >> "${REPORT_FILE}"
  fi
  
  # RSK Smart Contract Verification
  if grep -q "verify_bitcoin_payment" "${PROJECT_ROOT}/src" -r; then
    echo "| RSK Smart Contract Verification | ✅ Implemented | src/core/system_awareness.rs |" >> "${REPORT_FILE}"
  else
    echo "| RSK Smart Contract Verification | ❌ Not Found | - |" >> "${REPORT_FILE}"
  fi
  
  # Hexagonal Architecture
  if [ -d "${PROJECT_ROOT}/src/ports" ] || [ -d "${PROJECT_ROOT}/src/adapters" ]; then
    echo "| Hexagonal Architecture | ✅ Implemented | src/ports, src/adapters |" >> "${REPORT_FILE}"
  else
    echo "| Hexagonal Architecture | ❌ Not Found | - |" >> "${REPORT_FILE}"
  fi
  
  # System Awareness
  if grep -q "NetworkStateMonitor" "${PROJECT_ROOT}/src/core/system_awareness.rs" -r 2>/dev/null; then
    echo "| System Awareness | ✅ Implemented | src/core/system_awareness.rs |" >> "${REPORT_FILE}"
  else
    echo "| System Awareness | ❌ Not Found | - |" >> "${REPORT_FILE}"
  fi
  
  # Monitoring Metrics
  if grep -q "TPS Capacity" "${PROJECT_ROOT}/src" -r || 
     grep -q "Block Propagation" "${PROJECT_ROOT}/src" -r; then
    echo "| Monitoring Metrics | ✅ Implemented | src/monitoring |" >> "${REPORT_FILE}"
  else
    echo "| Monitoring Metrics | ❌ Not Found | - |" >> "${REPORT_FILE}"
  fi
  
  log_message "SUCCESS" "BDF feature analysis completed"
}

# Check for code duplication
check_duplication() {
  log_message "INFO" "Analyzing code duplication"
  
  # Check for duplicate Bitcoin implementations
  local bitcoin_dirs=$(find "${PROJECT_ROOT}/src" -path "*bitcoin*" -type d | wc -l)
  
  cat >> "${REPORT_FILE}" << EOF

### Code Duplication Analysis

- **Bitcoin implementation directories:** ${bitcoin_dirs}

EOF
  
  if [ "$bitcoin_dirs" -gt 3 ]; then
    cat >> "${REPORT_FILE}" << EOF
⚠️ **Duplication Warning:** Multiple Bitcoin implementation directories detected.
Consider running \`./scripts/consolidate_bitcoin_impl.sh\` to consolidate implementations.

EOF
  fi
  
  # Check for empty layer2 directory
  if [ -d "${PROJECT_ROOT}/src/layer2" ] && [ -z "$(ls -A "${PROJECT_ROOT}/src/layer2")" ]; then
    cat >> "${REPORT_FILE}" << EOF
⚠️ **Structure Warning:** Empty layer2 directory detected while implementations exist in other locations.
Consider consolidating layer2 implementations from other directories.

EOF
  fi
  
  log_message "SUCCESS" "Duplication analysis completed"
}

# Generate recommendations
generate_recommendations() {
  log_message "INFO" "Generating recommendations"
  
  cat >> "${REPORT_FILE}" << EOF

## Recommendations

Based on the compliance analysis, consider the following recommendations:

1. **AI Labeling**: Run \`./scripts/enforce_ai_labels.sh --auto-fix\` to enforce consistent AI labeling across the codebase.

2. **Code Consolidation**: Execute \`./scripts/consolidate_bitcoin_impl.sh\` to consolidate Bitcoin implementations into a coherent structure.

3. **Directory Structure**: Ensure a clean hexagonal architecture pattern with proper separation of ports and adapters.

4. **System Awareness**: Verify the implementation of system awareness components according to BDF v2.5.

5. **Testing**: Update test suite to validate all BDF v2.5 features.

EOF
  
  log_message "SUCCESS" "Recommendations generated"
}

# Finish report
finish_report() {
  log_message "INFO" "Finalizing compliance report"
  
  cat >> "${REPORT_FILE}" << EOF

## Conclusion

This automatic compliance report provides an overview of the current state of the Anya Core
codebase with respect to Bitcoin Development Framework v2.5 requirements and AI labeling standards.

Follow the recommendations above to address any compliance gaps and improve code quality.

**Report Path:** \`${REPORT_FILE}\`
EOF
  
  log_message "SUCCESS" "Compliance report generated successfully: ${REPORT_FILE}"
  echo -e "${GREEN}Compliance report generated successfully${NC}"
  echo -e "${BLUE}Report: ${REPORT_FILE}${NC}"
}

# Main execution
main() {
  log_message "INFO" "Starting compliance analysis"
  
  initialize_report
  check_ai_labeling
  check_bdf_compliance
  check_duplication
  generate_recommendations
  finish_report
  
  # Show report summary
  echo -e "\n${YELLOW}=== Compliance Report Summary ===${NC}"
  echo -e "${BLUE}$(grep "Total Rust Files:" "${REPORT_FILE}")${NC}"
  echo -e "${BLUE}$(grep "Compliance Rate:" "${REPORT_FILE}")${NC}"
}

# Execute main function
main
