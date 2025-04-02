// Find Duplicate Code Modules
//
// This tool analyzes the Anya Core codebase to identify potentially redundant
// modules and suggest consolidation opportunities.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::error::Error;
use std::process::Command;

/// Module information
struct ModuleInfo {
    path: PathBuf,
    size: u64,
    functions: Vec<String>,
    dependencies: Vec<String>,
}

/// Group of potentially redundant modules
struct DuplicateGroup {
    name: String,
    modules: Vec<PathBuf>,
    similarity_score: f64,
    consolidation_path: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("🔍 Analyzing Anya Core codebase for redundant modules...");
    
    // Collect all Rust files
    let rust_files = find_rust_files(".")?;
    println!("Found {} Rust files to analyze", rust_files.len());
    
    // Group files by module name
    let module_groups = group_by_module(&rust_files)?;
    println!("Grouped into {} module categories", module_groups.len());
    
    // Find potential duplicates
    let duplicates = find_potential_duplicates(&module_groups)?;
    
    // Output results
    if duplicates.is_empty() {
        println!("✅ No significant redundancy detected!");
    } else {
        println!("\n🚨 Identified {} potentially redundant module groups:", duplicates.len());
        for (i, group) in duplicates.iter().enumerate() {
            println!("\n{}. {} (Similarity: {:.1}%)", i+1, group.name, group.similarity_score * 100.0);
            println!("   Affected modules:");
            for module in &group.modules {
                println!("   - {}", module.display());
            }
            if let Some(consolidation_path) = &group.consolidation_path {
                println!("   ➡️ Suggested consolidation: {}", consolidation_path.display());
            }
        }
        
        println!("\n📋 Consolidation Recommendations:");
        println!("1. Start with highest similarity score modules first");
        println!("2. Use the 'packages/' structure for consolidated implementations");
        println!("3. Add comprehensive tests for newly consolidated modules");
        println!("4. Update imports in dependent code");
        println!("5. Run this tool again after each consolidation to check progress");
    }
    
    Ok(())
}

/// Find all Rust files in the specified directory and subdirectories
fn find_rust_files(dir: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files = Vec::new();
    
    // Use a system command for better performance with large codebases
    let output = Command::new("find")
        .args(&[dir, "-name", "*.rs", "-type", "f"])
        .output()?;
    
    if output.status.success() {
        let paths = String::from_utf8(output.stdout)?;
        for path in paths.lines() {
            files.push(PathBuf::from(path));
        }
    } else {
        return Err(format!("Error finding Rust files: {}", 
            String::from_utf8_lossy(&output.stderr)).into());
    }
    
    Ok(files)
}

/// Group files by module name
fn group_by_module(files: &[PathBuf]) -> Result<HashMap<String, Vec<PathBuf>>, Box<dyn Error>> {
    let mut groups: HashMap<String, Vec<PathBuf>> = HashMap::new();
    
    for file in files {
        // Extract module name from path
        let module_name = extract_module_name(file)?;
        groups.entry(module_name).or_default().push(file.clone());
    }
    
    // Filter out groups with only one file
    groups.retain(|_, files| files.len() > 1);
    
    Ok(groups)
}

/// Extract module name from a file path
fn extract_module_name(path: &Path) -> Result<String, Box<dyn Error>> {
    let file_name = path.file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Invalid file path"))?
        .to_string_lossy();
    
    // Use the file name without extension unless it's mod.rs
    if file_name == "mod.rs" {
        if let Some(parent) = path.parent() {
            if let Some(module_name) = parent.file_name() {
                return Ok(module_name.to_string_lossy().to_string());
            }
        }
    }
    
    // Remove .rs extension
    let module_name = if file_name.ends_with(".rs") {
        file_name[..file_name.len() - 3].to_string()
    } else {
        file_name.to_string()
    };
    
    Ok(module_name)
}

/// Find potential duplicates based on module names and content similarity
fn find_potential_duplicates(
    module_groups: &HashMap<String, Vec<PathBuf>>
) -> Result<Vec<DuplicateGroup>, Box<dyn Error>> {
    let mut duplicates = Vec::new();
    
    // Check each group of files with the same module name
    for (name, paths) in module_groups {
        // Skip if not a common redundancy pattern we're looking for
        if !is_target_module(name) {
            continue;
        }
        
        // Calculate similarity between files
        let similarity = calculate_similarity(paths)?;
        
        // Only consider groups with significant similarity
        if similarity > 0.3 {
            let suggested_path = suggest_consolidation_path(name, paths)?;
            
            duplicates.push(DuplicateGroup {
                name: name.clone(),
                modules: paths.clone(),
                similarity_score: similarity,
                consolidation_path: Some(suggested_path),
            });
        }
    }
    
    // Sort by similarity score (highest first)
    duplicates.sort_by(|a, b| b.similarity_score.partial_cmp(&a.similarity_score).unwrap());
    
    Ok(duplicates)
}

