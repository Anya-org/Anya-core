# PRD: Canonical System Index Auto-Update, Checkin Work Tracking & Duplication Elimination

## Document Information

- **Date**: July 5, 2025
- **Version**: 3.0.0 (CANONICAL SOURCE OF TRUTH)
- **Status**: MANDATORY ENFORCEMENT
- **Purpose**: Canonical PRD for auto-updating system indexes, checkin work tracking, and eliminating all duplication
- **Authority**: SINGLE SOURCE OF TRUTH for all duplication prevention and work tracking

## Executive Summary

This PRD mandates the implementation of auto-updating system indexes, comprehensive checkin work tracking, and zero-tolerance duplication elimination across the entire Anya Core codebase. All future development must adhere to these canonical standards with automatic enforcement and real-time work progress tracking.

## 🚨 CRITICAL: CHECKIN WORK TRACKING REQUIREMENTS

### Checkin Work Documentation Standards

**ALL WORK MUST BE TRACKED IN CANONICAL FORMAT**:

```yaml
# CANONICAL CHECKIN FORMAT
work_item:
  id: "WI-{YYYY-MM-DD}-{sequential_number}"
  title: "{Brief description}"
  status: "{Planning|InProgress|CodeReview|Testing|Completed|Blocked}"
  component: "{Component affected}"
  files_modified: ["{list of all files}"]
  duplication_check: "{PASSED|FAILED}"
  source_of_truth_updated: "{YES|NO}"
  verification_hash: "{blake3_hash_of_changes}"
  completion_timestamp: "{ISO8601}"
  evidence_link: "{path to verification evidence}"
```

### Source of Truth Registry

**MANDATORY: All work items must update the central Source of Truth Registry**:

```rust
// CANONICAL: Source of Truth Registry Implementation
pub struct SourceOfTruthRegistry {
    /// Registry of all canonical documents
    canonical_documents: DashMap<String, CanonicalDocument>,
    /// Work item tracking
    work_items: DashMap<String, WorkItem>,
    /// Duplication prevention index
    duplication_index: DashMap<String, DuplicationEntry>,
    /// Last registry update
    last_updated: AtomicU64,
    /// Registry version
    version: AtomicU32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkItem {
    pub id: String,
    pub title: String,
    pub status: WorkStatus,
    pub component: String,
    pub files_modified: Vec<String>,
    pub duplication_check: DuplicationCheckStatus,
    pub source_of_truth_updated: bool,
    pub verification_hash: [u8; 32],
    pub completion_timestamp: Option<u64>,
    pub evidence_link: String,
    pub dependencies: Vec<String>,
    pub blockers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkStatus {
    Planning,
    InProgress, 
    CodeReview,
    Testing,
    Completed,
    Blocked(String), // Reason for blocking
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DuplicationCheckStatus {
    Passed,
    Failed(String), // Duplication details
    NotChecked,
}
```

## MANDATORY REQUIREMENTS

### 1. Comprehensive Checkin Work Tracking

#### 1.1 Work Item Creation Standards

**EVERY DEVELOPMENT TASK MUST**:

- ✅ Create unique work item ID: `WI-{YYYY-MM-DD}-{sequential}`
- ✅ Document all files being modified
- ✅ Run duplication check BEFORE starting work
- ✅ Update Source of Truth Registry
- ✅ Provide verification evidence
- ✅ Include completion timestamp and verification hash

```rust
// CANONICAL: Work Item Creation
impl WorkItemManager {
    pub async fn create_work_item(&self, title: &str, component: &str) -> Result<WorkItem, WorkItemError> {
        // 1. Generate unique ID with date prefix
        let work_id = self.generate_work_id().await?;
        
        // 2. Run pre-work duplication check
        let duplication_status = self.check_for_duplicates(&title, &component).await?;
        if matches!(duplication_status, DuplicationCheckStatus::Failed(_)) {
            return Err(WorkItemError::DuplicationDetected(duplication_status));
        }
        
        // 3. Create work item
        let work_item = WorkItem {
            id: work_id,
            title: title.to_string(),
            status: WorkStatus::Planning,
            component: component.to_string(),
            files_modified: Vec::new(),
            duplication_check: duplication_status,
            source_of_truth_updated: false,
            verification_hash: [0u8; 32], // Updated on completion
            completion_timestamp: None,
            evidence_link: String::new(),
            dependencies: Vec::new(),
            blockers: Vec::new(),
        };
        
        // 4. Register in Source of Truth Registry
        self.registry.register_work_item(&work_item).await?;
        
        Ok(work_item)
    }
}
```

#### 1.2 Real-Time Progress Tracking

