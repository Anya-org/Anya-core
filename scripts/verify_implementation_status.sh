#!/bin/bash
# Anya Core Implementation Status Verification Script
# Enforces adherence to verified reality over aspirational claims

set -euo pipefail

print_usage() {
    cat <<'USAGE'
Usage: scripts/verify_implementation_status.sh [options]

Options:
    --auto-run           Run builds and tests as part of verification
    --yes-all            Do not fail the script; convert failures into warnings (exit 0)
    --network <mode>     Override network mode: regtest | testnet | mainnet
    --json               Emit a compact JSON summary to stdout at the end
    -h, --help           Show this help

Behavior:
    - Defaults to ANYA_NETWORK_TYPE env or 'testnet' if not set
    - In regtest: mocks/simulations allowed; in testnet/mainnet: disallowed
    - With --yes-all: never exit non-zero; useful for auto pipelines
USAGE
}

AUTO_RUN=0
YES_ALL=0
EMIT_JSON=0
OVERRIDE_NETWORK=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --auto-run) AUTO_RUN=1; shift ;;
        --yes-all) YES_ALL=1; shift ;;
        --network) OVERRIDE_NETWORK="${2:-}"; shift 2 ;;
        --json) EMIT_JSON=1; shift ;;
        -h|--help) print_usage; exit 0 ;;
        *) echo "Unknown option: $1"; print_usage; exit 2 ;;
    esac
done

echo "🔍 ANYA CORE IMPLEMENTATION STATUS VERIFICATION"
echo "================================================"
echo "Date: $(date)"
echo "Purpose: Verify actual implementation status against claims"
echo ""

# Determine network mode early (env -> override)
NETWORK_MODE=${ANYA_NETWORK_TYPE:-${NETWORK_MODE:-testnet}}
if [ -n "$OVERRIDE_NETWORK" ]; then
    NETWORK_MODE="$OVERRIDE_NETWORK"
fi

# Check compilation status (and optionally run tests)
echo "📋 COMPILATION STATUS:"
echo "----------------------"
if cargo check --all-features >/dev/null 2>&1; then
    echo "✅ Compilation: PASSING"
else
    echo "❌ Compilation: FAILING"
    echo "   → Must fix compilation before claiming any completeness"
fi

if [ "$AUTO_RUN" -eq 1 ]; then
    echo ""
    echo "🧪 TEST EXECUTION:"
    echo "------------------"
    TEST_TIMEOUT_SECS=${TEST_TIMEOUT_SECS:-600}
    if command -v timeout >/dev/null 2>&1; then
        if timeout "${TEST_TIMEOUT_SECS}s" cargo test --all-features -- --nocapture >/dev/null 2>&1; then
            echo "✅ Unit tests: PASSING"
            TEST_FAIL=0
        else
            echo "❌ Unit tests: FAILING or TIMED OUT (${TEST_TIMEOUT_SECS}s)"
            TEST_FAIL=1
        fi
    else
        if cargo test --all-features -- --nocapture >/dev/null 2>&1; then
        echo "✅ Unit tests: PASSING"
        TEST_FAIL=0
    else
        echo "❌ Unit tests: FAILING"
        TEST_FAIL=1
        fi
    fi
else
    TEST_FAIL=0
fi

# Count unimplemented macros
echo ""
echo "🚫 UNIMPLEMENTED FUNCTIONS:"
echo "---------------------------"
unimpl_count=$(grep -r "unimplemented!" --include="*.rs" . 2>/dev/null | wc -l)
echo "Total unimplemented!() macros: $unimpl_count"

if [ $unimpl_count -eq 0 ]; then
    echo "✅ No unimplemented!() macros found"
else
    echo "❌ $unimpl_count unimplemented!() macros remaining"
    echo "   → Cannot claim '100% complete' with unimplemented!() macros"
    echo ""
    echo "   Locations:"
    grep -r "unimplemented!" --include="*.rs" . 2>/dev/null | head -5
    if [ $unimpl_count -gt 5 ]; then
        echo "   ... and $((unimpl_count - 5)) more"
    fi
fi

# Count TODO stubs
echo ""
echo "📝 TODO STUBS:"
echo "--------------"
todo_count=$(grep -r "todo!" --include="*.rs" . 2>/dev/null | wc -l)
echo "Total todo!() stubs: $todo_count"

