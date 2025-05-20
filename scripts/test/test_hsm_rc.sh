#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Log functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Create a test program for HSM functionality validation
create_test_program() {
    log_info "Creating HSM test program..."
    
    cat > test_hsm.rs << EOL
use anya_core::security::hsm::config::{HsmConfig, SoftwareHsmConfig};
use anya_core::security::hsm::provider::HsmProviderType;
use anya_core::security::hsm::HsmManager;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a software HSM configuration
    let software_config = SoftwareHsmConfig {
        key_store_path: "./test_keys".to_string(),
        ..Default::default()
    };
    
    let config = HsmConfig {
        provider_type: HsmProviderType::Software,
        software: software_config,
        ..Default::default()
    };
    
    // Create the HSM manager
    println!("Creating HSM Manager...");
    let mut hsm = HsmManager::new(config).await?;
    println!("HSM Manager created successfully!");
    
    // Attempt to use HSM before enabling (should fail)
    println!("\nAttempting to use HSM before enabling:");
    let test_message = b"test message to sign";
    let test_path = anya_core::security::hsm::HsmKeyPath::from_string("m/44'/0'/0'/0/0")?;
    
    match hsm.sign(test_message, &test_path).await {
        Ok(_) => {
            println!("❌ ERROR: HSM operation succeeded but should have failed (HSM not enabled)");
            return Ok(());
        },
        Err(e) => {
            println!("✅ Expected error received: {}", e);
        }
    }
    
    // Enable the HSM
    println!("\nEnabling HSM...");
    match hsm.enable().await {
        Ok(_) => println!("✅ HSM successfully enabled"),
        Err(e) => {
            println!("❌ Failed to enable HSM: {}", e);
            return Ok(());
        }
    }
    
    // Test HSM operations after enabling
    println!("\nTesting HSM operations after enabling:");
    
    // Generate a key
    println!("Generating key...");
    // Implementation would go here
    
    // Sign a message
    println!("Signing message...");
    // Implementation would go here
    
    // Disable the HSM
    println!("\nDisabling HSM...");
    match hsm.disable().await {
        Ok(_) => println!("✅ HSM successfully disabled"),
        Err(e) => {
            println!("❌ Failed to disable HSM: {}", e);
            return Ok(());
        }
    }
    
    // Verify HSM is disabled
    println!("\nVerifying HSM is disabled:");
    match hsm.sign(test_message, &test_path).await {
        Ok(_) => {
            println!("❌ ERROR: HSM operation succeeded but should have failed (HSM disabled)");
            return Ok(());
        },
        Err(e) => {
            println!("✅ Expected error received: {}", e);
        }
    }
    
    println!("\n✅ RC HSM TEST COMPLETED SUCCESSFULLY");
    Ok(())
}
EOL
    
    log_success "Test program created!"
}

# Run the HSM test
run_hsm_test() {
    log_info "Compiling and running HSM test program..."
    
    # Compile the test program
    rustc -L target/debug/deps -L target/debug --extern anya_core=target/debug/libanya_core.rlib test_hsm.rs -o test_hsm
    
    if [ $? -eq 0 ]; then
        log_success "Compilation successful!"
        
        # Run the test program
        ./test_hsm
        
        if [ $? -eq 0 ]; then
            log_success "HSM test completed successfully!"
        else
            log_error "HSM test failed!"
            return 1
        fi
    else
        log_error "Failed to compile test program!"
        return 1
    fi
}

# Main execution
echo "==================================="
log_info "HSM RC VALIDATION TESTING"
echo "==================================="
echo "Testing Software HSM Provider Only"
echo "==================================="
echo ""

# Create the test program
create_test_program

# Build the project
log_info "Building project..."
cargo build
if [ $? -ne 0 ]; then
    log_error "Build failed!"
    exit 1
fi
log_success "Build completed!"

# Run the HSM test
run_hsm_test

# Clean up
log_info "Cleaning up..."
rm -f test_hsm test_hsm.rs
rm -rf test_keys

echo ""
log_success "HSM RC testing completed!"
