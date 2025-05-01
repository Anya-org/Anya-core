# Hardware Optimization Features Guide

This document explains how to enable GPU and NPU acceleration in the Anya Core hardware optimization framework.

## Features Overview

The Universal Adaptive Hardware Optimization Framework automatically detects and leverages available hardware accelerators to optimize Bitcoin consensus operations while maintaining full protocol compliance.

### Available Hardware Accelerators

1. **CPU Architecture-Specific Optimizations**
   - RISC-V Optimizations (RVV, Crypto Extensions)
   - ARM Optimizations (NEON, SVE, big.LITTLE awareness)
   - x86_64 Optimizations:
     - AMD Optimizations (CCX-aware threading, Zen architecture)
     - Intel Optimizations (AVX-512, cache hierarchy)

2. **GPU Acceleration**
   - CUDA (NVIDIA GPUs)
   - ROCm (AMD GPUs)
   - OpenCL (Cross-platform)
   - Vulkan Compute (Cross-platform)
   - Metal (Apple)

3. **NPU Acceleration**
   - Apple Neural Engine
   - Intel NPU
   - Google TPU
   - Qualcomm AI Engine

## Building with Hardware Acceleration

### Prerequisites

Depending on which hardware acceleration you want to use, you'll need the corresponding SDKs/libraries:

- **CUDA**: Install NVIDIA CUDA Toolkit 11.0+
- **ROCm**: Install AMD ROCm 4.0+
- **OpenCL**: Install OpenCL runtime for your platform
- **Metal**: Requires macOS with Apple Silicon or compatible GPU
- **Neural Engines**: Requires compatible hardware and SDKs

### Enabling Features in Cargo.toml

Add the following to your dependencies section in `Cargo.toml`:

```toml
[dependencies]
anya-core = { version = "0.1.0", features = ["gpu-acceleration", "npu-acceleration"] }
```

Available features:

- `gpu-acceleration`: Enable all GPU backends (CUDA, ROCm, OpenCL, Vulkan, Metal)
- `npu-acceleration`: Enable NPU support where available
- `cuda`: Enable CUDA support specifically
- `rocm`: Enable ROCm support specifically
- `opencl`: Enable OpenCL support specifically
- `vulkan`: Enable Vulkan Compute support
- `metal`: Enable Metal support (macOS only)

### Automatic Hardware Detection

The framework automatically detects available hardware at runtime and selects the most performant implementation. You can use the `hardware_optimization::integration::create_accelerated_optimizer()` function to get an optimizer that automatically leverages the best available hardware:

```rust
let manager = hardware_optimization::integration::create_accelerated_optimizer().await?;
```

Or you can specifically request a GPU or NPU accelerated implementation:

```rust
// GPU acceleration
let manager = hardware_optimization::HardwareOptimizationManager::with_gpu().await?;

// NPU acceleration
let manager = hardware_optimization::HardwareOptimizationManager::with_npu().await?;
```

## Running the Demo

The hardware optimization framework includes a demo application that benchmarks different operations across available hardware. To run it:

```bash
cargo run --example hardware_optimization_demo --features="gpu-acceleration npu-acceleration"
```

## Benchmarking Your Hardware

To generate a comprehensive benchmark report for your specific hardware:

```bash
cargo run --example hardware_optimization_benchmark --features="gpu-acceleration npu-acceleration"
```

This will produce a markdown report showing the performance of different operations across CPU, GPU, and NPU implementations.

## Performance Considerations

- **Batch Operations**: GPU and NPU acceleration provides the most benefit for batch operations like batch signature verification where parallelism can be exploited.
- **Memory Transfer**: For small operations, the overhead of transferring data to and from the GPU or NPU may outweigh the benefits. The framework automatically selects the most efficient implementation based on input size.
- **Power Usage**: GPU acceleration typically uses more power than CPU-only implementations. The framework allows tuning for power efficiency via the `PowerTarget` option in the `WorkloadProfile`.

## Bitcoin Protocol Compliance

All hardware-accelerated implementations undergo rigorous correctness verification to ensure they produce identical results to the reference implementation, maintaining full Bitcoin protocol compliance.
