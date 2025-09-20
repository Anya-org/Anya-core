//! Analytics Module
//!
//! This module provides ML-powered analytics capabilities for the Anya system,
//! including anomaly detection, pattern recognition, and performance analysis.
//! This replaces the Python monitoring scripts with high-performance Rust implementations.

use crate::ml::{MLSystem, MLConfig, MLInput};
use crate::{AnyaError, AnyaResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use std::sync::Arc;

/// Configuration for analytics system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsConfig {
    /// Enable anomaly detection
    pub anomaly_detection_enabled: bool,
    /// Minimum confidence threshold for anomaly detection
    pub anomaly_threshold: f64,
    /// Window size for time series analysis (in seconds)
    pub time_window_seconds: u64,
    /// Maximum number of data points to store
    pub max_data_points: usize,
    /// Enable real-time analysis
    pub real_time_analysis: bool,
}

impl Default for AnalyticsConfig {
    fn default() -> Self {
        Self {
            anomaly_detection_enabled: true,
            anomaly_threshold: 0.8,
            time_window_seconds: 300, // 5 minutes
            max_data_points: 10000,
            real_time_analysis: true,
        }
    }
}

/// Analytics data point for time series analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    pub timestamp: u64,
    pub metric_name: String,
    pub value: f64,
    pub metadata: HashMap<String, String>,
}

/// Anomaly detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyResult {
    pub timestamp: u64,
    pub metric_name: String,
    pub anomaly_score: f64,
    pub is_anomaly: bool,
    pub confidence: f64,
    pub expected_range: (f64, f64),
    pub actual_value: f64,
}

/// Pattern analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternResult {
    pub pattern_type: String,
    pub confidence: f64,
    pub description: String,
    pub recommendations: Vec<String>,
}

/// Bitcoin-specific metrics for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitcoinMetrics {
    /// Current mempool depth
    pub mempool_depth: u64,
    /// Average fee rate (sat/vB)
    pub avg_fee_rate: f64,
    /// Block propagation time (ms)
    pub block_propagation_ms: f64,
    /// Number of connected peers
    pub peer_count: u32,
    /// Network hash rate estimate
    pub network_hashrate: f64,
    /// Current difficulty
    pub difficulty: f64,
    /// Unconfirmed transaction count
    pub unconfirmed_tx_count: u64,
}

/// System performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Memory usage percentage
    pub memory_usage_percent: f64,
    /// Disk usage percentage
    pub disk_usage_percent: f64,
    /// Network I/O bytes per second
    pub network_io_bps: f64,
    /// Number of active connections
    pub active_connections: u32,
    /// Response time percentiles (p50, p95, p99)
    pub response_times_ms: (f64, f64, f64),
}

/// Analytics engine with ML-powered capabilities
pub struct AnalyticsEngine {
    config: AnalyticsConfig,
    ml_system: Arc<RwLock<MLSystem>>,
    data_store: Arc<RwLock<HashMap<String, Vec<DataPoint>>>>,
    anomaly_models: Arc<RwLock<HashMap<String, String>>>, // metric_name -> model_id
}

