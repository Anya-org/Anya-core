//! Hardware detection module for optimization framework
//!
//! This module provides functions for detecting hardware capabilities
//! at runtime, including CPU vendor, architecture, core counts, cache topology,
//! available instruction set extensions, GPU capabilities, and NPU availability.
//!
//! The detection is performed at startup and cached for efficient access during
//! consensus operations, ensuring optimal hardware utilization without adding overhead.
//! The detection is performed in a non-invasive way that respects system resources
//! and privacy.

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::process::Command;
use serde::{Deserialize, Serialize};

use super::{Architecture, HardwareCapabilities, OptimizationError, Vendor};
use super::gpu::{GpuCapabilities, GpuVendor, GpuBackend, NpuType};

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;

#[cfg(target_arch = "riscv64")]
use std::arch::riscv64::*;

/// Detect hardware capabilities at runtime
pub async fn detect_hardware() -> Result<HardwareCapabilities, OptimizationError> {
    // Detect basic information first
    let architecture = detect_architecture()?;
    let vendor = detect_vendor()?;
    let model = detect_model_name()?;
    let core_count = detect_core_count()?;
    let thread_count = detect_thread_count()?;
    
    // Detect extended capabilities
    let vector_extensions = detect_vector_extensions(&architecture, &vendor)?;
    let crypto_extensions = detect_crypto_extensions(&architecture, &vendor)?;
    let cache_sizes = detect_cache_sizes()?;
    let numa_nodes = detect_numa_nodes()?;
    let topology = detect_topology(&architecture, &vendor)?;
    
    // Detect GPU and NPU capabilities
    let gpu_capabilities = detect_gpu_capabilities().await?;
    
    Ok(HardwareCapabilities {
        architecture,
        vendor,
        model,
        core_count,
        thread_count,
        vector_extensions,
        crypto_extensions,
        cache_sizes,
        numa_nodes,
        topology,
        gpu_capabilities: Some(gpu_capabilities),
    })
}

/// Detect GPU capabilities
pub async fn detect_gpu_capabilities() -> Result<GpuCapabilities, OptimizationError> {
    // Try platform-specific detection methods
    #[cfg(target_os = "windows")]
    if let Some(gpu_info) = detect_gpu_windows()? {
        return Ok(gpu_info);
    }
    
    #[cfg(target_os = "linux")]
    if let Some(gpu_info) = detect_gpu_linux()? {
        return Ok(gpu_info);
    }
    
    #[cfg(target_os = "macos")]
    if let Some(gpu_info) = detect_gpu_macos()? {
        return Ok(gpu_info);
    }
    
    // Fallback to generic GPU detection
    detect_gpu_generic()
}

/// Detect GPU capabilities on Windows
#[cfg(target_os = "windows")]
fn detect_gpu_windows() -> Result<Option<GpuCapabilities>, OptimizationError> {
    // First try to detect NVIDIA GPUs
    if let Some(gpu_info) = detect_nvidia_gpu_windows()? {
        return Ok(Some(gpu_info));
    }
    
    // Then try AMD GPUs
    if let Some(gpu_info) = detect_amd_gpu_windows()? {
        return Ok(Some(gpu_info));
    }
    
    // Then try Intel GPUs
    if let Some(gpu_info) = detect_intel_gpu_windows()? {
        return Ok(Some(gpu_info));
    }
    
    Ok(None)
}

