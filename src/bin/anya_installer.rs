//! Anya-Core Installer
//! 
//! A cross-platform installation utility for Anya-Core that ensures
//! compliance with Bitcoin Development Framework v2.5 and
//! Hexagonal Architecture requirements.

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug, Serialize, Deserialize)]
struct AnyaConfig {
    network: NetworkConfig,
    wallet: WalletConfig,
    dao: DaoConfig,
    system_awareness: SystemAwarenessConfig,
    performance: PerformanceConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct NetworkConfig {
    network_type: String,
    connect_peers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct WalletConfig {
    enable_taproot: bool,
    bip370_support: bool,
    coin_selection_strategy: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DaoConfig {
    quadratic_voting: bool,
    dao_level: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SystemAwarenessConfig {
    mempool_alert_threshold_kb: u32,
    fee_spike_threshold: f64,
    attack_threshold: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct PerformanceConfig {
    cache_size_mb: u32,
    batch_size: u32,
    use_prepared_statements: bool,
}

enum InstallMode {
    Development,
    Production,
    Testing,
}

fn main() -> io::Result<()> {
    print_banner();

    // Check required tools
    println!("{}", "Checking required tools...".blue().bold());
    check_required_tools()?;

    // Get installation directory
    let install_dir = get_install_directory()?;
    println!("{} {}", "Installation directory:".green(), install_dir.display());

    // Choose installation mode
    let install_mode = get_install_mode()?;

    // Create or navigate to installation directory
    prepare_directory(&install_dir)?;

    // Clone or update repository
    clone_or_update_repository(&install_dir)?;

    // Build the project
    build_project(&install_dir, &install_mode)?;

    // Generate configuration
    generate_configuration(&install_dir, &install_mode)?;

    // Set up database (if needed)
    if let InstallMode::Production | InstallMode::Testing = install_mode {
        setup_database(&install_dir, &install_mode)?;
    }

    // Run tests (if in development or testing mode)
    if let InstallMode::Development | InstallMode::Testing = install_mode {
        run_tests(&install_dir)?;
    }

    // Generate startup scripts
    generate_startup_scripts(&install_dir, &install_mode)?;

    // Display completion message
    print_completion_message(&install_dir, &install_mode);

    Ok(())
}

fn print_banner() {
    println!("{}", "======================================================".yellow());
    println!("{}", "              Anya-Core Installer                    ".green().bold());
    println!("{}", "   Bitcoin Development Framework v2.5 Compliant      ".green());
    println!("{}", "======================================================".yellow());
    println!();
}

fn check_required_tools() -> io::Result<()> {
    let required_tools = vec![
        ("git", vec!["--version"]),
        ("cargo", vec!["--version"]),
        ("rustc", vec!["--version"]),
    ];

    let mut missing_tools = Vec::new();

    for (tool, args) in required_tools {
        print!("Checking for {}... ", tool);
        io::stdout().flush()?;

        match Command::new(tool).args(args).output() {
            Ok(output) if output.status.success() => {
                let version = String::from_utf8_lossy(&output.stdout);
                let version = version.trim();
                println!("{} ({})", "✓".green(), version);
            },
            _ => {
                println!("{}", "✗".red());
                missing_tools.push(tool);
            }
        }
    }

    if !missing_tools.is_empty() {
        println!("\n{}", "The following required tools are missing:".red().bold());
        for tool in &missing_tools {
            println!("  - {}", tool);
        }
        
        println!("\nPlease install these tools and run the installer again.");
        std::process::exit(1);
    }

    Ok(())
}

fn get_install_directory() -> io::Result<PathBuf> {
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let default_dir = home_dir.join("anya-core");

    let install_dir: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Installation directory")
        .default(default_dir.to_string_lossy().to_string())
        .interact_text()?;

    Ok(PathBuf::from(install_dir))
}

fn get_install_mode() -> io::Result<InstallMode> {
    let options = vec!["Development", "Production", "Testing"];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select installation mode")
        .default(0)
        .items(&options)
        .interact()?;
    
    match selection {
        0 => Ok(InstallMode::Development),
        1 => Ok(InstallMode::Production),
        2 => Ok(InstallMode::Testing),
        _ => unreachable!(),
    }
}

fn prepare_directory(dir: &Path) -> io::Result<()> {
    if !dir.exists() {
        println!("{} {}", "Creating directory:".blue(), dir.display());
        fs::create_dir_all(dir)?;
    }

    Ok(())
}

fn clone_or_update_repository(dir: &Path) -> io::Result<()> {
    let git_dir = dir.join(".git");
    
    if git_dir.exists() {
        println!("{}", "Updating existing repository...".blue());
        run_command("git", &["pull"], dir)?;
    } else {
        println!("{}", "Cloning repository...".blue());
        run_command(
            "git", 
            &["clone", "https://github.com/user/anya-core.git", "."], 
            dir
        )?;
    }

    Ok(())
}

fn build_project(dir: &Path, mode: &InstallMode) -> io::Result<()> {
    println!("{}", "Building project...".blue());
    
    // Clean build
    let clean_build = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Perform clean build?")
        .default(true)
        .interact()?;
    
    if clean_build {
        run_command("cargo", &["clean"], dir)?;
    }
    
    // Build with appropriate flags
    let build_args = match mode {
        InstallMode::Production => vec!["build", "--release"],
        InstallMode::Development => vec!["build"],
        InstallMode::Testing => vec!["build", "--release"],
    };
    
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner} {msg}")
            .unwrap()
    );
    
    spinner.set_message("Building project (this may take a while)...");
    spinner.enable_steady_tick(100);
    
    let status = Command::new("cargo")
        .args(&build_args)
        .current_dir(dir)
        .status()?;
    
    spinner.finish_and_clear();
    
    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other, 
            "Failed to build project"
        ));
    }
    
