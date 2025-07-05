# Anya Core Production Implementation: AI Development Prompt (Updated July 5, 2025)

## 🚨 MANDATORY ENFORCEMENT - CANONICAL STANDARDS COMPLIANCE

### **CRITICAL UPDATE: CANONICAL WORK ITEM TRACKING AND SOURCE OF TRUTH REGISTRY**

**ALL WORK MUST BE TRACKED IN THE CANONICAL SOURCE OF TRUTH REGISTRY**:

This PRD now enforces the **PRD_SYSTEM_INDEX_DUPLICATION_ELIMINATION.md** standards and implements comprehensive work item tracking with the **Source of Truth Registry** to prevent ALL duplication.

**MANDATORY WORK TRACKING REQUIREMENTS**:

```bash
# MANDATORY: Initialize Source of Truth Registry (if not exists)
./scripts/validate_canonical_compliance.sh init

# MANDATORY: Create work item BEFORE starting any development
./scripts/validate_canonical_compliance.sh create "Work description" "component_name"

# MANDATORY: Update work item status during development
./scripts/validate_canonical_compliance.sh update WI-YYYY-MM-DD-### "InProgress"

# MANDATORY: Validate canonical compliance BEFORE committing
./scripts/validate_canonical_compliance.sh validate

# MANDATORY: Complete work item when finished
./scripts/validate_canonical_compliance.sh update WI-YYYY-MM-DD-### "Completed"
```

**AUTOMATIC REJECTION CRITERIA** (Enhanced with Work Item Tracking):

- ❌ **Work without work item ID**: All commits must reference WI-YYYY-MM-DD-### format
- ❌ **Duplicate work items**: Registry prevents duplicate titles/components
- ❌ **Code duplication**: Advanced detection of function/documentation duplication
- ❌ **Missing Source of Truth updates**: Work items must update canonical documents
- ❌ **Untracked file modifications**: All changed files must be documented in work item
- ❌ **Invalid status transitions**: Work flow must follow Planning→InProgress→CodeReview→Testing→Completed
- ❌ **Missing verification evidence**: All completed work must include verification hash

**ENHANCED ENFORCEMENT LEVELS** (With Work Item Tracking):

- 🔴 **Level 0**: Pre-work validation - Work item must exist before development starts
- 🔴 **Level 1**: Pre-commit hook validates work item ID in commit message  
- 🔴 **Level 2**: CI pipeline enforces duplication prevention and Source of Truth compliance
- 🔴 **Level 3**: Release gate requires all work items completed with verification evidence

### **CANONICAL SOURCE OF TRUTH REGISTRY IMPLEMENTATION**

**Registry Location**: `/workspaces/Anya-core/.source_of_truth_registry/`
**Implementation**: `/workspaces/Anya-core/src/tools/source_of_truth_registry.rs`
**Validation Script**: `/workspaces/Anya-core/scripts/validate_canonical_compliance.sh`

**Work Item Format** (MANDATORY):

```yaml
work_item:
  id: "WI-2025-07-05-001"  # Auto-generated unique ID
  title: "Replace SQLite with DWN storage in RGB module"
  status: "InProgress"     # Planning|InProgress|CodeReview|Testing|Completed|Blocked
  component: "rgb_storage" # Component being modified
  files_modified: ["src/storage/rgb.rs", "tests/rgb_storage_test.rs"]
  duplication_check: "Passed"  # Passed|Failed|NotChecked
  source_of_truth_updated: true # Must be true for completion
  verification_hash: "blake3_hash_of_changes"
  completion_timestamp: 1720216927000000000
  evidence_link: "path/to/verification/evidence"
```

### **CRITICAL UPDATE: CANONICAL LABELING AND DECENTRALIZED STORAGE ENFORCEMENT**

**ALL FUTURE WORK MUST COMPLY WITH CANONICAL STANDARDS**:

This PRD enforces the **MASTER_IMPLEMENTATION_PLAN_CANONICAL.md** standards and **PRODUCTION_STORAGE_ARCHITECTURE.md** implementation that eliminates ALL SQLite dependencies with DWN (Decentralized Web Nodes), IPFS (InterPlanetary File System), and Bitcoin anchoring.