/// Check if this is a module type we're targeting for consolidation
fn is_target_module(name: &str) -> bool {
    // Target common module names that tend to be duplicated
    let target_modules = [
        "psbt", "tapscript", "bitcoin", "validation", "security",
        "network", "http", "transport", "metrics", "lib"
    ];
    
    target_modules.contains(&name)
}

/// Calculate similarity between files
fn calculate_similarity(paths: &[PathBuf]) -> Result<f64, Box<dyn Error>> {
    if paths.len() < 2 {
        return Ok(0.0);
    }
    
    // For this simple version, we'll check:
    // 1. Similar imports
    // 2. Similar function names
    // 3. File size ratio
    
    let mut total_similarity = 0.0;
    let mut comparisons = 0;
    
    for i in 0..paths.len() {
        for j in i+1..paths.len() {
            let imports_a = extract_imports(&paths[i])?;
            let imports_b = extract_imports(&paths[j])?;
            let import_similarity = calculate_set_similarity(&imports_a, &imports_b);
            
            let fns_a = extract_functions(&paths[i])?;
            let fns_b = extract_functions(&paths[j])?;
            let function_similarity = calculate_set_similarity(&fns_a, &fns_b);
            
            let size_a = fs::metadata(&paths[i])?.len();
            let size_b = fs::metadata(&paths[j])?.len();
            let size_ratio = if size_a > size_b {
                size_b as f64 / size_a as f64
            } else {
                size_a as f64 / size_b as f64
            };
            
            // Combined similarity score (weighted)
            let similarity = 0.5 * import_similarity + 0.3 * function_similarity + 0.2 * size_ratio;
            total_similarity += similarity;
            comparisons += 1;
        }
    }
    
    if comparisons > 0 {
        Ok(total_similarity / comparisons as f64)
    } else {
        Ok(0.0)
    }
}

/// Calculate Jaccard similarity between two sets
fn calculate_set_similarity<T: Eq + std::hash::Hash>(set_a: &HashSet<T>, set_b: &HashSet<T>) -> f64 {
    if set_a.is_empty() && set_b.is_empty() {
        return 1.0;
    }
    
    let intersection_size = set_a.intersection(set_b).count();
    let union_size = set_a.union(set_b).count();
    
    intersection_size as f64 / union_size as f64
}

/// Extract imports from a Rust file
fn extract_imports(path: &Path) -> Result<HashSet<String>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut imports = HashSet::new();
    
    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        
        if trimmed.starts_with("use ") || trimmed.starts_with("extern crate ") {
            imports.insert(trimmed.to_string());
        }
    }
    
    Ok(imports)
}

/// Extract function names from a Rust file
fn extract_functions(path: &Path) -> Result<HashSet<String>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut functions = HashSet::new();
    
    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        
        // Very simple function detection
        if trimmed.starts_with("fn ") && trimmed.contains('(') {
            if let Some(name_end) = trimmed.find('(') {
                let fn_declaration = &trimmed[3..name_end];
                let fn_name = fn_declaration.trim().to_string();
                functions.insert(fn_name);
            }
        }
    }
    
    Ok(functions)
}

/// Suggest a path for consolidation
fn suggest_consolidation_path(name: &str, paths: &[PathBuf]) -> Result<PathBuf, Box<dyn Error>> {
    // Map module names to suggested package locations
    let package_mapping = [
        ("psbt", "packages/protocol-adapters/src/bitcoin/psbt.rs"),
        ("tapscript", "packages/protocol-adapters/src/bitcoin/tapscript.rs"),
        ("bitcoin", "packages/protocol-adapters/src/bitcoin/mod.rs"),
        ("validation", "packages/protocol-adapters/src/bitcoin/validation.rs"),
        ("http", "packages/mcp-interface/src/http.rs"),
        ("transport", "packages/mcp-interface/src/transport.rs"),
        ("metrics", "packages/metrics/src/lib.rs"),
        ("network", "packages/bitcoin-network/src/lib.rs"),
        ("security", "packages/core/src/security/mod.rs"),
    ];
    
    // Find the suggested path for this module name
    for (module, path) in package_mapping {
        if name == module {
            return Ok(PathBuf::from(path));
        }
    }
    
    // Default to the first path - not ideal, but a fallback
    Ok(paths[0].clone())
}
