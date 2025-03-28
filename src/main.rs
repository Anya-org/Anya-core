#![feature(edition2021)]
// AIR-008: Main Application Entry Point
// Demonstrates P1 components with auto-save functionality

// Import modules
pub mod core;
pub mod ml;
pub mod security;

use std::collections::HashMap;
use std::time::Duration;

use crate::core::CoreSystem;
use crate::core::ResourceType;
use crate::core::SecurityLevel;
use crate::ml::SystemStage;

fn main() {
    println!("Starting Anya Core with P1 components...");
    println!("AIR-008: Auto-save functionality set to every 20th input");
    
    // Initialize the core system with auto-save every 20 inputs
    let core = CoreSystem::new(20);
    
    // Demo 1: Process inputs with auto-save
    println!("\n== Demo 1: Processing inputs with auto-save ==");
    process_demo_inputs(&core);
    
    // Demo 2: System hardening with auto-save
    println!("\n== Demo 2: System hardening with auto-save ==");
    configure_system_security(&core);
    
    // Demo 3: Performance optimization with auto-save
    println!("\n== Demo 3: Performance optimization with auto-save ==");
    optimize_system_performance(&core);
    
    // Display final stats
    let (agent_inputs, hardening_changes, performance_changes) = core.get_auto_save_stats();
    println!("\n== Final Auto-Save Statistics ==");
    println!("Total Agent Checker inputs processed: {}", agent_inputs);
    println!("Total System Hardening changes: {}", hardening_changes);
    println!("Total Performance Optimizer changes: {}", performance_changes);
    
    println!("\nAnya Core demonstration complete!");
}

// Process a series of inputs to demonstrate the agent checker
fn process_demo_inputs(core: &CoreSystem) {
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
        
        println!("  Input [{}]: {}", i+1, input);
        core.process_input(&input).unwrap();
    }
    
    // Check the system stage
    let stage = core.agent_checker().get_system_stage();
    println!("Current system stage: {:?}", stage);
}

// Configure system security components to demonstrate hardening functionality
fn configure_system_security(core: &CoreSystem) {
    let hardening = core.system_hardening();
    
    // Configure network security
    println!("Configuring network security...");
    let mut network_settings = security::create_basic_security_config("network");
    network_settings.insert("vpn_required", "true".to_string());
    
    hardening.configure_component(
        "network", 
        SecurityLevel::Strict,
        network_settings,
        true
    ).unwrap();
    
    // Configure database security
    println!("Configuring database security...");
    let db_settings = security::create_basic_security_config("database");
    
    hardening.configure_component(
        "database", 
        SecurityLevel::Enhanced,
        db_settings,
        true
    ).unwrap();
    
    // Configure API security
    println!("Configuring API security...");
    let api_settings = security::create_basic_security_config("api");
    
    hardening.configure_component(
        "api", 
        SecurityLevel::Enhanced,
        api_settings,
        true
    ).unwrap();
    
    // Apply hardening to configured components
    println!("Applying security hardening...");
    hardening.apply_hardening("network").unwrap();
    hardening.apply_hardening("database").unwrap();
    hardening.apply_hardening("api").unwrap();
    
    // Configure 20 more components to trigger auto-save
    println!("Configuring 20 additional components to trigger auto-save...");
    for i in 0..20 {
        let component_name = format!("component_{}", i);
        let settings = security::create_basic_security_config(&component_name);
        
        hardening.configure_component(
            &component_name,
            SecurityLevel::Basic,
            settings,
            true
        ).unwrap();
    }
}

// Configure and optimize system performance
fn optimize_system_performance(core: &CoreSystem) {
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
        0.8, // Target utilization
        1000.0, // Target throughput
        Duration::from_millis(10), // Target latency
    ).unwrap();
    
    // Configure memory resource
    println!("Configuring memory optimization...");
    let mut mem_settings = HashMap::new();
    mem_settings.insert("cache_size".to_string(), "1024".to_string());
    mem_settings.insert("gc_threshold".to_string(), "75".to_string());
    
    optimizer.configure_resource(
        "memory",
        ResourceType::Memory,
        mem_settings,
        0.7, // Target utilization
        2000.0, // Target throughput
        Duration::from_millis(5), // Target latency
    ).unwrap();
    
    // Update metrics to simulate resource states
    println!("Updating performance metrics...");
    
    // CPU metrics - needs optimization
    let mut cpu_metrics = HashMap::new();
    cpu_metrics.insert("temperature".to_string(), 65.0);
    cpu_metrics.insert("context_switches".to_string(), 1500.0);
    
    optimizer.update_metrics(
        "cpu",
        0.85, // Current utilization (above target)
        950.0, // Current throughput (below target)
        Duration::from_millis(15), // Current latency (above target)
        cpu_metrics,
    ).unwrap();
    
    // Memory metrics - needs optimization
    let mut mem_metrics = HashMap::new();
    mem_metrics.insert("page_faults".to_string(), 25.0);
    mem_metrics.insert("allocation_rate".to_string(), 500.0);
    
    optimizer.update_metrics(
        "memory",
        0.75, // Current utilization (above target)
        1800.0, // Current throughput (below target)
        Duration::from_millis(8), // Current latency (above target)
        mem_metrics,
    ).unwrap();
    
    // Optimize resources
    println!("Optimizing resources...");
    optimizer.optimize_resource("cpu").unwrap();
    optimizer.optimize_resource("memory").unwrap();
    
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
        ).unwrap();
    }
} 