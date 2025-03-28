#![feature(edition2021)]
use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use prometheus::{Counter, Gauge, Histogram, HistogramOpts, Registry};

pub struct MonitoringSystem {
    registry: Registry,
    metrics: HashMap<String, Box<dyn Metric>>,
}

pub trait Metric {
    fn update(&self, value: f64);
    fn get_value(&self) -> f64;
    fn description(&self) -> &str;
}

impl MonitoringSystem {
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
        
        Self { registry, metrics }
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
}
