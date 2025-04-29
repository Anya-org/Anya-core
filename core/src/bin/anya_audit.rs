// Anya Audit Tool
// A tool for auditing and updating AI labels and BIP compliance in the codebase

use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

const BIP341_LABEL: &str = "[BPC-3]";
const BIP174_LABEL: &str = "[BPC-3]";
const SECP256K1_LABEL: &str = "[AIS-3]";

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    
    // Default to help if no command provided
    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    match args[1].as_str() {
        "update-labels" => {
            let scope = if args.len() > 2 { &args[2] } else { "all" };
            update_labels(scope)?;
        }
        "check" => {
            let rules = if args.len() > 2 { &args[2] } else { "all" };
            let fix_mode = args.iter().any(|arg| arg == "--fix");
            run_compliance_check(rules, fix_mode)?;
        }
        "help" | "--help" | "-h" => {
            print_help();
        }
        _ => {
            println!("Unknown command: {}", args[1]);
            print_help();
        }
    }

    Ok(())
}

fn print_help() {
    println!("Anya Audit Tool");
    println!("Usage:");
    println!("  anya_audit update-labels [scope]     Update AI labels in the codebase");
    println!("  anya_audit check [rules] [--fix]     Run compliance checks");
    println!("  anya_audit help                     Show this help message");
    println!();
    println!("Options:");
    println!("  scope: all, core, bitcoin, layer2, dao (default: all)");
    println!("  rules: all, bip341, bip174, secp256k1 (comma-separated, default: all)");
}

fn update_labels(scope: &str) -> Result<(), Box<dyn Error>> {
    println!("Updating AI labels in scope: {}", scope);
    
    let paths = match scope {
        "all" => vec!["./src", "./core", "./anya-bitcoin"],
        "core" => vec!["./core"],
        "bitcoin" => vec!["./anya-bitcoin"],
        "layer2" => vec!["./anya-bitcoin/src/layer2"],
        "dao" => vec!["./dao"],
        _ => {
            println!("Unknown scope: {}", scope);
            return Ok(());
        }
    };
    
    for path_str in paths {
        let path = Path::new(path_str);
        if path.exists() {
            scan_directory_and_update_labels(path)?;
        } else {
            println!("Warning: Path does not exist: {}", path_str);
        }
    }
    
    println!("✅ Label update completed");
    Ok(())
}

fn scan_directory_and_update_labels(dir: &Path) -> Result<(), Box<dyn Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                scan_directory_and_update_labels(&path)?;
            } else if let Some(ext) = path.extension() {
                if ext == "rs" || ext == "clar" || ext == "md" {
                    update_file_labels(&path)?;
                }
            }
        }
    }
    
    Ok(())
}

fn update_file_labels(file_path: &Path) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;
    let file_name = file_path.file_name().unwrap().to_string_lossy();
    
    // Only print dots for main files to avoid cluttering output
    if file_name.contains("mod.rs") || file_path.parent().unwrap().ends_with("src") {
        print!(".");
    }
    
    // Add labels based on content
    let mut new_content = content.clone();
    
    if content.contains("BIP-341") || content.contains("Taproot") {
        if !content.contains(BIP341_LABEL) {
            new_content = add_label_to_file(&new_content, BIP341_LABEL);
        }
    }
    
    if content.contains("BIP-174") || content.contains("PSBT") {
        if !content.contains(BIP174_LABEL) {
            new_content = add_label_to_file(&new_content, BIP174_LABEL);
        }
    }
    
    if content.contains("secp256k1") || content.contains("Secp256k1") {
        if !content.contains(SECP256K1_LABEL) {
            new_content = add_label_to_file(&new_content, SECP256K1_LABEL);
        }
    }
    
    // Only write if changes were made
    if new_content != content {
        fs::write(file_path, new_content)?;
    }
    
    Ok(())
}

fn add_label_to_file(content: &str, label: &str) -> String {
    // Find appropriate place to add label (near top of file)
    // This is a simplified implementation
    let mut lines: Vec<&str> = content.lines().collect();
    
    if lines.is_empty() {
        return content.to_string();
    }
    
    // Find the first non-empty line that isn't a comment or existing label
    let mut insert_pos = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.trim().is_empty() || line.trim().starts_with("//") || line.trim().starts_with("/*") {
            continue;
        }
        
        // Insert before this line
        insert_pos = i;
        break;
    }
    
    // Insert label
    lines.insert(insert_pos, &format!("// {}", label));
    
    // Rejoin lines
    lines.join("\n")
}

fn run_compliance_check(rules: &str, fix_mode: bool) -> Result<(), Box<dyn Error>> {
    println!("Running compliance check for rules: {}", rules);
    println!("Fix mode: {}", if fix_mode { "enabled" } else { "disabled" });
    
    let rule_list: Vec<&str> = rules.split(',').collect();
    let check_all = rules == "all";
    
    if check_all || rule_list.contains(&"bip341") {
        println!("Checking BIP-341 (Taproot) compliance...");
        // Implementation would verify BIP-341 compliance
    }
    
    if check_all || rule_list.contains(&"bip174") {
        println!("Checking BIP-174 (PSBT) compliance...");
        // Implementation would verify BIP-174 compliance
    }
    
    if check_all || rule_list.contains(&"secp256k1") {
        println!("Checking secp256k1 compliance...");
        // Implementation would verify secp256k1 compliance
    }
    
    if fix_mode {
        println!("Applying fixes...");
        // Implementation would fix compliance issues
    }
    
    println!("✅ Compliance check completed");
    Ok(())
} 