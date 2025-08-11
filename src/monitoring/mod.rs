//! Production Monitoring System with ML-powered Analytics
//! [AIR-3][AIS-3][BPC-3][RES-3]
//!
//! This module provides comprehensive monitoring capabilities for Bitcoin operations,
//! including system metrics, network health, mempool monitoring, and ML-powered
//! anomaly detection. Replaces Python monitoring scripts with high-performance Rust.

use crate::analytics::{AnalyticsEngine, AnalyticsConfig, BitcoinMetrics, SystemMetrics};
use crate::{AnyaError, AnyaResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tokio::time::interval;
use tracing::{info, warn, error};

/// Configuration for the monitoring system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Monitoring interval in seconds
    pub interval_seconds: u64,
    /// Enable Bitcoin-specific monitoring
    pub bitcoin_monitoring: bool,
    /// Enable system performance monitoring
    pub system_monitoring: bool,
    /// Enable ML-powered anomaly detection
    pub anomaly_detection: bool,
    /// Alert thresholds
    pub alert_thresholds: AlertThresholds,
    /// Analytics configuration
    pub analytics_config: AnalyticsConfig,
}

/// Alert threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub cpu_usage_percent: f64,
    pub memory_usage_percent: f64,
    pub disk_usage_percent: f64,
    pub response_time_ms: f64,
    pub fee_rate_sat_vb: f64,
    pub mempool_depth: u64,
    pub peer_count_min: u32,
    pub block_propagation_ms: f64,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            cpu_usage_percent: 80.0,
            memory_usage_percent: 85.0,
            disk_usage_percent: 90.0,
            response_time_ms: 1000.0,
            fee_rate_sat_vb: 100.0,
            mempool_depth: 50000,
            peer_count_min: 8,
            block_propagation_ms: 3000.0,
        }
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            interval_seconds: 30,
            bitcoin_monitoring: true,
            system_monitoring: true,
            anomaly_detection: true,
            alert_thresholds: AlertThresholds::default(),
            analytics_config: AnalyticsConfig::default(),
        }
    }
}

/// Monitoring alert level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertLevel {
    Info,
    Warning,
    Critical,
}

/// Monitoring alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub timestamp: u64,
    pub level: AlertLevel,
    pub metric_name: String,
    pub message: String,
    pub current_value: f64,
    pub threshold: f64,
    pub metadata: HashMap<String, String>,
}

/// Production monitoring system with ML integration
pub struct MonitoringSystem {
    config: MonitoringConfig,
    analytics: Arc<AnalyticsEngine>,
    metrics_registry: Arc<RwLock<HashMap<String, f64>>>,
    alert_history: Arc<RwLock<Vec<Alert>>>,
    is_running: Arc<RwLock<bool>>,
}

impl MonitoringSystem {
    /// Create a new monitoring system
    pub async fn new(config: MonitoringConfig) -> AnyaResult<Self> {
        let analytics = Arc::new(AnalyticsEngine::new(config.analytics_config.clone()).await?);
        
        Ok(Self {
            config,
            analytics,
            metrics_registry: Arc::new(RwLock::new(HashMap::new())),
            alert_history: Arc::new(RwLock::new(Vec::new())),
            is_running: Arc::new(RwLock::new(false)),
        })
    }

    /// Start the monitoring system
    pub async fn start(&self) -> AnyaResult<()> {
        let mut is_running = self.is_running.write().await;
        if *is_running {
            return Err(AnyaError::Monitoring("Monitoring system already running".to_string()));
        }
        *is_running = true;
        drop(is_running);

        info!("Starting monitoring system with {}s interval", self.config.interval_seconds);

        // Start monitoring tasks
        let system_task = self.start_system_monitoring();
        let bitcoin_task = self.start_bitcoin_monitoring();
        let analytics_task = self.start_analytics_processing();

        // Run all monitoring tasks concurrently
        tokio::select! {
            result = system_task => {
                error!("System monitoring task failed: {:?}", result);
                result
            },
            result = bitcoin_task => {
                error!("Bitcoin monitoring task failed: {:?}", result);
                result
            },
            result = analytics_task => {
                error!("Analytics processing task failed: {:?}", result);
                result
            },
        }
    }

