use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use lazy_static::lazy_static;
use log::{error, warn, info};
use serde::{Deserialize, Serialize};

use crate::monitoring::blockchain_metrics;

lazy_static! {
    static ref ALERTS: Mutex<BlockchainAlerts> = Mutex::new(BlockchainAlerts::new());
}

/// Alert severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Alert status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertStatus {
    Active,
    Acknowledged,
    Resolved,
}

/// Blockchain metric alert definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// Alert ID
    id: String,
    
    /// Alert name
    name: String,
    
    /// Alert description
    description: String,
    
    /// Severity level
    severity: AlertSeverity,
    
    /// Current status
    status: AlertStatus,
    
    /// When the alert was triggered
    triggered_at: u64,
    
    /// When the alert was resolved (if applicable)
    resolved_at: Option<u64>,
    
    /// Metric name that triggered the alert
    metric_name: String,
    
    /// Threshold value that triggered the alert
    threshold: f64,
    
    /// Current value of the metric
    current_value: f64,
    
    /// Comparison operator (e.g., ">", "<", "==", ">=", "<=")
    comparison_operator: String,
}

/// Blockchain alerts manager
#[derive(Debug)]
pub struct BlockchainAlerts {
    /// Active alerts
    active_alerts: HashMap<String, Alert>,
    
    /// Alert definitions (name -> threshold config)
    alert_definitions: HashMap<String, AlertDefinition>,
    
    /// Last check time
    last_check: Instant,
    
    /// Alert history (last 100 alerts)
    alert_history: Vec<Alert>,
}

/// Alert threshold definition
#[derive(Debug, Clone)]
struct AlertDefinition {
    /// Alert name
    name: String,
    
    /// Alert description template
    description_template: String,
    
    /// Metric name to monitor
    metric_name: String,
    
    /// Threshold value
    threshold: f64,
    
    /// Comparison operator
    comparison_operator: String,
    
    /// Default severity level
    severity: AlertSeverity,
    
    /// Duration the threshold must be exceeded before an alert is triggered (in seconds)
    duration_threshold_seconds: u64,
    
    /// Consecutive readings that must exceed threshold
    consecutive_readings: u32,
    
    /// Current consecutive readings count
    current_consecutive: u32,
    
    /// First exceeded at timestamp
    first_exceeded_at: Option<Instant>,
}

impl BlockchainAlerts {
    /// Create new blockchain alerts manager
    pub fn new() -> Self {
        let mut alerts = Self {
            active_alerts: HashMap::new(),
            alert_definitions: HashMap::new(),
            last_check: Instant::now(),
            alert_history: Vec::new(),
        };
        
        // Define default alert thresholds
        alerts.define_default_alerts();
        
        alerts
    }
    
    /// Define default alert thresholds
    fn define_default_alerts(&mut self) {
        // SegWit adoption alerts
        self.add_alert_definition(
            "segwit_adoption_low",
            "SegWit Adoption Below Threshold",
            "SegWit adoption percentage is below {threshold}% (currently {value}%)",
            "segwit_percentage",
            80.0,
            "<",
            AlertSeverity::Warning,
            300, // 5 minutes
            3,
        );
        
        // Taproot adoption alerts
        self.add_alert_definition(
            "taproot_adoption_low",
            "Taproot Adoption Below Threshold",
            "Taproot adoption percentage is below {threshold}% (currently {value}%)",
            "taproot_percentage",
            5.0,
            "<",
            AlertSeverity::Warning,
            300, // 5 minutes
            3,
        );
        
        // Error rate alerts
        self.add_alert_definition(
            "high_error_rate",
            "High Error Rate",
            "Error rate is above {threshold}% (currently {value}%)",
            "error_rates.connection_failure", 
            5.0,
            ">",
            AlertSeverity::Error,
            60, // 1 minute
            2,
        );
        
        // Block propagation alerts
        self.add_alert_definition(
            "slow_block_propagation",
            "Slow Block Propagation",
            "Block propagation time is above {threshold}ms (currently {value}ms)",
            "block_propagation_ms",
            1000.0, // 1 second
            ">",
            AlertSeverity::Warning,
            0, // Immediate
            1,
        );
        
        // Mempool size alerts
        self.add_alert_definition(
            "mempool_size_high",
            "High Mempool Size",
            "Mempool size is above {threshold}MB (currently {value}MB)",
            "mempool_size_bytes",
            50.0 * 1024.0 * 1024.0, // 50MB
            ">", 
            AlertSeverity::Warning,
            300, // 5 minutes
            3,
        );
        
        // Fee rate alerts
        self.add_alert_definition(
            "high_fee_rate",
            "High Fee Rate",
            "Average fee rate is above {threshold} sats/vB (currently {value} sats/vB)",
            "avg_fee_rate_sats_per_vb",
            100.0,
            ">",
            AlertSeverity::Warning,
            300, // 5 minutes
            2,
        );
    }
    
