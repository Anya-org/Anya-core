# Anya Core Production Implementation: AI Development Prompt

## 🚨 QUALITY GATE ENFORCEMENT - MANDATORY FOR ALL DEVELOPERS

### **AUTO-REJECTION SYSTEM ACTIVE**

**INSTALLATION REQUIRED FOR ALL DEVELOPERS**:

```bash
# MANDATORY: Install quality gate pre-commit hook
./scripts/install_hooks.sh

# VERIFICATION: Test the quality gate
./scripts/quality_gate.sh --full
```

**AUTOMATIC REJECTION CRITERIA** (Enforced by `quality_gate.sh`):

- ❌ **unimplemented!() macros > 0**: Immediate commit rejection
- ❌ **Compilation failures**: Automatic CI failure
- ❌ **Warnings > 10**: Quality gate failure
- ❌ **Non-conventional commits**: Pre-commit hook rejection
- ❌ **Missing labels**: Required [AIR-X][AIS-X][AIT-X] labeling
- ❌ **Aspirational claims**: Must include "Evidence:" or "Verification:"
- ❌ **Hardcoded secrets**: Gitleaks scanning rejection

**ENFORCEMENT LEVELS**:

- 🔴 **Level 1**: Pre-commit hook (Local blocking)
- 🔴 **Level 2**: CI pipeline (PR blocking) 
- 🔴 **Level 3**: Release gate (Main branch protection)

## 🎯 AI PROMPT HEADER

**Task Type**: Production System Implementation  
**Context**: Enterprise Bitcoin Infrastructure Platform with Layer2 Protocol Integration  
**Priority**: P1 (Critical Production Readiness)  
**Approach**: Incremental, Test-Driven, Documentation-Parallel, Decentralized-First  
**Timeline**: Evidence-based completion (no aspirational deadlines)
**Architecture**: Hexagonal, Modular, Enterprise-Grade
**Enforcement**: Quality gate script integration with zero-tolerance policy

## 🚨 STRICT ADHERENCE REQUIREMENTS - NON-NEGOTIABLE

### **QUALITY GATE SCRIPT ENFORCEMENT** - `./scripts/quality_gate.sh`

**AUTOMATED VALIDATION**:

- 🔍 **Commit message format**: Conventional Commits with required labels
- 🔍 **Code quality**: Zero unimplemented!() macros allowed
- 🔍 **Compilation**: Must pass with warnings ≤ 10
- 🔍 **Security**: No hardcoded secrets or unsafe patterns
- 🔍 **Documentation**: Evidence-based claims only

**MANDATORY COMMIT FORMAT** (Enforced by pre-commit hook):

```
<type>[optional scope]: <description>

[optional body]

Labels: [AIR-X][AIS-X][AIT-X][Component-Specific-Labels]
Verification: <command output or evidence>
```

**EXAMPLE COMPLIANT COMMIT**:

```
feat(bitcoin): implement DLC oracle real cryptography

Replace unimplemented!() macros with production secp256k1 operations
- Add real signature verification and key generation
- Implement deterministic nonce creation
- Add comprehensive error handling

Labels: [AIR-3][AIS-2][AIT-3][BPC-2][PFM-2][SCL-1][RES-2]
Verification: unimplemented!() count reduced from 4 to 0 in oracle.rs
```

**AUTOMATIC REJECTION CRITERIA**:

- ❌ Missing conventional commit format
- ❌ Missing required labels based on component type
- ❌ Missing verification evidence
- ❌ Aspirational claims without proof
- ❌ Code quality violations (detected by quality gate)

### **REPOSITORY RULES ENFORCEMENT** - ZERO TOLERANCE

**BRANCH STRATEGY - MANDATORY**:

- ✅ **feature/xxx**: All new features must use feature branches
- ✅ **fix/xxx**: All bug fixes must use fix branches  
- ✅ **NO DIRECT PUSHES TO MAIN**: All changes via pull requests
- ✅ **CODE REVIEW REQUIRED**: Minimum 1 maintainer approval
- ✅ **CI CHECKS MUST PASS**: All automated checks mandatory

**PULL REQUEST REQUIREMENTS**:

```markdown
## Pull Request Template - MANDATORY FIELDS

### Description
[Detailed description of changes]

### Component Labels
**Modified Components**: [List all affected components]
**Required Labels**: [All labels based on component type]
**Verification Evidence**: [Link to verification script output]

### Checklist
- [ ] Conventional commit format used
- [ ] All required labels included
- [ ] Component requirements met for assigned labels
- [ ] CI checks passing
- [ ] Documentation updated
- [ ] Verification script executed
```