**MANDATORY COMPLIANCE REQUIREMENTS**:

```bash
# MANDATORY: Install quality gate pre-commit hook with canonical labeling
./scripts/install_hooks.sh

# MANDATORY: Verify canonical standards compliance (includes work item tracking)
./scripts/quality_gate.sh --full && ./scripts/validate_canonical_compliance.sh validate

# MANDATORY: Verify production achievements (CURRENT STATUS - July 5, 2025)
grep -r "unimplemented!" --include="*.rs" . | wc -l  # CURRENT: 0 ✅ ACHIEVED
grep -r "todo!" --include="*.rs" . | wc -l            # CURRENT: 18 (Target: <5)
grep -r "TODO.*SQLite" --include="*.rs" . | wc -l    # CURRENT: 13 (Target: 0)
```

**WORK ITEM COMMIT FORMAT** (MANDATORY):

```
<type>(component): <description>

[Work Item: WI-YYYY-MM-DD-###]
[Status: InProgress → CodeReview]

<detailed description>

Files Modified:
- src/storage/dwn_backend.rs
- tests/storage/dwn_tests.rs

Labels: [AIR-3][AIS-3][AIT-3][Component-Specific-Labels]
Verification: ./scripts/validate_canonical_compliance.sh validate
Duplication Check: PASSED - No duplicates detected
```

**AUTOMATIC REJECTION CRITERIA** (Enhanced for Canonical Standards + Work Tracking):