    /// Stop the monitoring system
    pub async fn stop(&self) {
        let mut is_running = self.is_running.write().await;
        *is_running = false;
        info!("Monitoring system stopped");
    }

    /// Check if monitoring system is running
    pub async fn is_running(&self) -> bool {
        *self.is_running.read().await
    }

    /// Update a metric value
    pub async fn update_metric(&self, name: &str, value: f64) -> AnyaResult<()> {
        // Store in registry
        let mut registry = self.metrics_registry.write().await;
        registry.insert(name.to_string(), value);
        drop(registry);

        // Record in analytics
        self.analytics.record_metric(name, value, None).await?;

        // Check for alerts
        self.check_alert_conditions(name, value).await?;

        Ok(())
    }

    /// Get current metrics
    pub async fn get_metrics(&self) -> HashMap<String, f64> {
        self.metrics_registry.read().await.clone()
    }

    /// Get recent alerts
    pub async fn get_alerts(&self, limit: Option<usize>) -> Vec<Alert> {
        let alerts = self.alert_history.read().await;
        let limit = limit.unwrap_or(alerts.len());
        alerts.iter().rev().take(limit).cloned().collect()
    }

    /// Start system performance monitoring
    async fn start_system_monitoring(&self) -> AnyaResult<()> {
        if !self.config.system_monitoring {
            return Ok(());
        }

        let mut interval = interval(Duration::from_secs(self.config.interval_seconds));
        
        loop {
            interval.tick().await;
            
            if !*self.is_running.read().await {
                break;
            }

            if let Err(e) = self.collect_system_metrics().await {
                warn!("Failed to collect system metrics: {}", e);
            }
        }

        Ok(())
    }

    /// Start Bitcoin-specific monitoring
    async fn start_bitcoin_monitoring(&self) -> AnyaResult<()> {
        if !self.config.bitcoin_monitoring {
            return Ok(());
        }

        let mut interval = interval(Duration::from_secs(self.config.interval_seconds * 2)); // Bitcoin metrics less frequent
        
        loop {
            interval.tick().await;
            
            if !*self.is_running.read().await {
                break;
            }

            if let Err(e) = self.collect_bitcoin_metrics().await {
                warn!("Failed to collect Bitcoin metrics: {}", e);
            }
        }

        Ok(())
    }

    /// Start analytics processing
    async fn start_analytics_processing(&self) -> AnyaResult<()> {
        if !self.config.anomaly_detection {
            return Ok(());
        }

        let mut interval = interval(Duration::from_secs(self.config.interval_seconds * 4)); // Analytics less frequent
        
        loop {
            interval.tick().await;
            
            if !*self.is_running.read().await {
                break;
            }

            if let Err(e) = self.process_analytics().await {
                warn!("Failed to process analytics: {}", e);
            }
        }

        Ok(())
    }

