#[cfg(test)]
mod tests {
    use crate::tools::doc_duplication_scanner::*;
    use crate::tools::source_of_truth_registry::*;
    use std::path::PathBuf;
    use tempfile::tempdir;
    use std::fs;
    use std::path::Path;

    #[tokio::test]
    async fn test_doc_duplication_detection() {
        // Create temporary test directory
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path();
        
        // Create test files with duplicated content
        let file1_path = test_dir.join("doc1.md");
        let file2_path = test_dir.join("doc2.md");
        
        let duplicate_content = "# Test Heading

This is some duplicate content that should be detected by the scanner.
It contains enough text to be considered a substantial section.";
        
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
        
        // Check similarity score
        if let Some(dup) = report.duplications.first() {
            assert!(dup.similarity > 0.8); // High similarity for identical content
            assert_eq!(dup.entries.len(), 2); // Should have two entries in the group
        }
    }
}