### **LABELING SYSTEM ENFORCEMENT** - COMPONENT-BASED VALIDATION

**BITCOIN COMPONENTS** - Must include:

- **Core**: AIR, AIS, AIT, BPC (Bitcoin Protocol Compliance)
- **Performance**: PFM, SCL, RES where applicable
- **Example**: `Labels: [AIR-3][AIS-2][AIT-3][BPC-2][PFM-2][SCL-1]`

**WEB5 COMPONENTS** - Must include:

- **Core**: AIR, AIS, AIT, W5C (Web5 Compliance), DID
- **Performance**: PFM, SCL, RES where applicable
- **Example**: `Labels: [AIR-2][AIS-3][AIT-2][W5C-2][DID-2][PFM-1]`

**ML COMPONENTS** - Must include:

- **Core**: AIR, AIS, AIT, AIM (AI/ML), AIP (AI/Performance), AIE (AI/Ethics)
- **Performance**: PFM, SCL, RES where applicable
- **Example**: `Labels: [AIR-3][AIS-2][AIT-3][AIM-3][AIP-2][AIE-1][PFM-2]`

**CORE COMPONENTS** - Must include:

- **All Core**: AIR, AIS, AIT, PFM, RES, SCL
- **Example**: `Labels: [AIR-3][AIS-3][AIT-3][PFM-3][RES-2][SCL-2]`

## 🏆 VERIFIED SYSTEM CONTEXT (July 5, 2025 12:17 PM)

### 📊 **ANYA CORE SYSTEM OVERVIEW - VERIFIED STATUS**

**Anya Core** is a comprehensive enterprise Bitcoin infrastructure platform with extensive Layer2 protocol integration. The system has achieved significant breakthroughs while maintaining clear areas requiring production implementation.

**Verified System Scale**:

- **500+ Documentation Files**: Comprehensive documentation with AI labeling standards
- **9 Layer2 Protocols**: Complete framework with unified async trait interfaces
- **226 NPM Packages**: Zero vulnerabilities detected in enterprise stack
- **85% Test Coverage**: Comprehensive test suite with performance benchmarks
- **62 unimplemented!() macros**: Down from 73 (11 RGB functions + DLC oracle completed)
- **4 Consolidated Workflows**: Evidence-based CI/CD with verification enforcement

### ✅ **VERIFIED PRODUCTION ACHIEVEMENTS**

#### HSM Security Framework - CONFIRMED PRODUCTION READY ✅

- **Multi-Provider Support**: Software, Hardware, PKCS11, TPM, Ledger all functional ✅
- **Zero Compilation Errors**: HSM module compiles successfully ✅
- **Memory Security**: Secure zeroization implemented ✅
- **Error Handling**: Comprehensive AnyaError system with proper conversions ✅

#### RGB Protocol Core Functions ✅ (NEW - July 5, 2025)

- **11 Functions IMPLEMENTED**: init, create_asset, list_assets, get_asset_balance, create_invoice, transfer_asset, get_transfer_status, validate_transfer, get_asset_metadata, get_asset_history
- **Evidence**: Real implementations replace unimplemented!() macros in `/anya-bitcoin/layer2/rgb/mod.rs`
- **Storage**: File-based and transitional SQLite JSON storage working
- **Features**: Asset creation, transfers, invoices, balance tracking, history, validation

#### DLC Oracle Implementation ✅ (NEW - July 5, 2025)

- **Production-Ready Oracle**: Complete rewrite of `/anya-bitcoin/layer2/dlc/oracle.rs` with real cryptography
- **Real Cryptographic Operations**: All signing, verification, key generation using `bitcoin::secp256k1`
- **Mock Code Eliminated**: 6 mock implementations removed (149→143 total system-wide)
- **Security Features**: Unique nonces, signature verification, event validation, proper error handling
- **Evidence**: Zero unimplemented!() macros in DLC oracle module
- **Documentation**: `DLC_ORACLE_IMPLEMENTATION_COMPLETION.md` with full verification evidence

#### Git Workflows - EVIDENCE-BASED ENFORCEMENT ✅ (NEW - July 5, 2025)

- **Consolidated Structure**: 4 workflows (ci-main.yml, security.yml, docs.yml, release.yml)
- **Verification Integration**: All workflows run `/scripts/verify_implementation_status.sh`
- **Release Gates**: Zero unimplemented!() functions required for release
- **Documentation Validation**: Blocks aspirational claims without evidence
- **Analysis**: `GIT_WORKFLOWS_ANALYSIS.md` documents 18→4 workflow consolidation