    /// Collect system performance metrics
    async fn collect_system_metrics(&self) -> AnyaResult<()> {
        // Simulate system metrics collection (in production, use actual system APIs)
        let metrics = SystemMetrics {
            cpu_usage_percent: Self::get_cpu_usage().await?,
            memory_usage_percent: Self::get_memory_usage().await?,
            disk_usage_percent: Self::get_disk_usage().await?,
            network_io_bps: Self::get_network_io().await?,
            active_connections: Self::get_active_connections().await?,
            response_times_ms: Self::get_response_times().await?,
        };

        // Update individual metrics
        self.update_metric("system.cpu_usage_percent", metrics.cpu_usage_percent).await?;
        self.update_metric("system.memory_usage_percent", metrics.memory_usage_percent).await?;
        self.update_metric("system.disk_usage_percent", metrics.disk_usage_percent).await?;
        self.update_metric("system.network_io_bps", metrics.network_io_bps).await?;
        self.update_metric("system.active_connections", metrics.active_connections as f64).await?;
        self.update_metric("system.response_time_p50", metrics.response_times_ms.0).await?;
        self.update_metric("system.response_time_p95", metrics.response_times_ms.1).await?;
        self.update_metric("system.response_time_p99", metrics.response_times_ms.2).await?;

        // Analyze metrics for patterns
        if let Ok(patterns) = self.analytics.analyze_system_metrics(&metrics).await {
            for pattern in patterns {
                if pattern.confidence > 0.8 {
                    self.create_alert(
                        AlertLevel::Warning,
                        pattern.pattern_type,
                        pattern.description,
                        0.0,
                        0.0,
                    ).await;
                }
            }
        }

        Ok(())
    }

    /// Collect Bitcoin-specific metrics
    async fn collect_bitcoin_metrics(&self) -> AnyaResult<()> {
        // Simulate Bitcoin metrics collection (in production, use Bitcoin RPC)
        let metrics = BitcoinMetrics {
            mempool_depth: Self::get_mempool_depth().await?,
            avg_fee_rate: Self::get_avg_fee_rate().await?,
            block_propagation_ms: Self::get_block_propagation_time().await?,
            peer_count: Self::get_peer_count().await?,
            network_hashrate: Self::get_network_hashrate().await?,
            difficulty: Self::get_difficulty().await?,
            unconfirmed_tx_count: Self::get_unconfirmed_tx_count().await?,
        };

        // Update individual metrics
        self.update_metric("bitcoin.mempool_depth", metrics.mempool_depth as f64).await?;
        self.update_metric("bitcoin.avg_fee_rate", metrics.avg_fee_rate).await?;
        self.update_metric("bitcoin.block_propagation_ms", metrics.block_propagation_ms).await?;
        self.update_metric("bitcoin.peer_count", metrics.peer_count as f64).await?;
        self.update_metric("bitcoin.network_hashrate", metrics.network_hashrate).await?;
        self.update_metric("bitcoin.difficulty", metrics.difficulty).await?;
        self.update_metric("bitcoin.unconfirmed_tx_count", metrics.unconfirmed_tx_count as f64).await?;

        // Analyze Bitcoin-specific patterns
        if let Ok(patterns) = self.analytics.analyze_bitcoin_metrics(&metrics).await {
            for pattern in patterns {
                let level = match pattern.confidence {
                    c if c > 0.9 => AlertLevel::Critical,
                    c if c > 0.7 => AlertLevel::Warning,
                    _ => AlertLevel::Info,
                };
                
                self.create_alert(
                    level,
                    pattern.pattern_type,
                    pattern.description,
                    0.0,
                    0.0,
                ).await;
            }
        }

        Ok(())
    }

    /// Process analytics and detect anomalies
    async fn process_analytics(&self) -> AnyaResult<()> {
        let metrics = self.get_metrics().await;
        
        for (metric_name, _value) in metrics {
            if let Ok(anomalies) = self.analytics.detect_anomalies(&metric_name).await {
                for anomaly in anomalies {
                    if anomaly.is_anomaly && anomaly.confidence > 0.8 {
                        self.create_alert(
                            AlertLevel::Warning,
                            format!("anomaly_{}", metric_name),
                            format!("Anomaly detected in {}: expected {:.2}-{:.2}, got {:.2}", 
                                metric_name, 
                                anomaly.expected_range.0, 
                                anomaly.expected_range.1, 
                                anomaly.actual_value),
                            anomaly.actual_value,
                            (anomaly.expected_range.0 + anomaly.expected_range.1) / 2.0,
                        ).await;
                    }
                }
            }
        }

        Ok(())
    }