if [ $todo_count -eq 0 ]; then
    echo "✅ No todo!() stubs found"
else
    echo "❌ $todo_count todo!() stubs remaining"
    echo "   → Core functionality incomplete"
fi

# Check for SQLite TODOs
echo ""
echo "💾 STORAGE IMPLEMENTATION:"
echo "--------------------------"
sqlite_todo_count=$(grep -r "TODO.*SQLite\|TODO.*database\|TODO.*storage" --include="*.rs" . 2>/dev/null | wc -l)
echo "SQLite implementation TODOs: $sqlite_todo_count"

if [ $sqlite_todo_count -eq 0 ]; then
    echo "✅ No SQLite implementation TODOs found"
else
    echo "❌ $sqlite_todo_count SQLite TODOs remaining"
    echo "   → Storage layer not production-ready"
fi

# Check for mock implementations
echo ""
echo "🎭 MOCK IMPLEMENTATIONS:"
echo "-----------------------"
mock_count=$(grep -r "MockImpl\|placeholder.*implementation" --include="*.rs" . 2>/dev/null | wc -l)
echo "Mock implementations found: $mock_count"

if [ $mock_count -gt 100 ]; then
    echo "❌ High number of mock implementations detected"
    echo "   → Network/Oracle layers may use placeholder implementations"
else
    echo "✅ Mock implementations: $mock_count (acceptable for network/oracle layers)"
fi

# Detailed mock analysis
echo ""
echo "📊 MOCK IMPLEMENTATION BREAKDOWN:"
echo "--------------------------------"

# Layer2 protocol mocks
layer2_mocks=$(grep -r "NoopAdapter\|MockProtocol" --include="*.rs" . 2>/dev/null | wc -l)
echo "Layer2 Protocol Mocks: $layer2_mocks"

# HSM/Security mocks
hsm_mocks=$(grep -r "MockHsm\|SoftwareProvider" --include="*.rs" . 2>/dev/null | wc -l)
echo "HSM/Security Mocks: $hsm_mocks"

# ML/AI service mocks
ml_mocks=$(grep -r "MockMLService\|MockPredictor" --include="*.rs" . 2>/dev/null | wc -l)
echo "ML/AI Service Mocks: $ml_mocks"

# Network client mocks
network_mocks=$(grep -r "MockClient\|MockNetwork" --include="*.rs" . 2>/dev/null | wc -l)
echo "Network Client Mocks: $network_mocks"

# Warning count
echo ""
echo "⚠️ COMPILATION WARNINGS:"
echo "------------------------"
warning_count=$(cargo check --all-features 2>&1 | grep "warning:" | wc -l)
echo "Total warnings: $warning_count"

if [ $warning_count -lt 10 ]; then
    echo "✅ Warning count acceptable (<10)"
else
    echo "❌ Warning count too high (>10)"
    echo "   → Code quality needs improvement"
fi

# Simulation/fallback detection
echo ""
echo "🧪 SIMULATION/FALLBACK DETECTION (env-aware):"
echo "-------------------------------------------"
echo "Network mode: $NETWORK_MODE"

# Build candidate file list excluding dev-sim and test/mock paths
SIM_FILE_LIST=$(find src -type f -name "*.rs" \
    ! -path "*/providers/simulator.rs" \
    ! -path "*/layer2/dev_sim.rs" \
    ! -path "*/tests/*" \
    ! -path "*/*_test.rs" \
    ! -path "*/mock/*" \
    ! -path "*/mocks/*" \
    ! -name "*.toml" ! -name "*.yaml" ! -name "*.yml" ! -name "*.json" ! -name "*.conf" 2>/dev/null)

# Count simulate function definitions or calls only (not config field names)
sim_count=$(echo "$SIM_FILE_LIST" | xargs -r grep -E "^[[:space:]]*fn[[:space:]]+simulate_[A-Za-z0-9_]*[[:space:]]*\(|[^A-Za-z0-9_]simulate_[A-Za-z0-9_]*[[:space:]]*\(" 2>/dev/null | wc -l)
# Count explicit mock adapter/protocol types in non-test code
mock_proto_count=$(echo "$SIM_FILE_LIST" | xargs -r grep -E "NoopAdapter|MockProtocol" 2>/dev/null | wc -l)
fallback_flag_count=$(echo "$SIM_FILE_LIST" | xargs -r grep -E "enable_self_node_fallback" 2>/dev/null | wc -l)
echo "Simulation fns/calls: $sim_count"
echo "Mock protocol refs: $mock_proto_count"
echo "Fallback flags (enable_self_node_fallback): $fallback_flag_count"

