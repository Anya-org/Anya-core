use std::sync::Arc;
use log::{info, error};
use tokio::runtime::Runtime;

use crate::monitoring::metrics_service::MetricsService;
use crate::monitoring::metrics_controller::MetricsController;

/// Default port for metrics API
const DEFAULT_METRICS_PORT: u16 = 9200;

/// Initialize and start the metrics monitoring system
pub fn initialize_metrics_monitoring(runtime: &Runtime) -> Result<(), String> {
    let metrics_interval = crate::monitoring::metrics_service::get_metrics_interval();
    info!("Initializing metrics monitoring with interval {}ms", metrics_interval);
    
    // Create metrics service
    let metrics_service = Arc::new(MetricsService::new(Some(metrics_interval)));
    
    // Get metrics port from environment or use default
    let metrics_port = std::env::var("ANYA_METRICS_PORT")
        .ok()
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(DEFAULT_METRICS_PORT);
    
    // Create metrics controller
    let metrics_controller = MetricsController::new(
        Arc::clone(&metrics_service),
        metrics_port,
    );
    
    // Start controller in the tokio runtime
    runtime.block_on(async {
        metrics_controller.start().await;
    });
    
    info!("Metrics monitoring started on port {}", metrics_port);
    Ok(())
}

/// Get metrics service URL based on config
pub fn get_metrics_service_url() -> String {
    let metrics_port = std::env::var("ANYA_METRICS_PORT")
        .ok()
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(DEFAULT_METRICS_PORT);
    
    let metrics_host = std::env::var("ANYA_METRICS_HOST")
        .unwrap_or_else(|_| "localhost".to_string());
    
    format!("http://{}:{}", metrics_host, metrics_port)
}
