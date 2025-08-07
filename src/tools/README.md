# tools Module

Tools Module [AIR-3][AIS-3][BPC-3][AIT-3]

This module provides various utility tools for the Anya Core system,
following official Bitcoin Improvement Proposals (BIPs) and canonical
Source of Truth Registry standards.

## Overview

The `tools` module provides essential utility tools and infrastructure for the Anya Core system, focusing on documentation management, source code tracking, and canonical documentation registry. This module ensures consistency, accuracy, and compliance with Bitcoin Improvement Proposals (BIPs) and maintains a canonical source of truth for all system documentation.

## Key Components

### Source of Truth Registry

Canonical documentation management system:

- **Document Registry**: Centralized registry of all canonical documents
- **Duplication Detection**: Automatic detection of duplicate documentation
- **Version Control**: Track document versions and changes
- **Status Management**: Monitor document status and validation states

```rust
use anya_core::tools::{get_global_registry, initialize_global_registry};
use anya_core::tools::{CanonicalDocument, CanonicalStatus, WorkItem};

// Initialize the global registry
initialize_global_registry().await?;

// Get registry instance
let registry = get_global_registry().await?;

// Register a canonical document
let doc = CanonicalDocument {
    path: "/docs/api/bitcoin.md".to_string(),
    status: CanonicalStatus::Canonical,
    last_updated: SystemTime::now(),
};

registry.register_document(doc).await?;
```

### Documentation Validation

Comprehensive documentation management tools:

- **Markdown Validation**: Validate Markdown document structure and content
- **Link Checking**: Verify all internal and external links
- **Format Compliance**: Ensure compliance with documentation standards
- **Content Analysis**: Analyze documentation for completeness and accuracy

```rust
use anya_core::tools::{DocumentationValidator, DocError};

// Create validator
let validator = DocumentationValidator::new();

// Validate documentation
let result = validator.validate_document("/docs/README.md").await?;

// Check for errors
if let Err(DocError::InvalidFormat(msg)) = result {
    println!("Documentation error: {}", msg);
}
```

### Commit Tracking

Git commit and change tracking system:

- **AI Labelling**: Automatic AI labelling for commits
- **Change Detection**: Track meaningful changes in the codebase
- **Compliance Tracking**: Monitor compliance with BIP standards
- **Audit Trail**: Maintain detailed audit trail of all changes

```rust
use anya_core::tools::{update_ai_labelling_file, CommitInfo};

// Create commit info
let commit_info = CommitInfo {
    hash: "abc123".to_string(),
    author: "developer@example.com".to_string(),
    message: "Implement new Bitcoin feature".to_string(),
    timestamp: SystemTime::now(),
    files_changed: vec!["src/bitcoin/mod.rs".to_string()],
};

// Update AI labelling
update_ai_labelling_file(&commit_info).await?;
```

### Documentation Duplication Scanner

Advanced duplication detection and management:

- **Content Analysis**: Deep content analysis for duplicate detection
- **Similarity Scoring**: Calculate content similarity scores
- **Automated Reporting**: Generate duplication reports
- **CLI Interface**: Command-line interface for batch operations

```rust
use anya_core::tools::doc_duplication_scanner;

// Scan for duplicates
let duplicates = doc_duplication_scanner::scan_workspace("/workspaces/Anya-core").await?;

// Process results
for duplicate in duplicates {
    println!("Found duplicate: {} similar to {}",
        duplicate.source_path, duplicate.target_path);
}
```

## API Reference

### SourceOfTruthRegistry

- `register_document(doc)`: Register canonical document
- `get_document_status(path)`: Get document canonical status
- `check_for_duplicates()`: Check for documentation duplicates
- `update_work_item(item)`: Update work item status

### DocumentationValidator

- `validate_document(path)`: Validate single document
- `validate_workspace()`: Validate entire workspace documentation
- `check_links(doc)`: Verify document links
- `analyze_content(content)`: Analyze document content quality

### CommitInfo

- `hash`: Git commit hash identifier
- `author`: Commit author information
- `message`: Commit message content
- `timestamp`: Commit timestamp
- `files_changed`: List of modified files

### Error Types

- `SourceOfTruthError`: Registry-related errors
- `DocError`: Documentation validation errors
- `DuplicationCheckStatus`: Duplication detection status

## For more information

See the comprehensive documentation in the [docs/](/docs/) directory.