sim_fail=0
sim_fail=0
if [ "$NETWORK_MODE" = "regtest" ]; then
    # Regtest allows simulation/mocks for rapid iteration
    if [ "$sim_count" -gt 0 ] || [ "$mock_proto_count" -gt 0 ]; then
        echo "🟡 Simulation/mocks present (allowed in regtest)"
    else
        echo "✅ No simulation/mocks detected"
    fi
elif [ "$sim_count" -gt 0 ] || [ "$mock_proto_count" -gt 0 ]; then
    if [ "${ALLOW_SIMULATION:-0}" -eq 1 ]; then
        echo "🟡 Simulation/mocks detected but allowed via ALLOW_SIMULATION=1"
    else
        echo "❌ Simulation/mocks detected in production mode (testnet/mainnet). Set ALLOW_SIMULATION=1 to temporarily bypass."
        sim_fail=1
    fi
else
    echo "✅ No simulation/mocks detected in production code paths"
fi

# List offenders (top 10) to speed up remediation
if [ $sim_fail -eq 1 ]; then
    echo ""
    echo "🔎 Offending simulation/mock references (top 10):"
    echo "$SIM_FILE_LIST" | xargs -r grep -nE "(^|[^A-Za-z0-9_])(fn[[:space:]]+simulate_|simulate_[A-Za-z0-9_]*\(|NoopAdapter|MockProtocol)" 2>/dev/null | head -10 || true
fi

# Dev-sim gating visibility
DEV_SIM_GATES=$(grep -R "cfg(feature=\"dev-sim\")" --include "*.rs" src 2>/dev/null | wc -l)
echo "Dev-sim feature gates present: $DEV_SIM_GATES"

# Overall assessment
echo ""
echo "📊 OVERALL ASSESSMENT:"
echo "====================="

OVERALL_FAIL=0
if [ $unimpl_count -eq 0 ] && [ $todo_count -eq 0 ] && [ $sqlite_todo_count -eq 0 ] && [ $sim_fail -eq 0 ] && [ $TEST_FAIL -eq 0 ]; then
    echo "✅ PRODUCTION READY: All core implementations complete"
elif [ $sim_fail -eq 1 ]; then
    echo "❌ NOT PRODUCTION READY: Simulation/fallback paths present"
    echo "   Priority: Replace simulate_* and NoopAdapter/MockProtocol with real adapters"
    OVERALL_FAIL=1
elif [ $unimpl_count -gt 0 ]; then
    echo "❌ NOT PRODUCTION READY: $unimpl_count unimplemented!() functions"
    echo "   Priority: Complete RGB/DLC core functions first"
    OVERALL_FAIL=1
elif [ $sqlite_todo_count -gt 0 ]; then
    echo "🟡 PARTIAL IMPLEMENTATION: Storage layer has $sqlite_todo_count TODOs"
    echo "   Priority: Complete storage implementation"
    OVERALL_FAIL=1
elif [ $TEST_FAIL -ne 0 ]; then
    echo "❌ TEST FAILURES detected"
    echo "   Priority: Fix failing tests"
    OVERALL_FAIL=1
else
    echo "🟡 PARTIAL IMPLEMENTATION: $todo_count todo!() stubs remaining"
    echo "   Priority: Complete Web5/DID implementation"
    OVERALL_FAIL=1
fi

# Available systems inventory
echo ""
echo "🏗️ AVAILABLE SYSTEMS INVENTORY:"
echo "-------------------------------"

# Check Bitcoin system availability
if [ -d "anya-bitcoin" ] || [ -d "src/bitcoin" ]; then
    bitcoin_files=$(find . -name "*.rs" -path "*bitcoin*" | wc -l)
    echo "🪙 Bitcoin Core System: ✅ Available ($bitcoin_files files)"
else
    echo "🪙 Bitcoin Core System: ❌ Not found"
