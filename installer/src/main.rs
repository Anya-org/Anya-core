#![feature(edition2021)]
use anyhow::Result;
use sysinfo::{System, SystemExt, CpuExt, DiskExt};
use std::net::TcpStream;
use log::{info, warn, error};

mod requirements;
mod network;
mod hardware;
mod components;

use requirements::{SystemRequirements, validate_requirements};
use network::{NetworkChecker, ServiceStatus};
use hardware::HardwareAnalyzer;
use components::ComponentManager;

const MIN_RAM_GB: u64 = 8;
const MIN_CPU_CORES: usize = 4;
const MIN_DISK_GB: u64 = 50;
const REQUIRED_PORTS: &[u16] = &[8332, 9735, 3000, 8080];

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("Starting Anya Core System Analysis...");

    let sys = System::new_all();
    let hw_analyzer = HardwareAnalyzer::new(&sys);
    let net_checker = NetworkChecker::new();
    let component_mgr = ComponentManager::new();

    // System Requirements Check
    let requirements = SystemRequirements {
        min_ram_gb: MIN_RAM_GB,
        min_cpu_cores: MIN_CPU_CORES,
        min_disk_gb: MIN_DISK_GB,
        required_ports: REQUIRED_PORTS.to_vec(),
    };

    match validate_requirements(&requirements, &hw_analyzer) {
        Ok(_) => info!("System requirements met"),
        Err(e) => {
            error!("System requirements not met: {}", e);
            return Err(e.into());
        }
    }

    // Network Analysis
    let network_status = net_checker.analyze_network().await?;
    info!("Network Analysis Complete: {:?}", network_status);

    // Component Status Check
    let components = component_mgr.check_installed_components()?;
    info!("Installed Components: {:?}", components);

    // Generate Installation Recommendation
    let recommendation = generate_install_recommendation(
        &hw_analyzer,
        &network_status,
        &components
    )?;

    println!("\nInstallation Recommendation:");
    println!("{}", recommendation);

    Ok(())
}

fn generate_install_recommendation(
    hw: &HardwareAnalyzer,
    network: &ServiceStatus,
    components: &[String]
) -> Result<String> {
    let mut rec = String::new();
    
    // Hardware-based recommendations
    if hw.get_ram_gb() > 16 && hw.get_cpu_cores() > 8 {
        rec.push_str("✅ Recommended: Full Node Installation\n");
    } else {
        rec.push_str("⚠️ Recommended: Light Client Installation\n");
    }

    // Network-based recommendations
    match network {
        ServiceStatus::FullyConnected => {
            rec.push_str("✅ Network: Ready for distributed deployment\n");
        },
        ServiceStatus::PartiallyConnected => {
            rec.push_str("⚠️ Network: Limited connectivity - Local deployment recommended\n");
        },
        ServiceStatus::Offline => {
            rec.push_str("❌ Network: Offline - Standalone deployment only\n");
        }
    }

    // Component recommendations
    if components.is_empty() {
        rec.push_str("✅ Clean installation recommended\n");
    } else {
        rec.push_str("⚠️ Upgrade/migration required for existing components:\n");
        for component in components {
            rec.push_str(&format!("  - {}\n", component));
        }
    }

    Ok(rec)
}