/// Detect NVIDIA GPU on Windows
#[cfg(target_os = "windows")]
fn detect_nvidia_gpu_windows() -> Result<Option<GpuCapabilities>, OptimizationError> {
    // Try to run nvidia-smi
    let output = match Command::new("nvidia-smi").args(&["-L"]).output() {
        Ok(output) => output,
        Err(_) => return Ok(None), // nvidia-smi not available
    };
    
    if !output.status.success() {
        return Ok(None);
    }
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    if output_str.contains("GPU ") {
        // Parse NVIDIA GPU info
        let model = if let Some(model_start) = output_str.find("GPU ") {
            if let Some(model_end) = output_str[model_start..].find(":") {
                output_str[model_start + 4..model_start + model_end].trim().to_string()
            } else {
                "NVIDIA GPU".to_string()
            }
        } else {
            "NVIDIA GPU".to_string()
        };
        
        // Get memory info
        let memory_output = match Command::new("nvidia-smi").args(&["--query-gpu=memory.total", "--format=csv,noheader"]).output() {
            Ok(output) => output,
            Err(_) => return Ok(None),
        };
        
        let memory_str = String::from_utf8_lossy(&memory_output.stdout);
        let memory_mb = memory_str.trim().split_whitespace().next()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(1024);
        
        // Get compute capability
        let cc_output = match Command::new("nvidia-smi").args(&["--query-gpu=compute_cap", "--format=csv,noheader"]).output() {
            Ok(output) => output,
            Err(_) => return Ok(None),
        };
        
        let cc_str = String::from_utf8_lossy(&cc_output.stdout).trim().to_string();
        let cuda_compute_capability = if cc_str.contains(".") {
            let parts: Vec<&str> = cc_str.split(".").collect();
            if parts.len() == 2 {
                if let (Ok(major), Ok(minor)) = (parts[0].parse::<u8>(), parts[1].parse::<u8>()) {
                    Some((major, minor))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        
        return Ok(Some(GpuCapabilities {
            gpu_available: true,
            vendor: GpuVendor::Nvidia,
            model,
            memory_mb,
            compute_units: 16, // Placeholder
            backends: vec![GpuBackend::CUDA, GpuBackend::OpenCL],
            max_threads_per_block: 1024,
            cuda_compute_capability,
            npu_available: false,
            npu_type: None,
        }));
    }
    
    Ok(None)
}

/// Detect AMD GPU on Windows
#[cfg(target_os = "windows")]
fn detect_amd_gpu_windows() -> Result<Option<GpuCapabilities>, OptimizationError> {
    // Simplified detection using WMI
    // In a real implementation, this would use the Windows Management Instrumentation (WMI)
    // to query AMD GPU information
    
    // Try running rocm-smi
    let output = match Command::new("rocm-smi").output() {
        Ok(output) => output,
        Err(_) => return Ok(None), // rocm-smi not available
    };
    
    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        if output_str.contains("GPU[") {
            return Ok(Some(GpuCapabilities {
                gpu_available: true,
                vendor: GpuVendor::AMD,
                model: "AMD GPU".to_string(),
                memory_mb: 4096, // Placeholder
                compute_units: 32, // Placeholder
                backends: vec![GpuBackend::ROCm, GpuBackend::OpenCL],
                max_threads_per_block: 1024,
                cuda_compute_capability: None,
                npu_available: false,
                npu_type: None,
            }));
        }
    }
    
    Ok(None)
}

/// Detect Intel GPU on Windows
#[cfg(target_os = "windows")]
fn detect_intel_gpu_windows() -> Result<Option<GpuCapabilities>, OptimizationError> {
    // Simplified detection
    // In a real implementation, this would use WMI to query Intel GPU information
    
    // For now, just check if OpenCL is available with Intel platforms
    let opencl_available = false; // Placeholder - would check for OpenCL properly
    
    if opencl_available {
        return Ok(Some(GpuCapabilities {
            gpu_available: true,
            vendor: GpuVendor::Intel,
            model: "Intel GPU".to_string(),
            memory_mb: 2048, // Placeholder
            compute_units: 16, // Placeholder
            backends: vec![GpuBackend::OpenCL, GpuBackend::OneAPI],
            max_threads_per_block: 256,
            cuda_compute_capability: None,
            npu_available: false,
            npu_type: None,
        }));
    }
    
    Ok(None)
}

/// Detect GPU capabilities on Linux
#[cfg(target_os = "linux")]
fn detect_gpu_linux() -> Result<Option<GpuCapabilities>, OptimizationError> {
    // First try to detect NVIDIA GPUs
    if let Some(gpu_info) = detect_nvidia_gpu_linux()? {
        return Ok(Some(gpu_info));
    }
    
    // Then try AMD GPUs
    if let Some(gpu_info) = detect_amd_gpu_linux()? {
        return Ok(Some(gpu_info));
    }
    
    // Then try Intel GPUs
    if let Some(gpu_info) = detect_intel_gpu_linux()? {
        return Ok(Some(gpu_info));
    }
    
    Ok(None)
}

/// Detect NVIDIA GPU on Linux
#[cfg(target_os = "linux")]
fn detect_nvidia_gpu_linux() -> Result<Option<GpuCapabilities>, OptimizationError> {
    // Try to run nvidia-smi
    let output = match Command::new("nvidia-smi").args(&["-L"]).output() {
        Ok(output) => output,
        Err(_) => return Ok(None), // nvidia-smi not available
    };
    
    if !output.status.success() {
        return Ok(None);
    }
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    if output_str.contains("GPU ") {
        // Similar parsing to Windows implementation
        return Ok(Some(GpuCapabilities {
            gpu_available: true,
            vendor: GpuVendor::Nvidia,
            model: "NVIDIA GPU".to_string(),
            memory_mb: 4096, // Placeholder
            compute_units: 16, // Placeholder
            backends: vec![GpuBackend::CUDA, GpuBackend::OpenCL],
            max_threads_per_block: 1024,
            cuda_compute_capability: Some((7, 5)), // Placeholder
            npu_available: false,
            npu_type: None,
        }));
    }
    
    Ok(None)
}

/// Detect AMD GPU on Linux
#[cfg(target_os = "linux")]
fn detect_amd_gpu_linux() -> Result<Option<GpuCapabilities>, OptimizationError> {
    // Try running rocm-smi
    let output = match Command::new("rocm-smi").output() {
        Ok(output) => output,
        Err(_) => return Ok(None), // rocm-smi not available
    };
    
    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        if output_str.contains("GPU[") {
            return Ok(Some(GpuCapabilities {
                gpu_available: true,
                vendor: GpuVendor::AMD,
                model: "AMD GPU".to_string(),
                memory_mb: 4096, // Placeholder
                compute_units: 32, // Placeholder
                backends: vec![GpuBackend::ROCm, GpuBackend::OpenCL],
                max_threads_per_block: 1024,
                cuda_compute_capability: None,
                npu_available: false,
                npu_type: None,
            }));
        }
    }
    
    // Check for amdgpu in lspci
    let lspci_output = match Command::new("lspci").args(&["-nn"]).output() {
        Ok(output) => output,
        Err(_) => return Ok(None),
    };
    
    let lspci_str = String::from_utf8_lossy(&lspci_output.stdout);
    if lspci_str.contains("AMD") && (lspci_str.contains("VGA") || lspci_str.contains("Display")) {
        return Ok(Some(GpuCapabilities {
            gpu_available: true,
            vendor: GpuVendor::AMD,
            model: "AMD GPU".to_string(),
            memory_mb: 2048, // Placeholder
            compute_units: 16, // Placeholder
            backends: vec![GpuBackend::OpenCL],
            max_threads_per_block: 256,
            cuda_compute_capability: None,
            npu_available: false,
            npu_type: None,
        }));
    }
    
    Ok(None)
}

/// Detect Intel GPU on Linux
#[cfg(target_os = "linux")]
fn detect_intel_gpu_linux() -> Result<Option<GpuCapabilities>, OptimizationError> {
    // Check for Intel GPU in lspci
    let lspci_output = match Command::new("lspci").args(&["-nn"]).output() {
        Ok(output) => output,
        Err(_) => return Ok(None),
    };
    
    let lspci_str = String::from_utf8_lossy(&lspci_output.stdout);
    if lspci_str.contains("Intel") && (lspci_str.contains("VGA") || lspci_str.contains("Display")) {
        return Ok(Some(GpuCapabilities {
            gpu_available: true,
            vendor: GpuVendor::Intel,
            model: "Intel GPU".to_string(),
            memory_mb: 1024, // Placeholder
            compute_units: 8, // Placeholder
            backends: vec![GpuBackend::OpenCL, GpuBackend::OneAPI],
            max_threads_per_block: 256,
            cuda_compute_capability: None,
            npu_available: false,
            npu_type: None,
        }));
    }
    
    Ok(None)
}

/// Detect GPU capabilities on macOS
#[cfg(target_os = "macos")]
fn detect_gpu_macos() -> Result<Option<GpuCapabilities>, OptimizationError> {
    // Check for Apple Silicon with Neural Engine
    if let Some(npu_info) = detect_apple_npu()? {
        return Ok(Some(npu_info));
    }
    
    // Check for Metal support
    let metal_available = true; // Placeholder - would properly check Metal
    
    if metal_available {
        return Ok(Some(GpuCapabilities {
            gpu_available: true,
            vendor: GpuVendor::Apple,
            model: "Apple GPU".to_string(),
            memory_mb: 4096, // Shared memory - placeholder
            compute_units: 8, // Placeholder
            backends: vec![GpuBackend::Metal],
            max_threads_per_block: 1024,
            cuda_compute_capability: None,
            npu_available: false,
            npu_type: None,
        }));
    }
    
    Ok(None)
}

/// Detect Apple Neural Engine
#[cfg(target_os = "macos")]
fn detect_apple_npu() -> Result<Option<GpuCapabilities>, OptimizationError> {
    // Check for Apple Silicon
    let output = match Command::new("sysctl").args(&["machdep.cpu.brand_string"]).output() {
        Ok(output) => output,
        Err(_) => return Ok(None),
    };
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    if output_str.contains("Apple") {
        // Apple Silicon detected, assume Neural Engine is available
        return Ok(Some(GpuCapabilities {
            gpu_available: true,
            vendor: GpuVendor::Apple,
            model: "Apple Silicon GPU".to_string(),
            memory_mb: 8192, // Shared memory - placeholder
            compute_units: 8, // Placeholder
            backends: vec![GpuBackend::Metal],
            max_threads_per_block: 1024,
            cuda_compute_capability: None,
            npu_available: true,
            npu_type: Some(NpuType::AppleNeuralEngine),
        }));
    }
    
    Ok(None)
}

/// Generic GPU detection fallback
fn detect_gpu_generic() -> Result<GpuCapabilities, OptimizationError> {
    // Fallback when no GPU is detected
    Ok(GpuCapabilities {
        gpu_available: false,
        vendor: GpuVendor::Other,
        model: "No GPU Detected".to_string(),
        memory_mb: 0,
        compute_units: 0,
        backends: Vec::new(),
        max_threads_per_block: 0,
        cuda_compute_capability: None,
        npu_available: false,
        npu_type: None,
    })
}

// Helper functions

// ... rest of the code remains the same ...
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn read_sysfs_value<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    Ok(line.trim().to_string())
}

