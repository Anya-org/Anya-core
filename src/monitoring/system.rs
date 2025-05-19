//! Monitoring System Module
//! [AIR-3][AIS-3][AIM-3][BPC-3][RES-3]
//!
//! This module provides monitoring capabilities according to BDF v2.5 requirements.

use std::error::Error;
use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use prometheus::{Counter, Gauge, Histogram, HistogramOpts, Registry};
use std::time::Instant;

/// Bitcoin monitoring system according to BDF v2.5 requirements
/// [AIR-3][AIS-3][AIM-3][BPC-3][RES-3]
pub struct MonitoringSystem {
    registry: Registry,
    metrics: HashMap<String, Box<dyn Metric>>,
    last_update: Instant,
}

/// Metric category for classification
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MetricCategory {
    MemPool,
    Blocks,
    Security,
    Performance,
    Network,
    Fees,
}

/// Enhanced metric trait with BDF v2.5 compliance
/// [AIR-3][AIS-3][BPC-3]
pub trait Metric {
    fn update(&self, value: f64);
    fn get_value(&self) -> f64;
    fn description(&self) -> &str;
    fn category(&self) -> MetricCategory;
    fn threshold(&self) -> Option<f64>;
    fn is_critical(&self) -> bool;
}

impl MonitoringSystem {
    /// Create a new monitoring system with BDF v2.5 compliant metrics
    /// [AIR-3][AIS-3][BPC-3]
    pub fn new() -> Self {
        let registry = Registry::new();
        let mut metrics = HashMap::new();
        
        // Add basic metrics
        metrics.insert(
            "network_monitor".to_string(),
            Box::new(NetworkMetric::new(&registry)) as Box<dyn Metric>,
        );
        metrics.insert(
            "fee_monitor".to_string(),
            Box::new(FeeMetric::new(&registry)) as Box<dyn Metric>,
        );
        
        // Add BDF v2.5 required metrics
        metrics.insert(
            "mempool_depth".to_string(),
            Box::new(MempoolMetric::new(&registry)) as Box<dyn Metric>,
        );
        metrics.insert(
            "block_propagation".to_string(),
            Box::new(BlockPropagationMetric::new(&registry)) as Box<dyn Metric>,
        );
        metrics.insert(
            "tps".to_string(),
            Box::new(TPSMetric::new(&registry)) as Box<dyn Metric>,
        );
        
        Self { 
            registry, 
            metrics,
            last_update: Instant::now(),
        }
    }

    pub fn update_metric(&self, name: &str, value: f64) -> Result<()> {
        if let Some(metric) = self.metrics.get(name) {
            metric.update(value);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Metric {} not found", name))
        }
    }

    pub fn get_metrics(&self) -> HashMap<String, f64> {
        self.metrics
            .iter()
            .map(|(name, metric)| (name.clone(), metric.get_value()))
            .collect()
    }
}

struct NetworkMetric {
    gauge: Gauge,
}

impl NetworkMetric {
    pub fn new(registry: &Registry) -> Self {
        let gauge = Gauge::new("network_health", "Network health status").unwrap();
        registry.register(Box::new(gauge.clone())).unwrap();
        Self { gauge }
    }
}

impl Metric for NetworkMetric {
    fn update(&self, value: f64) {
        self.gauge.set(value);
    }

    fn get_value(&self) -> f64 {
        self.gauge.get()
    }

    fn description(&self) -> &str {
        "Network health status"
    }
    
    fn category(&self) -> MetricCategory {
        MetricCategory::Network
    }
    
    fn threshold(&self) -> Option<f64> {
        Some(0.8) // Alert if network health drops below 80%
    }
    
    fn is_critical(&self) -> bool {
        true
    }
}

struct FeeMetric {
    gauge: Gauge,
    histogram: Histogram,
}

impl FeeMetric {
    pub fn new(registry: &Registry) -> Self {
        let gauge = Gauge::new("fee_rate", "Current fee rate").unwrap();
        let histogram_opts = HistogramOpts::new("fee_rate_distribution", "Fee rate distribution")
            .buckets(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let histogram = Histogram::with_opts(histogram_opts).unwrap();
        
        registry.register(Box::new(gauge.clone())).unwrap();
        registry.register(Box::new(histogram.clone())).unwrap();
        
        Self { gauge, histogram }
    }
}

impl Metric for FeeMetric {
    fn update(&self, value: f64) {
        self.gauge.set(value);
        self.histogram.observe(value);
    }

    fn get_value(&self) -> f64 {
        self.gauge.get()
    }

    fn description(&self) -> &str {
        "Current fee rate and distribution"
    }
    
    fn category(&self) -> MetricCategory {
        MetricCategory::Fees
    }
    
    fn threshold(&self) -> Option<f64> {
        Some(200.0) // Alert if fee rate spikes by 200% as per BDF v2.5
    }
    
    fn is_critical(&self) -> bool {
        false
    }
}

