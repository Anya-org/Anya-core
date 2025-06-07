//! Performance monitoring and metrics collection
//!
//! This module provides utilities for tracking performance metrics
//! across the Bitcoin implementation.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Performance metrics collection
#[derive(Debug, Clone, Default)]
pub struct Metrics {
    /// Timing measurements
    pub timings: HashMap<String, Duration>,
    /// Counter metrics
    pub counters: HashMap<String, u64>,
    /// Gauge metrics (current values)
    pub gauges: HashMap<String, f64>,
    /// Start time for timing operations
    start_times: HashMap<String, Instant>,
}

impl Metrics {
    /// Create a new metrics instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Start timing an operation
    pub fn start_timer(&mut self, name: &str) {
        self.start_times.insert(name.to_string(), Instant::now());
    }

    /// Stop timing an operation and record the duration
    pub fn stop_timer(&mut self, name: &str) {
        if let Some(start_time) = self.start_times.remove(name) {
            let duration = start_time.elapsed();
            self.timings.insert(name.to_string(), duration);
        }
    }

    /// Record a timing measurement
    pub fn record_timing(&mut self, name: &str, duration: Duration) {
        self.timings.insert(name.to_string(), duration);
    }

    /// Increment a counter
    pub fn increment_counter(&mut self, name: &str) {
        let count = self.counters.get(name).unwrap_or(&0) + 1;
        self.counters.insert(name.to_string(), count);
    }

    /// Set a counter value
    pub fn set_counter(&mut self, name: &str, value: u64) {
        self.counters.insert(name.to_string(), value);
    }

    /// Set a gauge value
    pub fn set_gauge(&mut self, name: &str, value: f64) {
        self.gauges.insert(name.to_string(), value);
    }

    /// Get a timing measurement
    pub fn get_timing(&self, name: &str) -> Option<&Duration> {
        self.timings.get(name)
    }

    /// Get a counter value
    pub fn get_counter(&self, name: &str) -> u64 {
        self.counters.get(name).copied().unwrap_or(0)
    }

    /// Get a gauge value
    pub fn get_gauge(&self, name: &str) -> f64 {
        self.gauges.get(name).copied().unwrap_or(0.0)
    }

    /// Clear all metrics
    pub fn clear(&mut self) {
        self.timings.clear();
        self.counters.clear();
        self.gauges.clear();
        self.start_times.clear();
    }

    /// Get summary of all metrics
    pub fn summary(&self) -> String {
        let mut summary = String::new();
        
        if !self.timings.is_empty() {
            summary.push_str("Timings:\n");
            for (name, duration) in &self.timings {
                summary.push_str(&format!("  {}: {:?}\n", name, duration));
            }
        }
        
        if !self.counters.is_empty() {
            summary.push_str("Counters:\n");
            for (name, count) in &self.counters {
                summary.push_str(&format!("  {}: {}\n", name, count));
            }
        }
        
        if !self.gauges.is_empty() {
            summary.push_str("Gauges:\n");
            for (name, value) in &self.gauges {
                summary.push_str(&format!("  {}: {:.2}\n", name, value));
            }
        }
        
        summary
    }
}

/// Global metrics instance
static mut GLOBAL_METRICS: Option<Metrics> = None;
static INIT: std::sync::Once = std::sync::Once::new();

/// Get the global metrics instance
pub fn global_metrics() -> &'static mut Metrics {
    unsafe {
        INIT.call_once(|| {
            GLOBAL_METRICS = Some(Metrics::new());
        });
        GLOBAL_METRICS.as_mut().unwrap()
    }
}
