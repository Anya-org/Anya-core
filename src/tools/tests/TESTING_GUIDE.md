# Documentation Duplication Detection Testing Guide

## Overview

This document provides guidance for implementing and running tests for the documentation duplication detection functionality in the Source of Truth Registry v1.3. Note that these tests should be implemented after resolving the current implementation issues in the codebase.

## Current Implementation Issues

The following issues need to be fixed in the main codebase before tests can run properly:

1. Unresolved imports in `doc_duplication_scanner.rs`:
   - `DuplicationCheck` and `RegistryError` are missing or misnamed in `source_of_truth_registry.rs`

2. Field mismatches in `DocumentationEntry` struct:
   - The scanner is attempting to use fields like `content_snippet`, `modification_date`, `language`, and `similarity_scores` which don't exist in the actual `DocumentationEntry` struct

3. Function signature issues:
   - `initialize_global_registry()` is called without required arguments
   - `sync_wait()` method is used but doesn't exist for the Future type

4. Type mismatch issues:
   - Type mismatch between `u32` and `u8` for `required_confirmations` field

## Recommended Test Structure

Once the implementation issues are resolved, implement tests in the following files:

### 1. `/workspaces/Anya-core/src/tools/tests/test_doc_duplication.rs`

```rust
#[cfg(test)]
mod tests {
    use crate::tools::doc_duplication_scanner::*;
    use crate::tools::source_of_truth_registry::*;
    use std::path::PathBuf;
    use tempfile::tempdir;
    use std::fs;

    #[tokio::test]
    async fn test_doc_duplication_detection() {
        // Create temporary test directory
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path();
        
        // Create test files with duplicated content
        let file1_path = test_dir.join("doc1.md");
        let file2_path = test_dir.join("doc2.md");
        
        let duplicate_content = "# Test Heading\n\nThis is duplicate content.";
        
        fs::write(&file1_path, duplicate_content).unwrap();
        fs::write(&file2_path, duplicate_content).unwrap();
        
        // Create options for scan
        let options = ScanOptions {
            scan_path: test_dir.to_path_buf(),
            file_extensions: vec!["md".to_string()],
            similarity_threshold: 0.85,
            ignore_patterns: Vec::new(),
            output_format: None,
            fail_on_duplicates: false,
            max_results: None,
        };
        
        // Run scan
        let report = scan_for_duplicates(&options).await.unwrap();
        
        // Assert that duplication was found
        assert_eq!(report.files_scanned, 2);
        assert!(report.duplications.len() > 0);
    }

    #[tokio::test]
    async fn test_file_format_support() {
        // Test that the scanner supports multiple file formats
        let temp_dir = tempdir().unwrap();
        
        // Create files with different formats
        let formats = vec![
            ("doc.md", "# Markdown Heading\n\nContent."),
            ("doc.txt", "Plain text content."),
            ("doc.rst", "RST Content\n==========\n\nContent."),
        ];
        
        for (filename, content) in &formats {
            let path = temp_dir.path().join(filename);
            fs::write(&path, content).unwrap();
        }
        
        // Scan for duplications
        let options = ScanOptions {
            scan_path: temp_dir.path().to_path_buf(),
            file_extensions: vec!["md".to_string(), "txt".to_string(), "rst".to_string()],
            similarity_threshold: 0.7,
            ignore_patterns: Vec::new(),
            output_format: None,
            fail_on_duplicates: false,
            max_results: None,
        };
        
        let report = scan_for_duplicates(&options).await.unwrap();
        
        // Verify all files were scanned
        assert_eq!(report.files_scanned, 3);
    }
}
```

### 2. `/workspaces/Anya-core/src/tools/tests/test_cli.rs`

```rust
#[cfg(test)]
mod tests {
    use crate::tools::doc_duplication_scanner_cli::*;
    use crate::tools::doc_duplication_scanner::*;
    use tempfile::tempdir;
    use std::env;
    
    #[test]
    fn test_parse_cli_args() {
        // Test valid CLI arguments parsing
        let args = vec![
            "doc_scanner".to_string(),
            "--scan-path".to_string(), 
            "/tmp/test".to_string(),
            "--extensions".to_string(), 
            "md,txt".to_string(),
            "--similarity".to_string(), 
            "0.8".to_string(),
            "--fail-on-duplicates".to_string()
        ];
        
        let options = parse_cli_args(&args).unwrap();
        
        assert_eq!(options.scan_path.to_string_lossy(), "/tmp/test");
        assert_eq!(options.file_extensions, vec!["md", "txt"]);
        assert_eq!(options.similarity_threshold, 0.8);
        assert_eq!(options.fail_on_duplicates, true);
    }
    
    #[test]
    fn test_format_output() {
        // Create a simple report
        let report = DuplicationReport {
            files_scanned: 10,
            sections_analyzed: 20,
            duplications: Vec::new(),
        };
        
        // Test different output formats
        let json_output = format_duplication_report(&report, "json");
        assert!(json_output.is_ok());
        assert!(json_output.unwrap().contains("\"files_scanned\":10"));
        
        let text_output = format_duplication_report(&report, "text");
        assert!(text_output.is_ok());
        assert!(text_output.unwrap().contains("Files scanned: 10"));
        
        let unknown_format = format_duplication_report(&report, "unknown");
        assert!(unknown_format.is_err());
    }
}
```

## Integration with `mod.rs`

Ensure these test modules are properly referenced in `/workspaces/Anya-core/src/tools/tests/mod.rs`:

```rust
// Integration tests for Source of Truth Registry
#[cfg(feature = "web5")]
pub mod integration_test_web5;

// Basic tests for documentation duplication detection
#[cfg(test)]
pub mod test_doc_duplication;

// CLI tests for documentation duplication scanner
#[cfg(test)]
pub mod test_cli;
```

## Manual Testing Steps

1. Create a directory with sample documentation files, including some with duplicate content
2. Run the CLI scanner:

   ```bash
   cargo run --bin doc_scanner -- --scan-path ./docs --extensions md,txt --similarity 0.8
   ```

3. Verify the output shows proper detection of duplicate content
4. Test different output formats:

   ```bash
   cargo run --bin doc_scanner -- --scan-path ./docs --extensions md,txt --output json
   ```

## Implementation Recommendations

1. Align `DocumentationEntry` fields between the scanner and registry
2. Fix the import issues for `DuplicationCheck` and `RegistryError`
3. Ensure CLI parameters properly align with the scanner options
4. Add the missing `taproot` feature flag to Cargo.toml if needed
5. Fix the type mismatches with proper conversions
