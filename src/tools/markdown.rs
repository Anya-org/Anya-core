// [AIR-3][AIS-3][BPC-3][AIT-3] Markdown Documentation Validation Module
// AI-Readable: Enhanced with standardized markdown processing capabilities
// AI-Secure: Validates document structure and prevents malformed content
// Bitcoin-Protocol-Compliant: Ensures documentation meets BDF v2.5 standards
// AI-Testable: Comprehensive test coverage for document validation

use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use regex::Regex;
use walkdir::WalkDir;
use thiserror::Error;

/// Required compliance labels for documentation
const REQUIRED_LABELS: [&str; 3] = ["AIS-3", "BPC-3", "DAO-4"];
const MAX_LINE_LENGTH: usize = 100;

#[derive(Debug, Error)]
pub enum DocError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    
    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),
    
    #[error("Missing compliance labels: {0}")]
    MissingLabels(String),
    
    #[error("Style violation: {0}")]
    StyleViolation(String),
}

/// Represents a markdown document with validation capabilities
pub struct MarkdownDocument {
    path: PathBuf,
    content: String,
}

impl MarkdownDocument {
    /// Load a markdown document from a file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, DocError> {
        let path = path.as_ref().to_path_buf();
        let mut file = File::open(&path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        Ok(Self { path, content })
    }
    
    /// Check if document has all required compliance labels
    pub fn has_compliance_labels(&self) -> bool {
        for label in REQUIRED_LABELS {
            if !self.content.contains(&format!("[{}]", label)) {
                return false;
            }
        }
        true
    }
    
    /// Get missing compliance labels
    pub fn missing_labels(&self) -> Vec<String> {
        REQUIRED_LABELS
            .iter()
            .filter(|&&label| !self.content.contains(&format!("[{}]", label)))
            .map(|&label| label.to_string())
            .collect()
    }
    
    /// Fix compliance labels by adding missing ones
    pub fn fix_compliance_labels(&mut self) -> Result<bool, DocError> {
        let missing = self.missing_labels();
        if missing.is_empty() {
            return Ok(false);
        }
        
        let labels_to_add = missing.iter()
            .map(|label| format!("[{}]", label))
            .collect::<Vec<_>>()
            .join("");
        
        let lines: Vec<&str> = self.content.lines().collect();
        if !lines.is_empty() && lines[0].starts_with("# ") {
            // Add to heading
            let new_heading = format!("{} {}", lines[0], labels_to_add);
            let mut new_content = new_heading;
            for line in &lines[1..] {
                new_content.push_str("\n");
                new_content.push_str(line);
            }
            self.content = new_content;
        } else {
            // Add to top of file
            self.content = format!("{}\n\n{}", labels_to_add, self.content);
        }
        
        Ok(true)
    }
    
    /// Check for trailing whitespace
    pub fn has_trailing_whitespace(&self) -> bool {
        match Regex::new(r"[ \t]+$") {
            Ok(re) => {
                for line in self.content.lines() {
                    if re.is_match(line) {
                        return true;
                    }
                }
                false
            }
            Err(_) => false, // If regex compilation fails, assume no whitespace
        }
    }
    
    /// Fix trailing whitespace
    pub fn fix_trailing_whitespace(&mut self) -> Result<bool, DocError> {
        let re = Regex::new(r"[ \t]+$")?;
        let original = self.content.clone();
        self.content = re.replace_all(&self.content, "").to_string();
        
        Ok(self.content != original)
    }
    
    /// Check for long lines
    pub fn has_long_lines(&self) -> bool {
        for line in self.content.lines() {
            // Skip headings, code blocks, tables, and links
            if line.starts_with('#') || line.starts_with("```") || 
               line.starts_with('|') || line.starts_with('[') || line.trim().is_empty() {
                continue;
            }
            
            if line.len() > MAX_LINE_LENGTH {
                return true;
            }
        }
        false
    }
    