    /// Add a new alert definition
    fn add_alert_definition(
        &mut self,
        id: &str,
        name: &str,
        description_template: &str,
        metric_name: &str,
        threshold: f64,
        comparison_operator: &str,
        severity: AlertSeverity,
        duration_threshold_seconds: u64,
        consecutive_readings: u32,
    ) {
        self.alert_definitions.insert(
            id.to_string(),
            AlertDefinition {
                name: name.to_string(),
                description_template: description_template.to_string(),
                metric_name: metric_name.to_string(),
                threshold,
                comparison_operator: comparison_operator.to_string(),
                severity,
                duration_threshold_seconds,
                consecutive_readings,
                current_consecutive: 0,
                first_exceeded_at: None,
            },
        );
    }
    
    /// Check metrics against all defined alert thresholds
    pub fn check_alerts(&mut self) {
        self.last_check = Instant::now();

        // Get current metrics
        let metrics_json = blockchain_metrics::get_metrics_json();

        let mut checks = Vec::new();

        for (alert_id, definition) in &self.alert_definitions {
            let metric_value = self.get_metric_value(&metrics_json, &definition.metric_name);

            match metric_value {
                Some(value) => {
                    let threshold_exceeded = self.is_threshold_exceeded(
                        value,
                        definition.threshold,
                        &definition.comparison_operator,
                    );

                    checks.push((alert_id.clone(), value, threshold_exceeded));
                },
                None => {
                    checks.push((alert_id.clone(), 0.0, false));
                }
            }
        }

        for (alert_id, value, threshold_exceeded) in checks {
            let definition = match self.alert_definitions.get_mut(&alert_id) {
                Some(def) => def,
                None => continue,
            };

            if threshold_exceeded {
                definition.current_consecutive += 1;

                if definition.first_exceeded_at.is_none() {
                    definition.first_exceeded_at = Some(Instant::now());
                }

                let duration_exceeded = definition
                    .first_exceeded_at
                    .map(|time| time.elapsed().as_secs() >= definition.duration_threshold_seconds)
                    .unwrap_or(false);

                if duration_exceeded && definition.current_consecutive >= definition.consecutive_readings {
                    let description = definition.description_template
                        .replace("{threshold}", &definition.threshold.to_string())
                        .replace("{value}", &value.to_string());

                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    let alert = Alert {
                        id: alert_id.clone(),
                        name: definition.name.clone(),
                        description: description.clone(),
                        severity: definition.severity,
                        status: AlertStatus::Active,
                        triggered_at: now,
                        resolved_at: None,
                        metric_name: definition.metric_name.clone(),
                        threshold: definition.threshold,
                        current_value: value,
                        comparison_operator: definition.comparison_operator.clone(),
                    };

                    self.active_alerts.insert(alert_id.clone(), alert.clone());
                    self.alert_history.push(alert);

                    if self.alert_history.len() > 100 {
                        self.alert_history.remove(0);
                    }

                    match definition.severity {
                        AlertSeverity::Info => info!("ALERT: {}", description),
                        AlertSeverity::Warning => warn!("ALERT: {}", description),
                        AlertSeverity::Error => error!("ALERT: {}", description),
                        AlertSeverity::Critical => error!("CRITICAL ALERT: {}", description),
                    }
                }
            } else {
                definition.current_consecutive = 0;
                definition.first_exceeded_at = None;

                if let Some(mut alert) = self.active_alerts.remove(&alert_id) {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    alert.status = AlertStatus::Resolved;
                    alert.resolved_at = Some(now);
                    
                    let description = alert.description.clone();
                    self.alert_history.push(alert);

                    if self.alert_history.len() > 100 {
                        self.alert_history.remove(0);
                    }

                    info!("RESOLVED: {}", description);
                }
            }
        }
    }
    
