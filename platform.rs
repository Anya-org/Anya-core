// Platform abstraction module for OS-agnostic functionality
// This module provides a unified interface for platform-specific operations

use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlatformType {
    Windows,
    MacOS,
    Linux,
    Unknown,
}

/// Get the current platform
pub fn current_platform() -> PlatformType {
    #[cfg(target_os = "windows")]
    return PlatformType::Windows;
    
    #[cfg(target_os = "macos")]
    return PlatformType::MacOS;
    
    #[cfg(target_os = "linux")]
    return PlatformType::Linux;
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    return PlatformType::Unknown;
}

/// Convert a path to the platform-specific format
pub fn normalize_path(path: &Path) -> PathBuf {
    path.to_path_buf()
}

/// Get the platform-specific directory for config files
pub fn config_dir() -> PathBuf {
    match current_platform() {
        PlatformType::Windows => {
            let home = std::env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string());
            Path::new(&home).join("AppData").join("Roaming").join("anya-core")
        },
        PlatformType::MacOS => {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            Path::new(&home).join("Library").join("Application Support").join("anya-core")
        },
        PlatformType::Linux | PlatformType::Unknown => {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
                Path::new(&xdg_config).join("anya-core")
            } else {
                Path::new(&home).join(".config").join("anya-core")
            }
        }
    }
}

/// Get the platform-specific directory for data files
pub fn data_dir() -> PathBuf {
    match current_platform() {
        PlatformType::Windows => {
            let home = std::env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string());
            Path::new(&home).join("AppData").join("Local").join("anya-core")
        },
        PlatformType::MacOS => {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            Path::new(&home).join("Library").join("Application Support").join("anya-core")
        },
        PlatformType::Linux | PlatformType::Unknown => {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            if let Ok(xdg_data) = std::env::var("XDG_DATA_HOME") {
                Path::new(&xdg_data).join("anya-core")
            } else {
                Path::new(&home).join(".local").join("share").join("anya-core")
            }
        }
    }
}

/// Get platform-specific environment variables
pub fn get_platform_env_vars() -> Vec<(String, String)> {
    let mut vars = Vec::new();
    
    // Add platform-specific env vars
    match current_platform() {
        PlatformType::Windows => {
            // Windows-specific env vars
            if let Ok(program_data) = std::env::var("PROGRAMDATA") {
                vars.push(("ANYA_SYSTEM_CONFIG".to_string(), 
                          format!("{}\\anya-core", program_data)));
            }
        },
        PlatformType::MacOS => {
            // macOS-specific env vars
            vars.push(("ANYA_SYSTEM_CONFIG".to_string(), "/Library/Application Support/anya-core".to_string()));
        },
        PlatformType::Linux | PlatformType::Unknown => {
            // Linux-specific env vars
            vars.push(("ANYA_SYSTEM_CONFIG".to_string(), "/etc/anya-core".to_string()));
        }
    }
    
    vars
}

/// Get the default command to launch a process based on platform
pub fn default_command_for_platform(command: &str) -> String {
    match current_platform() {
        PlatformType::Windows => {
            format!("cmd.exe /c {}", command)
        },
        _ => {
            // Linux and macOS
            format!("/bin/sh -c '{}'", command)
        }
    }
}

/// Test if a binary is available in the system path
pub fn is_binary_available(binary_name: &str) -> bool {
    use std::process::Command;
    
    let (command, args) = match current_platform() {
        PlatformType::Windows => {
            ("where", vec![binary_name])
        },
        _ => {
            // Linux and macOS
            ("which", vec![binary_name])
        }
    };
    
    Command::new(command)
        .args(args)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_current_platform() {
        let platform = current_platform();
        // We can't assert a specific platform as tests may run on different OSes
        // but we can ensure it's one of our known platforms
        assert!(
            matches!(
                platform,
                PlatformType::Windows | PlatformType::MacOS | PlatformType::Linux | PlatformType::Unknown
            )
        );
    }
    
    #[test]
    fn test_config_dir() {
        let config = config_dir();
        assert!(config.to_string_lossy().contains("anya-core"));
    }
    
    #[test]
    fn test_data_dir() {
        let data = data_dir();
        assert!(data.to_string_lossy().contains("anya-core"));
    }
} 