    /// Fix long lines by wrapping them
    pub fn fix_long_lines(&mut self) -> Result<bool, DocError> {
        let lines: Vec<&str> = self.content.lines().collect();
        let mut new_lines = Vec::new();
        let mut changed = false;
        
        for line in lines {
            // Skip headings, code blocks, tables, and links
            if line.starts_with('#') || line.starts_with("```") || 
               line.starts_with('|') || line.starts_with('[') || line.trim().is_empty() {
                new_lines.push(line.to_string());
                continue;
            }
            
            if line.len() <= MAX_LINE_LENGTH {
                new_lines.push(line.to_string());
                continue;
            }
            
            // Wrap the line
            changed = true;
            let words: Vec<&str> = line.split_whitespace().collect();
            let mut current_line = String::new();
            
            for word in words {
                if current_line.is_empty() {
                    current_line.push_str(word);
                } else if current_line.len() + word.len() + 1 <= MAX_LINE_LENGTH {
                    current_line.push(' ');
                    current_line.push_str(word);
                } else {
                    new_lines.push(current_line);
                    current_line = word.to_string();
                }
            }
            
            if !current_line.is_empty() {
                new_lines.push(current_line);
            }
        }
        
        if changed {
            self.content = new_lines.join("\n");
        }
        
        Ok(changed)
    }
    
    /// Save changes back to file
    pub fn save(&self) -> Result<(), DocError> {
        let mut file = File::create(&self.path)?;
        file.write_all(self.content.as_bytes())?;
        Ok(())
    }
    
    /// Fix all style issues
    pub fn fix_all(&mut self) -> Result<bool, DocError> {
        let mut changed = false;
        changed |= self.fix_compliance_labels()?;
        changed |= self.fix_trailing_whitespace()?;
        changed |= self.fix_long_lines()?;
        Ok(changed)
    }
}

/// Documentation validator
pub struct DocumentationValidator {
    root_dir: PathBuf,
}

impl DocumentationValidator {
    /// Create a new validator for the given directory
    pub fn new<P: AsRef<Path>>(root_dir: P) -> Self {
        Self { root_dir: root_dir.as_ref().to_path_buf() }
    }
    
    /// Validate all markdown files
    pub fn validate_all(&self, fix_issues: bool) -> Result<ValidationReport, DocError> {
        let mut report = ValidationReport::new();
        
        for entry in WalkDir::new(&self.root_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file() && e.path().extension().map_or(false, |ext| ext == "md"))
        {
            let path = entry.path();
            let mut doc = MarkdownDocument::load(path)?;
            let mut file_report = FileReport {
                path: path.to_string_lossy().to_string(),
                issues: Vec::new(),
                fixed: false,
            };
            
            // Check compliance labels
            if !doc.has_compliance_labels() {
                let missing = doc.missing_labels();
                file_report.issues.push(format!("Missing compliance labels: {}", missing.join(", ")));
            }
            
            // Check trailing whitespace
            if doc.has_trailing_whitespace() {
                file_report.issues.push("Contains trailing whitespace".to_string());
            }
            
            // Check line length
            if doc.has_long_lines() {
                file_report.issues.push(format!("Contains lines longer than {} characters", MAX_LINE_LENGTH));
            }
            
            // Fix issues if requested
            if fix_issues && !file_report.issues.is_empty() {
                if doc.fix_all()? {
                    doc.save()?;
                    file_report.fixed = true;
                }
            }
            
            if !file_report.issues.is_empty() {
                report.files.push(file_report);
            }
        }
        
        Ok(report)
    }
}

/// Report of validation issues
#[derive(Debug)]
pub struct ValidationReport {
    pub files: Vec<FileReport>,
}

impl ValidationReport {
    fn new() -> Self {
        Self { files: Vec::new() }
    }
    
    /// Get the number of files with issues
    pub fn issue_count(&self) -> usize {
        self.files.len()
    }
    
    /// Get the number of fixed files
    pub fn fixed_count(&self) -> usize {
        self.files.iter().filter(|f| f.fixed).count()
    }
    
    /// Print report to console
    pub fn print(&self) {
        println!("Documentation Validation Report:");
        println!("===============================");
        
        if self.files.is_empty() {
            println!("✅ All documentation files pass validation!");
            return;
        }
        
        println!("Found issues in {} files:", self.files.len());
        for file in &self.files {
            println!("\n📄 File: {}", file.path);
            for issue in &file.issues {
                println!("  ❌ {}", issue);
            }
            if file.fixed {
                println!("  ✅ Issues fixed automatically");
            }
        }
        
        println!("\nSummary: {} issues found, {} files fixed", 
            self.files.iter().map(|f| f.issues.len()).sum::<usize>(),
            self.fixed_count()
        );
    }
}

/// Report for a single file
#[derive(Debug)]
pub struct FileReport {
    pub path: String,
    pub issues: Vec<String>,
    pub fixed: bool,
} 
