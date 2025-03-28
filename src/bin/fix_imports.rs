#![feature(edition2021)]
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Fixing import errors...");
    
    // Common import fixes
    let fixes = [
        // Bitcoin imports
        (r"use bitcoin::TxIn;", r"use bitcoin::{TxIn, Transaction, TxOut, OutPoint};"),
        
        // Web5 imports
        (r"use web5::\{", r"use web5::{Web5Adapter, Web5Config, Web5Error, "),
        
        // HSM imports 
        (r"use crate::security::hsm::provider::\{", 
         r"use crate::security::hsm::provider::{HsmProvider, HsmProviderType, HsmType, SimulatorHsmProvider, SoftwareHsmProvider, HardwareHsmProvider, BitcoinHsmProvider};"),
        
        // Crypto fixes
        (r"Sha256::hash", r"bitcoin::hashes::sha256::Hash::hash"),
        (r"opcodes::", r"bitcoin::blockdata::opcodes::all::"),
        
        // Missing structs
        (r"let hsm = Yubihsm2Provider", r"use crate::security::hsm::Yubihsm2Provider;\n        let hsm = Yubihsm2Provider"),
        (r"MLAgentSystem", r"crate::ml::agents::MLAgentSystem"),
        (r"TokenomicsEngine", r"crate::tokenomics::TokenomicsEngine"),
    ];
    
    fix_files_recursively(Path::new("src"), &fixes)?;
    
    println!("Done fixing imports!");
    Ok(())
}

fn fix_files_recursively(dir: &Path, fixes: &[(& str, &str)]) -> Result<(), Box<dyn std::error::Error>> {
    if !dir.is_dir() {
        return Ok(());
    }
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            fix_files_recursively(&path, fixes)?;
        } else if let Some(ext) = path.extension() {
            if ext == "rs" {
                let content = fs::read_to_string(&path)?;
                let mut modified_content = content.clone();
                
                for (pattern, replacement) in fixes {
                    modified_content = modified_content.replace(pattern, replacement);
                }
                
                if content != modified_content {
                    println!("Fixing import errors in {:?}", path);
                    fs::write(&path, modified_content)?;
                }
            }
        }
    }
    
    Ok(())
}