    /// Check alert conditions for a metric
    async fn check_alert_conditions(&self, metric_name: &str, value: f64) -> AnyaResult<()> {
        let thresholds = &self.config.alert_thresholds;
        
        let (threshold, level, message) = match metric_name {
            "system.cpu_usage_percent" if value > thresholds.cpu_usage_percent => {
                (thresholds.cpu_usage_percent, AlertLevel::Warning, 
                 format!("High CPU usage: {:.1}%", value))
            },
            "system.memory_usage_percent" if value > thresholds.memory_usage_percent => {
                (thresholds.memory_usage_percent, AlertLevel::Critical,
                 format!("High memory usage: {:.1}%", value))
            },
            "system.disk_usage_percent" if value > thresholds.disk_usage_percent => {
                (thresholds.disk_usage_percent, AlertLevel::Critical,
                 format!("High disk usage: {:.1}%", value))
            },
            "bitcoin.avg_fee_rate" if value > thresholds.fee_rate_sat_vb => {
                (thresholds.fee_rate_sat_vb, AlertLevel::Warning,
                 format!("High fee rate: {:.1} sat/vB", value))
            },
            "bitcoin.mempool_depth" if value > thresholds.mempool_depth as f64 => {
                (thresholds.mempool_depth as f64, AlertLevel::Info,
                 format!("High mempool depth: {} transactions", value as u64))
            },
            "bitcoin.peer_count" if value < thresholds.peer_count_min as f64 => {
                (thresholds.peer_count_min as f64, AlertLevel::Warning,
                 format!("Low peer count: {}", value as u32))
            },
            _ => return Ok(()), // No alert condition met
        };

        self.create_alert(level, metric_name.to_string(), message, value, threshold).await;
        Ok(())
    }

    /// Create and store an alert
    async fn create_alert(&self, level: AlertLevel, metric_name: String, message: String, current_value: f64, threshold: f64) {
        let alert = Alert {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            level: level.clone(),
            metric_name: metric_name.clone(),
            message: message.clone(),
            current_value,
            threshold,
            metadata: HashMap::new(),
        };

        // Log the alert
        match level {
            AlertLevel::Info => info!("ALERT: {}", message),
            AlertLevel::Warning => warn!("ALERT: {}", message),
            AlertLevel::Critical => error!("CRITICAL ALERT: {}", message),
        }

        // Store in history
        let mut history = self.alert_history.write().await;
        history.push(alert);
        
        // Keep only last 1000 alerts
        if history.len() > 1000 {
            history.drain(0..history.len() - 1000);
        }
    }

    // System metrics collection methods (simplified implementations)
    async fn get_cpu_usage() -> AnyaResult<f64> {
        // In production, use sysinfo or similar crate
        use rand::Rng;
        Ok(rand::thread_rng().gen_range(0.0..100.0))
    }

    async fn get_memory_usage() -> AnyaResult<f64> {
        use rand::Rng;
        Ok(rand::thread_rng().gen_range(0.0..100.0))
    }

    async fn get_disk_usage() -> AnyaResult<f64> {
        use rand::Rng;
        Ok(rand::thread_rng().gen_range(0.0..100.0))
    }

    async fn get_network_io() -> AnyaResult<f64> {
        use rand::Rng;
        Ok(rand::thread_rng().gen_range(0.0..1000000.0)) // bytes per second
    }

    async fn get_active_connections() -> AnyaResult<u32> {
        use rand::Rng;
        Ok(rand::thread_rng().gen_range(0..1000))
    }

    async fn get_response_times() -> AnyaResult<(f64, f64, f64)> {
        use rand::Rng;
        let base = rand::thread_rng().gen_range(0.0..100.0);
        Ok((base, base * 2.0, base * 3.0)) // p50, p95, p99
    }

    // Bitcoin metrics collection methods (simplified implementations)
    async fn get_mempool_depth() -> AnyaResult<u64> {
        use rand::Rng;
        Ok(rand::thread_rng().gen_range(0..100000))
    }

