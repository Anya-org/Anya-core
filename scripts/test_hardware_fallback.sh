#!/bin/bash
# Script to test hardware fallback functionality

set -e

echo "========================================"
echo "Hardware Fallback Testing"
echo "========================================"

# Create a simple test program
cat > test_hardware_fallback.rs << 'EOL'
use anya_core::HardwareFallbackTest;
use anyhow::Result;

fn main() -> Result<()> {
    println!("Hardware Fallback Test");
    println!("=====================");
    
    // Create and initialize the hardware test
    let mut hardware_test = HardwareFallbackTest::new();
    println!("Hardware detection initialized.");
    
    // Detect available hardware
    hardware_test.detect_hardware()?;
    println!("\nHardware Detection Results:");
    println!("  - HSM available: {}", hardware_test.hsm_available);
    println!("  - SGX available: {}", hardware_test.sgx_available);
    println!("  - FPGA available: {}", hardware_test.fpga_available);
    println!("  - TPM available: {}", hardware_test.tpm_available);
    
    // Test HSM fallback
    println!("\nTesting HSM functionality:");
    let hsm_result = hardware_test.test_hsm_fallback()?;
    println!("  - HSM test result: {}", if hsm_result { "✅ Pass" } else { "❌ Fail" });
    
    // Test SGX fallback
    println!("\nTesting SGX functionality:");
    let sgx_result = hardware_test.test_sgx_fallback()?;
    println!("  - SGX test result: {}", if sgx_result { "✅ Pass" } else { "❌ Fail" });
    
    // Run all tests
    println!("\nRunning comprehensive tests:");
    let test_report = hardware_test.run_all_tests()?;
    
    println!("\nTest Summary:");
    println!("  - HSM Hardware: {}", if test_report.hsm_hardware_available { "Available" } else { "Unavailable" });
    println!("  - HSM Test: {}", if test_report.hsm_test_passed { "✅ Pass" } else { "❌ Fail" });
    println!("  - SGX Hardware: {}", if test_report.sgx_hardware_available { "Available" } else { "Unavailable" });
    println!("  - SGX Test: {}", if test_report.sgx_test_passed { "✅ Pass" } else { "❌ Fail" });
    
    println!("\nConclusion:");
    if !test_report.hsm_hardware_available && test_report.hsm_test_passed {
        println!("  ✅ HSM software fallback is working correctly");
    }
    if !test_report.sgx_hardware_available && test_report.sgx_test_passed {
        println!("  ✅ SGX software fallback is working correctly");
    }
    
    Ok(())
}
EOL

# Compile and run the test program
echo "Compiling hardware fallback test..."
rustc -o test_hardware_fallback test_hardware_fallback.rs --extern anya_core=./target/debug/libanya_core.rlib

echo "Running hardware fallback test..."
./test_hardware_fallback

# Cleanup
rm -f test_hardware_fallback test_hardware_fallback.rs

echo ""
echo "========================================"
echo "Hardware Fallback Test Complete"
echo "========================================" 