impl AnalyticsEngine {
    /// Create a new analytics engine
    pub async fn new(config: AnalyticsConfig) -> AnyaResult<Self> {
        let ml_config = MLConfig {
            enabled: true,
            model_path: Some("./data/analytics_models".to_string()),
            use_gpu: false, // CPU-only for analytics
            federated_learning: false,
            max_model_size: 50 * 1024 * 1024, // 50 MB
        };

        let ml_system = MLSystem::new(ml_config).await?;

        Ok(Self {
            config,
            ml_system: Arc::new(RwLock::new(ml_system)),
            data_store: Arc::new(RwLock::new(HashMap::new())),
            anomaly_models: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Record a data point for analysis
    pub async fn record_metric(&self, 
        metric_name: &str, 
        value: f64, 
        metadata: Option<HashMap<String, String>>
    ) -> AnyaResult<()> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let data_point = DataPoint {
            timestamp,
            metric_name: metric_name.to_string(),
            value,
            metadata: metadata.unwrap_or_default(),
        };

        let mut store = self.data_store.write().await;
        let series = store.entry(metric_name.to_string()).or_insert_with(Vec::new);
        
        series.push(data_point);
        
        // Maintain window size
        if series.len() > self.config.max_data_points {
            series.drain(0..series.len() - self.config.max_data_points);
        }

        // Trigger real-time analysis if enabled
        if self.config.real_time_analysis {
            drop(store); // Release write lock
            self.analyze_real_time(metric_name, value).await?;
        }

        Ok(())
    }

    /// Perform anomaly detection on a metric
    pub async fn detect_anomalies(&self, metric_name: &str) -> AnyaResult<Vec<AnomalyResult>> {
        let store = self.data_store.read().await;
        let series = store.get(metric_name)
            .ok_or_else(|| AnyaError::Analytics(format!("No data for metric: {}", metric_name)))?;

        if series.len() < 10 {
            return Ok(Vec::new()); // Need at least 10 points for analysis
        }

        let mut anomalies = Vec::new();
        let ml_system = self.ml_system.read().await;

        // Prepare features for ML model
        let window_size = 10;
        for i in window_size..series.len() {
            let features: Vec<f64> = series[i-window_size..i]
                .iter()
                .map(|dp| dp.value)
                .collect();

            let input = MLInput {
                features,
                label: series[i].value,
                metadata: Some(series[i].metadata.clone()),
            };

            // Use ML service for anomaly detection
            let inference_result = ml_system.service()
                .predict("anomaly_detector", &serde_json::to_vec(&input)?)
                .await
                .map_err(|e| AnyaError::Analytics(format!("ML inference failed: {}", e)))?;

            let anomaly_score: f64 = serde_json::from_slice(&inference_result.output)?;
            let is_anomaly = anomaly_score > self.config.anomaly_threshold;

            if is_anomaly {
                // Calculate expected range based on historical data
                let recent_values: Vec<f64> = series[i-window_size..i]
                    .iter()
                    .map(|dp| dp.value)
                    .collect();
                
                let mean = recent_values.iter().sum::<f64>() / recent_values.len() as f64;
                let variance = recent_values.iter()
                    .map(|x| (x - mean).powi(2))
                    .sum::<f64>() / recent_values.len() as f64;
                let std_dev = variance.sqrt();

                let expected_range = (mean - 2.0 * std_dev, mean + 2.0 * std_dev);

                anomalies.push(AnomalyResult {
                    timestamp: series[i].timestamp,
                    metric_name: metric_name.to_string(),
                    anomaly_score,
                    is_anomaly,
                    confidence: inference_result.overall_confidence,
                    expected_range,
                    actual_value: series[i].value,
                });
            }
        }

        Ok(anomalies)
    }

    /// Analyze Bitcoin-specific metrics for potential issues
    pub async fn analyze_bitcoin_metrics(&self, metrics: &BitcoinMetrics) -> AnyaResult<Vec<PatternResult>> {
        let mut patterns = Vec::new();

        // Fee spike detection
        if metrics.avg_fee_rate > 50.0 {
            let severity = if metrics.avg_fee_rate > 200.0 { "critical" } else { "warning" };
            patterns.push(PatternResult {
                pattern_type: "fee_spike".to_string(),
                confidence: 0.9,
                description: format!("High fee rate detected: {} sat/vB", metrics.avg_fee_rate),
                recommendations: vec![
                    "Monitor mempool congestion".to_string(),
                    "Consider batching transactions".to_string(),
                    "Use RBF for stuck transactions".to_string(),
                ],
            });
        }

        // Mempool congestion analysis
        if metrics.mempool_depth > 100_000 {
            patterns.push(PatternResult {
                pattern_type: "mempool_congestion".to_string(),
                confidence: 0.85,
                description: format!("High mempool depth: {} transactions", metrics.mempool_depth),
                recommendations: vec![
                    "Expect delayed confirmations".to_string(),
                    "Use higher fee rates for priority transactions".to_string(),
                ],
            });
        }

        // Network connectivity issues
        if metrics.peer_count < 8 {
            patterns.push(PatternResult {
                pattern_type: "low_connectivity".to_string(),
                confidence: 0.7,
                description: format!("Low peer count: {}", metrics.peer_count),
                recommendations: vec![
                    "Check network connectivity".to_string(),
                    "Verify firewall settings".to_string(),
                    "Consider manual peer connections".to_string(),
                ],
            });
        }

        // Block propagation issues
        if metrics.block_propagation_ms > 5000.0 {
            patterns.push(PatternResult {
                pattern_type: "slow_block_propagation".to_string(),
                confidence: 0.8,
                description: format!("Slow block propagation: {:.2}ms", metrics.block_propagation_ms),
                recommendations: vec![
                    "Check network bandwidth".to_string(),
                    "Verify Bitcoin Core version".to_string(),
                    "Consider compact block relay".to_string(),
                ],
            });
        }

        Ok(patterns)
    }

    /// Analyze system performance metrics
    pub async fn analyze_system_metrics(&self, metrics: &SystemMetrics) -> AnyaResult<Vec<PatternResult>> {
        let mut patterns = Vec::new();

        // High CPU usage
        if metrics.cpu_usage_percent > 80.0 {
            patterns.push(PatternResult {
                pattern_type: "high_cpu_usage".to_string(),
                confidence: 0.9,
                description: format!("High CPU usage: {:.1}%", metrics.cpu_usage_percent),
                recommendations: vec![
                    "Check for resource-intensive processes".to_string(),
                    "Consider scaling up compute resources".to_string(),
                    "Review indexing and validation settings".to_string(),
                ],
            });
        }

        // High memory usage
        if metrics.memory_usage_percent > 85.0 {
            patterns.push(PatternResult {
                pattern_type: "high_memory_usage".to_string(),
                confidence: 0.9,
                description: format!("High memory usage: {:.1}%", metrics.memory_usage_percent),
                recommendations: vec![
                    "Reduce dbcache setting".to_string(),
                    "Consider memory optimization".to_string(),
                    "Check for memory leaks".to_string(),
                ],
            });
        }

        // High response times
        if metrics.response_times_ms.2 > 1000.0 { // p99 > 1 second
            patterns.push(PatternResult {
                pattern_type: "high_response_times".to_string(),
                confidence: 0.85,
                description: format!("High response times - P99: {:.2}ms", metrics.response_times_ms.2),
                recommendations: vec![
                    "Optimize database queries".to_string(),
                    "Check disk I/O performance".to_string(),
                    "Consider request rate limiting".to_string(),
                ],
            });
        }

        Ok(patterns)
    }

    /// Generate comprehensive analytics report
    pub async fn generate_report(&self, time_range_hours: u32) -> AnyaResult<AnalyticsReport> {
        let cutoff_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() - (time_range_hours as u64 * 3600);

        let store = self.data_store.read().await;
        let mut anomaly_summary = HashMap::new();
        let mut metric_summary = HashMap::new();

        for (metric_name, series) in store.iter() {
            // Filter data points within time range
            let recent_points: Vec<_> = series.iter()
                .filter(|dp| dp.timestamp >= cutoff_time)
                .collect();

            if recent_points.is_empty() {
                continue;
            }

            // Calculate basic statistics
            let values: Vec<f64> = recent_points.iter().map(|dp| dp.value).collect();
            let mean = values.iter().sum::<f64>() / values.len() as f64;
            let min = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let max = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

            metric_summary.insert(metric_name.clone(), MetricSummary {
                data_points: recent_points.len(),
                mean,
                min,
                max,
                trend: calculate_trend(&values),
            });
        }
        
        // Release read lock before anomaly detection
        drop(store);

        // Collect metric names for anomaly detection
        let metric_names: Vec<_> = metric_summary.keys().cloned().collect();
        
        // Process anomaly detection for each metric outside the lock
        for metric_name in metric_names {
            let anomalies = self.detect_anomalies(&metric_name).await?;
            let recent_anomalies = anomalies.into_iter()
                .filter(|a| a.timestamp >= cutoff_time)
                .collect::<Vec<_>>();
            anomaly_summary.insert(metric_name, recent_anomalies);
        }

        Ok(AnalyticsReport {
            time_range_hours,
            generated_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            metric_summary,
            recommendations: self.generate_recommendations(&anomaly_summary).await,
            anomaly_summary,
        })
    }

    /// Perform real-time analysis on incoming data
    async fn analyze_real_time(&self, metric_name: &str, value: f64) -> AnyaResult<()> {
        // Record metric for real-time analysis (avoid recursion)
        let mut metadata = HashMap::new();
        metadata.insert("analysis_type".to_string(), "real_time".to_string());
        
        // Direct storage update to avoid recursion
        let realtime_metric = format!("{}_realtime", metric_name);
        let data_point = DataPoint {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            metric_name: realtime_metric.clone(),
            value,
            metadata,
        };
        
        {
            let mut store = self.data_store.write().await;
            store.entry(realtime_metric).or_default().push(data_point);
        }

        // Trigger immediate anomaly check if significant deviation
        // This is a simplified check - in production, you'd use more sophisticated methods
        let store = self.data_store.read().await;
        if let Some(series) = store.get(metric_name) {
            if series.len() > 5 {
                let recent_avg = series.iter()
                    .rev()
                    .take(5)
                    .map(|dp| dp.value)
                    .sum::<f64>() / 5.0;
                
                let deviation = (value - recent_avg).abs() / recent_avg;
                if deviation > 0.5 { // 50% deviation triggers alert
                    tracing::warn!(
                        "Real-time anomaly detected: {} = {} (avg: {}, deviation: {:.1}%)",
                        metric_name, value, recent_avg, deviation * 100.0
                    );
                }
            }
        }

        Ok(())
    }

    /// Generate recommendations based on detected anomalies
    async fn generate_recommendations(&self, anomaly_summary: &HashMap<String, Vec<AnomalyResult>>) -> Vec<String> {
        let mut recommendations = Vec::new();

        for (metric_name, anomalies) in anomaly_summary {
            if anomalies.is_empty() {
                continue;
            }

            let high_confidence_anomalies = anomalies.iter()
                .filter(|a| a.confidence > 0.8)
                .count();

            if high_confidence_anomalies > 0 {
                match metric_name.as_str() {
                    name if name.contains("cpu") => {
                        recommendations.push("Consider CPU optimization or scaling".to_string());
                    },
                    name if name.contains("memory") => {
                        recommendations.push("Review memory usage and optimize allocations".to_string());
                    },
                    name if name.contains("fee") => {
                        recommendations.push("Monitor fee market conditions and adjust strategies".to_string());
                    },
                    name if name.contains("block") => {
                        recommendations.push("Check block propagation and network connectivity".to_string());
                    },
                    _ => {
                        recommendations.push(format!("Investigate anomalies in {}", metric_name));
                    }
                }
            }
        }

        recommendations
    }

    /// Get current analytics configuration
    pub fn get_config(&self) -> &AnalyticsConfig {
        &self.config
    }

    /// Update analytics configuration
    pub fn update_config(&mut self, config: AnalyticsConfig) {
        self.config = config;
    }
}

/// Summary statistics for a metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricSummary {
    pub data_points: usize,
    pub mean: f64,
    pub min: f64,
    pub max: f64,
    pub trend: TrendDirection,
}

/// Trend direction for time series analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
    Volatile,
}

