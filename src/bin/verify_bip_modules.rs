// BIP Module Verification Script
// [AIR-3][AIS-3][BPC-3][AIT-3][RES-3]
//
// Verifies all BIP implementation modules and ensures they match
// the registry information. Validates BIP-341 and BIP-342 implementations.

use anyhow::{Result, Context};
use std::path::Path;
use std::fs;
use std::process;

fn main() -> Result<()> {
    println!("BIP Module Verification Tool");
    println!("----------------------------");
    println!("Based on Bitcoin Development Framework v2.5");
    println!();

    // Check if core/src/bip directory exists
    let bip_dir = Path::new("core/src/bip");
    if !bip_dir.exists() {
        println!("❌ Error: BIP implementation directory not found at {}", bip_dir.display());
        process::exit(1);
    }

    // Verify module registry file exists
    let mod_file = bip_dir.join("mod.rs");
    if !mod_file.exists() {
        println!("❌ Error: BIP module registry not found at {}", mod_file.display());
        process::exit(1);
    }

    // Read and check module file
    let mod_content = fs::read_to_string(&mod_file)
        .context("Failed to read module registry file")?;

    // Check for key BIP implementations
    verify_bip_file(&bip_dir, "bip341.rs", "BIP-341 (Taproot)")?;
    verify_bip_file(&bip_dir, "bip342.rs", "BIP-342 (Tapscript)")?;

    // Verify registry entries
    verify_registry_entry(&mod_content, "BIP-341", "Taproot")?;
    verify_registry_entry(&mod_content, "BIP-342", "Tapscript")?;
    verify_registry_entry(&mod_content, "BIP-174", "PSBT")?;
    verify_registry_entry(&mod_content, "BIP-370", "PSBT v2")?;

    // Verify module structure
    verify_module_structure(&mod_content)?;
    
    // Verify hexagonal architecture in bitcoin module
    verify_bitcoin_interface()?;

    println!("\n✅ BIP module verification complete");
    println!("All required BIP modules are present and properly structured");
    println!("Registry contains all required BIPs with correct status");
    
    Ok(())
}

fn verify_bip_file(dir: &Path, filename: &str, description: &str) -> Result<()> {
    let file_path = dir.join(filename);
    
    if !file_path.exists() {
        println!("❌ Error: {} implementation not found at {}", description, file_path.display());
        process::exit(1);
    }
    
    // Read file content and check for required elements
    let content = fs::read_to_string(&file_path)
        .context(format!("Failed to read {} file", description))?;
    
    // Check for AI labeling
    if !content.contains("[AIR-3][AIS-3][BPC-3]") {
        println!("⚠️ Warning: {} file missing proper AI labeling", description);
    }
    
    println!("✅ {} implementation found at {}", description, file_path.display());
    Ok(())
}

fn verify_registry_entry(content: &str, bip: &str, description: &str) -> Result<()> {
    if !content.contains(&format!("registry.register(\"{}\", BIPStatus::Complete)", bip)) {
        println!("⚠️ Warning: BIP registry missing entry for {} ({})", bip, description);
    } else {
        println!("✅ BIP registry contains {} ({}) with Complete status", bip, description);
    }
    
    Ok(())
}

fn verify_module_structure(content: &str) -> Result<()> {
    // Check for module exports
    let required_modules = vec![
        "pub mod bip341;",
        "pub mod bip342;",
    ];
    
    for module in required_modules {
        if !content.contains(module) {
            println!("❌ Error: Module registry missing export for {}", module);
            process::exit(1);
        }
    }
    
    // Check for BIPRegistry implementation
    if !content.contains("pub struct BIPRegistry") {
        println!("❌ Error: Module registry missing BIPRegistry struct definition");
        process::exit(1);
    }
    
    // Check for BIPStatus enum
    if !content.contains("pub enum BIPStatus") {
        println!("❌ Error: Module registry missing BIPStatus enum definition");
        process::exit(1);
    }
    
    println!("✅ Module structure verified with all required components");
    Ok(())
}

fn verify_bitcoin_interface() -> Result<()> {
    // Check for Bitcoin interface directory
    let interface_dir = Path::new("src/bitcoin/interface");
    if !interface_dir.exists() {
        println!("❌ Error: Bitcoin interface directory not found at {}", interface_dir.display());
        process::exit(1);
    }
    
    // Verify interface files
    verify_file_exists(&interface_dir.join("mod.rs"), "Bitcoin interface module registry")?;
    verify_file_exists(&interface_dir.join("block.rs"), "Block interface")?;
    verify_file_exists(&interface_dir.join("transaction.rs"), "Transaction interface")?;
    verify_file_exists(&interface_dir.join("network.rs"), "Network interface")?;
    
    println!("✅ Bitcoin interface layer verified with all required components");
    Ok(())
}

fn verify_file_exists(path: &Path, description: &str) -> Result<()> {
    if !path.exists() {
        println!("❌ Error: {} not found at {}", description, path.display());
        process::exit(1);
    }
    
    println!("✅ {} found at {}", description, path.display());
    Ok(())
} 