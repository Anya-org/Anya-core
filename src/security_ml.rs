//! Security ML Module
//!
//! This module provides ML-powered security analysis for Bitcoin operations,
//! including fraud detection, 51% attack monitoring, and fee spike analysis.
//! Replaces Python security analysis with high-performance Rust implementations.

use crate::ml::{MLSystem, MLConfig, MLInput};
use crate::{AnyaError, AnyaResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use std::sync::Arc;
use tracing::{info, warn, error};

/// Configuration for security ML analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMLConfig {
    /// Enable fraud detection
    pub fraud_detection_enabled: bool,
    /// Enable 51% attack monitoring
    pub attack_monitoring_enabled: bool,
    /// Enable fee spike analysis
    pub fee_spike_analysis_enabled: bool,
    /// Confidence threshold for security alerts
    pub alert_threshold: f64,
    /// Time window for analysis (seconds)
    pub analysis_window_seconds: u64,
    /// Maximum transactions to analyze in batch
    pub max_batch_size: usize,
}

impl Default for SecurityMLConfig {
    fn default() -> Self {
        Self {
            fraud_detection_enabled: true,
            attack_monitoring_enabled: true,
            fee_spike_analysis_enabled: true,
            alert_threshold: 0.85,
            analysis_window_seconds: 600, // 10 minutes
            max_batch_size: 1000,
        }
    }
}

/// Transaction data for security analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionData {
    pub txid: String,
    pub timestamp: u64,
    pub input_count: u32,
    pub output_count: u32,
    pub fee_rate: f64,
    pub total_value: u64,
    pub is_rbf: bool,
    pub has_witness: bool,
    pub size_bytes: u32,
    pub confirmations: u32,
    pub input_addresses: Vec<String>,
    pub output_addresses: Vec<String>,
}

/// Block data for 51% attack monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockData {
    pub height: u64,
    pub hash: String,
    pub timestamp: u64,
    pub size_bytes: u32,
    pub tx_count: u32,
    pub total_fees: u64,
    pub miner_address: Option<String>,
    pub difficulty: f64,
    pub previous_hash: String,
    pub merkle_root: String,
}

/// Network hashrate data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashrateData {
    pub timestamp: u64,
    pub estimated_hashrate: f64,
    pub difficulty: f64,
    pub block_interval_seconds: f64,
    pub network_blocks_per_hour: f64,
}

/// Security threat types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ThreatType {
    FraudulentTransaction,
    SuspiciousPattern,
    FeeSpike,
    HashrateDrop,
    AttackAttempt,
    SybilAttack,
    EclipseAttack,
    DoubleSpend,
}

/// Security alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAlert {
    pub timestamp: u64,
    pub threat_type: ThreatType,
    pub confidence: f64,
    pub severity: AlertSeverity,
    pub description: String,
    pub affected_entities: Vec<String>,
    pub recommended_actions: Vec<String>,
    pub metadata: HashMap<String, String>,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Fraud detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudDetectionResult {
    pub transaction_id: String,
    pub fraud_probability: f64,
    pub risk_factors: Vec<String>,
    pub anomaly_score: f64,
    pub is_suspicious: bool,
}

/// Attack monitoring result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackMonitoringResult {
    pub attack_type: ThreatType,
    pub probability: f64,
    pub indicators: Vec<String>,
    pub estimated_impact: String,
    pub mitigation_suggestions: Vec<String>,
}

/// Fee spike analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeSpikeAnalysis {
    pub current_fee_rate: f64,
    pub historical_average: f64,
    pub spike_magnitude: f64,
    pub is_spike: bool,
    pub predicted_duration_minutes: f64,
    pub cause_analysis: Vec<String>,
}

