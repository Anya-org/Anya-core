use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlatformType {
    Windows,
    MacOS,
    Linux,
    Android,
    IOS,
    Unknown,
}

/// Detects and returns the current platform type
pub fn current_platform() -> PlatformType {
    if cfg!(target_os = "windows") {
        PlatformType::Windows
    } else if cfg!(target_os = "macos") {
        PlatformType::MacOS
    } else if cfg!(target_os = "ios") {
        PlatformType::IOS
    } else if cfg!(target_os = "linux") {
        if cfg!(target_env = "android") {
            PlatformType::Android
        } else {
            PlatformType::Linux
        }
    } else {
        PlatformType::Unknown
    }
}

/// Returns the platform-specific configuration directory
pub fn config_dir() -> PathBuf {
    let path = match current_platform() {
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
    };
    
    // Sanitize path to prevent path traversal
    let canonical = path.canonicalize().unwrap_or(path);
    canonical
}

/// Returns the platform-specific data directory
pub fn data_dir() -> PathBuf {
    let path = match current_platform() {
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
        PlatformType::Android => {
            // Android app-specific storage
            PathBuf::from("/data/data/org.anya.core/files")
        },
        PlatformType::IOS => {
            // iOS app bundle Documents directory
            PathBuf::from("Documents")
        },
        PlatformType::Unknown => PathBuf::from("./data"),
    };
    
    // Sanitize path
    path.canonicalize().unwrap_or(path)
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
        PlatformType::Android => {
            // Android app-specific cache directory
            PathBuf::from("/data/data/org.anya.core/cache")
        },
        PlatformType::IOS => {
            // iOS app bundle Caches directory
            PathBuf::from("Library/Caches/com.anya.core")
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

/// Returns the Bitcoin data directory for the current platform
pub fn get_bitcoin_data_dir() -> PathBuf {
    let path = match current_platform() {
        PlatformType::Windows => {
            let app_data = std::env::var_os("APPDATA")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(r"C:\Users\Default\AppData\Roaming"));
            app_data.join("Bitcoin")
        },
        PlatformType::MacOS => {
            let home = std::env::var_os("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("/Users/Shared"));
            home.join("Library/Application Support/Bitcoin")
        },
        PlatformType::Linux => {
            let home = std::env::var_os("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("/tmp"));
            home.join(".bitcoin")
        },
        PlatformType::Android => {
            // Android app-specific storage
            PathBuf::from("/data/data/org.anya.core/files/Bitcoin")
        },
        PlatformType::IOS => {
            // iOS app bundle Documents directory
            PathBuf::from("Documents/Bitcoin")
        },
        PlatformType::Unknown => PathBuf::from("./bitcoin-data"),
    };

    // Create directory if it doesn't exist
    if !path.exists() {
        if let Err(e) = std::fs::create_dir_all(&path) {
            eprintln!("Warning: Failed to create Bitcoin data directory: {}", e);
        }
    }
    
    // Sanitize path
    path.canonicalize().unwrap_or(path)
}

pub fn get_web5_data_dir() -> PathBuf {
    let path = match current_platform() {
        PlatformType::Windows => {
            let app_data = std::env::var_os("LOCALAPPDATA")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(r"C:\Users\Default\AppData\Local"));
            app_data.join("Web5")
        },
        PlatformType::MacOS => {
            let home = std::env::var_os("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("/Users/Shared"));
            home.join("Library/Application Support/Web5")
        },
        PlatformType::Linux => {
            let home = std::env::var_os("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("/tmp"));
            home.join(".local/share/web5")
        },
        PlatformType::Android => {
            // Android app-specific storage
            PathBuf::from("/data/data/org.anya.core/files/Web5")
        },
        PlatformType::IOS => {
            // iOS app bundle Documents directory
            PathBuf::from("Documents/Web5")
        },
        PlatformType::Unknown => PathBuf::from("./web5-data"),
    };

    // Create directory if it doesn't exist
    if !path.exists() {
        if let Err(e) = std::fs::create_dir_all(&path) {
            eprintln!("Warning: Failed to create Web5 data directory: {}", e);
        }
    }
    
    // Sanitize path
    path.canonicalize().unwrap_or(path)
}

pub fn get_taproot_assets_dir() -> PathBuf {
    let path = match current_platform() {
        PlatformType::Windows => {
            let app_data = std::env::var_os("APPDATA")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(r"C:\Users\Default\AppData\Roaming"));
            app_data.join("Taproot-Assets")
        },
        PlatformType::MacOS => {
            let home = std::env::var_os("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("/Users/Shared"));
            home.join("Library/Application Support/Taproot-Assets")
        },
        PlatformType::Linux => {
            let home = std::env::var_os("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("/tmp"));
            home.join(".taproot-assets")
        },
        PlatformType::Android => {
            // Android app-specific storage
            PathBuf::from("/data/data/org.anya.core/files/Taproot-Assets")
        },
        PlatformType::IOS => {
            // iOS app bundle Documents directory
            PathBuf::from("Documents/Taproot-Assets")
        },
        PlatformType::Unknown => PathBuf::from("./taproot-assets"),
    };

    // Create directory if it doesn't exist
    if !path.exists() {
        if let Err(e) = std::fs::create_dir_all(&path) {
            eprintln!("Warning: Failed to create Taproot Assets directory: {}", e);
        }
    }
    
    // Sanitize path
    path.canonicalize().unwrap_or(path)
} 