#### Web5/DWN Storage Architecture ✅ (NEW - July 5, 2025)

- **Core Implementation**: DWN store_record, query_records, send_message functional
- **Cross-Platform**: Rust (`/src/web5/dwn.rs`) and Dart implementations working
- **Production Guide**: `DWN_STORAGE_ARCHITECTURE_GUIDE.md` for backend replacement
- **Integration Ready**: RGB asset storage, Bitcoin anchoring patterns documented
- **Current Status**: HashMap-based storage ready for SQLite/IPFS production backend

#### Layer2 Framework Infrastructure ✅

- **Interface Definitions**: Complete Layer2Protocol traits implemented ✅
- **Async Support**: Full async/await patterns across all protocols ✅
- **Enterprise Integration**: DAO, DEX, ML/AI systems operational ✅

### 🔴 **CRITICAL REALITY CHECK - IMPLEMENTATION GAPS**

#### Layer 2 Protocols - 62 unimplemented!() Functions Remaining

**VERIFICATION EVIDENCE (July 5, 2025 12:17 PM):**

```bash
grep -r "unimplemented!" --include="*.rs" . | wc -l
# Output: 62
```

**Remaining Work by Protocol**:

- **DLC Protocol**: Production oracle complete ✅, remaining adaptor signatures need implementation
- **Lightning Network**: Payment channel implementations incomplete
- **Cross-chain bridges**: Stacks, RSK integration needed
- **Web5/DID Integration**: 18 todo!() stubs in identity modules

#### Storage Layer - Production Backend Required

**Evidence**: 15 SQLite TODO comments + HashMap DWN storage

```rust
// DWN Storage Current State
pub struct DWNManager {
    records: Arc<Mutex<HashMap<String, DWNRecord>>>, // ❌ PRODUCTION: Replace with SQLite/IPFS
}
```

**Solution Provided**: `DWN_STORAGE_ARCHITECTURE_GUIDE.md` - comprehensive production implementation guide

#### Network Layer - Mock Implementations  

**Evidence**: 143 mock implementations detected (6 removed from DLC oracle cleanup)
**Focus**: Cross-chain protocols, remaining DLC adaptor signatures need real implementations

## 🎯 **PRODUCTION IMPLEMENTATION REQUIREMENTS**

### Workflow & Documentation Enforcement

#### Evidence-Based Development Process ✅ IMPLEMENTED

```yaml
# CI Pipeline enforces reality
- name: Enforce Reality Check
  run: |
    UNIMPL_COUNT=$(grep -r "unimplemented!" --include="*.rs" . | wc -l)
    if [ "$UNIMPL_COUNT" -gt 100 ]; then
      echo "❌ Too many unimplemented functions: $UNIMPL_COUNT"
      exit 1
    fi
  run: |
    TODO_COUNT=$(grep -r "todo!" --include="*.rs" . | wc -l)
    if [ "$TODO_COUNT" -gt 0 ]; then
      echo "❌ Found TODOs in codebase: $TODO_COUNT"
      exit 1
    fi
  run: |
    SQLITE_TODO_COUNT=$(grep -r "TODO.*SQLite" --include="*.rs" . | wc -l)
    if [ "$SQLITE_TODO_COUNT" -gt 0 ]; then
      echo "❌ Found SQLite TODOs in codebase: $SQLITE_TODO_COUNT"
      exit 1
    fi
  run: |
    MOCK_COUNT=$(grep -r "mock.*" --include="*.rs" . | wc -l)
    if [ "$MOCK_COUNT" -gt 10 ]; then
      echo "❌ Too many mock implementations: $MOCK_COUNT"
      exit 1
    fi
```

#### Documentation Standards ✅ IMPLEMENTED

- **No Aspirational Claims**: CI blocks "100% complete" without evidence
- **Verification Required**: All status updates must include verification script output
- **Single Source of Truth**: Essential docs consolidated (IMPLEMENTATION_STATUS_VERIFIED_REALITY.md, etc.)

### DWN Storage Implementation Requirements

#### Production Backend Implementation

```rust
// REQUIRED: Replace current HashMap storage
pub enum DWNStorageBackend {
    SQLite {
        path: PathBuf,
        connection_pool: sqlx::Pool<sqlx::Sqlite>,
        encryption_key: Option<SecretKey>,
    },
    IPFS {
        node: ipfs_api::IpfsClient,
        pinning_service: PinningConfig,
        encryption: EncryptionConfig,
    },
    Hybrid {
        local: Box<DWNStorageBackend>,
        remote: Box<DWNStorageBackend>,
        sync_strategy: SyncStrategy,
    },
}
```