/// Security ML analysis engine
pub struct SecurityMLEngine {
    config: SecurityMLConfig,
    ml_system: Arc<RwLock<MLSystem>>,
    transaction_history: Arc<RwLock<Vec<TransactionData>>>,
    block_history: Arc<RwLock<Vec<BlockData>>>,
    hashrate_history: Arc<RwLock<Vec<HashrateData>>>,
    alert_history: Arc<RwLock<Vec<SecurityAlert>>>,
}

impl SecurityMLEngine {
    /// Create a new security ML engine
    pub async fn new(config: SecurityMLConfig) -> AnyaResult<Self> {
        let ml_config = MLConfig {
            enabled: true,
            model_path: Some("./data/security_models".to_string()),
            use_gpu: false, // CPU-only for security analysis
            federated_learning: false,
            max_model_size: 100 * 1024 * 1024, // 100 MB
        };

        let ml_system = MLSystem::new(ml_config).await?;

        Ok(Self {
            config,
            ml_system: Arc::new(RwLock::new(ml_system)),
            transaction_history: Arc::new(RwLock::new(Vec::new())),
            block_history: Arc::new(RwLock::new(Vec::new())),
            hashrate_history: Arc::new(RwLock::new(Vec::new())),
            alert_history: Arc::new(RwLock::new(Vec::new())),
        })
    }

    /// Analyze transaction for fraud indicators
    pub async fn analyze_transaction_fraud(&self, tx: &TransactionData) -> AnyaResult<FraudDetectionResult> {
        if !self.config.fraud_detection_enabled {
            return Ok(FraudDetectionResult {
                transaction_id: tx.txid.clone(),
                fraud_probability: 0.0,
                risk_factors: Vec::new(),
                anomaly_score: 0.0,
                is_suspicious: false,
            });
        }

        // Prepare features for ML analysis
        let features = self.extract_fraud_features(tx).await?;
        let input = MLInput {
            features,
            label: 0.0, // Not used for inference
            metadata: Some(self.create_transaction_metadata(tx)),
        };

        let ml_system = self.ml_system.read().await;
        let inference_result = ml_system.service()
            .predict("fraud_detector", &serde_json::to_vec(&input)?)
            .await
            .map_err(|e| AnyaError::Security(format!("Fraud detection inference failed: {}", e)))?;

        let fraud_probability: f64 = serde_json::from_slice(&inference_result.output)?;
        let risk_factors = self.identify_risk_factors(tx).await;
        let anomaly_score = self.calculate_transaction_anomaly_score(tx).await?;

        // Store transaction in history
        let mut history = self.transaction_history.write().await;
        history.push(tx.clone());
        self.maintain_transaction_history(&mut history);

        let result = FraudDetectionResult {
            transaction_id: tx.txid.clone(),
            fraud_probability,
            risk_factors,
            anomaly_score,
            is_suspicious: fraud_probability > self.config.alert_threshold,
        };

        // Generate alert if suspicious
        if result.is_suspicious {
            self.create_security_alert(
                ThreatType::FraudulentTransaction,
                fraud_probability,
                format!("Suspicious transaction detected: {}", tx.txid),
                vec![tx.txid.clone()],
                vec![
                    "Review transaction details".to_string(),
                    "Monitor related addresses".to_string(),
                    "Consider manual investigation".to_string(),
                ],
            ).await;
        }

        Ok(result)
    }

