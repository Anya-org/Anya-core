use crate::tools::source_of_truth_registry::DocumentationEntry;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

/// Duplication report structure for documenting found duplications
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DuplicationReport {
    pub files_scanned: usize,
    pub sections_analyzed: usize,
    pub duplications: Vec<DuplicationGroup>,
}

/// Group of duplicate documents/sections
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DuplicationGroup {
    pub similarity: f32,
    pub entries: Vec<DocumentationEntry>,
}

/// Options for configuring the duplication scan
#[derive(Debug)]
pub struct ScanOptions {
    pub scan_path: PathBuf,
    pub file_extensions: Vec<String>,
    pub similarity_threshold: f32,
    pub ignore_patterns: Vec<String>,
    pub output_format: Option<String>,
    pub fail_on_duplicates: bool,
    pub max_results: Option<usize>,
}

/// Scan the repository for documentation duplication
pub async fn scan_for_duplicates(
    options: &ScanOptions,
) -> Result<DuplicationReport, Box<dyn Error>> {
    let mut report = DuplicationReport {
        files_scanned: 0,
        sections_analyzed: 0,
        duplications: Vec::new(),
    };

    // Find all documentation files
    let files = find_documentation_files(
        &options.scan_path,
        &options.file_extensions,
        &options.ignore_patterns,
    )?;
    report.files_scanned = files.len();

    // Extract sections from each file
    let mut sections: Vec<DocumentationEntry> = Vec::new();
    for file_path in &files {
        let content = fs::read_to_string(file_path)?;
        let file_sections = extract_sections(file_path, &content)?;
        sections.extend(file_sections);
    }
    report.sections_analyzed = sections.len();

    // Detect duplications
    report.duplications = find_duplications(&sections, options.similarity_threshold).await?;

    // Limit results if needed
    if let Some(max) = options.max_results {
        report.duplications.truncate(max);
    }

    Ok(report)
}

/// Find all documentation files in the given directory
fn find_documentation_files(
    dir: &Path,
    extensions: &[String],
    ignore_patterns: &[String],
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut result = Vec::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            // Skip if matches ignore pattern
            if should_ignore(&path, ignore_patterns) {
                continue;
            }

            if path.is_dir() {
                // Recursively scan subdirectories
                let mut sub_files = find_documentation_files(&path, extensions, ignore_patterns)?;
                result.append(&mut sub_files);
            } else if path.is_file() {
                // Check if file has a documentation extension
                if let Some(ext) = path.extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    if extensions.iter().any(|e| e.to_lowercase() == ext_str) {
                        result.push(path);
                    }
                }
            }
        }
    }

    Ok(result)
}

/// Check if a path should be ignored
fn should_ignore(path: &Path, ignore_patterns: &[String]) -> bool {
    let path_str = path.to_string_lossy();

    for pattern in ignore_patterns {
        // Simple wildcard matching
        if pattern.starts_with("**/") {
            let suffix = &pattern[3..];
            if path_str.ends_with(suffix) {
                return true;
            }
        } else if pattern.ends_with("/**") {
            let prefix = &pattern[0..pattern.len() - 3];
            if path_str.starts_with(prefix) {
                return true;
            }
        } else if path_str.contains(pattern) {
            return true;
        }
    }

    false
}

/// Extract sections from a documentation file
fn extract_sections(
    file_path: &Path,
    content: &str,
) -> Result<Vec<DocumentationEntry>, Box<dyn Error>> {
    let mut sections = Vec::new();

    // Different extraction based on file type
    let extension = file_path
        .extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    match extension.as_ref() {
        "md" => extract_markdown_sections(file_path, content, &mut sections)?,
        "rst" => extract_rst_sections(file_path, content, &mut sections)?,
        "html" => extract_html_sections(file_path, content, &mut sections)?,
        _ => {
            // For unknown formats, treat the whole file as one section
            let title = file_path
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_else(|| "Unknown".to_string());

            let normalized = normalize_documentation_content(content);
            let mut hasher = Sha256::new();
            hasher.update(content);
            let content_hash = hasher.finalize();

            let mut norm_hasher = Sha256::new();
            norm_hasher.update(&normalized);
            let normalized_hash = norm_hasher.finalize();

            sections.push(DocumentationEntry {
                content_hash: content_hash.into(),
                normalized_hash: normalized_hash.into(),
                title,
                file_path: file_path.to_string_lossy().to_string(),
                section: "Whole File".to_string(),
                word_count: content.split_whitespace().count(),
                similarity_score: None,
                similar_to: None,
                created_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                updated_at: fs::metadata(file_path)
                    .and_then(|m| m.modified())
                    .map(|m| m.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs())
                    .unwrap_or(0),
            });
        }
    }

    Ok(sections)
}