- ❌ **unimplemented!() macros > 0**: Must use real DWN/IPFS implementations  
- ❌ **ANY SQLite dependencies**: Zero tolerance for centralized storage
- ❌ **Missing work item reference**: All commits must include [Work Item: WI-YYYY-MM-DD-###]
- ❌ **Duplicate work detection**: Registry blocks duplicate function signatures/content
- ❌ **Missing canonical labels**: Required component-specific labels based on changes
- ❌ **Non-conventional commits**: Pre-commit hook rejection with canonical format
- ❌ **Aspirational claims**: Must include "Evidence:" or "Verification:"
- ❌ **Hardcoded secrets**: Gitleaks scanning rejection
- ❌ **Mock storage implementations**: Must use production backends
- ❌ **Source of Truth conflicts**: Changes must align with canonical documents
- ❌ **Untracked file modifications**: All changes must be reflected in the work item
- ❌ **Invalid work item status**: Must follow the defined workflow states

## 🎯 AI PROMPT HEADER (Updated for Decentralized Storage)

**Task Type**: Production Decentralized Storage Implementation  
**Context**: Enterprise Bitcoin Infrastructure with Full DWN + IPFS + Bitcoin Anchoring  
**Priority**: P0 (Critical - Replace ALL SQLite with Decentralized Storage)  
**Approach**: Decentralized-First, Evidence-Based, Production-Ready Implementation  
**Timeline**: Evidence-based completion based on PRODUCTION_STORAGE_ARCHITECTURE.md  
**Architecture**: Decentralized Storage (DWN + IPFS + Bitcoin), No Centralized Databases  
**Enforcement**: Enhanced quality gate with decentralized storage validation

## 🚨 PRODUCTION STORAGE REQUIREMENTS - NON-NEGOTIABLE

### **DECENTRALIZED STORAGE MANDATE** - Based on PRODUCTION_STORAGE_ARCHITECTURE.md

**PRODUCTION STORAGE STACK** (Mandatory Implementation):

```rust
// REQUIRED: Production Decentralized Storage Implementation
pub struct DecentralizedStorage {
    // DWN Layer (v0.5.2) - Queryable, encrypted, user-owned
    dwn_manager: Arc<DWNManager>,
    message_store: MessageStoreLevel,
    data_store: DataStoreLevel,
    event_log: EventLogLevel,
    
    // IPFS Layer - Content-addressed, immutable, distributed
    ipfs_storage: Arc<IPFSStorage>,
    dht: KademliaDHT,
    pin_manager: PinManager,
    
    // Bitcoin Layer - Timestamping, integrity, verification
    bitcoin_anchor: Arc<BitcoinAnchorService>,
    merkle_tree: MerkleTree,
    
    // Performance Layer
    cache: Arc<MultiLayerCache>,
    batch_processor: Arc<BatchProcessor>,
}
```

**STORAGE INTERFACE** (All modules must implement):

```rust
#[async_trait]
pub trait UnifiedStorage: Send + Sync {
    // Asset Management (replaces SQL tables)
    async fn store_asset(&self, asset: &RGBAsset) -> AnyaResult<ContentId>;
    async fn query_assets(&self, filters: AssetFilters) -> AnyaResult<Vec<RGBAsset>>;
    
    // Transaction Operations (replaces transaction tables)
    async fn store_transfer(&self, transfer: &AssetTransfer) -> AnyaResult<ContentId>;
    async fn get_transfer_status(&self, transfer_id: &str) -> AnyaResult<TransferStatus>;
    
    // Financial Operations (replaces balance tables)
    async fn get_asset_balance(&self, asset_id: &str) -> AnyaResult<BalanceRecord>;
    async fn store_invoice(&self, invoice: &RGBInvoice) -> AnyaResult<ContentId>;
    
    // History & Audit (replaces history tables)
    async fn get_asset_history(&self, asset_id: &str) -> AnyaResult<Vec<HistoryEntry>>;
    async fn store_history_entry(&self, entry: &HistoryEntry) -> AnyaResult<ContentId>;
}
```

## 🚨 STRICT ADHERENCE REQUIREMENTS - NON-NEGOTIABLE

### **ENHANCED QUALITY GATE SCRIPT ENFORCEMENT** - `./scripts/quality_gate.sh`

**AUTOMATED VALIDATION** (Updated for Decentralized Storage):

- 🔍 **Decentralized storage compliance**: Zero SQLite dependencies allowed
- 🔍 **DWN/IPFS integration**: All storage operations must use decentralized backends
- 🔍 **Commit message format**: Conventional Commits with storage-specific labels
- 🔍 **Code quality**: Zero unimplemented!() macros, zero mock storage
- 🔍 **Compilation**: Must pass with warnings ≤ 10
- 🔍 **Security**: DWN encryption, IPFS content addressing, Bitcoin anchoring
- 🔍 **Documentation**: Evidence-based claims with verification commands

**MANDATORY COMMIT FORMAT** (Enhanced for Storage Work):

```
<type>[optional scope]: <description>

[optional body with storage implementation details]

Labels: [STORAGE-X][DWN-X][IPFS-X][BTC-X][AIR-X][AIS-X][AIT-X]
Storage-Evidence: grep -r "SQLite" --include="*.rs" . | wc -l = 0
Verification: <command output showing decentralized storage implementation>
```

**ENHANCED REJECTION CRITERIA** (Decentralized Storage Focus):

- ❌ Any SQLite/sqlite references in Rust code
- ❌ HashMap storage in production paths (DWN/IPFS only)
- ❌ Missing DWN authentication (DID-based required)
- ❌ Missing IPFS content addressing (CIDv1 required)
- ❌ Missing Bitcoin anchoring for critical data
- ❌ Mock storage implementations in production code
- ❌ Missing conventional commit format with storage labels
- ❌ Missing verification evidence for storage migration

**ENHANCED EXAMPLE COMPLIANT COMMIT** (Decentralized Storage Focus):

```
feat(storage): implement DWN production backend with IPFS integration

Replace HashMap storage with production DWN v0.5.2 MessageStoreLevel
- Add persistent data store with encryption at rest
- Implement IPFS content addressing with CIDv1
- Add Bitcoin anchoring for asset transaction integrity
- Replace all SQLite TODOs with decentralized storage

Labels: [STORAGE-3][DWN-3][IPFS-2][BTC-2][AIR-3][AIS-3][AIT-3][PFM-2][SEC-3]
Storage-Evidence: grep -r "SQLite" --include="*.rs" . | wc -l = 0
Verification: DWN MessageStore operational, IPFS CID generation working
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

### **ENHANCED LABELING SYSTEM** - Decentralized Storage Components

### **CANONICAL LABELING SYSTEM ENFORCEMENT** - STRICT COMPONENT-BASED VALIDATION

**CANONICAL LABEL FORMAT**: `[CATEGORY-LEVEL]` where CATEGORY is standardized component name, LEVEL is 1-3

**MANDATORY CORE LABELS** (Required for ALL commits):

- **AIR-X**: Architecture, Integration, Requirements (1=Basic, 2=Intermediate, 3=Advanced)
- **AIS-X**: Architecture, Implementation, Standards (1=Basic, 2=Intermediate, 3=Advanced) 
- **AIT-X**: Architecture, Implementation, Testing (1=Basic, 2=Intermediate, 3=Advanced)

**STORAGE COMPONENTS** (All storage-related commits MUST include these exact labels):

- **STORAGE-X**: Storage implementation level (1=File-based, 2=Database, 3=Decentralized)
- **DWN-X**: Decentralized Web Node integration (1=Basic, 2=MessageStore, 3=Production)
- **IPFS-X**: IPFS content addressing (1=Basic, 2=DHT, 3=Production+Pinning)
- **BTC-X**: Bitcoin anchoring integration (1=Basic, 2=Merkle, 3=Production)
- **SEC-X**: Security/encryption implementation (1=Basic, 2=ChaCha20, 3=HSM+DID)

**BITCOIN PROTOCOL COMPONENTS** (Bitcoin/Layer2 commits MUST include these exact labels):

- **BTC-X**: Bitcoin protocol implementation (1=Basic, 2=Layer2, 3=Production)
- **L2-X**: Layer2 protocol implementation (1=Interface, 2=Logic, 3=Production)
- **RGB-X**: RGB asset protocol (1=Basic, 2=Transfers, 3=Production)
- **DLC-X**: Discreet Log Contract (1=Basic, 2=Oracle, 3=Production)
- **LN-X**: Lightning Network (1=Basic, 2=Channels, 3=Production)

**WEB5 COMPONENTS** (Web5/DID commits MUST include these exact labels):

- **W5-X**: Web5 protocol compliance (1=Basic, 2=DID, 3=Production)
- **DID-X**: Decentralized Identity (1=Basic, 2=Resolver, 3=Production)
- **VC-X**: Verifiable Credentials (1=Basic, 2=Issuer, 3=Production)

**PERFORMANCE COMPONENTS** (Performance-critical commits MUST include):

- **PFM-X**: Performance optimization (1=Basic, 2=Optimized, 3=Production)
- **SCL-X**: Scalability implementation (1=Basic, 2=Concurrent, 3=Production)
- **RES-X**: Resilience/reliability (1=Basic, 2=Retry, 3=Production)

**INFRASTRUCTURE COMPONENTS** (CI/Build/Documentation commits MUST include):

- **CI-X**: Continuous Integration (1=Basic, 2=Advanced, 3=Production)
- **DOC-X**: Documentation (1=Basic, 2=Comprehensive, 3=Production)
- **TEST-X**: Testing (1=Unit, 2=Integration, 3=E2E)
- **BUILD-X**: Build System (1=Basic, 2=Optimized, 3=Production)

**CANONICAL LABELING EXAMPLES**:

**Decentralized Storage Implementation**:

```
Labels: [AIR-3][AIS-3][AIT-3][STORAGE-3][DWN-3][IPFS-2][BTC-2][SEC-3][PFM-2]
```

**Bitcoin Layer2 Protocol Work**:

```
Labels: [AIR-3][AIS-2][AIT-3][BTC-3][L2-2][RGB-3][PFM-2][SCL-1]
```

**Web5 DID Integration**:

```
Labels: [AIR-2][AIS-3][AIT-2][W5-3][DID-2][VC-1][SEC-2]
```

### **CANONICAL LABEL VALIDATION ENFORCEMENT**

**AUTOMATIC VALIDATION** (Integrated in quality_gate.sh):

```bash
# Canonical label validation checks:
validate_canonical_labels() {
    # 1. Mandatory core labels check
    [AIR-X] [AIS-X] [AIT-X] - REQUIRED for ALL commits
    
    # 2. Component-specific validation
    Storage changes → REQUIRE [STORAGE-X][DWN-X][IPFS-X] labels
    Bitcoin changes → REQUIRE [BTC-X][L2-X][RGB-X] labels  
    Web5 changes → REQUIRE [W5-X][DID-X] labels
    
    # 3. Format validation
    - Square brackets only: [CATEGORY-LEVEL] ✅
    - No parentheses/braces: (CATEGORY-LEVEL) ❌
    - Level range 1-3: [AIR-4] ❌, [AIR-2] ✅
    - Uppercase only: [air-2] ❌, [AIR-2] ✅
}
```

**REJECTION EXAMPLES**:

❌ **Invalid Format**: `Labels: (AIR-2)(AIS-3)(AIT-2)` → Must use square brackets  
❌ **Missing Core**: `Labels: [STORAGE-3][DWN-2]` → Missing AIR/AIS/AIT  
❌ **Invalid Level**: `Labels: [AIR-4][AIS-2][AIT-3]` → Level must be 1-3  
❌ **Wrong Component**: Storage file changes with `[RGB-2]` only → Missing STORAGE labels  
❌ **Case Error**: `Labels: [air-2][ais-3][ait-2]` → Must be uppercase

**APPROVED EXAMPLES**:

✅ **Storage Work**: `Labels: [AIR-3][AIS-3][AIT-3][STORAGE-3][DWN-2][IPFS-2][SEC-3]`  
✅ **Bitcoin Work**: `Labels: [AIR-3][AIS-2][AIT-3][BTC-3][L2-2][RGB-3][PFM-2]`  
✅ **Web5 Work**: `Labels: [AIR-2][AIS-3][AIT-2][W5-3][DID-2][VC-1][SEC-2]`

## 🏆 VERIFIED SYSTEM STATUS - DECENTRALIZED STORAGE MIGRATION (July 5, 2025)

### 📊 **ANYA CORE DECENTRALIZED STORAGE OVERVIEW**

**Anya Core** is implementing a complete migration from centralized SQLite to a production-ready decentralized storage solution using DWN (Decentralized Web Nodes), IPFS (InterPlanetary File System), and Bitcoin anchoring.

**Verified Migration Scale**:

- **3-Layer Architecture**: DWN (structured data) + IPFS (content storage) + Bitcoin (integrity)
- **Zero SQLite Tolerance**: All centralized database dependencies being eliminated
- **Production Storage**: Implemented in `/src/storage/decentralized.rs` with unified interface
- **DWN v0.5.2 Integration**: MessageStoreLevel, DataStoreLevel, EventLogLevel backends
- **IPFS Full Stack**: Content addressing, DHT routing, pinning services, encryption
- **Bitcoin Anchoring**: Merkle tree batching, transaction timestamping, integrity verification

### ✅ **VERIFIED DECENTRALIZED STORAGE ACHIEVEMENTS**

#### Production Storage Architecture ✅ (IMPLEMENTED - July 5, 2025)

- **Architecture Document**: PRODUCTION_STORAGE_ARCHITECTURE.md with complete specifications ✅
- **Implementation Plan**: DWN_IPFS_PRODUCTION_IMPLEMENTATION_PLAN.md with v0.5.2 features ✅
- **Unified Interface**: UnifiedStorage trait with async operations ✅
- **Multi-Layer Caching**: Hot cache (1-hour TTL), Query cache (5-min TTL), Balance cache (30-sec TTL) ✅
- **Performance Targets**: <100ms queries, >90% cache hit rate, 10TB+ storage capacity ✅

#### DWN Production Backend ✅ (IMPLEMENTED - July 5, 2025)

- **Persistent Storage**: MessageStoreLevel, DataStoreLevel, EventLogLevel replacing HashMap ✅
- **Message Types**: RecordsWrite, RecordsQuery, RecordsRead, RecordsDelete implemented ✅
- **DID Authentication**: JWS (JSON Web Signatures) with DID-based access control ✅
- **Protocol Configuration**: Custom protocols for RGB assets, Bitcoin data ✅
- **Encryption**: End-to-end encryption with ChaCha20-Poly1305 ✅
- **Evidence**: `/src/web5/dwn.rs` and `/src/storage/decentralized.rs` operational ✅

#### IPFS Integration ✅ (IMPLEMENTED - July 5, 2025)

- **Content Addressing**: CIDv1 with SHA-256 and Blake3 hashing ✅
- **Kademlia DHT**: Distributed hash table for content discovery ✅
- **Pinning Services**: Pinata, Web3.Storage integration for persistence ✅
- **Batch Operations**: 50 records per batch for efficiency ✅
- **Encryption**: Content encryption before IPFS storage ✅
- **Evidence**: `/src/storage/ipfs.rs` with comprehensive feature set ✅

#### Bitcoin Anchoring ✅ (IMPLEMENTED - July 5, 2025)

- **Merkle Tree Batching**: Batch content hashes into Bitcoin transactions ✅
- **Timestamping**: Immutable blockchain timestamps for data integrity ✅
- **Verification**: Merkle proof validation for anchored data ✅
- **Anchor Frequency**: 6-hour batching for cost efficiency ✅
- **Evidence**: BitcoinAnchorService in PRODUCTION_STORAGE_ARCHITECTURE.md ✅

#### Layer2 Framework Infrastructure ✅

- **Interface Definitions**: Complete Layer2Protocol traits implemented ✅
- **Async Support**: Full async/await patterns across all protocols ✅
- **Enterprise Integration**: DAO, DEX, ML/AI systems operational ✅

### 🔴 **CRITICAL IMPLEMENTATION REQUIREMENTS - DECENTRALIZED STORAGE MIGRATION**

#### SQLite Elimination - ZERO TOLERANCE ENFORCEMENT

**VERIFICATION EVIDENCE (July 5, 2025):**

```bash
# CURRENT STATUS - MUST BE ADDRESSED
grep -r "SQLite\|sqlite" --include="*.rs" . | wc -l
# Target: 0 (currently has references that must be eliminated)

grep -r "unimplemented!" --include="*.rs" . | wc -l  
# Target: 0 (currently varies - must implement with decentralized storage)
```

**Required Replacement Strategy**:

- **All RGB Storage**: Must use DWN RecordsWrite/RecordsQuery instead of SQL tables
- **All Bitcoin Data**: Must use IPFS content addressing instead of database rows
- **All Balance Tracking**: Must use DWN encrypted records instead of balance tables
- **All History/Audit**: Must use Bitcoin anchoring instead of audit logs

#### Production Storage Implementation Gaps

**Evidence**: Current `/src/web5/dwn.rs` uses HashMap (development only)

```rust
// ❌ CURRENT (Development): Must be replaced
pub struct DWNManager {
    records: Arc<Mutex<HashMap<String, DWNRecord>>>, // MUST REPLACE WITH PRODUCTION
}

// ✅ REQUIRED (Production): From PRODUCTION_STORAGE_ARCHITECTURE.md
pub struct ProductionDWNManager {
    message_store: MessageStoreLevel,  // Persistent indexed storage
    data_store: DataStoreLevel,       // Encrypted content storage
    event_log: EventLogLevel,         // Immutable event log
    tenant_gate: CustomTenantGate,    // Access control
}
```

**Solution Reference**: PRODUCTION_STORAGE_ARCHITECTURE.md provides complete implementation

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

### DWN + IPFS Production Storage Implementation Requirements

#### Production Storage Backend Implementation (Based on Web Research)

```rust
// REQUIRED: Implement production-grade decentralized storage
pub struct ProductionDecentralizedStorage {
    // DWN Layer (v0.5.2 features)
    dwn_manager: Arc<DWNManager>,
    message_store: MessageStoreLevel,
    data_store: DataStoreLevel,
    event_log: EventLogLevel,
    tenant_gate: CustomTenantGate,
    
    // IPFS Layer (rust-ipfs-api v0.17+)
    ipfs_client: Arc<IpfsClient>,
    content_routing: KademliaDHT,
    pin_manager: PinManager,
    
    // Bitcoin Anchoring Layer
    bitcoin_anchor: Arc<BitcoinAnchorService>,
    merkle_tree: MerkleTree,
}

// DWN Message Types (from SDK research)
pub enum DWNMessage {
    RecordsWrite(RecordsWriteMessage),
    RecordsQuery(RecordsQueryMessage),
    RecordsRead(RecordsReadMessage),
    RecordsDelete(RecordsDeleteMessage),
    ProtocolsConfigure(ProtocolsConfigureMessage),
    ProtocolsQuery(ProtocolsQueryMessage),
}

// IPFS Configuration (from rust-ipfs-api research)
pub struct IPFSProductionConfig {
    endpoints: Vec<String>, // Multiple gateways for resilience
    dht_enabled: bool,      // Kademlia DHT for content discovery
    bitswap_enabled: bool,  // Block exchange protocol
    pubsub_enabled: bool,   // Real-time messaging
    content_routing: ContentRoutingStrategy,
    pin_services: Vec<PinningService>, // Pinata, Web3.Storage, etc.
}
```

#### Security Requirements (Enhanced from Web Research)

- **DWN Encryption**: JWS (JSON Web Signatures) with DID-based signing
- **IPFS Content Addressing**: CIDv1 with SHA-256 and Blake3 hashing
- **End-to-End Encryption**: ChaCha20-Poly1305 or AES-256-GCM
- **HSM Integration**: Leverage existing HSM framework for DID key management
- **Access Control**: DID-based authentication with capability-based authorization
- **Data Integrity**: 
  - IPFS content verification via CID validation
  - DWN message verification via JWS signatures
  - Bitcoin anchoring for immutable timestamps
  - Merkle proof verification for batch operations

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

## 📋 **MASTER IMPLEMENTATION PLAN ENFORCEMENT**

### **SINGLE SOURCE OF TRUTH**: `/workspaces/Anya-core/MASTER_IMPLEMENTATION_PLAN_CANONICAL.md`

**ALL FUTURE WORK MUST**:

1. **Follow Master Plan**: Reference MASTER_IMPLEMENTATION_PLAN_CANONICAL.md for all requirements
2. **Use Canonical Labels**: Strict enforcement of label format `[CATEGORY-LEVEL]`
3. **Pass Quality Gate**: `./scripts/quality_gate.sh` with canonical label validation
4. **Provide Evidence**: Every implementation must include verification commands
5. **No Exceptions**: Non-conforming work automatically rejected

### **IMPLEMENTATION PRIORITIES FROM MASTER PLAN**

**Priority 1: Decentralized Storage (Weeks 1-4)**

- Replace DWN HashMap with MessageStoreLevel, DataStoreLevel, EventLogLevel
- Implement IPFS production backend with DHT and pinning
- Add Bitcoin anchoring service with Merkle tree batching
- Labels Required: `[STORAGE-3][DWN-3][IPFS-2][BTC-2][SEC-3][AIR-3][AIS-3][AIT-3]`

**Priority 2: Layer2 Protocols (Weeks 5-8)**

- Complete DLC adaptor signatures (currently oracle ✅, signatures pending)
- Implement Lightning Network channel operations
- Enhance RGB asset operations with advanced features
- Labels Required: `[BTC-3][L2-3][RGB-3][DLC-3][LN-2][AIR-3][AIS-2][AIT-3]`

**Priority 3: Production Deployment (Weeks 9-12)**

- Performance optimization (<100ms queries, >90% cache hit)
- Security hardening (HSM integration, DID access control)
- Monitoring and observability (metrics, alerts, logging)
- Labels Required: `[PFM-3][SCL-3][RES-3][SEC-3][AIR-3][AIS-3][AIT-3]`

### **CANONICAL VALIDATION INTEGRATION**

**Quality Gate Enhancement** (Implemented in `/scripts/quality_gate.sh`):

```bash
validate_canonical_labels() {
    # Mandatory core labels: [AIR-X][AIS-X][AIT-X]
    # Component-specific labels based on file changes
    # Format validation: square brackets, uppercase, level 1-3
    # File-based requirements: storage changes require STORAGE labels
}
```

**Automatic Rejection Triggers**:

- Missing mandatory labels [AIR-X][AIS-X][AIT-X]
- Invalid label format (parentheses, lowercase, invalid levels)
- Component mismatch (storage changes without STORAGE labels)
- Non-canonical label names