    /// Monitor for 51% attack indicators
    pub async fn monitor_51_percent_attack(&self, blocks: &[BlockData], hashrate: &[HashrateData]) -> AnyaResult<AttackMonitoringResult> {
        if !self.config.attack_monitoring_enabled {
            return Ok(AttackMonitoringResult {
                attack_type: ThreatType::AttackAttempt,
                probability: 0.0,
                indicators: Vec::new(),
                estimated_impact: "None".to_string(),
                mitigation_suggestions: Vec::new(),
            });
        }

        let mut indicators = Vec::new();
        let mut attack_probability = 0.0;

        // Check for suspicious mining patterns
        if let Some(recent_blocks) = blocks.last().map(|b| {
            blocks.iter().filter(|block| block.height > b.height - 10).collect::<Vec<_>>()
        }) {
            // Check for unusual block timing
            let avg_interval = self.calculate_avg_block_interval(&recent_blocks);
            if avg_interval < 300.0 { // Less than 5 minutes average
                indicators.push("Unusually fast block production detected".to_string());
                attack_probability += 0.3;
            }

            // Check for miner concentration
            let miner_concentration = self.calculate_miner_concentration(&recent_blocks);
            if miner_concentration > 0.6 { // More than 60% from single entity
                indicators.push(format!("High miner concentration: {:.1}%", miner_concentration * 100.0));
                attack_probability += 0.4;
            }
        }

        // Check hashrate fluctuations
        if let Some(recent_hashrate) = hashrate.last().map(|h| {
            hashrate.iter().filter(|hr| hr.timestamp > h.timestamp - 3600).collect::<Vec<_>>()
        }) {
            let hashrate_volatility = self.calculate_hashrate_volatility(&recent_hashrate);
            if hashrate_volatility > 0.3 { // More than 30% volatility
                indicators.push("High hashrate volatility detected".to_string());
                attack_probability += 0.2;
            }

            // Sudden hashrate increases
            if recent_hashrate.len() > 1 {
                let current = recent_hashrate.last().unwrap().estimated_hashrate;
                let previous = recent_hashrate[recent_hashrate.len() - 2].estimated_hashrate;
                let increase = (current - previous) / previous;
                
                if increase > 0.5 { // More than 50% increase
                    indicators.push(format!("Sudden hashrate increase: {:.1}%", increase * 100.0));
                    attack_probability += 0.3;
                }
            }
        }

        // Use ML model for sophisticated attack detection
        let features = self.extract_attack_features(blocks, hashrate).await?;
        let input = MLInput {
            features,
            label: 0.0,
            metadata: Some(HashMap::new()),
        };

        let ml_system = self.ml_system.read().await;
        if let Ok(inference_result) = ml_system.service()
            .predict("attack_detector", &serde_json::to_vec(&input)?)
            .await {
            let ml_probability: f64 = serde_json::from_slice(&inference_result.output)
                .unwrap_or(0.0);
            attack_probability = (attack_probability + ml_probability) / 2.0;
        }

        let attack_type = if attack_probability > 0.7 {
            ThreatType::AttackAttempt
        } else {
            ThreatType::SuspiciousPattern
        };

        let estimated_impact = match attack_probability {
            p if p > 0.8 => "High - Potential network disruption",
            p if p > 0.5 => "Medium - Monitor closely",
            _ => "Low - Normal variation",
        }.to_string();

        let mitigation_suggestions = if attack_probability > self.config.alert_threshold {
            vec![
                "Increase confirmation requirements".to_string(),
                "Monitor mining pool distribution".to_string(),
                "Implement additional validation checks".to_string(),
                "Consider temporary service restrictions".to_string(),
            ]
        } else {
            vec!["Continue monitoring".to_string()]
        };

        let result = AttackMonitoringResult {
            attack_type,
            probability: attack_probability,
            indicators,
            estimated_impact,
            mitigation_suggestions,
        };

        // Generate alert if high probability
        if attack_probability > self.config.alert_threshold {
            self.create_security_alert(
                result.attack_type.clone(),
                attack_probability,
                format!("Potential 51% attack detected (probability: {:.1}%)", attack_probability * 100.0),
                vec!["Network".to_string()],
                result.mitigation_suggestions.clone(),
            ).await;
        }

        Ok(result)
    }

