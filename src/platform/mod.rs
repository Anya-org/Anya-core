//! Platform-specific utilities for cross-platform compatibility
//! [AIR-3][AIS-3][BPC-3]

use std::env;
use std::path::{Path, PathBuf};

/// Represents the operating system platform
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    /// Microsoft Windows
    Windows,
    /// Apple macOS
    MacOS,
    /// Linux-based systems
    Linux,
    /// Other/unknown platforms
    Unknown,
}

impl Platform {
    /// Get the current platform
    pub fn current() -> Self {
        match env::consts::OS {
            "windows" => Platform::Windows,
            "macos" => Platform::MacOS,
            "linux" => Platform::Linux,
            _ => Platform::Unknown,
        }
    }
    
    /// Check if the platform is Unix-like (macOS or Linux)
    pub fn is_unix(&self) -> bool {
        matches!(self, Platform::MacOS | Platform::Linux)
    }
    
    /// Get the platform's path separator
    pub fn path_separator(&self) -> &'static str {
        match self {
            Platform::Windows => "\\",
            _ => "/",
        }
    }
    
    /// Get the platform's config directory
    pub fn config_dir(&self) -> PathBuf {
        match self {
            Platform::Windows => {
                if let Some(appdata) = env::var_os("APPDATA") {
                    Path::new(&appdata).join("Anya")
                } else {
                    Path::new("C:\\ProgramData\\Anya").to_path_buf()
                }
            }
            _ => {
                if let Some(home) = env::var_os("HOME") {
                    Path::new(&home).join(".anya")
                } else {
                    Path::new(std::path::Path::new("/").join("etc/anya").to_string_lossy()).to_path_buf()
                }
            }
        }
    }
    
    /// Get the platform's temp directory
    pub fn temp_dir(&self) -> PathBuf {
        env::temp_dir()
    }
}