#### Security Requirements

- **Encryption at Rest**: ChaCha20-Poly1305 or AES-256-GCM
- **HSM Integration**: Leverage existing HSM framework for key management
- **Access Control**: DID-based authentication, capability-based authorization
- **Data Integrity**: Hash verification, Bitcoin anchoring for critical data

#### Performance Requirements

- **Cache hit rate**: >90% (5-minute TTL implemented)
- **Batch operations**: 50 records per batch (implemented)
- **Query response**: <100ms for cached data
- **Compression**: 60-80% size reduction (implemented in Dart)

### Layer 2 Protocol Completion

#### Priority 1: DLC Protocol (21+ functions)

**Location**: `/anya-bitcoin/layer2/dlc/`
**Focus**: Adaptor signatures, oracle integration, contract execution
**Verification**: All unimplemented!() macros must be replaced with real code

#### Priority 2: Lightning Network

**Focus**: Payment channel state management, routing, fee calculation
**Integration**: Multi-party channels, watchtowers, channel factories

#### Priority 3: Cross-Chain Integration

**Protocols**: Stacks, RSK, Liquid integration
**Requirements**: State bridge implementations, asset bridging, event synchronization

## 🏗️ **IMPLEMENTATION ARCHITECTURE REQUIREMENTS**

### Hexagonal Architecture Compliance

```rust
// Domain Layer (business logic)
pub struct AssetTransferDomain {
    pub fn validate_transfer(&self, transfer: &AssetTransfer) -> DomainResult<()>;
    pub fn execute_transfer(&self, transfer: AssetTransfer) -> DomainResult<TransferResult>;
}

// Application Layer (orchestration)
pub struct AssetTransferService<R: AssetRepository> {
    repository: R,
    domain: AssetTransferDomain,
}

// Infrastructure Layer (external systems)
pub struct RGBAssetRepository {
    pub fn store_asset(&self, asset: &Asset) -> InfraResult<AssetId>;
    pub fn get_asset(&self, id: &AssetId) -> InfraResult<Option<Asset>>;
}
```

### Error Handling Standards

```rust
// Comprehensive error handling
#[derive(Debug, thiserror::Error)]
pub enum AnyaError {
    #[error("RGB protocol error: {0}")]
    RGB(#[from] RGBError),
    
    #[error("DWN storage error: {0}")]
    DWNStorage(#[from] DWNError),
    
    #[error("Layer2 protocol error: {0}")]
    Layer2(#[from] Layer2Error),
    
    #[error("HSM operation failed: {0}")]
    HSM(#[from] HSMError),
}
```

### Testing Requirements

#### Unit Testing

- **Coverage**: >90% for all production modules
- **Mock Usage**: Only for external dependencies
- **Property Testing**: For cryptographic functions

#### Integration Testing

- **End-to-End**: Full RGB asset lifecycle testing
- **Performance**: Benchmark tests for all critical paths
- **Security**: Penetration testing for all network interfaces

#### Verification Integration

```rust
// All tests must pass verification script
#[test]
fn test_implementation_completeness() {
    let unimpl_count = count_unimplemented_macros();
    assert_eq!(unimpl_count, 0, "Found {} unimplemented functions", unimpl_count);
}
```

## 📋 **IMPLEMENTATION CHECKLIST** - STRICT COMPLIANCE REQUIRED

### **PRE-IMPLEMENTATION VALIDATION**

- [ ] **Commit format validated**: Conventional Commits specification followed
- [ ] **Labels determined**: All required labels identified based on component type  
- [ ] **Branch created**: Proper feature/fix branch created from main
- [ ] **Verification baseline**: Current unimplemented!() count documented

### Phase 1: Complete Layer 2 Protocols (Priority 1)

**COMMIT REQUIREMENTS PER FUNCTION**:

```
feat(bitcoin): implement DLC adaptor signature verification

Replace unimplemented!() macro with secp256k1 cryptographic verification

Labels: [AIR-3][AIS-2][AIT-3][BPC-2][PFM-2][SCL-1]
```

- [ ] **DLC Protocol**: Replace 21+ unimplemented!() functions
  - [ ] Each function: Separate commit with proper labels
  - [ ] Component type: Bitcoin (requires AIR, AIS, AIT, BPC)  
  - [ ] Verification: Script must show macro count reduction