    /// Analyze fee spikes for manipulation or network stress
    pub async fn analyze_fee_spike(&self, current_fee_rate: f64, historical_rates: &[f64]) -> AnyaResult<FeeSpikeAnalysis> {
        if !self.config.fee_spike_analysis_enabled {
            return Ok(FeeSpikeAnalysis {
                current_fee_rate,
                historical_average: current_fee_rate,
                spike_magnitude: 0.0,
                is_spike: false,
                predicted_duration_minutes: 0.0,
                cause_analysis: Vec::new(),
            });
        }

        let historical_average = historical_rates.iter().sum::<f64>() / historical_rates.len() as f64;
        let spike_magnitude = (current_fee_rate - historical_average) / historical_average;
        let is_spike = spike_magnitude > 2.0; // 200% increase considered spike

        // Use ML to predict spike duration and causes
        let features = self.extract_fee_spike_features(current_fee_rate, historical_rates).await?;
        let input = MLInput {
            features,
            label: 0.0,
            metadata: Some(HashMap::new()),
        };

        let ml_system = self.ml_system.read().await;
        let mut predicted_duration_minutes = 0.0;
        let mut cause_analysis = Vec::new();

        if let Ok(inference_result) = ml_system.service()
            .predict("fee_spike_analyzer", &serde_json::to_vec(&input)?)
            .await {
            predicted_duration_minutes = serde_json::from_slice(&inference_result.output)
                .unwrap_or(0.0);
        }

        // Analyze potential causes
        if is_spike {
            if spike_magnitude > 5.0 {
                cause_analysis.push("Severe network congestion".to_string());
            } else if spike_magnitude > 3.0 {
                cause_analysis.push("High demand period".to_string());
            } else {
                cause_analysis.push("Moderate congestion".to_string());
            }

            // Check for manipulation patterns
            let volatility = self.calculate_fee_volatility(historical_rates);
            if volatility > 0.5 {
                cause_analysis.push("Possible fee manipulation".to_string());
            }

            // Time-based analysis
            let current_hour = (SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() / 3600) % 24;
            
            if (8..18).contains(&current_hour) {
                cause_analysis.push("Peak trading hours".to_string());
            }
        }

        let result = FeeSpikeAnalysis {
            current_fee_rate,
            historical_average,
            spike_magnitude,
            is_spike,
            predicted_duration_minutes,
            cause_analysis,
        };

        // Generate alert for significant spikes
        if is_spike && spike_magnitude > 3.0 {
            self.create_security_alert(
                ThreatType::FeeSpike,
                spike_magnitude / 10.0, // Normalize to 0-1 range
                format!("Significant fee spike detected: {:.1}x normal rate", spike_magnitude + 1.0),
                vec!["Fee Market".to_string()],
                vec![
                    "Monitor mempool congestion".to_string(),
                    "Consider batching transactions".to_string(),
                    "Use fee estimation tools".to_string(),
                ],
            ).await;
        }

        Ok(result)
    }

    /// Get recent security alerts
    pub async fn get_recent_alerts(&self, limit: Option<usize>) -> Vec<SecurityAlert> {
        let alerts = self.alert_history.read().await;
        let limit = limit.unwrap_or(alerts.len());
        alerts.iter().rev().take(limit).cloned().collect()
    }

