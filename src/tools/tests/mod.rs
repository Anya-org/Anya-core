// Integration tests for Source of Truth Registry
#[cfg(feature = "web5")]
pub mod integration_test_web5;

// Basic tests for documentation duplication detection
#[cfg(test)]
pub mod test_doc_duplication;

// CLI tests for documentation duplication scanner
#[cfg(test)]
pub mod test_cli;
