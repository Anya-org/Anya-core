use crate::ports::platform_port::PlatformPort;
use platform::{self, PlatformType};
use std::path::PathBuf;

pub struct PlatformAdapter {
    platform_type: PlatformType,
}

impl PlatformAdapter {
    pub fn new() -> Self {
        Self {
            platform_type: platform::current_platform(),
        }
    }
}

impl PlatformPort for PlatformAdapter {
    fn get_installation_directory(&self) -> PathBuf {
        match self.platform_type {
            PlatformType::Windows => PathBuf::from("C:\\Program Files\\Anya-Core"),
            PlatformType::MacOS => PathBuf::from("/Applications/Anya-Core.app"),
            PlatformType::Linux => PathBuf::from("/usr/local/bin/anya-core"),
            PlatformType::Unknown => PathBuf::from("./anya-core"),
        }
    }
    
    fn get_config_directory(&self) -> PathBuf {
        platform::config_dir()
    }
    
    fn get_data_directory(&self) -> PathBuf {
        platform::data_dir()
    }
    
    fn is_elevated_permissions(&self) -> bool {
        match self.platform_type {
            PlatformType::Windows => {
                // Windows-specific elevated check
                // Can be implemented using windows-rs crate
                false // Placeholder
            },
            _ => {
                // Unix-based systems
                std::env::var("USER").unwrap_or_default() == "root"
            }
        }
    }
    
    fn create_service(&self, service_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        match self.platform_type {
            PlatformType::Windows => {
                // Create Windows service
                println!("Creating Windows service: {}", service_name);
                Ok(())
            },
            PlatformType::MacOS => {
                // Create macOS launchd service
                println!("Creating macOS launchd service: {}", service_name);
                Ok(())
            },
            PlatformType::Linux => {
                // Create Linux systemd service
                println!("Creating Linux systemd service: {}", service_name);
                Ok(())
            },
            PlatformType::Unknown => {
                // Generic service handling
                println!("Creating generic service: {}", service_name);
                Ok(())
            },
        }
    }
    
    fn get_bitcoin_config_location(&self) -> PathBuf {
        match self.platform_type {
            PlatformType::Windows => PathBuf::from(r"C:\ProgramData\AnyaCore\bitcoin"),
            PlatformType::MacOS => PathBuf::from("/Library/Application Support/AnyaCore/bitcoin"),
            PlatformType::Linux => PathBuf::from("/etc/anya-core/bitcoin"),
            _ => self.get_config_directory().join("bitcoin"),
        }
    }
    
    fn get_web5_config_location(&self) -> PathBuf {
        match self.platform_type {
            PlatformType::Windows => PathBuf::from(r"C:\ProgramData\AnyaCore\web5"),
            PlatformType::MacOS => PathBuf::from("/Library/Application Support/AnyaCore/web5"),
            PlatformType::Linux => PathBuf::from("/etc/anya-core/web5"),
            _ => self.get_config_directory().join("web5"),
        }
    }
}