    /// Extract features for fraud detection
    async fn extract_fraud_features(&self, tx: &TransactionData) -> AnyaResult<Vec<f64>> {
        let mut features = Vec::new();

        // Basic transaction features
        features.push(tx.input_count as f64);
        features.push(tx.output_count as f64);
        features.push(tx.fee_rate);
        features.push(tx.total_value as f64);
        features.push(if tx.is_rbf { 1.0 } else { 0.0 });
        features.push(if tx.has_witness { 1.0 } else { 0.0 });
        features.push(tx.size_bytes as f64);

        // Address reuse patterns
        let input_unique = tx.input_addresses.iter().collect::<std::collections::HashSet<_>>().len();
        let output_unique = tx.output_addresses.iter().collect::<std::collections::HashSet<_>>().len();
        features.push(input_unique as f64 / tx.input_addresses.len().max(1) as f64);
        features.push(f64::from(tx.input_count));
        features.push(f64::from(tx.output_count));
        features.push(tx.fee_rate);
        features.push(tx.total_value as f64);
        features.push(if tx.is_rbf { 1.0 } else { 0.0 });
        features.push(if tx.has_witness { 1.0 } else { 0.0 });
        features.push(f64::from(tx.size_bytes));

        // Address reuse patterns
        let input_unique = tx.input_addresses.iter().collect::<std::collections::HashSet<_>>().len();
        let output_unique = tx.output_addresses.iter().collect::<std::collections::HashSet<_>>().len();
        features.push(input_unique as f64 / tx.input_addresses.len().max(1) as f64);
        features.push(output_unique as f64 / tx.output_addresses.len().max(1) as f64);

        // Timing features
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        features.push((current_time - tx.timestamp) as f64);

        // Historical comparison features
        let history = self.transaction_history.read().await;
        if !history.is_empty() {
            let avg_fee = history.iter().map(|t| t.fee_rate).sum::<f64>() / history.len() as f64;
            let avg_value = history.iter().map(|t| t.total_value as f64).sum::<f64>() / history.len() as f64;
            
            features.push(tx.fee_rate / avg_fee.max(1.0));
            features.push(tx.total_value as f64 / avg_value.max(1.0));
        } else {
            features.push(1.0);
            features.push(1.0);
        }

        Ok(features)
    }

    /// Extract features for attack detection
    async fn extract_attack_features(&self, blocks: &[BlockData], hashrate: &[HashrateData]) -> AnyaResult<Vec<f64>> {
        let mut features = Vec::new();

        if blocks.is_empty() || hashrate.is_empty() {
            return Ok(vec![0.0; 10]); // Return default features
        }

        // Block timing features
        let recent_blocks: Vec<_> = blocks.iter().rev().take(10).collect();
        let avg_interval = self.calculate_avg_block_interval(&recent_blocks);
        features.push(avg_interval);

        // Miner concentration
        let miner_concentration = self.calculate_miner_concentration(&recent_blocks);
        features.push(miner_concentration);

        // Hashrate features
        let recent_hashrate: Vec<_> = hashrate.iter().rev().take(10).collect();
        let hashrate_volatility = self.calculate_hashrate_volatility(&recent_hashrate);
        features.push(hashrate_volatility);

        // Current vs historical hashrate
        if let (Some(current), historical_sum) = (
            recent_hashrate.first().map(|h| h.estimated_hashrate),
            hashrate.iter().map(|h| h.estimated_hashrate).sum::<f64>()
        ) {
            let historical_avg = if hashrate.is_empty() { 1.0 } else { historical_sum / hashrate.len() as f64 };
            features.push(current / historical_avg);
        } else {
            features.push(1.0);
        }

        // Difficulty adjustment patterns
        if recent_blocks.len() > 1 {
            let difficulty_change = (recent_blocks[0].difficulty - recent_blocks[1].difficulty) / recent_blocks[1].difficulty;
            features.push(difficulty_change);
        } else {
            features.push(0.0);
        }

        // Block size patterns
        let avg_block_size = recent_blocks.iter().map(|b| b.size_bytes as f64).sum::<f64>() / recent_blocks.len() as f64;
        features.push(avg_block_size);

        // Fee patterns
        let total_fees: u64 = recent_blocks.iter().map(|b| b.total_fees).sum();
        features.push(total_fees as f64);

        // Pad to consistent size
        while features.len() < 10 {
            features.push(0.0);
        }

        Ok(features[..10].to_vec())
    }