    /// Get a metric value from the metrics JSON structure
    fn get_metric_value(&self, metrics: &serde_json::Value, path: &str) -> Option<f64> {
        let parts: Vec<&str> = path.split('.').collect();
        
        let mut current = metrics;
        for part in &parts[0..parts.len() - 1] {
            current = &current[*part];
            if current.is_null() {
                return None;
            }
        }
        
        let last_part = parts.last()?;
        let value = &current[*last_part];
        
        if value.is_f64() {
            Some(value.as_f64().unwrap())
        } else if value.is_i64() {
            Some(value.as_i64().unwrap() as f64)
        } else if value.is_u64() {
            Some(value.as_u64().unwrap() as f64)
        } else {
            None
        }
    }
    
    /// Check if a threshold is exceeded based on the comparison operator
    fn is_threshold_exceeded(&self, value: f64, threshold: f64, operator: &str) -> bool {
        match operator {
            ">" => value > threshold,
            "<" => value < threshold,
            ">=" => value >= threshold,
            "<=" => value <= threshold,
            "==" => (value - threshold).abs() < f64::EPSILON,
            "!=" => (value - threshold).abs() > f64::EPSILON,
            _ => false,
        }
    }
    
    /// Acknowledge an alert
    pub fn acknowledge_alert(&mut self, alert_id: &str) {
        if let Some(alert) = self.active_alerts.get_mut(alert_id) {
            alert.status = AlertStatus::Acknowledged;
            info!("Alert '{}' acknowledged", alert_id);
        }
    }
    
    /// Get all active alerts
    pub fn get_active_alerts(&self) -> Vec<&Alert> {
        self.active_alerts.values().collect()
    }
    
    /// Get alert history
    pub fn get_alert_history(&self) -> &Vec<Alert> {
        &self.alert_history
    }
}

/// Check all alert thresholds against current metrics
pub fn check_alerts() {
    let mut alerts: std::sync::MutexGuard<BlockchainAlerts> = ALERTS.lock().unwrap();
    alerts.check_alerts();
}

/// Acknowledge an alert
pub fn acknowledge_alert(alert_id: &str) {
    let mut alerts: std::sync::MutexGuard<BlockchainAlerts> = ALERTS.lock().unwrap();
    let _ = alerts.acknowledge_alert(alert_id);
}

/// Get all active alerts
pub fn get_active_alerts() -> Vec<Alert> {
    let alerts: std::sync::MutexGuard<BlockchainAlerts> = ALERTS.lock().unwrap();
    alerts.get_active_alerts().iter().map(|a| (*a).clone()).collect()
}

/// Get alert history
pub fn get_alert_history() -> Vec<Alert> {
    let alerts: std::sync::MutexGuard<BlockchainAlerts> = ALERTS.lock().unwrap();
    alerts.get_alert_history().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain_alerts() {
        // This is a minimal test as we can't easily modify the metrics
        let alerts = get_active_alerts();
        assert!(alerts.is_empty(), "Should have no active alerts initially");
    }
}


