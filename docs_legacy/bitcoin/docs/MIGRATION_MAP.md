# Bitcoin Implementation Migration Map

This document maps the source files from the original structure to their new locations in the reorganized structure.

## Core Bitcoin Implementation

| Original Location | New Location | Description |
|-------------------|--------------|-------------|
| `anya-bitcoin/src/core/` | `reorganized/bitcoin/core/` | Core Bitcoin functionality |
| `src/bitcoin/bip340.rs` | `reorganized/bitcoin/core/consensus/bip340.rs` | BIP-340 implementation |
| `src/bitcoin/bip341.rs` | `reorganized/bitcoin/core/consensus/bip341.rs` | BIP-341 implementation |
| `src/bitcoin/validation.rs` | `reorganized/bitcoin/core/consensus/validation.rs` | Validation logic |
| `src/bitcoin/merkle.rs` | `reorganized/bitcoin/core/consensus/merkle.rs` | Merkle tree implementation |
| `src/bitcoin/protocol.rs` | `reorganized/bitcoin/protocol/core_protocol.rs` | Protocol implementation |

## Layer 2 Implementations

| Original Location | New Location | Description |
|-------------------|--------------|-------------|
| `anya-bitcoin/src/layer2/` | `reorganized/bitcoin/layer2/` | Layer 2 implementation base |
| `src/layer2/bob/` | `reorganized/bitcoin/layer2/bob/` | Bitcoin Optimistic Blockchain |
| `src/layer2/lightning/` | `reorganized/bitcoin/layer2/lightning/` | Lightning Network implementation |
| `src/layer2/rgb/` | `reorganized/bitcoin/layer2/rgb/` | RGB Protocol implementation |
| `src/layer2/rsk/` | `reorganized/bitcoin/layer2/rsk/` | RSK integration |
| `src/bitcoin/layer2/rgb/` | `reorganized/bitcoin/layer2/rgb/` | RGB Protocol (merged) |

## Testing Infrastructure

| Original Location | New Location | Description |
|-------------------|--------------|-------------|
| `tests/bitcoin/` | `reorganized/bitcoin/testing/` | Bitcoin tests base |
| `tests/bitcoin/riscv_tests.rs` | `reorganized/bitcoin/testing/riscv/riscv_tests.rs` | RISC-V tests |
| `tests/bitcoin/riscv_vm_tests.rs` | `reorganized/bitcoin/testing/riscv/riscv_vm_tests.rs` | RISC-V VM tests |
| `tests/bitcoin/cross_layer_tests.rs` | `reorganized/bitcoin/testing/integration/cross_layer_tests.rs` | Cross-layer integration tests |
| `src/bitcoin/tests/` | `reorganized/bitcoin/testing/core/` | Core Bitcoin tests |

## Documentation

| Original Location | New Location | Description |
|-------------------|--------------|-------------|
| `docs/bitcoin/` | `reorganized/bitcoin/docs/` | Bitcoin documentation base |
| `docs/bitcoin/LAYER2_SUPPORT.md` | `reorganized/bitcoin/docs/layer2/OVERVIEW.md` | Layer 2 documentation |
| `docs/architecture/` (Bitcoin-related) | `reorganized/bitcoin/docs/architecture/` | Architecture documentation |
| `docs/standards/` (Bitcoin-related) | `reorganized/bitcoin/docs/standards/` | Standards documentation | 