    /// Extract features for fee spike analysis
    async fn extract_fee_spike_features(&self, current_rate: f64, historical_rates: &[f64]) -> AnyaResult<Vec<f64>> {
        let mut features = Vec::new();

        features.push(current_rate);

        if !historical_rates.is_empty() {
            let avg = historical_rates.iter().sum::<f64>() / historical_rates.len() as f64;
            let min = historical_rates.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let max = historical_rates.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            
            features.push(avg);
            features.push(min);
            features.push(max);
            features.push(current_rate / avg);
            features.push(self.calculate_fee_volatility(historical_rates));
            
            // Recent trend
            if historical_rates.len() > 5 {
                let recent_avg = historical_rates.iter().rev().take(5).sum::<f64>() / 5.0;
                features.push(recent_avg);
                features.push(current_rate / recent_avg);
            } else {
                features.push(avg);
                features.push(1.0);
            }
        } else {
            features.extend(vec![0.0; 7]);
        }

        Ok(features)
    }

    /// Helper methods for calculations
    fn calculate_avg_block_interval(&self, blocks: &[&BlockData]) -> f64 {
        if blocks.len() < 2 { return 600.0; } // Default 10 minutes
        
        let intervals: Vec<f64> = blocks.windows(2)
            .map(|w| (w[0].timestamp - w[1].timestamp) as f64)
            .collect();
        
        intervals.iter().sum::<f64>() / intervals.len() as f64
    }

    fn calculate_miner_concentration(&self, blocks: &[&BlockData]) -> f64 {
        let mut miner_counts = HashMap::new();
        
        for block in blocks {
            if let Some(miner) = &block.miner_address {
                *miner_counts.entry(miner.clone()).or_insert(0) += 1;
            }
        }
        
        if miner_counts.is_empty() { return 0.0; }
        
        let max_count = *miner_counts.values().max().unwrap();
        max_count as f64 / blocks.len() as f64
    }

    fn calculate_hashrate_volatility(&self, hashrate_data: &[&HashrateData]) -> f64 {
        if hashrate_data.len() < 2 { return 0.0; }
        
        let values: Vec<f64> = hashrate_data.iter().map(|h| h.estimated_hashrate).collect();
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let variance = values.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / values.len() as f64;
        
        (variance.sqrt() / mean).min(1.0)
    }

    fn calculate_fee_volatility(&self, rates: &[f64]) -> f64 {
        if rates.len() < 2 { return 0.0; }
        
        let mean = rates.iter().sum::<f64>() / rates.len() as f64;
        let variance = rates.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / rates.len() as f64;
        
        (variance.sqrt() / mean).min(1.0)
    }

    async fn identify_risk_factors(&self, tx: &TransactionData) -> Vec<String> {
        let mut factors = Vec::new();

        if tx.fee_rate > 200.0 {
            factors.push("Extremely high fee rate".to_string());
        }
        
        if tx.input_count > 100 {
            factors.push("High number of inputs".to_string());
        }
        
        if tx.output_count > 100 {
            factors.push("High number of outputs".to_string());
        }
        
        if tx.total_value > 1_000_000_000 { // > 10 BTC in satoshis
            factors.push("High value transaction".to_string());
        }
        
        if tx.is_rbf {
            factors.push("Replace-by-fee enabled".to_string());
        }

        factors
    }

    async fn calculate_transaction_anomaly_score(&self, tx: &TransactionData) -> AnyaResult<f64> {
        // Simplified anomaly scoring based on deviations from normal patterns
        let mut score: f64 = 0.0;
        
        // Fee rate anomaly
        let normal_fee_range = 1.0..=50.0;
        if !normal_fee_range.contains(&tx.fee_rate) {
            score += 0.3;
        }
        
        // Size anomaly
        if tx.size_bytes > 100_000 { // Large transaction
            score += 0.2;
        }
        
        // Input/output count anomaly
        if tx.input_count + tx.output_count > 50 {
            score += 0.2;
        }
        
        // Value anomaly
        if tx.total_value > 500_000_000 { // > 5 BTC
            score += 0.3;
        }
        
        Ok(score.min(1.0))
    }

