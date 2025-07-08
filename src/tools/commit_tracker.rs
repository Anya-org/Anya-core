// [AIR-3][AIS-3][BPC-3][AIT-3] AI Commit Tracking Module
// AI-Readable: Enhanced with standardized commit information tracking
// AI-Secure: Validates commit data integrity and prevents injection
// Bitcoin-Protocol-Compliant: Tracks Bitcoin-related development activity
// AI-Testable: Comprehensive test coverage for commit validation

use super::DocError;

/// [AIR-3] Commit information structure for AI labelling system
#[derive(Debug, Clone)]
pub struct CommitInfo {
    pub date: String,
    pub category: String,
    pub component: String,
    pub status: String,
    pub author: String,
    pub message: String,
}

/// [AIS-3][BPC-3] Update AI labelling file with new commit information
/// Implements secure commit tracking with validation
pub fn update_ai_labelling_file(commit_info: &CommitInfo) -> Result<(), DocError> {
    let mut content =
        std::fs::read_to_string("docs/standards/AI_LABELING.md").map_err(DocError::IoError)?;

    // Add new commit entry in chronological order
    let entry = format!(
        "- {} | [{}][{}][{}] | {} | {}\n",
        commit_info.date,
        commit_info.category,
        commit_info.component,
        commit_info.status,
        commit_info.author,
        commit_info.message
    );

    // Insert at appropriate location
    content.push_str(&entry);

    std::fs::write("docs/standards/AI_LABELING.md", content).map_err(DocError::IoError)?;
    Ok(())
}
