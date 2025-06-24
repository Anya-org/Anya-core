//! Monitoring System for Anya Core
//! 
//! This module provides real-time monitoring and metrics collection
//! for blockchain and system performance.

pub mod metrics;
pub mod generic_metrics;
pub mod blockchain_metrics;
pub mod blockchain_alerts;
pub mod metrics_service;
pub mod metrics_api;
pub mod metrics_controller;
pub mod service_integration;

use std::collections::HashMap;
use std::sync::Arc;
use log::info;

/// Legacy monitoring system (kept for backward compatibility)
pub struct MonitoringSystem {
    metrics_service: Arc<metrics_service::MetricsService>,
}

impl Default for MonitoringSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl MonitoringSystem {
    pub fn new() -> Self {
        let interval = metrics_service::get_metrics_interval();
        let metrics_service = Arc::new(metrics_service::MetricsService::new(Some(interval)));
        Self {
            metrics_service,
        }
    }

    /// Start the monitoring service
    pub fn start(&self) {
        info!("Starting monitoring system");
        self.metrics_service.start();
    }
    
    /// Stop the monitoring service
    pub fn stop(&self) {
        info!("Stopping monitoring system");
        self.metrics_service.stop();
    }
    
    /// Update a metric (legacy API)
    pub fn update_metric(&self, name: &str, value: f64) -> Result<(), String> {
        // Map legacy metrics to new system
        match name {
            "segwit_percentage" => {
                blockchain_metrics::update_segwit_percentage(value);
            },
            "taproot_percentage" => {
                blockchain_metrics::update_taproot_percentage(value);
            },
            "mempool_size" => {
                blockchain_metrics::update_mempool_size((value * 1024.0) as u64); // Convert KB to bytes
            },
            "block_propagation_time" => {
                blockchain_metrics::update_block_propagation_time("legacy", value as u64);
            },
            _ => {
                // For other metrics, store in the generic system
                metrics::register_metric(name, value);
            }
        }
        Ok(())
    }
    pub fn get_metrics(&self) -> HashMap<String, f64> {
        HashMap::new()
    }
}

pub struct Registry;
impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry {
    pub fn new() -> Self {
        Self
    }
}

pub struct NetworkMetric;
impl NetworkMetric {
    pub fn new(_registry: &Registry) -> Self {
        Self
    }
    pub fn update(&self, _value: f64) {}
    pub fn get_value(&self) -> f64 {
        0.0
    }
    pub fn description(&self) -> &'static str {
        "Network health status"
    }
}

pub struct FeeMetric;
impl FeeMetric {
    pub fn new(_registry: &Registry) -> Self {
        Self
    }
    pub fn update(&self, _value: f64) {}
    pub fn get_value(&self) -> f64 {
        0.0
    }
    pub fn description(&self) -> &'static str {
        "Current fee rate and distribution"
    }
}