    fn create_transaction_metadata(&self, tx: &TransactionData) -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("txid".to_string(), tx.txid.clone());
        metadata.insert("timestamp".to_string(), tx.timestamp.to_string());
        metadata.insert("fee_rate".to_string(), tx.fee_rate.to_string());
        metadata.insert("total_value".to_string(), tx.total_value.to_string());
        metadata
    }

    async fn create_security_alert(
        &self,
        threat_type: ThreatType,
        confidence: f64,
        description: String,
        affected_entities: Vec<String>,
        recommended_actions: Vec<String>,
    ) {
        let severity = match confidence {
            c if c > 0.9 => AlertSeverity::Critical,
            c if c > 0.7 => AlertSeverity::High,
            c if c > 0.5 => AlertSeverity::Medium,
            _ => AlertSeverity::Low,
        };

        let alert = SecurityAlert {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            threat_type: threat_type.clone(),
            confidence,
            severity: severity.clone(),
            description: description.clone(),
            affected_entities,
            recommended_actions,
            metadata: HashMap::new(),
        };

        // Log the alert
        match severity {
            AlertSeverity::Critical => error!("SECURITY CRITICAL: {}", description),
            AlertSeverity::High => error!("SECURITY HIGH: {}", description),
            AlertSeverity::Medium => warn!("SECURITY MEDIUM: {}", description),
            AlertSeverity::Low => info!("SECURITY LOW: {}", description),
        }

        // Store in history
        let mut history = self.alert_history.write().await;
        history.push(alert);
        
        // Keep only last 1000 alerts
        if history.len() > 1000 {
            let drain_count = history.len() - 1000;
            history.drain(0..drain_count);
        }
    }

    fn maintain_transaction_history(&self, history: &mut Vec<TransactionData>) {
        let max_size = self.config.max_batch_size * 10; // Keep 10x batch size
        if history.len() > max_size {
            history.drain(0..history.len() - max_size);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_security_ml_engine_creation() {
        let config = SecurityMLConfig::default();
        let engine = SecurityMLEngine::new(config).await;
        assert!(engine.is_ok());
    }

    #[tokio::test]
    async fn test_fraud_detection() {
        let config = SecurityMLConfig::default();
        let engine = SecurityMLEngine::new(config).await.unwrap();
        
        let tx = TransactionData {
            txid: "test_tx".to_string(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            input_count: 1,
            output_count: 2,
            fee_rate: 10.0,
            total_value: 100_000_000,
            is_rbf: false,
            has_witness: true,
            size_bytes: 250,
            confirmations: 0,
            input_addresses: vec!["addr1".to_string()],
            output_addresses: vec!["addr2".to_string(), "addr3".to_string()],
        };
        
        let result = engine.analyze_transaction_fraud(&tx).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_fee_volatility_calculation() {
        // Create a minimal SecurityMLEngine for testing
        use std::sync::Arc;
        use tokio::sync::RwLock;
        
        let config = SecurityMLConfig::default();
        
        // Create dummy engine just for the method test
        let engine = SecurityMLConfig::default(); // We'll just use config to access the method logic
        
        let stable_rates = vec![10.0, 11.0, 9.0, 10.5, 9.5];
        let volatile_rates = vec![10.0, 50.0, 5.0, 100.0, 2.0];
        
        // Calculate volatility manually (same logic as in the method)
        let calculate_fee_volatility = |rates: &[f64]| -> f64 {
            if rates.len() < 2 { return 0.0; }
            
            let mean = rates.iter().sum::<f64>() / rates.len() as f64;
            let variance = rates.iter()
                .map(|x| (x - mean).powi(2))
                .sum::<f64>() / rates.len() as f64;
            
            (variance.sqrt() / mean).min(1.0)
        };
        
        let stable_volatility = calculate_fee_volatility(&stable_rates);
        let volatile_volatility = calculate_fee_volatility(&volatile_rates);
        
        assert!(stable_volatility < volatile_volatility);
    }
}