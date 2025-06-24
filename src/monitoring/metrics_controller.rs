use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, error};
use warp::Filter;

use crate::monitoring::metrics_api;
use crate::monitoring::metrics_service::MetricsService;

/// Metrics controller handling metrics API and service
pub struct MetricsController {
    /// Metrics collection service
    metrics_service: Arc<MetricsService>,
    
    /// API server port
    port: u16,
    
    /// Whether the controller is running
    running: Arc<Mutex<bool>>,
}

impl MetricsController {
    /// Create a new metrics controller
    pub fn new(metrics_service: Arc<MetricsService>, port: u16) -> Self {
        Self {
            metrics_service,
            port,
            running: Arc::new(Mutex::new(false)),
        }
    }
    
    /// Start the metrics controller (API server and metrics collection)
    pub async fn start(&self) {
        let mut running = self.running.lock().await;
        
        if *running {
            info!("Metrics controller already running");
            return;
        }
        
        *running = true;
        
        // Start metrics collection service
        self.metrics_service.start();
        
        // Create API routes
        let routes = metrics_api::create_routes();
        
        // Clone shared state for the server thread
        let running_clone = Arc::clone(&self.running);
        let port = self.port;
        
        // Start API server in separate task
        tokio::spawn(async move {
            info!("Starting metrics API server on port {}", port);
            
            // Create server with graceful shutdown
            let (_addr, server_future) = warp::serve(routes.clone()).bind_with_graceful_shutdown(
                ([0, 0, 0, 0], port),
                async move {
                    // Wait until running becomes false
                    loop {
                        let is_running = *running_clone.lock().await;
                        if !is_running {
                            break;
                        }
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    }
                },
            );
            
            // Start the server
            server_future.await;
            info!("Metrics API server stopped");
        });
        
        info!("Metrics controller started");
    }
    
    /// Stop the metrics controller
    pub async fn stop(&self) {
        let mut running = self.running.lock().await;
        
        if !*running {
            info!("Metrics controller not running");
            return;
        }
        
        // Signal the server to stop
        *running = false;
        
        // Stop metrics collection service
        self.metrics_service.stop();
        
        info!("Metrics controller stopping");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monitoring::metrics_service;
    
    #[tokio::test]
    async fn test_metrics_controller() {
        // Create dependencies
        // Using fixed interval of 100ms for tests
        let metrics_service = Arc::new(metrics_service::MetricsService::new(Some(100)));
        
        // Create controller with test port
        let controller = MetricsController::new(Arc::clone(&metrics_service), 9090);
        
        // Start controller
        controller.start().await;
        
        // Let it run for a moment
        tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
        
        // Stop controller
        controller.stop().await;
    }
}
