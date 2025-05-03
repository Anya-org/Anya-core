// Monitoring module
pub mod metrics;
pub mod server;

// Re-export commonly used types
pub use server::start_metrics_server;
pub use metrics::{
    register_bip_compliance,
    register_mempool_size,
    register_taproot_usage,
    register_block_propagation_time,
    export_metrics
};

/// Starts the monitoring subsystem
pub fn start_monitoring() -> std::io::Result<()> {
    // Start the metrics server on port 3000
    server::start_metrics_server("0.0.0.0:3000")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_metrics_registration_export() {
        // Register some metrics
        register_bip_compliance("341", true);
        register_bip_compliance("342", true);
        
        // Export metrics
        let metrics_output = export_metrics();
        
        // Verify metrics data
        assert!(metrics_output.contains("bip_compliance{type=\"341\"} 1"));
        assert!(metrics_output.contains("bip_compliance{type=\"342\"} 1"));
    }
} 