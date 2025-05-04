mod api;
mod bitcoin;
mod storage;
mod web5;

use anyhow::Result;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::bitcoin::wallet::BitcoinWallet;
use crate::storage::memory::MemoryStorage;
use crate::web5::identity::IdentityManager;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting Anya Core API Server...");

    // Initialize storage
    let storage = Arc::new(MemoryStorage::new());

    // Initialize Bitcoin wallet
    let wallet = Arc::new(BitcoinWallet::new(
        storage.clone(),
        bitcoin::Network::Testnet,
    ));

    // Initialize Identity manager
    let identity = Arc::new(IdentityManager::new(storage.clone(), "key".to_string()));

    // Set up API server
    let host = "127.0.0.1";
    let port = 8080;
    let addr = format!("{}:{}", host, port);

    info!("Binding API server to {}", addr);
    let listener = TcpListener::bind(&addr).await?;

    // Start server
    api::server::start_server(wallet, identity, listener).await?;

    Ok(())
}

// Process a series of inputs to demonstrate the agent checker
fn process_demo_inputs(core: &CoreSystem) -> Result<(), Box<dyn Error>> {
    // Process 25 inputs to trigger auto-save (every 20 inputs)
    println!("Processing 25 inputs (auto-save at 20)...");

    for i in 0..25 {
        let input = match i % 5 {
            0 => format!("success: Component initialization {}", i),
            1 => format!("info: Normal operation {}", i),
            2 => format!("warning: Resource usage high {}", i),
            3 => format!("error: Connection timeout {}", i),
            _ => format!("success: Task completed {}", i),
        };

        println!("  Input [{}]: {}", i + 1, input);
        core.process_input(&input)?;
    }

    // Check the system stage
    let stage = core.agent_checker().get_system_stage();
    println!("Current system stage: {:?}", stage);
}

// Configure system security components to demonstrate hardening functionality
fn configure_system_security(core: &CoreSystem) -> Result<(), Box<dyn Error>> {
    let hardening = core.system_hardening();

    // Configure network security
    println!("Configuring network security...");
    let mut network_settings = security::create_basic_security_config("network");
    network_settings.insert("vpn_required", "true".to_string());

    hardening.configure_component("network", SecurityLevel::Strict, network_settings, true)?;

    // Configure database security
    println!("Configuring database security...");
    let db_settings = security::create_basic_security_config("database");

    hardening.configure_component("database", SecurityLevel::Enhanced, db_settings, true)?;

    // Configure API security
    println!("Configuring API security...");
    let api_settings = security::create_basic_security_config("api");

    hardening.configure_component("api", SecurityLevel::Enhanced, api_settings, true)?;

    // Apply hardening to configured components
    println!("Applying security hardening...");
    hardening.apply_hardening("network")?;
    hardening.apply_hardening("database")?;
    hardening.apply_hardening("api")?;

    // Configure 20 more components to trigger auto-save
    println!("Configuring 20 additional components to trigger auto-save...");
    for i in 0..20 {
        let component_name = format!("component_{}", i);
        let settings = security::create_basic_security_config(&component_name);

        hardening.configure_component(&component_name, SecurityLevel::Basic, settings, true)?;
    }
}

// Configure and optimize system performance
fn optimize_system_performance(core: &CoreSystem) -> Result<(), Box<dyn Error>> {
    let optimizer = core.performance_optimizer();

    // Configure CPU resource
    println!("Configuring CPU optimization...");
    let mut cpu_settings = HashMap::new();
    cpu_settings.insert("max_threads".to_string(), "8".to_string());
    cpu_settings.insert("priority".to_string(), "high".to_string());

    optimizer.configure_resource(
        "cpu",
        ResourceType::CPU,
        cpu_settings,
        0.8,                       // Target utilization
        1000.0,                    // Target throughput
        Duration::from_millis(10), // Target latency
    )?;

    // Configure memory resource
    println!("Configuring memory optimization...");
    let mut mem_settings = HashMap::new();
    mem_settings.insert("cache_size".to_string(), "1024".to_string());
    mem_settings.insert("gc_threshold".to_string(), "75".to_string());

    optimizer.configure_resource(
        "memory",
        ResourceType::Memory,
        mem_settings,
        0.7,                      // Target utilization
        2000.0,                   // Target throughput
        Duration::from_millis(5), // Target latency
    )?;

    // Update metrics to simulate resource states
    println!("Updating performance metrics...");

    // CPU metrics - needs optimization
    let mut cpu_metrics = HashMap::new();
    cpu_metrics.insert("temperature".to_string(), 65.0);
    cpu_metrics.insert("context_switches".to_string(), 1500.0);

    optimizer.update_metrics(
        "cpu",
        0.85,                      // Current utilization (above target)
        950.0,                     // Current throughput (below target)
        Duration::from_millis(15), // Current latency (above target)
        cpu_metrics,
    )?;

    // Memory metrics - needs optimization
    let mut mem_metrics = HashMap::new();
    mem_metrics.insert("page_faults".to_string(), 25.0);
    mem_metrics.insert("allocation_rate".to_string(), 500.0);

    optimizer.update_metrics(
        "memory",
        0.75,                     // Current utilization (above target)
        1800.0,                   // Current throughput (below target)
        Duration::from_millis(8), // Current latency (above target)
        mem_metrics,
    )?;

    // Optimize resources
    println!("Optimizing resources...");
    optimizer.optimize_resource("cpu")?;
    optimizer.optimize_resource("memory")?;

    // Configure additional resources to trigger auto-save
    println!("Configuring 18 additional resources to trigger auto-save...");
    for i in 0..18 {
        let resource_name = format!("resource_{}", i);
        let mut settings = HashMap::new();
        settings.insert("setting1".to_string(), "value1".to_string());

        optimizer.configure_resource(
            &resource_name,
            ResourceType::Custom(i),
            settings,
            0.7,
            500.0,
            Duration::from_millis(50),
        )?;
    }
}
