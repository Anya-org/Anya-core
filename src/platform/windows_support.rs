use anyhow::{anyhow, Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;
use std::io::Write;
use winreg::enums::*;
use winreg::RegKey;

pub struct WindowsInstaller {
    install_dir: PathBuf,
    program_files_dir: PathBuf,
    service_name: String,
}

impl WindowsInstaller {
    pub fn new(install_dir: Option<PathBuf>) -> Result<Self> {
        // Determine installation directory - use provided dir or default to Program Files
        let program_files_dir = get_program_files_dir()?;
        
        let install_dir = match install_dir {
            Some(dir) => dir,
            None => program_files_dir.join("AnyaCore"),
        };
        
        Ok(Self {
            install_dir,
            program_files_dir,
            service_name: "AnyaCoreService".to_string(),
        })
    }
    
    pub fn detect_windows_features(&self) -> Result<WindowsFeatures> {
        let mut features = WindowsFeatures {
            wsl_available: false,
            powershell_version: "Unknown".to_string(),
            net_framework_version: "Unknown".to_string(),
            win_version: "Unknown".to_string(),
        };
        
        // Check WSL availability
        let wsl_output = Command::new("wsl")
            .args(["--status"])
            .output();
            
        features.wsl_available = match wsl_output {
            Ok(output) => output.status.success(),
            Err(_) => false,
        };
        
        // Check PowerShell version
        let ps_output = Command::new("powershell")
            .args(["-Command", "$PSVersionTable.PSVersion.ToString()"])
            .output();
            
        if let Ok(output) = ps_output {
            if output.status.success() {
                features.powershell_version = String::from_utf8_lossy(&output.stdout)
                    .trim().to_string();
            }
        }
        
        // Check .NET Framework version using registry
        if let Ok(hklm) = RegKey::predef(HKEY_LOCAL_MACHINE)
            .open_subkey("SOFTWARE\\Microsoft\\NET Framework Setup\\NDP\\v4\\Full") {
            
            if let Ok(version) = hklm.get_value::<String, _>("Version") {
                features.net_framework_version = version;
            }
        }
        
        // Get Windows version
        let win_ver_output = Command::new("cmd")
            .args(["/c", "ver"])
            .output();
            
        if let Ok(output) = win_ver_output {
            if output.status.success() {
                features.win_version = String::from_utf8_lossy(&output.stdout)
                    .trim().to_string();
            }
        }
        
        Ok(features)
    }
    
    pub fn install_windows_service(&self) -> Result<()> {
        // Create the service binary path
        let service_binary = self.install_dir.join("bin\\anya-service.exe");
        
        // Ensure the binary exists
        if !service_binary.exists() {
            return Err(anyhow!("Service binary not found at: {:?}", service_binary));
        }
        
        // Create service using sc command
        let output = Command::new("sc")
            .args([
                "create", 
                &self.service_name,
                "binPath=", &service_binary.to_string_lossy(),
                "start=", "auto",
                "DisplayName=", "Anya Core Bitcoin Service"
            ])
            .output()
            .context("Failed to create Windows service")?;
            
        if !output.status.success() {
            return Err(anyhow!("Failed to create service: {}", 
                String::from_utf8_lossy(&output.stderr)));
        }
        
        // Set service description
        let desc_output = Command::new("sc")
            .args([
                "description", 
                &self.service_name,
                "Bitcoin Core service for Anya Core with CertiK auditing."
            ])
            .output()
            .context("Failed to set service description")?;
            
        if !desc_output.status.success() {
            println!("Warning: Failed to set service description: {}", 
                String::from_utf8_lossy(&desc_output.stderr));
        }
        
        Ok(())
    }
    
    pub fn create_firewall_rules(&self) -> Result<()> {
        // Create firewall rule for Bitcoin P2P
        let rule_output = Command::new("netsh")
            .args([
                "advfirewall", "firewall", "add", "rule",
                "name=AnyaCore_Bitcoin",
                "dir=in",
                "action=allow",
                "protocol=TCP",
                "localport=8333",
                "program=", &self.install_dir.join("bin\\bitcoind.exe").to_string_lossy()
            ])
            .output()
            .context("Failed to create firewall rule")?;
            
        if !rule_output.status.success() {
            return Err(anyhow!("Failed to create firewall rule: {}", 
                String::from_utf8_lossy(&rule_output.stderr)));
        }
        
        // Create firewall rule for RPC
        let rpc_rule_output = Command::new("netsh")
            .args([
                "advfirewall", "firewall", "add", "rule",
                "name=AnyaCore_RPC",
                "dir=in",
                "action=allow",
                "protocol=TCP",
                "localport=8332",
                "program=", &self.install_dir.join("bin\\bitcoind.exe").to_string_lossy()
            ])
            .output()
            .context("Failed to create RPC firewall rule")?;
            
        if !rpc_rule_output.status.success() {
            println!("Warning: Failed to create RPC firewall rule: {}", 
                String::from_utf8_lossy(&rpc_rule_output.stderr));
        }
        
        Ok(())
    }
    
    pub fn create_start_menu_shortcut(&self) -> Result<()> {
        // Get start menu programs path
        let appdata = std::env::var("APPDATA")
            .context("Failed to get APPDATA environment variable")?;
        let start_menu = Path::new(&appdata)
            .join("Microsoft\\Windows\\Start Menu\\Programs\\Anya Core");
            
        // Create directory if it doesn't exist
        if !start_menu.exists() {
            fs::create_dir_all(&start_menu)
                .context("Failed to create Start Menu directory")?;
        }
        
        // Create shortcut file (.lnk alternative using .url for simplicity)
        let shortcut_path = start_menu.join("Anya Core Dashboard.url");
        let mut shortcut_file = fs::File::create(shortcut_path)
            .context("Failed to create shortcut file")?;
            
        writeln!(shortcut_file, "[InternetShortcut]")
            .context("Failed to write shortcut content")?;
        writeln!(shortcut_file, "URL=http://localhost:8555")
            .context("Failed to write shortcut content")?;
        writeln!(shortcut_file, "IconFile={}", 
            self.install_dir.join("icons\\anya.ico").to_string_lossy())
            .context("Failed to write shortcut content")?;
            
        Ok(())
    }
    
    pub fn register_event_sources(&self) -> Result<()> {
        // Register event source
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let event_sources = hklm
            .open_subkey("SYSTEM\\CurrentControlSet\\Services\\EventLog\\Application")
            .context("Failed to open EventLog registry key")?;
            
        let anya_source = event_sources
            .create_subkey("Anya Core")
            .context("Failed to create EventLog source")?
            .0;
            
        // Set event message file
        let message_file = self.install_dir.join("bin\\anya-events.dll");
        anya_source.set_value("EventMessageFile", &message_file.to_string_lossy().to_string())
            .context("Failed to set EventMessageFile")?;
            
        // Set supported types
        let types: u32 = 7; // INFO | WARNING | ERROR (1 | 2 | 4)
        anya_source.set_value("TypesSupported", &types)
            .context("Failed to set TypesSupported")?;
            
        Ok(())
    }
    
    pub fn integrate_with_certik(&self) -> Result<()> {
        // Create CertiK integration directory
        let certik_dir = self.install_dir.join("certik");
        if !certik_dir.exists() {
            fs::create_dir_all(&certik_dir)
                .context("Failed to create CertiK directory")?;
        }
        
        // Copy CertiK audit files
        let audit_script = r#"
@echo off
echo Running CertiK Audit Validation
powershell -Command "& { .\bin\anya-cli.exe certik validate --full --report }"
if %ERRORLEVEL% NEQ 0 (
    echo Validation Failed! See log for details.
    exit /b 1
)
echo Validation Successful!
exit /b 0
"#;
        
        fs::write(certik_dir.join("run_audit.bat"), audit_script)
            .context("Failed to create audit script")?;
            
        // Create scheduled task for weekly validation
        let task_xml = format!(r#"<?xml version="1.0" encoding="UTF-16"?>
<Task version="1.2" xmlns="http://schemas.microsoft.com/windows/2004/02/mit/task">
  <RegistrationInfo>
    <Description>Weekly CertiK Audit Validation</Description>
  </RegistrationInfo>
  <Triggers>
    <CalendarTrigger>
      <StartBoundary>2023-03-01T03:00:00</StartBoundary>
      <Enabled>true</Enabled>
      <ScheduleByWeek>
        <WeeksInterval>1</WeeksInterval>
        <DaysOfWeek>
          <Sunday />
        </DaysOfWeek>
      </ScheduleByWeek>
    </CalendarTrigger>
  </Triggers>
  <Settings>
    <Enabled>true</Enabled>
    <AllowStartOnDemand>true</AllowStartOnDemand>
  </Settings>
  <Actions>
    <Exec>
      <Command>{}</Command>
    </Exec>
  </Actions>
</Task>"#, certik_dir.join("run_audit.bat").to_string_lossy());
        
        // Save task XML
        fs::write(certik_dir.join("certik_task.xml"), task_xml)
            .context("Failed to create task XML")?;
            
        // Create task using the XML
        let task_output = Command::new("schtasks")
            .args([
                "/Create", 
                "/TN", "AnyaCore\\CertiKAudit", 
                "/XML", &certik_dir.join("certik_task.xml").to_string_lossy()
            ])
            .output()
            .context("Failed to create scheduled task")?;
            
        if !task_output.status.success() {
            println!("Warning: Failed to create scheduled task: {}", 
                String::from_utf8_lossy(&task_output.stderr));
            println!("You may need to run as administrator to create tasks.");
        }
        
        Ok(())
    }
    
    pub fn run_windows_tests(&self) -> Result<()> {
        println!("Running Windows-specific tests...");
        
        // Test bitcoin directory permissions
        let bitcoin_dir = PathBuf::from(format!("{}\\AppData\\Roaming\\Bitcoin", 
            std::env::var("USERPROFILE").unwrap_or_else(|_| "C:\\Users\\Default".to_string())));
            
        // Check if we can write to the directory
        let test_file = bitcoin_dir.join("anya_test_file");
        match fs::write(&test_file, b"test") {
            Ok(_) => {
                println!("✅ Bitcoin directory is writable");
                // Clean up test file
                let _ = fs::remove_file(test_file);
            },
            Err(e) => {
                println!("❌ Bitcoin directory is not writable: {}", e);
            }
        }
        
        // Test service status if installed
        let service_status = Command::new("sc")
            .args(["query", &self.service_name])
            .output();
            
        match service_status {
            Ok(output) => {
                if output.status.success() && 
                   String::from_utf8_lossy(&output.stdout).contains("RUNNING") {
                    println!("✅ Anya service is running");
                } else {
                    println!("⚠️ Anya service is not running or not installed");
                }
            },
            Err(_) => println!("⚠️ Could not check service status"),
        }
        
        // Test firewall rules
        let firewall_status = Command::new("netsh")
            .args([
                "advfirewall", "firewall", "show", "rule", 
                "name=AnyaCore_Bitcoin"
            ])
            .output();
            
        match firewall_status {
            Ok(output) => {
                if output.status.success() && 
                   !String::from_utf8_lossy(&output.stdout).contains("No rules match") {
                    println!("✅ Firewall rules are configured");
                } else {
                    println!("⚠️ Firewall rules are not configured");
                }
            },
            Err(_) => println!("⚠️ Could not check firewall status"),
        }
        
        Ok(())
    }
}

// Helper types and functions
pub struct WindowsFeatures {
    pub wsl_available: bool,
    pub powershell_version: String,
    pub net_framework_version: String,
    pub win_version: String,
}

fn get_program_files_dir() -> Result<PathBuf> {
    // First try the environment variable
    if let Ok(pf) = std::env::var("ProgramFiles") {
        return Ok(PathBuf::from(pf));
    }
    
    // Use default if not found
    Ok(PathBuf::from("C:\\Program Files"))
} 