//! Metrics collection for MCP interface
//!
//! This module provides metrics collection and reporting functionality

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

/// Metrics collector for MCP
#[derive(Debug, Clone)]
pub struct MetricsCollector {
    metrics: Arc<Mutex<HashMap<String, u64>>>,
    durations: Arc<Mutex<HashMap<String, Vec<Duration>>>>,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(Mutex::new(HashMap::new())),
            durations: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Record a request
    pub fn record_request(&self, method: &str, status: u16, duration: Duration) {
        // Increment request count
        let mut metrics = self.metrics.lock().unwrap();
        let count = metrics.entry(method.to_string()).or_insert(0);
        *count += 1;
        
        // Record duration
        let mut durations = self.durations.lock().unwrap();
        let durations_vec = durations.entry(method.to_string()).or_insert_with(Vec::new);
        durations_vec.push(duration);
    }
    
    /// Get metrics data
    pub fn get_metrics(&self) -> HashMap<String, u64> {
        self.metrics.lock().unwrap().clone()
    }
    
    /// Get average duration for a method
    pub fn get_average_duration(&self, method: &str) -> Option<Duration> {
        let durations = self.durations.lock().unwrap();
        if let Some(duration_vec) = durations.get(method) {
            if duration_vec.is_empty() {
                return None;
            }
            
            let total = duration_vec.iter().sum::<Duration>();
            let count = duration_vec.len() as u32;
            
            Some(total / count)
        } else {
            None
        }
    }
    
    /// Reset metrics
    pub fn reset(&self) {
        let mut metrics = self.metrics.lock().unwrap();
        metrics.clear();
        
        let mut durations = self.durations.lock().unwrap();
        durations.clear();
    }
}