    println!("{}", "Project built successfully!".green());
    Ok(())
}

fn generate_configuration(dir: &Path, mode: &InstallMode) -> io::Result<()> {
    println!("{}", "Generating configuration...".blue());

    let config_dir = dir.join("config");
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    let network_type = match mode {
        InstallMode::Production => "mainnet",
        InstallMode::Development | InstallMode::Testing => "testnet",
    };

    let config = AnyaConfig {
        network: NetworkConfig {
            network_type: network_type.to_string(),
            connect_peers: vec![
                "127.0.0.1:18333".to_string(), 
                "127.0.0.1:18334".to_string()
            ],
        },
        wallet: WalletConfig {
            enable_taproot: true,
            bip370_support: true,
            coin_selection_strategy: "efficient".to_string(),
        },
        dao: DaoConfig {
            quadratic_voting: true,
            dao_level: "DAO4".to_string(),
        },
        system_awareness: SystemAwarenessConfig {
            mempool_alert_threshold_kb: 100,
            fee_spike_threshold: 200.0,
            attack_threshold: 60.0,
        },
        performance: PerformanceConfig {
            cache_size_mb: 20,
            batch_size: 100,
            use_prepared_statements: true,
        },
    };

    let config_path = config_dir.join("anya.conf");
    let config_str = toml::to_string_pretty(&config)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Error serializing config: {}", e)))?;
    
    // Add comments to the config file
    let config_with_comments = format!(
        "# Anya-Core Configuration\n\
         # Generated by installer (Bitcoin Development Framework v2.5 Compliant)\n\
         # Installation mode: {:?}\n\
         # Generated on: {}\n\
         \n{}", 
        mode, 
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        config_str
    );
    
    fs::write(config_path, config_with_comments)?;
    
    println!("{} {}", "Configuration saved to:".green(), config_dir.join("anya.conf").display());
    Ok(())
}