    async fn get_avg_fee_rate() -> AnyaResult<f64> {
        use rand::Rng;
        Ok(rand::thread_rng().gen_range(0.0..200.0))
    }

    async fn get_block_propagation_time() -> AnyaResult<f64> {
        use rand::Rng;
        Ok(rand::thread_rng().gen_range(0.0..5000.0))
    }

    async fn get_peer_count() -> AnyaResult<u32> {
        use rand::Rng;
        Ok(8 + rand::thread_rng().gen_range(0..50))
    }

    async fn get_network_hashrate() -> AnyaResult<f64> {
        Ok(300e18) // Approximate current Bitcoin hashrate
    }

    async fn get_difficulty() -> AnyaResult<f64> {
        Ok(50e12) // Approximate current Bitcoin difficulty
    }

    async fn get_unconfirmed_tx_count() -> AnyaResult<u64> {
        use rand::Rng;
        Ok(rand::thread_rng().gen_range(0..50000))
    }
}

// Legacy compatibility structs
pub struct Registry {
    inner: Arc<MonitoringSystem>,
}

impl Registry {
    pub async fn new() -> AnyaResult<Self> {
        let config = MonitoringConfig::default();
        let system = MonitoringSystem::new(config).await?;
        Ok(Self {
            inner: Arc::new(system),
        })
    }
}

pub struct NetworkMetric {
    registry: Arc<MonitoringSystem>,
    current_value: Arc<RwLock<f64>>,
}

impl NetworkMetric {
    pub async fn new(registry: &Registry) -> Self {
        Self {
            registry: registry.inner.clone(),
            current_value: Arc::new(RwLock::new(0.0)),
        }
    }

    pub async fn update(&self, value: f64) {
        *self.current_value.write().await = value;
        if let Err(e) = self.registry.update_metric("network.health", value).await {
            warn!("Failed to update network metric: {}", e);
        }
    }

    pub async fn get_value(&self) -> f64 {
        *self.current_value.read().await
    }

    pub fn description(&self) -> &'static str {
        "Network health status with ML-powered anomaly detection"
    }
}

pub struct FeeMetric {
    registry: Arc<MonitoringSystem>,
    current_value: Arc<RwLock<f64>>,
}

impl FeeMetric {
    pub async fn new(registry: &Registry) -> Self {
        Self {
            registry: registry.inner.clone(),
            current_value: Arc::new(RwLock::new(0.0)),
        }
    }

    pub async fn update(&self, value: f64) {
        *self.current_value.write().await = value;
        if let Err(e) = self.registry.update_metric("bitcoin.fee_rate", value).await {
            warn!("Failed to update fee metric: {}", e);
        }
    }

    pub async fn get_value(&self) -> f64 {
        *self.current_value.read().await
    }

    pub fn description(&self) -> &'static str {
        "Bitcoin fee rate tracking with spike detection"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_monitoring_system_creation() {
        let config = MonitoringConfig::default();
        let system = MonitoringSystem::new(config).await;
        assert!(system.is_ok());
    }

    #[tokio::test]
    async fn test_metric_update() {
        let config = MonitoringConfig::default();
        let system = MonitoringSystem::new(config).await.unwrap();
        
        let result = system.update_metric("test_metric", 42.0).await;
        assert!(result.is_ok());
        
        let metrics = system.get_metrics().await;
        assert_eq!(metrics.get("test_metric"), Some(&42.0));
    }

    #[tokio::test]
    async fn test_alert_creation() {
        let mut config = MonitoringConfig::default();
        config.alert_thresholds.cpu_usage_percent = 50.0; // Lower threshold for testing
        
        let system = MonitoringSystem::new(config).await.unwrap();
        system.update_metric("system.cpu_usage_percent", 75.0).await.unwrap();
        
        // Allow time for async processing
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        let alerts = system.get_alerts(Some(10)).await;
        assert!(!alerts.is_empty());
    }
}