fi

# Check Layer2 protocols
if [ -d "src/layer2" ]; then
    lightning_impl=$(find src/layer2 -name "*lightning*" -type f | wc -l)
    rgb_impl=$(find src/layer2 -name "*rgb*" -type f | wc -l)
    dlc_impl=$(find src/layer2 -name "*dlc*" -type f | wc -l)
    echo "⚡ Layer2 Protocols: ✅ Available"
    echo "   Lightning Network: $lightning_impl implementations"
    echo "   RGB Protocol: $rgb_impl implementations"
    echo "   DLC Contracts: $dlc_impl implementations"
else
    echo "⚡ Layer2 Protocols: ❌ Not found"
fi

# Check HSM/Security system
if [ -d "src/security" ]; then
    hsm_providers=$(find src/security -name "*provider*" -type f | wc -l)
    echo "🔐 Security/HSM System: ✅ Available ($hsm_providers providers)"
else
    echo "🔐 Security/HSM System: ❌ Not found"
fi

# Check Web5 system
if [ -d "src/web5" ]; then
    web5_components=$(find src/web5 -name "*.rs" | wc -l)
    echo "🌐 Web5 Protocol System: ✅ Available ($web5_components components)"
else
    echo "🌐 Web5 Protocol System: ❌ Not found"
fi

# Check DAO system
if [ -d "src/dao" ]; then
    dao_contracts=$(find src/dao -name "*.rs" | wc -l)
    echo "🏛️ DAO Governance: ✅ Available ($dao_contracts contracts)"
else
    echo "🏛️ DAO Governance: ❌ Not found"
fi

# Check API system
if [ -d "src/api" ]; then
    api_routes=$(find src/api -name "*.rs" | wc -l)
    echo "🌍 API System: ✅ Available ($api_routes route files)"
else
    echo "🌍 API System: ❌ Not found"
fi

# Check ML/AI system
if [ -d "src/ml" ]; then
    ml_components=$(find src/ml -name "*.rs" | wc -l)
    echo "🤖 ML/AI System: ✅ Available ($ml_components components)"
else
    echo "🤖 ML/AI System: ❌ Not found"
fi

    # Docs check: Multiple H1 headings in PRDs (MD025 heuristic)
    echo ""
    echo "📚 DOCUMENTATION HEALTH:"
    echo "------------------------"
    PRD_DIR="docs/prd"
    if [ -d "$PRD_DIR" ]; then
        md025_issues=0
        while IFS= read -r -d '' file; do
            h1_count=$(grep -c '^# ' "$file" || true)
            if [ "$h1_count" -gt 1 ]; then
                echo "⚠️  Multiple H1 headings: $file ($h1_count)"
                md025_issues=$((md025_issues+1))
            fi
        done < <(find "$PRD_DIR" -maxdepth 1 -type f -name "*.md" -print0 2>/dev/null)
        if [ "$md025_issues" -eq 0 ]; then
            echo "✅ PRD files: single H1 heading per document"
        fi
    else
        echo "ℹ️ PRD directory not found at $PRD_DIR"
    fi

    # Docker Compose sanity (optional)
    echo ""
    echo "🐳 DOCKER COMPOSE CONFIG:"
    echo "-------------------------"
    if command -v docker >/dev/null 2>&1; then
        if docker compose config -q >/dev/null 2>&1; then
            echo "✅ docker-compose.*: valid configuration"
        else
            echo "⚠️ docker-compose config failed (check YAML/indentation)"
            OVERALL_FAIL=1
        fi
    else
        echo "(docker not available) skipping docker compose validation"
    fi

    # RPC wiring checks
    echo ""
    echo "🔌 BITCOIN RPC WIRING:"
    echo "----------------------"
    RPC_INFO=$(grep -R "get_blockchain_info\|estimate_smart_fee" --include "*.rs" src 2>/dev/null | wc -l)
    if [ "$RPC_INFO" -gt 0 ]; then
        echo "✅ RPC integration symbols present (height/hash/fees)"
    else
        echo "❌ RPC integration symbols missing"
        OVERALL_FAIL=1
    fi

    # Default network is testnet check (alignment)
    echo ""
    echo "🌐 DEFAULT NETWORK ALIGNMENT:"
    echo "-----------------------------"
    if grep -R "network_id:\s*\"bitcoin-testnet\"" src/layer2/production.rs >/dev/null 2>&1; then
        echo "✅ Default network: testnet"
    else
        echo "⚠️ Default network not testnet"
    fi