/// Comprehensive analytics report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsReport {
    pub time_range_hours: u32,
    pub generated_at: u64,
    pub metric_summary: HashMap<String, MetricSummary>,
    pub anomaly_summary: HashMap<String, Vec<AnomalyResult>>,
    pub recommendations: Vec<String>,
}

/// Calculate trend direction from a series of values
fn calculate_trend(values: &[f64]) -> TrendDirection {
    if values.len() < 3 {
        return TrendDirection::Stable;
    }

    let first_half_avg = values[..values.len()/2].iter().sum::<f64>() / (values.len()/2) as f64;
    let second_half_avg = values[values.len()/2..].iter().sum::<f64>() / (values.len() - values.len()/2) as f64;
    
    let change_percent = (second_half_avg - first_half_avg) / first_half_avg;
    
    // Calculate volatility
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let variance = values.iter()
        .map(|x| (x - mean).powi(2))
        .sum::<f64>() / values.len() as f64;
    let coefficient_of_variation = variance.sqrt() / mean;

    if coefficient_of_variation > 0.3 {
        TrendDirection::Volatile
    } else if change_percent > 0.1 {
        TrendDirection::Increasing
    } else if change_percent < -0.1 {
        TrendDirection::Decreasing
    } else {
        TrendDirection::Stable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_analytics_engine_creation() {
        let config = AnalyticsConfig::default();
        let engine = AnalyticsEngine::new(config).await;
        assert!(engine.is_ok());
    }

    #[tokio::test]
    async fn test_record_metric() {
        let config = AnalyticsConfig::default();
        let engine = AnalyticsEngine::new(config).await.unwrap();
        
        let result = engine.record_metric("test_metric", 42.0, None).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_trend_calculation() {
        let increasing = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let decreasing = vec![5.0, 4.0, 3.0, 2.0, 1.0];
        let stable = vec![3.0, 3.1, 2.9, 3.0, 3.1];
        let volatile = vec![1.0, 5.0, 2.0, 8.0, 3.0];

        assert!(matches!(calculate_trend(&increasing), TrendDirection::Increasing));
        assert!(matches!(calculate_trend(&decreasing), TrendDirection::Decreasing));
        assert!(matches!(calculate_trend(&stable), TrendDirection::Stable));
        assert!(matches!(calculate_trend(&volatile), TrendDirection::Volatile));
    }
}