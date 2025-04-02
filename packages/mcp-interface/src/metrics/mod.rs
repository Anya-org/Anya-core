//! Metrics collection for MCP interface [AIM-3]
//!
//! This module provides metrics collection and reporting functionality
//! following the Bitcoin Development Framework v2.5 requirements

pub mod data;
pub use data::MetricsData;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};
use chrono::{DateTime, Utc};

/// Metrics collector for MCP [AIM-3]
#[derive(Debug, Clone)]
pub struct MetricsCollector {
    /// Request metrics by method
    metrics: Arc<Mutex<HashMap<String, u64>>>,
    /// Request durations by method
    durations: Arc<Mutex<HashMap<String, Vec<Duration>>>>,
    /// Total request count
    requests_total: Arc<Mutex<u64>>,
    /// Start time of the server
    start_time: DateTime<Utc>,
}

impl MetricsCollector {
    /// Create a new metrics collector [AIM-3]
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(Mutex::new(HashMap::new())),
            durations: Arc::new(Mutex::new(HashMap::new())),
            requests_total: Arc::new(Mutex::new(0)),
            start_time: Utc::now(),
        }
    }
    
    /// Record a request [AIM-3]
    pub fn record_request(&self, method: &str, _status: u16, duration: Duration) {
        // Increment request count
        let mut metrics = self.metrics.lock().unwrap();
        let count = metrics.entry(method.to_string()).or_insert(0);
        *count += 1;
        
        // Increment total request count
        let mut total = self.requests_total.lock().unwrap();
        *total += 1;
        
        // Record duration
        let mut durations = self.durations.lock().unwrap();
        let durations_vec = durations.entry(method.to_string()).or_insert_with(Vec::new);
        durations_vec.push(duration);
    }
    
    /// Increment request count
    pub fn increment_request_count(&self) {
        let mut total = self.requests_total.lock().unwrap();
        *total += 1;
    }
    
    /// Get metrics data [AIM-3]
    pub fn get_metrics(&self) -> MetricsData {
        let uptime = Utc::now().signed_duration_since(self.start_time);
        
        MetricsData::new(
            *self.requests_total.lock().unwrap(),
            uptime.num_seconds() as u64,
            self.start_time,
        )
    }
    
    /// Get request metrics by method
    pub fn get_request_metrics(&self) -> HashMap<String, u64> {
        self.metrics.lock().unwrap().clone()
    }
    
    /// Get total requests
    pub fn get_total_requests(&self) -> u64 {
        *self.requests_total.lock().unwrap()
    }
    
    /// Get uptime in seconds
    pub fn get_uptime_seconds(&self) -> u64 {
        let uptime = Utc::now().signed_duration_since(self.start_time);
        uptime.num_seconds() as u64
    }
    
    /// Get average duration for a method
    pub fn get_average_duration(&self, method: &str) -> Option<Duration> {
        let durations = self.durations.lock().unwrap();
        if let Some(duration_vec) = durations.get(method) {
            if duration_vec.is_empty() {
                return None;
            }
            
            let total: Duration = duration_vec.iter().sum();
            let count = duration_vec.len();
            
            Some(total / count as u32)
        } else {
            None
        }
    }
}