echo ""
echo "📋 VERIFICATION COMMANDS FOR DOCUMENTATION:"
echo "==========================================="
echo "Compilation: cargo check --all-features"
echo "Unimplemented: grep -r \"unimplemented!\" --include=\"*.rs\" . | wc -l"
echo "Todo stubs: grep -r \"todo!\" --include=\"*.rs\" . | wc -l"
echo "SQLite TODOs: grep -r \"TODO.*SQLite\" --include=\"*.rs\" . | wc -l"
echo "Warnings: cargo check --all-features 2>&1 | grep \"warning:\" | wc -l"
echo "Simulation/fallback: find src -name '*.rs' -print0 | xargs -0 grep -E 'simulate_|NoopAdapter|MockProtocol|enable_self_node_fallback' | wc -l"

echo ""
echo "🎯 NEXT ACTIONS BASED ON VERIFICATION:"
echo "======================================"
if [ $unimpl_count -gt 0 ]; then
    echo "1. Fix unimplemented!() macros in RGB/DLC protocols"
    echo "2. Focus on /anya-bitcoin/layer2/rgb/mod.rs first"
    echo "3. Implement transfer_asset, get_transfer_status, validate_transfer"
elif [ $sqlite_todo_count -gt 0 ]; then
    echo "1. Replace SQLite placeholder implementations"
    echo "2. Add real database operations with connection pooling"
    echo "3. Test data persistence across application restarts"
elif [ $todo_count -gt 0 ]; then
    echo "1. Complete Web5/DID implementations"
    echo "2. Replace todo!() stubs with real functionality"
    echo "3. Test decentralized identity workflows"
else
    echo "1. Final testing and optimization"
    echo "2. Performance benchmarking"
    echo "3. Security audit preparation"
fi

echo ""
echo "🎭 MOCK REDUCTION PRIORITY GUIDANCE:"
echo "===================================="
echo "High Priority (Production Systems):"
echo "• Layer2 Protocol Adapters ($layer2_mocks mocks) - Replace NoopAdapter with real protocol communication"
echo "• ML/AI Services ($ml_mocks mocks) - Implement real model inference instead of MockMLService"
echo ""
echo "Medium Priority (Enterprise Features):"
echo "• HSM Providers ($hsm_mocks mocks) - Add hardware HSM integration for enterprise customers"
echo "• Network Clients ($network_mocks mocks) - Replace mock networking with real implementations"
echo ""
echo "Acceptable (Test Infrastructure):"
echo "• Test mocks are appropriate and should be maintained for testing"
echo "• Mock implementations in oracle/network layers are acceptable for MVP"

echo ""
echo "⚖️ ENFORCEMENT REMINDER:"
echo "========================"
echo "• No '100% complete' claims without unimplemented!() verification"
echo "• All documentation must include verification command evidence"
echo "• Progress tracked by macro reduction, not aspirational statements"
echo "• This script must be run before any major status updates"

# JSON summary (optional)
if [ "$EMIT_JSON" -eq 1 ]; then
    jq -n \
        --arg network "$NETWORK_MODE" \
        --arg compilation "$([ $? -eq 0 ] && echo PASS || echo FAIL)" \
        --argjson test_fail "$TEST_FAIL" \
        --argjson sim_fail "$sim_fail" \
        --argjson unimpl "$unimpl_count" \
        --argjson todos "$todo_count" \
        --argjson sqlite_todos "$sqlite_todo_count" \
        --argjson warnings "$(cargo check --all-features 2>&1 | grep -c 'warning:' || echo 0)" \
        '{network: $network, test_fail: $test_fail, sim_fail: $sim_fail, unimplemented: $unimpl, todos: $todos, sqlite_todos: $sqlite_todos, warnings: $warnings}' || true
fi

# Exit rules
if [ "$YES_ALL" -eq 1 ]; then
    exit 0
fi
if [ $sim_fail -eq 1 ] || [ $OVERALL_FAIL -eq 1 ]; then
    exit 2
fi
exit 0