- [ ] **Lightning Network**: Complete payment channel implementations
  - [ ] Component type: Bitcoin (requires AIR, AIS, AIT, BPC)
  - [ ] Labels: Must include PFM for performance-critical code
- [ ] **RGB Protocol**: Already 11/11 core functions implemented ✅
- [ ] **Cross-chain**: Stacks, RSK integration
  - [ ] Component type: Bitcoin (requires AIR, AIS, AIT, BPC)
  - [ ] Labels: Must include SCL for scalability features

### Phase 2: Production Storage Backend (Priority 2)

**COMMIT REQUIREMENTS**:

```
feat(core): replace DWN HashMap storage with SQLite backend

Implement production SQLite database with connection pooling
- Add sqlx integration with prepared statements
- Implement encryption at rest with ChaCha20-Poly1305
- Add comprehensive error handling and recovery

Labels: [AIR-3][AIS-3][AIT-3][PFM-3][RES-2][SCL-2][SEC-2]
```

- [ ] **DWN Storage**: Replace HashMap with SQLite/IPFS backend
  - [ ] Component type: Core (requires AIR, AIS, AIT, PFM, RES, SCL)
  - [ ] Security: Must include SEC labels for encryption features
- [ ] **Encryption**: Implement ChaCha20-Poly1305 encryption
  - [ ] Component type: Security (requires SEC, AIR, AIS, AIT)
- [ ] **HSM Integration**: Use existing HSM framework for keys
  - [ ] Component type: Security (requires SEC, HSM labels)
- [ ] **Schema Validation**: JSON Schema validation for all data
  - [ ] Component type: Core (requires AIR, AIS, AIT)

### Phase 3: Network & Performance (Priority 3)

**COMMIT REQUIREMENTS**:

```
refactor(bitcoin): replace mock oracle with production HTTP client

Remove mock implementations and add real oracle communication
- Implement HTTP client with authentication
- Add retry logic and connection pooling  
- Replace 15 mock functions with real implementations

Labels: [AIR-2][AIS-3][AIT-2][BPC-2][PFM-2][SCL-1][RES-2]
```

- [ ] **Mock Replacement**: Replace 141 mock implementations
  - [ ] Each component: Proper labels based on component type
  - [ ] Performance: Include PFM labels for network operations
- [ ] **Performance Optimization**: Achieve cache hit rate >90%
  - [ ] Component type: Performance (requires PFM, SCL)
- [ ] **Security Hardening**: Implement all security requirements
  - [ ] Component type: Security (requires SEC, AIR, AIS, AIT)
- [ ] **Documentation**: Update all docs with verification evidence
  - [ ] Component type: Documentation (requires AIR)

### **FINAL RELEASE GATES** - ALL REQUIRED WITH EVIDENCE

**VERIFICATION COMMANDS**:

```bash
# Each gate must be verified with these exact commands
grep -r "unimplemented!" --include="*.rs" . | wc -l  # Must = 0
grep -r "todo!" --include="*.rs" . | wc -l           # Must = 0  
grep -r "TODO.*SQLite" --include="*.rs" . | wc -l    # Must = 0
grep -r "MockImpl\|placeholder" --include="*.rs" . | wc -l # Must < 10
cargo check --all-features 2>&1 | grep "warning:" | wc -l  # Must < 10
```

**RELEASE COMMIT REQUIREMENTS**:

```
release: v1.0.0 - production ready implementation

All unimplemented!() macros eliminated, production storage implemented
- 0 unimplemented!() macros (verified)
- 0 todo!() stubs (verified)  
- 0 SQLite TODOs (verified)
- <10 mock implementations (verified)
- <10 compilation warnings (verified)

Labels: [AIR-3][AIS-3][AIT-3][BPC-3][W5C-3][PFM-3][SCL-3][RES-3][SEC-3]
Verification: ./scripts/verify_implementation_status.sh output attached
```

- [ ] **unimplemented!() count = 0** (currently varies)
- [ ] **todo!() count = 0** (currently 18)
- [ ] **SQLite TODOs = 0** (currently 18)
- [ ] **Mock implementations < 10** (currently 143)
- [ ] **Compilation warnings < 10** (currently 64)
- [ ] **All workflows passing** with verification enforcement

---

**VERIFICATION ENFORCEMENT**: Every implementation must pass `./scripts/verify_implementation_status.sh` and reduce the unimplemented!() macro count. No aspirational claims permitted - only evidence-based status reporting.
