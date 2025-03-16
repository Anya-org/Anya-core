pub fn update_ai_labelling_file(commit_info: &CommitInfo) -> Result<()> {
    let mut content = std::fs::read_to_string("ai_labelling.md")?;
    
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
    // ...
    
    std::fs::write("ai_labelling.md", content)?;
    Ok(())
} 