/// Extract sections from markdown files
fn extract_markdown_sections(
    file_path: &Path,
    content: &str,
    sections: &mut Vec<DocumentationEntry>,
) -> Result<(), Box<dyn Error>> {
    let lines: Vec<&str> = content.lines().collect();
    let mut current_section_title = file_path
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_else(|| "Unknown".to_string());
    let mut current_section_start = 0;

    // Process line by line looking for headers
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Check for markdown headers (# Header)
        if trimmed.starts_with('#') {
            // If we were tracking a section, add it
            if i > current_section_start {
                let section_content = lines[current_section_start..i].join("\n");

                if !section_content.trim().is_empty() {
                    add_section(
                        file_path,
                        &current_section_title,
                        &section_content,
                        sections,
                    )?;
                }
            }

            // Start new section
            current_section_title = trimmed.trim_start_matches('#').trim().to_string();
            current_section_start = i + 1;
        }
    }

    // Add the last section
    if current_section_start < lines.len() {
        let section_content = lines[current_section_start..].join("\n");

        if !section_content.trim().is_empty() {
            add_section(
                file_path,
                &current_section_title,
                &section_content,
                sections,
            )?;
        }
    }

    Ok(())
}

/// Extract sections from restructuredText files
fn extract_rst_sections(
    file_path: &Path,
    content: &str,
    sections: &mut Vec<DocumentationEntry>,
) -> Result<(), Box<dyn Error>> {
    // Simplified RST section extraction - headers have underlines
    let lines: Vec<&str> = content.lines().collect();
    let mut current_section_title = file_path
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_else(|| "Unknown".to_string());
    let mut current_section_start = 0;

    for i in 1..lines.len() {
        // Check for RST header (underlined with =, -, ~, etc.)
        if i > 0 && !lines[i - 1].trim().is_empty() {
            let current_line = lines[i].trim();
            if !current_line.is_empty()
                && current_line.chars().all(|c| "=-~^*+'\"`.,:_".contains(c))
                && current_line.len() >= 2
            {
                // If we were tracking a section, add it
                if i > current_section_start + 1 {
                    let section_content = lines[current_section_start..i - 1].join("\n");

                    if !section_content.trim().is_empty() {
                        add_section(
                            file_path,
                            &current_section_title,
                            &section_content,
                            sections,
                        )?;
                    }
                }

                // Start new section
                current_section_title = lines[i - 1].trim().to_string();
                current_section_start = i + 1;
            }
        }
    }

    // Add the last section
    if current_section_start < lines.len() {
        let section_content = lines[current_section_start..].join("\n");

        if !section_content.trim().is_empty() {
            add_section(
                file_path,
                &current_section_title,
                &section_content,
                sections,
            )?;
        }
    }

    Ok(())
}

/// Extract sections from HTML files
fn extract_html_sections(
    file_path: &Path,
    content: &str,
    sections: &mut Vec<DocumentationEntry>,
) -> Result<(), Box<dyn Error>> {
    // Very simplified HTML section extraction - headers only
    // In a real implementation, you'd want to use an HTML parser
    let lines: Vec<&str> = content.lines().collect();
    let mut current_section_title = file_path
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_else(|| "Unknown".to_string());
    let mut current_section_start = 0;
    let mut in_header = false;
    let mut header_content = String::new();

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Very simplified header detection - would use a parser in production
        if let Some(header_start) = trimmed.find("<h") {
            if let Some(_header_end) = trimmed[header_start..].find('>') {
                in_header = true;

                // If we were tracking a section, add it
                if i > current_section_start {
                    let section_content = lines[current_section_start..i].join("\n");

                    if !section_content.trim().is_empty() {
                        add_section(
                            file_path,
                            &current_section_title,
                            &section_content,
                            sections,
                        )?;
                    }
                }

                current_section_start = i + 1;
            }
        }

        if in_header {
            header_content.push_str(trimmed);
            if trimmed.contains("</h") {
                // Extract text between tags - very naive approach
                if let Some(start) = header_content.find('>') {
                    if let Some(end) = header_content[start + 1..].find('<') {
                        current_section_title = header_content[start + 1..start + 1 + end]
                            .trim()
                            .to_string();
                    }
                }
                in_header = false;
                header_content.clear();
            }
        }
    }

    // Add the last section
    if current_section_start < lines.len() {
        let section_content = lines[current_section_start..].join("\n");

        if !section_content.trim().is_empty() {
            add_section(
                file_path,
                &current_section_title,
                &section_content,
                sections,
            )?;
        }
    }

    Ok(())
}