fn parse_cache_size(size_str: &str) -> usize {
    let size_str = size_str.trim().to_lowercase();
    if let Some(suffix_idx) = size_str.find(|c: char| !c.is_digit(10)) {
        let size: usize = size_str[..suffix_idx].parse().unwrap_or(0);
        match &size_str[suffix_idx..] {
            "k" | "kb" => size * 1024,
            "m" | "mb" => size * 1024 * 1024,
            _ => size,
        }
    } else {
        size_str.parse().unwrap_or(0)
    }
}

#[cfg(target_arch = "aarch64")]
fn read_arm_implementer() -> io::Result<u32> {
    let cpuinfo = read_file_to_string("/proc/cpuinfo")?;
    for line in cpuinfo.lines() {
        if line.starts_with("CPU implementer") {
            if let Some(implementer) = line.split(':').nth(1) {
                if let Ok(val) = u32::from_str_radix(implementer.trim().trim_start_matches("0x"), 16) {
                    return Ok(val);
                }
            }
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "CPU implementer not found"))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_detect_architecture() {
        let arch = detect_architecture();
        println!("Detected architecture: {:?}", arch);
        
        #[cfg(target_arch = "x86_64")]
        assert_eq!(arch, Architecture::X86_64);
        
        #[cfg(target_arch = "aarch64")]
        assert_eq!(arch, Architecture::AArch64);
        
        #[cfg(target_arch = "riscv64")]
        assert_eq!(arch, Architecture::RISCV64);
    }
    
    #[test]
    fn test_detect_vendor() {
        let vendor = detect_vendor();
        println!("Detected vendor: {:?}", vendor);
    }
    
    #[test]
    fn test_cpu_counts() {
        let (cores, threads) = detect_cpu_counts();
        println!("Detected {} cores, {} threads", cores, threads);
        assert!(cores > 0);
        assert!(threads >= cores);
    }
    
    #[test]
    fn test_vector_extensions() {
        let extensions = detect_vector_extensions();
        println!("Detected vector extensions: {:?}", extensions);
    }
    
    #[tokio::test]
    async fn test_detect_hardware() {
        let capabilities = detect_hardware().await.unwrap();
        println!("Detected capabilities: {:?}", capabilities);
        assert!(capabilities.core_count > 0);
        assert!(capabilities.thread_count >= capabilities.core_count);
    }
}
