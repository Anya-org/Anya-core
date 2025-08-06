# Utils Module Documentation

**Compliance Tags**: [AIR-3][AIS-3][BPC-3][RES-3]

[AIS-3]: #ais-3 "Application Integration Standard Level 3"
[RES-3]: #res-3 "Resource Efficiency Standard Level 3"

## Overview

The Utils module provides utility functions and types for documentation validation, file operations, and style enforcement within the Anya Core system. It is used throughout the codebase to ensure documentation quality and compliance.

## Core Components

### MarkdownDocument

Represents a markdown document with validation capabilities.

#### Key Features

- Load markdown files from disk
- Validate presence of required compliance labels
- Enforce style rules (e.g., line length)

#### Usage Example

```rust
use anya_core::utils::MarkdownDocument;

let doc = MarkdownDocument::load("docs/README.md")?;
if doc.has_compliance_labels() {
    println!("Document is compliant");
}
```

### DocError

Error type for documentation validation and file operations.

- IO errors
- Regex errors
- Missing compliance labels
- Style violations

## Constants

- `REQUIRED_LABELS`: List of required compliance labels for documentation
- `MAX_LINE_LENGTH`: Maximum allowed line length for markdown files

## Integration Points

- **Documentation Management**: For validating and managing markdown files
- **Scripts**: Used in validation and management scripts
- **Testing**: For style and compliance tests

## Compliance Standards

### AIR-3

Ensures high availability and integrity by validating documentation and enforcing style rules.

### AIS-3

Comprehensive APIs for integration with documentation management tools and scripts.

### BPC-3

Supports Bitcoin protocol documentation standards for full compatibility.

### RES-3

Efficient file operations and validation for minimal resource usage.