/// Add a section to the sections list
fn add_section(
    file_path: &Path,
    title: &str,
    content: &str,
    sections: &mut Vec<DocumentationEntry>,
) -> Result<(), Box<dyn Error>> {
    let normalized = normalize_documentation_content(content);

    let mut hasher = Sha256::new();
    hasher.update(content);
    let content_hash = hasher.finalize();

    let mut norm_hasher = Sha256::new();
    norm_hasher.update(&normalized);
    let normalized_hash = norm_hasher.finalize();

    sections.push(DocumentationEntry {
        content_hash: content_hash.into(),
        normalized_hash: normalized_hash.into(),
        title: title.to_string(),
        file_path: file_path.to_string_lossy().to_string(),
        section: title.to_string(),
        word_count: content.split_whitespace().count(),
        similarity_score: None,
        similar_to: None,
        created_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        updated_at: fs::metadata(file_path)
            .and_then(|m| m.modified())
            .map(|m| m.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs())
            .unwrap_or(0),
    });

    Ok(())
}

/// Normalize documentation content for better comparison
pub fn normalize_documentation_content(content: &str) -> String {
    // Remove formatting
    let mut normalized = content.to_lowercase();

    // Remove common markdown/rst/html formatting
    let patterns = [
        "**", "__", "*", "_", "`", "#", "##", "###", "####", "#####", "######", "<br>", "<p>",
        "</p>", "<div>", "</div>", "<span>", "</span>", ">", "- ", "* ", "1. ", "2. ", "3. ",
        "4. ", "5. ", "6. ", "7. ", "8. ", "9. ", "10. ",
    ];

    for pattern in patterns {
        normalized = normalized.replace(pattern, " ");
    }

    // Normalize whitespace
    let normalized = normalized
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join(" ");

    // Collapse multiple spaces into one
    let re = regex::Regex::new(r"\s+").unwrap();
    let normalized = re.replace_all(&normalized, " ").to_string();

    normalized.trim().to_string()
}

/// Find duplications among document sections
async fn find_duplications(
    sections: &[DocumentationEntry],
    threshold: f32,
) -> Result<Vec<DuplicationGroup>, Box<dyn Error>> {
    // Group entries by normalized hash for exact matches
    let mut exact_matches: HashMap<[u8; 32], Vec<&DocumentationEntry>> = HashMap::new();
    for section in sections {
        exact_matches
            .entry(section.normalized_hash)
            .or_insert_with(Vec::new)
            .push(section);
    }

    // Create duplication groups from exact matches
    let mut duplication_groups = Vec::new();
    for (_, matches) in exact_matches {
        if matches.len() > 1 {
            duplication_groups.push(DuplicationGroup {
                similarity: 1.0,
                entries: matches.into_iter().cloned().collect(),
            });
        }
    }

    // Find fuzzy matches using similarity score
    if sections.len() > 1 {
        // Compare each section with every other section
        let mut fuzzy_matches = Vec::new();

        for i in 0..sections.len() {
            for j in i + 1..sections.len() {
                let similarity = calculate_similarity_score(&sections[i], &sections[j]);

                if similarity >= threshold && similarity < 1.0 {
                    fuzzy_matches.push((i, j, similarity));
                }
            }
        }

        // Group fuzzy matches
        if !fuzzy_matches.is_empty() {
            // Use a simple greedy algorithm to group matches
            // A more sophisticated clustering algorithm would be better in production
            let mut used = vec![false; sections.len()];

            for (i, j, similarity) in fuzzy_matches {
                if !used[i] && !used[j] {
                    duplication_groups.push(DuplicationGroup {
                        similarity,
                        entries: vec![sections[i].clone(), sections[j].clone()],
                    });

                    used[i] = true;
                    used[j] = true;
                }
            }
        }
    }

    Ok(duplication_groups)
}

/// Calculate similarity score between two documentation entries
pub fn calculate_similarity_score(a: &DocumentationEntry, b: &DocumentationEntry) -> f32 {
    // Since we don't store content_snippet, we'll use a simpler approach
    // Compare normalized hashes for exact matches, and use title/section similarity

    // If normalized hashes are identical, it's a 100% match
    if a.normalized_hash == b.normalized_hash {
        return 1.0;
    }

    // Otherwise, calculate basic similarity based on title and section
    let a_title_words: Vec<&str> = a.title.split_whitespace().collect();
    let b_title_words: Vec<&str> = b.title.split_whitespace().collect();

    let a_section_words: Vec<&str> = a.section.split_whitespace().collect();
    let b_section_words: Vec<&str> = b.section.split_whitespace().collect();

    // Combine title and section words
    let mut a_words = a_title_words;
    a_words.extend(a_section_words);
    let mut b_words = b_title_words;
    b_words.extend(b_section_words);

    // Create sets
    let a_set: std::collections::HashSet<&str> = a_words.into_iter().collect();
    let b_set: std::collections::HashSet<&str> = b_words.into_iter().collect();

    // Calculate Jaccard similarity: |A ∩ B| / |A ∪ B|
    let intersection_size = a_set.intersection(&b_set).count();
    let union_size = a_set.union(&b_set).count();

    if union_size == 0 {
        0.0
    } else {
        intersection_size as f32 / union_size as f32
    }
}
