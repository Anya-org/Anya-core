use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlatformType {
    Windows,
    MacOS,
    Linux,
    Unknown,
}

/// Detects and returns the current platform type
pub fn current_platform() -> PlatformType {
    if cfg!(target_os = "windows") {
        PlatformType::Windows
    } else if cfg!(target_os = "macos") {
        PlatformType::MacOS
    } else if cfg!(target_os = "linux") {
        PlatformType::Linux
    } else {
        PlatformType::Unknown
    }
}

/// Returns the platform-specific configuration directory
pub fn config_dir() -> PathBuf {
    match current_platform() {
        PlatformType::Windows => {
            if let Some(app_data) = std::env::var_os("APPDATA") {
                PathBuf::from(app_data).join("Anya-Core")
            } else {
                PathBuf::from(r"C:\Users\Default\AppData\Roaming\Anya-Core")
            }
        },
        PlatformType::MacOS => {
            let home = std::env::var_os("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("/Users/Shared"));
            home.join("Library/Application Support/Anya-Core")
        },
        PlatformType::Linux => {
            let home = std::env::var_os("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("/tmp"));
            home.join(".config/anya-core")
        },
        PlatformType::Unknown => PathBuf::from("./config"),
    }
}

/// Returns the platform-specific data directory
pub fn data_dir() -> PathBuf {
    match current_platform() {
        PlatformType::Windows => {
            if let Some(local_app_data) = std::env::var_os("LOCALAPPDATA") {
                PathBuf::from(local_app_data).join("Anya-Core")
            } else {
                PathBuf::from(r"C:\Users\Default\AppData\Local\Anya-Core")
            }
        },
        PlatformType::MacOS => {
            let home = std::env::var_os("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("/Users/Shared"));
            home.join("Library/Application Support/Anya-Core")
        },
        PlatformType::Linux => {
            let home = std::env::var_os("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("/tmp"));
            home.join(".local/share/anya-core")
        },
        PlatformType::Unknown => PathBuf::from("./data"),
    }
}

/// Returns the platform-specific cache directory
pub fn cache_dir() -> PathBuf {
    match current_platform() {
        PlatformType::Windows => {
            if let Some(local_app_data) = std::env::var_os("LOCALAPPDATA") {
                PathBuf::from(local_app_data).join("Anya-Core").join("Cache")
            } else {
                PathBuf::from(r"C:\Users\Default\AppData\Local\Anya-Core\Cache")
            }
        },
        PlatformType::MacOS => {
            let home = std::env::var_os("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("/Users/Shared"));
            home.join("Library/Caches/Anya-Core")
        },
        PlatformType::Linux => {
            let home = std::env::var_os("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("/tmp"));
            home.join(".cache/anya-core")
        },
        PlatformType::Unknown => PathBuf::from("./cache"),
    }
}

/// Returns the platform-specific executable extension
pub fn executable_extension() -> &'static str {
    match current_platform() {
        PlatformType::Windows => ".exe",
        _ => "",
    }
}

/// Returns whether the platform uses backslashes for paths
pub fn uses_backslashes() -> bool {
    current_platform() == PlatformType::Windows
}

/// Returns the platform-specific path separator
pub fn path_separator() -> &'static str {
    if uses_backslashes() {
        "\\"
    } else {
        "/"
    }
}

/// Formats a command for the current platform
pub fn format_command(command: &str) -> String {
    match current_platform() {
        PlatformType::Windows => format!("{}.exe", command),
        _ => command.to_string(),
    }
}

/// Returns if the current process is running with elevated permissions
pub fn is_elevated() -> bool {
    match current_platform() {
        PlatformType::Windows => {
            // This is a simplified check - a real implementation would use the Windows API
            std::env::var("ADMINISTRATOR").is_ok()
        },
        _ => {
            // Unix systems - check if running as root
            std::env::var("USER").map(|user| user == "root").unwrap_or(false)
        }
    }
} 