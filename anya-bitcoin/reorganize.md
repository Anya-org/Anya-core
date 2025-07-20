# Bitcoin Implementation Reorganization Guide

This document provides a step-by-step guide for implementing the reorganization of the Bitcoin codebase.

## Reorganization Steps

1. **Prepare the Directory Structure**

   ```bash
   # Create the main directory structure
   mkdir -p reorganized/bitcoin/{core,layer2,protocol,testing,docs,ports,adapters,riscv,security}
   
   # Create core subdirectories
   mkdir -p reorganized/bitcoin/core/{consensus,mempool,network,script}
   
   # Create layer2 subdirectories
   mkdir -p reorganized/bitcoin/layer2/{framework,bob,lightning,rgb,rsk,dlc,taproot_assets}
   
   # Create testing subdirectories
   mkdir -p reorganized/bitcoin/testing/{core,layer2,riscv,integration}
   
   # Create documentation subdirectories
   mkdir -p reorganized/bitcoin/docs/{architecture,standards,layer2}
   
   # Create adapters subdirectories
   mkdir -p reorganized/bitcoin/adapters/{rpc,storage,protocols}
   
   # Create RISC-V subdirectories
   mkdir -p reorganized/bitcoin/riscv/{vm,instructions,contracts}
   ```

2. **Copy Core Files**

   ```bash
   # Copy core consensus files
   cp anya-bitcoin/src/core/consensus/* reorganized/bitcoin/core/consensus/
   cp src/bitcoin/bip340.rs reorganized/bitcoin/core/consensus/
   cp src/bitcoin/bip341.rs reorganized/bitcoin/core/consensus/
   cp src/bitcoin/validation.rs reorganized/bitcoin/core/consensus/
   cp src/bitcoin/merkle.rs reorganized/bitcoin/core/consensus/
   
   # Copy core mempool files
   cp anya-bitcoin/src/core/mempool/* reorganized/bitcoin/core/mempool/
   
   # Copy core network files
   cp anya-bitcoin/src/core/network/* reorganized/bitcoin/core/network/
   
   # Copy core script files
   cp anya-bitcoin/src/core/script/* reorganized/bitcoin/core/script/
   cp src/bitcoin/taproot.rs reorganized/bitcoin/core/script/
   ```

3. **Copy Layer 2 Files**

   ```bash
   # Copy framework files
   cp anya-bitcoin/src/layer2/framework/* reorganized/bitcoin/layer2/framework/
   
   # Copy BOB files
   cp -r src/layer2/bob/* reorganized/bitcoin/layer2/bob/
   
   # Copy Lightning files
   cp -r src/layer2/lightning/* reorganized/bitcoin/layer2/lightning/
   cp src/bitcoin/lightning.rs reorganized/bitcoin/layer2/lightning/
   
   # Copy RGB files
   cp -r src/layer2/rgb/* reorganized/bitcoin/layer2/rgb/
   cp -r src/bitcoin/layer2/rgb/* reorganized/bitcoin/layer2/rgb/
   
   # Copy RSK files
   cp -r src/layer2/rsk/* reorganized/bitcoin/layer2/rsk/
   
   # Copy DLC files
   cp -r src/bitcoin/dlc/* reorganized/bitcoin/layer2/dlc/
   ```

4. **Copy Protocol Files**

   ```bash
   cp src/bitcoin/protocol.rs reorganized/bitcoin/protocol/core_protocol.rs
   cp anya-bitcoin/src/protocol/* reorganized/bitcoin/protocol/
   ```

5. **Copy Testing Files**

   ```bash
   # Copy core tests
   cp -r src/bitcoin/tests/* reorganized/bitcoin/testing/core/
   
   # Copy Layer 2 tests
   cp tests/bitcoin/validation_test.rs reorganized/bitcoin/testing/layer2/
   
   # Copy RISC-V tests
   cp tests/bitcoin/riscv_tests.rs reorganized/bitcoin/testing/riscv/
   cp tests/bitcoin/riscv_vm_tests.rs reorganized/bitcoin/testing/riscv/
   
   # Copy integration tests
   cp tests/bitcoin/cross_layer_tests.rs reorganized/bitcoin/testing/integration/
   cp tests/bitcoin/layer3_tests.rs reorganized/bitcoin/testing/integration/
   cp tests/bitcoin/vm_layer_tests.rs reorganized/bitcoin/testing/integration/
   ```

6. **Copy Documentation Files**

   ```bash
   # Copy architecture documentation
   cp docs/HEXAGONAL.md reorganized/bitcoin/docs/architecture/
   
   # Copy Layer 2 documentation
   cp docs/bitcoin/LAYER2_SUPPORT.md reorganized/bitcoin/docs/layer2/OVERVIEW.md
   ```

7. **Create Port Interfaces**

   ```bash
   # Create port files based on hexagonal architecture
   touch reorganized/bitcoin/ports/blockchain_port.rs
   touch reorganized/bitcoin/ports/transaction_port.rs
   touch reorganized/bitcoin/ports/layer2_port.rs
   ```

8. **Add Implementation Documentation**

   ```bash
   # Add implementation documentation
   cp -r docs/bitcoin/* reorganized/bitcoin/docs/
   ```

## Validation Process

After completing the reorganization, validate the structure:

1. **Ensure All Files Are Correctly Placed**

   ```bash
   find reorganized/bitcoin -type f | sort > reorganized_files.txt
   ```

2. **Verify No Duplicated Code**
   Check for any duplicated implementations across:
   - Core Bitcoin functionality
   - Layer 2 implementations
   - Testing code

3. **Update References and Imports**
   Update all module references and imports in the copied files to reflect the new structure.

4. **Run Tests**

   ```bash
   cd reorganized/bitcoin
   cargo test
   ```

## Integration with Existing Codebase

To integrate the reorganized structure with the existing codebase:

1. **Create a New Branch**

   ```bash
   git checkout -b bitcoin-reorganization
   ```

2. **Replace Existing Implementation**

   ```bash
   # Backup existing implementation
   mv anya-bitcoin anya-bitcoin.bak
   mv src/bitcoin src/bitcoin.bak
   
   # Move reorganized implementation
   mv reorganized/bitcoin anya-bitcoin
   ```

3. **Update Cargo.toml**
   Update package references in Cargo.toml files to reflect the new structure.

4. **Commit Changes**

   ```bash
   git add .
   git commit -m "feat(bitcoin): Reorganize Bitcoin implementation following hexagonal architecture"
   ```

## Additional Notes

- The reorganization maintains compliance with official Bitcoin Improvement Proposals (BIPs) standards
- The hexagonal architecture ensures clean separation of concerns
- All components are properly labeled according to the AI labeling system 