```rust
// CANONICAL: Progress tracking with automatic updates
impl ProgressTracker {
    pub async fn update_work_status(&self, work_id: &str, new_status: WorkStatus) -> Result<(), TrackingError> {
        // 1. Validate work item exists
        let mut work_item = self.registry.get_work_item(work_id)
            .ok_or(TrackingError::WorkItemNotFound)?;
            
        // 2. Validate status transition is valid
        self.validate_status_transition(&work_item.status, &new_status)?;
        
        // 3. Update with timestamp
        work_item.status = new_status;
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos() as u64;
        
        // 4. If completing, run final duplication check
        if matches!(new_status, WorkStatus::Completed) {
            work_item.completion_timestamp = Some(timestamp);
            work_item.verification_hash = self.generate_verification_hash(&work_item).await?;
            work_item.source_of_truth_updated = true;
        }
        
        // 5. Update registry
        self.registry.update_work_item(work_item).await?;
        
        // 6. Auto-update system index
        self.index_manager.auto_update().await?;
        
        Ok(())
    }
}
```

### 2. Duplication Prevention & Source of Truth Management

#### 2.1 Code Duplication Detection

```rust
// CANONICAL: Advanced duplication detection
pub struct DuplicationDetector {
    /// Code fingerprint index
    code_fingerprints: DashMap<String, CodeFingerprint>,
    /// Function signature index  
    function_signatures: DashMap<String, FunctionSignature>,
    /// Documentation content index
    documentation_index: DashMap<String, DocumentationEntry>,
    /// Source of truth mappings
    source_of_truth_map: DashMap<String, SourceOfTruthEntry>,
}

impl DuplicationDetector {
    /// Comprehensive duplication check for code
    pub async fn check_code_duplication(&self, file_path: &str, content: &str) -> DuplicationCheckResult {
        let mut result = DuplicationCheckResult::new();
        
        // 1. Function-level duplication check
        let functions = self.extract_functions(content)?;
        for function in functions {
            if let Some(duplicate) = self.find_duplicate_function(&function) {
                result.add_duplication(DuplicationType::Function {
                    original: duplicate.location,
                    duplicate_content: function.signature,
                });
            }
        }
        
        // 2. Documentation duplication check
        let doc_blocks = self.extract_documentation(content)?;
        for doc_block in doc_blocks {
            if let Some(duplicate) = self.find_duplicate_documentation(&doc_block) {
                result.add_duplication(DuplicationType::Documentation {
                    original: duplicate.location,
                    content_hash: doc_block.hash,
                });
            }
        }
        
        // 3. Check against Source of Truth
        self.validate_against_source_of_truth(file_path, content, &mut result).await?;
        
        result
    }
}
```

#### 2.2 Source of Truth Validation

```rust
// CANONICAL: Source of Truth validation and updates
impl SourceOfTruthManager {
    /// Validate that changes align with canonical source of truth
    pub async fn validate_against_canonical(&self, changes: &[FileChange]) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        for change in changes {
            // 1. Check if file has canonical source of truth
            if let Some(canonical) = self.get_canonical_source(&change.file_path) {
                // 2. Validate change doesn't conflict with canonical version
                let conflict_check = self.check_canonical_conflict(&change, &canonical).await?;
                if conflict_check.has_conflicts() {
                    result.add_conflict(CanonicalConflict {
                        file: change.file_path.clone(),
                        canonical_source: canonical.location.clone(),
                        conflict_details: conflict_check.details,
                    });
                }
            }
            
            // 3. Check if change should become new canonical source
            if self.should_become_canonical(&change).await? {
                result.mark_for_canonical_update(change.file_path.clone());
            }
        }
        
        result
    }
    
    /// Update Source of Truth Registry after successful work completion
    pub async fn update_canonical_registry(&self, work_item: &WorkItem) -> Result<(), SourceOfTruthError> {
        // 1. Mark completed work files as potential canonical sources
        for file_path in &work_item.files_modified {
            let entry = SourceOfTruthEntry {
                file_path: file_path.clone(),
                work_item_id: work_item.id.clone(),
                verification_hash: work_item.verification_hash,
                last_updated: work_item.completion_timestamp.unwrap(),
                canonical_status: CanonicalStatus::Verified,
            };
            
            self.registry.canonical_documents.insert(file_path.clone(), entry);
        }
        
        // 2. Update master index
        self.update_master_index().await?;
        
        Ok(())
    }
}
```

### 3. Automatic Enforcement Integration

#### 3.1 Quality Gate Integration