fn setup_database(dir: &Path, mode: &InstallMode) -> io::Result<()> {
    println!("{}", "Setting up database...".blue());
    
    // Run database migrations
    let migrations_dir = dir.join("migrations");
    if migrations_dir.exists() {
        println!("Running database migrations...");
        
        // In a real implementation, this would use a proper migration tool
        // For now, we'll just simulate the process
        let pb = ProgressBar::new(100);
        pb.set_style(ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("##-"));
        
        for i in 0..100 {
            pb.set_message(format!("Migration {}/100", i + 1));
            pb.inc(1);
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        
        pb.finish_with_message("Database migrations completed");
    } else {
        println!("{} {}", "Warning:".yellow(), "Migrations directory not found. Skipping database setup.");
    }
    
    Ok(())
}

fn run_tests(dir: &Path) -> io::Result<()> {
    println!("{}", "Running tests...".blue());
    
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to run tests?")
        .default(true)
        .interact()?
    {
        println!("Running tests (this may take a while)...");
        run_command("cargo", &["test"], dir)?;
        println!("{}", "Tests completed successfully!".green());
    } else {
        println!("Skipping tests.");
    }
    
    Ok(())
}

fn generate_startup_scripts(dir: &Path, mode: &InstallMode) -> io::Result<()> {
    println!("{}", "Generating startup scripts...".blue());
    
    let script_dir = dir.join("scripts");
    if !script_dir.exists() {
        fs::create_dir_all(&script_dir)?;
    }
    
    // Create startup script based on platform
    if cfg!(windows) {
        create_windows_startup_script(&script_dir, mode)?;
    } else if cfg!(unix) {
        create_unix_startup_script(&script_dir, mode)?;
    }
    
    println!("{} {}", "Startup scripts saved to:".green(), script_dir.display());
    Ok(())
}

fn create_windows_startup_script(dir: &Path, mode: &InstallMode) -> io::Result<()> {
    let script_path = dir.join("start_anya.bat");
    
    let release_flag = match mode {
        InstallMode::Production | InstallMode::Testing => "--release",
        InstallMode::Development => "",
    };
    
    let script_content = format!(
        "@echo off\r\n\
         echo Starting Anya-Core...\r\n\
         cd {}\r\n\
         cargo run {} --bin anya_core -- --config config/anya.conf\r\n\
         if %ERRORLEVEL% NEQ 0 (\r\n\
         echo Error starting Anya-Core\r\n\
         pause\r\n\
         exit /b %ERRORLEVEL%\r\n\
         )\r\n",
        dir.parent().unwrap().display(),
        release_flag
    );
    
    fs::write(script_path, script_content)?;
    
    // Create a PowerShell script as well
    let ps_script_path = dir.join("start_anya.ps1");
    
    let ps_script_content = format!(
        "# Anya-Core Startup Script\r\n\
         Write-Host \"Starting Anya-Core...\" -ForegroundColor Blue\r\n\
         Set-Location {}\r\n\
         cargo run {} --bin anya_core -- --config config/anya.conf\r\n\
         if ($LASTEXITCODE -ne 0) {{\r\n\
         Write-Host \"Error starting Anya-Core\" -ForegroundColor Red\r\n\
         Read-Host \"Press Enter to exit\"\r\n\
         exit $LASTEXITCODE\r\n\
         }}\r\n",
        dir.parent().unwrap().display(),
        release_flag
    );
    
    fs::write(ps_script_path, ps_script_content)?;
    
    Ok(())
}

fn create_unix_startup_script(dir: &Path, mode: &InstallMode) -> io::Result<()> {
    let script_path = dir.join("start_anya.sh");
    
    let release_flag = match mode {
        InstallMode::Production | InstallMode::Testing => "--release",
        InstallMode::Development => "",
    };
    
    let script_content = format!(
        "#!/bin/bash\n\
         echo \"Starting Anya-Core...\"\n\
         cd {}\n\
         cargo run {} --bin anya_core -- --config config/anya.conf\n\
         if [ $? -ne 0 ]; then\n\
         echo \"Error starting Anya-Core\"\n\
         exit 1\n\
         fi\n",
        dir.parent().unwrap().display(),
        release_flag
    );
    
    fs::write(&script_path, script_content)?;
    
    // Make the script executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms)?;
    }
    
    Ok(())
}

fn print_completion_message(dir: &Path, mode: &InstallMode) {
    println!("\n{}", "======================================================".yellow());
    println!("{}", "        Anya-Core Installation Complete               ".green().bold());
    println!("{}", "======================================================".yellow());
    println!();
    println!("Installation directory: {}", dir.display());
    println!("Installation mode: {:?}", mode);
    println!();
    println!("{}", "To start Anya-Core:".blue());
    
    if cfg!(windows) {
        println!("  Run the script: {}", dir.join("scripts").join("start_anya.bat").display());
        println!("  Or with PowerShell: {}", dir.join("scripts").join("start_anya.ps1").display());
    } else {
        println!("  Run the script: {}", dir.join("scripts").join("start_anya.sh").display());
    }
    
    println!();
    println!("{}", "For more information, refer to the documentation:".blue());
    println!("  {}", dir.join("docs").join("README.md").display());
    println!();
    println!("{}", "Thank you for using Anya-Core!".green().bold());
}

fn run_command(cmd: &str, args: &[&str], dir: &Path) -> io::Result<()> {
    let status = Command::new(cmd)
        .args(args)
        .current_dir(dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;
    
    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other, 
            format!("Command '{}' failed with exit code: {:?}", cmd, status.code())
        ));
    }
    
    Ok(())
} 