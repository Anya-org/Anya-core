#[cfg(test)]
mod tests {
    use crate::tools::doc_duplication_scanner_cli::*;
    use crate::tools::doc_duplication_scanner::*;
    use tempfile::tempdir;
    use std::fs;
    use std::path::Path;
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
    fn test_cli_help() {
        // Test help output
        let args = vec![
            "doc_scanner".to_string(),
            "--help".to_string()
        ];
        
        let result = parse_cli_args(&args);
        
        // Help should return an Err with the help message
        assert!(result.is_err());
        let err = result.unwrap_err();
        let err_msg = format!("{}", err);
        
        // Verify help message contains key information
        assert!(err_msg.contains("Usage:"));
        assert!(err_msg.contains("--scan-path"));
        assert!(err_msg.contains("--extensions"));
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