```bash
# CANONICAL: Enhanced quality gate with checkin tracking
#!/bin/bash

echo "🔍 Running Canonical Quality Gate with Checkin Tracking..."

# 1. Verify work item exists and is properly tracked
if ! validate_work_item_tracking; then
    echo "❌ FAIL: Work item not properly tracked in Source of Truth Registry"
    exit 1
fi

# 2. Run comprehensive duplication check
if ! check_comprehensive_duplication; then
    echo "❌ FAIL: Code duplication detected - work item creation failed"
    exit 1
fi

# 3. Validate against Source of Truth
if ! validate_source_of_truth_compliance; then
    echo "❌ FAIL: Changes conflict with canonical source of truth"
    exit 1
fi

# 4. Update checkin tracking
update_work_item_progress "CodeReview"

echo "✅ PASS: All canonical standards enforced"
```

#### 3.2 Pre-commit Hook Enhancement

```bash
# CANONICAL: Pre-commit hook with work tracking
validate_work_item_tracking() {
    echo "Validating work item tracking..."
    
    # Check for work item ID in commit message
    if ! echo "$commit_message" | grep -E "WI-[0-9]{4}-[0-9]{2}-[0-9]{2}-[0-9]+"; then
        echo "❌ Commit message must include work item ID (WI-YYYY-MM-DD-###)"
        return 1
    fi
    
    # Verify work item exists in registry
    work_id=$(echo "$commit_message" | grep -oE "WI-[0-9]{4}-[0-9]{2}-[0-9]{2}-[0-9]+")
    if ! registry_contains_work_item "$work_id"; then
        echo "❌ Work item $work_id not found in Source of Truth Registry"
        return 1
    fi
    
    echo "✅ Work item tracking validated"
    return 0
}

check_comprehensive_duplication() {
    echo "Running comprehensive duplication check..."
    
    # Run duplication detector on all changed files
    changed_files=$(git diff --cached --name-only --diff-filter=ACMR)
    for file in $changed_files; do
        if [[ "$file" == *.rs ]] || [[ "$file" == *.md ]]; then
            if ! ./scripts/duplication_detector.rs --file "$file"; then
                echo "❌ Duplication detected in $file"
                return 1
            fi
        fi
    done
    
    echo "✅ No duplication detected"
    return 0
}
```

## 4. Implementation Roadmap

### Phase 1: Source of Truth Registry (Week 1)

- [ ] **Day 1-2**: Implement `SourceOfTruthRegistry` struct
- [ ] **Day 3-4**: Create work item tracking system
- [ ] **Day 5**: Integrate with existing quality gate

### Phase 2: Duplication Detection (Week 2)  

- [ ] **Day 1-2**: Implement `DuplicationDetector` with code analysis
- [ ] **Day 3-4**: Add documentation duplication detection
- [ ] **Day 5**: Create comprehensive duplication reports

### Phase 3: Automatic Enforcement (Week 3)

- [ ] **Day 1-2**: Enhance quality gate script with work tracking
- [ ] **Day 3-4**: Update pre-commit hooks with duplication prevention
- [ ] **Day 5**: CI/CD integration and testing

### Phase 4: Verification & Documentation (Week 4)

- [ ] **Day 1-3**: Comprehensive testing of all systems
- [ ] **Day 4-5**: Documentation updates and training materials

## 5. Verification Requirements

### 5.1 Work Item Completion Checklist

**EVERY WORK ITEM MUST COMPLETE**:

- [ ] ✅ Work item created with unique ID
- [ ] ✅ Pre-work duplication check: PASSED
- [ ] ✅ All modified files documented  
- [ ] ✅ Source of Truth Registry updated
- [ ] ✅ Quality gate validation: PASSED
- [ ] ✅ Comprehensive duplication check: PASSED
- [ ] ✅ Final verification hash generated
- [ ] ✅ Evidence documentation completed
- [ ] ✅ Status updated to "Completed"

### 5.2 Registry Integrity Verification

```bash
# CANONICAL: Registry integrity check
./scripts/verify_registry_integrity.sh --full
# Expected output: "✅ Registry integrity: VALID - 0 duplicates found"
```

## 6. Enforcement Authority

**This document serves as the CANONICAL SOURCE OF TRUTH for**:

- Work item tracking standards
- Duplication prevention requirements  
- Source of Truth Registry management
- Quality gate enforcement rules
- Pre-commit hook specifications

**ALL FUTURE DEVELOPMENT MUST REFERENCE THIS DOCUMENT** and use the canonical implementations defined herein. Any deviation requires explicit approval and documentation of the exception.

---

**VERIFICATION COMMAND**: `./scripts/validate_canonical_compliance.sh --prd-checkin-tracking`
**ENFORCEMENT**: Automatic blocking of non-compliant work via quality gate
