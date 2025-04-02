// Metrics Data Module [AIM-3]
//
// This module defines the metrics data structures
// following Bitcoin Development Framework v2.5 requirements

use chrono::{DateTime, Utc};

/// Metrics data structure
#[derive(Debug, Clone)]
pub struct MetricsData {
    /// Total requests processed
    pub requests_total: u64,
    /// Uptime in seconds
    pub uptime_seconds: u64,
    /// Server start time
    pub start_time: DateTime<Utc>,
}

impl MetricsData {
    /// Create a new metrics data instance
    pub fn new(requests_total: u64, uptime_seconds: u64, start_time: DateTime<Utc>) -> Self {
        Self {
            requests_total,
            uptime_seconds,
            start_time,
        }
    }
    
    /// Format metrics as JSON
    pub fn as_json(&self) -> serde_json::Value {
        serde_json::json!({
            "requests_total": self.requests_total,
            "uptime_seconds": self.uptime_seconds,
            "start_time": self.start_time.to_rfc3339(),
        })
    }
}
