#!/bin/bash
# Test script for anya-core
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Starting anya-core test...${NC}"

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust is not installed.${NC}"
    echo "Please install Rust and Cargo: https://rustup.rs/"
    exit 1
fi

# Check current directory
if [[ ! -f "Cargo.toml" ]]; then
    echo -e "${RED}Error: This script must be run from the anya-core directory.${NC}"
    exit 1
fi

# Run Rust tests
echo -e "${YELLOW}Running Rust tests...${NC}"
cargo test

# Optional: Build the project
echo -e "${YELLOW}Building the project...${NC}"
cargo build

# Create a simple test file to verify core functionality
echo -e "${YELLOW}Creating test file...${NC}"
cat > test_core.rs << 'EOL'
use anya_core::{AnyaCore, AnyaConfig};

fn main() {
    println!("Testing anya-core...");
    
    match AnyaCore::default() {
        Ok(anya) => {
            println!("Successfully initialized anya-core!");
            
            let status = anya.get_status().unwrap();
            println!("System status:");
            println!("  ML enabled: {}", status.ml_enabled);
            println!("  Web5 enabled: {}", status.web5_enabled);
            println!("  Bitcoin enabled: {}", status.bitcoin_enabled);
            println!("  DAO enabled: {}", status.dao_enabled);
            
            for component in status.component_status {
                println!("Component {}: operational={}, health={}", 
                         component.name, component.operational, component.health_score);
            }
            
            println!("System is operational: {}", anya.is_operational());
        },
        Err(e) => {
            println!("Failed to initialize anya-core: {}", e);
            std::process::exit(1);
        }
    }
}
EOL

# Build and run the test file
echo -e "${YELLOW}Building and running test file...${NC}"
rustc -L target/debug -L target/debug/deps --extern anya_core=target/debug/libanya.rlib test_core.rs -o test_core

# Run the test executable
echo -e "${YELLOW}Running test executable...${NC}"
./test_core

# Cleanup
echo -e "${YELLOW}Cleaning up...${NC}"
rm -f test_core test_core.rs

echo -e "${GREEN}All tests completed successfully!${NC}"

# Verify operational status
echo -e "${BLUE}Verifying operational status...${NC}"
if cargo run --bin anya-status 2>/dev/null; then
    echo -e "${GREEN}Anya system is operational!${NC}"
else
    echo -e "${YELLOW}Status binary not found, running alternative check...${NC}"
    cat > check_status.rs << 'EOL'
use anya_core::AnyaCore;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let anya = AnyaCore::default()?;
    if anya.is_operational() {
        println!("Anya system is operational!");
        Ok(())
    } else {
        Err("System is not operational".into())
    }
}
EOL
    
    rustc -L target/debug -L target/debug/deps --extern anya_core=target/debug/libanya.rlib check_status.rs -o check_status
    
    if ./check_status; then
        echo -e "${GREEN}Anya system is operational!${NC}"
    else
        echo -e "${RED}Anya system is not operational. Please check the logs.${NC}"
        exit 1
    fi
    
    rm -f check_status check_status.rs
fi

echo -e "${GREEN}Anya-core is ready for packaging!${NC}"
exit 0 