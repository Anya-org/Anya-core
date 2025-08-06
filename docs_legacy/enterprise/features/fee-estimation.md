# Fee Estimation

Advanced fee estimation algorithms for optimal transaction cost management.

## Overview

The fee estimation system provides intelligent algorithms to determine optimal transaction fees based on network conditions, urgency requirements, and cost optimization strategies.

## Fee Estimation Algorithms

### Dynamic Fee Calculation

```rust
use std::collections::VecDeque;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeEstimate {
    pub sat_per_byte: f64,
    pub total_fee: u64,
    pub confirmation_target: u32,
    pub confidence_level: f64,
}

pub struct FeeEstimator {
    mempool_analyzer: MempoolAnalyzer,
    historical_data: VecDeque<FeeRecord>,
    network_monitor: NetworkMonitor,
}

impl FeeEstimator {
    pub fn estimate_fee(&self, target_confirmations: u32, tx_size: usize) -> FeeEstimate {
        let base_fee = self.calculate_base_fee(target_confirmations);
        let network_adjustment = self.get_network_adjustment();
        let congestion_multiplier = self.get_congestion_multiplier();
        
        let adjusted_fee = base_fee * network_adjustment * congestion_multiplier;
        
        FeeEstimate {
            sat_per_byte: adjusted_fee,
            total_fee: (adjusted_fee * tx_size as f64) as u64,
            confirmation_target: target_confirmations,
            confidence_level: self.calculate_confidence(adjusted_fee),
        }
    }
    
    fn calculate_base_fee(&self, target_blocks: u32) -> f64 {
        // Analyze recent blocks for fee distribution
        let recent_fees = self.get_recent_confirmed_fees(target_blocks * 2);
        
        match target_blocks {
            1 => recent_fees.percentile(90.0),  // High priority
            3 => recent_fees.percentile(75.0),  // Medium priority
            6 => recent_fees.percentile(50.0),  // Standard priority
            _ => recent_fees.percentile(25.0),   // Low priority
        }
    }
    
    fn get_network_adjustment(&self) -> f64 {
        let current_difficulty = self.network_monitor.get_difficulty();
        let hash_rate = self.network_monitor.get_hash_rate();
        let block_time = self.network_monitor.get_avg_block_time();
        
        // Adjust based on network health
        if block_time > 600.0 {  // Slower than 10 minutes
            1.2  // Increase fee
        } else if block_time < 480.0 {  // Faster than 8 minutes
            0.9  // Decrease fee
        } else {
            1.0  // No adjustment
        }
    }
}
```

### Machine Learning Fee Prediction

```python
import numpy as np
from sklearn.ensemble import RandomForestRegressor
from sklearn.preprocessing import StandardScaler

class MLFeePredictor:
    def __init__(self):
        self.model = RandomForestRegressor(
            n_estimators=100,
            max_depth=10,
            random_state=42
        )
        self.scaler = StandardScaler()
        self.feature_names = [
            'mempool_size',
            'avg_block_time',
            'difficulty',
            'hash_rate',
            'tx_count_1h',
            'tx_count_24h',
            'weekend_indicator',
            'hour_of_day',
            'day_of_week'
        ]
    
    def extract_features(self, network_state):
        """Extract features for fee prediction"""
        features = np.array([
            network_state['mempool_size'],
            network_state['avg_block_time'],
            network_state['difficulty'],
            network_state['hash_rate'],
            network_state['tx_count_1h'],
            network_state['tx_count_24h'],
            1 if network_state['is_weekend'] else 0,
            network_state['hour'],
            network_state['day_of_week']
        ]).reshape(1, -1)
        
        return self.scaler.transform(features)
    
    def predict_fee(self, network_state, target_confirmations):
        """Predict optimal fee for target confirmation time"""
        features = self.extract_features(network_state)
        base_prediction = self.model.predict(features)[0]
        
        # Adjust based on target confirmations
        urgency_multiplier = {
            1: 1.5,   # Next block
            2: 1.2,   # Within 2 blocks
            3: 1.0,   # Within 3 blocks
            6: 0.8,   # Within 6 blocks
            12: 0.6   # Within 12 blocks
        }.get(target_confirmations, 0.5)
        
        return max(1.0, base_prediction * urgency_multiplier)
```

## Real-time Fee Monitoring

### Mempool Analysis

```rust
#[derive(Debug)]
pub struct MempoolAnalyzer {
    tx_pool: HashMap<Txid, MempoolTransaction>,
    fee_histogram: FeeHistogram,
}

impl MempoolAnalyzer {
    pub fn analyze_mempool(&mut self) -> MempoolAnalysis {
        self.update_tx_pool();
        
        let size_buckets = self.create_size_buckets();
        let fee_distribution = self.calculate_fee_distribution();
        let congestion_level = self.assess_congestion();
        
        MempoolAnalysis {
            total_transactions: self.tx_pool.len(),
            total_size: self.calculate_total_size(),
            fee_distribution,
            size_buckets,
            congestion_level,
            estimated_clear_time: self.estimate_clear_time(),
        }
    }
    
    fn assess_congestion(&self) -> CongestionLevel {
        let size_mb = self.calculate_total_size() as f64 / 1_000_000.0;
        
        match size_mb {
            s if s < 50.0 => CongestionLevel::Low,
            s if s < 150.0 => CongestionLevel::Medium,
            s if s < 300.0 => CongestionLevel::High,
            _ => CongestionLevel::Critical,
        }
    }
}
```

### Fee Rate Tracking

```typescript
interface FeeRate {
  satPerByte: number;
  timestamp: Date;
  confirmationTarget: number;
  confidence: number;
}

class FeeRateTracker {
  private rates: Map<number, FeeRate[]> = new Map();
  
  async updateFeeRates(): Promise<void> {
    const targets = [1, 2, 3, 6, 12, 24];
    
    for (const target of targets) {
      const rate = await this.fetchFeeRate(target);
      
      if (!this.rates.has(target)) {
        this.rates.set(target, []);
      }
      
      const targetRates = this.rates.get(target)!;
      targetRates.push(rate);
      
      // Keep only last 100 readings
      if (targetRates.length > 100) {
        targetRates.shift();
      }
    }
  }
  
  getFeeRecommendation(targetBlocks: number, priority: 'low' | 'medium' | 'high'): number {
    const rates = this.rates.get(targetBlocks) || [];
    if (rates.length === 0) return 1; // Fallback
    
    const recent = rates.slice(-10);
    const average = recent.reduce((sum, rate) => sum + rate.satPerByte, 0) / recent.length;
    
    const multipliers = {
      low: 0.8,
      medium: 1.0,
      high: 1.3
    };
    
    return Math.ceil(average * multipliers[priority]);
  }
}
```

## Fee Optimization Strategies

### Batch Transaction Optimization

```rust
pub struct TransactionBatcher {
    pending_outputs: Vec<TxOutput>,
    fee_estimator: FeeEstimator,
    min_batch_size: usize,
}

impl TransactionBatcher {
    pub fn optimize_batch(&self, outputs: &[TxOutput]) -> BatchOptimization {
        let single_tx_fees: Vec<u64> = outputs.iter()
            .map(|output| self.estimate_single_tx_fee(output))
            .collect();
        
        let batch_fee = self.estimate_batch_fee(outputs);
        let total_single_fees: u64 = single_tx_fees.iter().sum();
        
        BatchOptimization {
            batch_fee,
            individual_fees: single_tx_fees,
            savings: total_single_fees.saturating_sub(batch_fee),
            recommended: batch_fee < total_single_fees,
        }
    }
    
    fn estimate_batch_fee(&self, outputs: &[TxOutput]) -> u64 {
        // Calculate batch transaction size
        let input_size = 148; // Average input size
        let output_size = 34;  // Average output size
        let overhead = 10;     // Transaction overhead
        
        let tx_size = input_size + (outputs.len() * output_size) + overhead;
        let fee_rate = self.fee_estimator.estimate_fee(6, tx_size).sat_per_byte;
        
        (tx_size as f64 * fee_rate) as u64
    }
}
```

### RBF (Replace-by-Fee) Strategy

```rust
pub struct RBFManager {
    pending_transactions: HashMap<Txid, PendingTransaction>,
    fee_estimator: FeeEstimator,
}

impl RBFManager {
    pub fn should_replace_fee(&self, txid: &Txid) -> Option<FeeReplacement> {
        let tx = self.pending_transactions.get(txid)?;
        let blocks_waiting = self.calculate_blocks_waiting(tx);
        
        if blocks_waiting < tx.target_confirmations {
            return None; // Still within target
        }
        
        let current_fee_rate = tx.fee as f64 / tx.size as f64;
        let recommended_fee_rate = self.fee_estimator
            .estimate_fee(tx.target_confirmations, tx.size)
            .sat_per_byte;
        
        if recommended_fee_rate > current_fee_rate * 1.1 {
            Some(FeeReplacement {
                old_fee: tx.fee,
                new_fee: (recommended_fee_rate * tx.size as f64) as u64,
                fee_rate: recommended_fee_rate,
                urgency: self.calculate_urgency(blocks_waiting, tx.target_confirmations),
            })
        } else {
            None
        }
    }
}
```

## API Integration

### REST API Endpoints

```bash
# Get fee estimates for different confirmation targets
GET /api/v1/fees/estimate?targets=1,3,6,12

Response:
{
  "estimates": {
    "1": {"sat_per_byte": 25.5, "confidence": 0.95},
    "3": {"sat_per_byte": 18.2, "confidence": 0.90},
    "6": {"sat_per_byte": 12.8, "confidence": 0.85},
    "12": {"sat_per_byte": 8.4, "confidence": 0.80}
  },
  "timestamp": "2025-06-17T10:00:00Z"
}

# Get optimal fee for specific transaction
POST /api/v1/fees/calculate
Content-Type: application/json

{
  "tx_size": 250,
  "target_confirmations": 3,
  "priority": "medium"
}

Response:
{
  "fee_estimate": {
    "sat_per_byte": 18.2,
    "total_fee": 4550,
    "confirmation_target": 3,
    "confidence_level": 0.90
  }
}
```

### WebSocket Streaming

```javascript
const ws = new WebSocket('wss://api.anya-core.org/v1/fees/stream');

ws.onmessage = (event) => {
  const feeUpdate = JSON.parse(event.data);
  console.log('Fee update:', feeUpdate);
  
  // Update UI with new fee recommendations
  updateFeeDisplay(feeUpdate.estimates);
};

// Subscribe to fee updates
ws.send(JSON.stringify({
  method: 'subscribe',
  params: ['fees.estimate', 'fees.mempool']
}));
```

## Performance Metrics

### Fee Accuracy Tracking

```rust
pub struct FeeAccuracyTracker {
    predictions: VecDeque<FeePrediction>,
    actual_results: HashMap<Txid, ActualResult>,
}

impl FeeAccuracyTracker {
    pub fn track_prediction(&mut self, prediction: FeePrediction) {
        self.predictions.push_back(prediction);
        
        // Keep only recent predictions
        while self.predictions.len() > 1000 {
            self.predictions.pop_front();
        }
    }
    
    pub fn calculate_accuracy(&self) -> AccuracyMetrics {
        let mut correct_predictions = 0;
        let mut total_predictions = 0;
        let mut fee_efficiency_sum = 0.0;
        
        for prediction in &self.predictions {
            if let Some(actual) = self.actual_results.get(&prediction.txid) {
                total_predictions += 1;
                
                if actual.confirmed_in_target {
                    correct_predictions += 1;
                }
                
                fee_efficiency_sum += actual.fee_paid as f64 / prediction.estimated_fee as f64;
            }
        }
        
        AccuracyMetrics {
            prediction_accuracy: correct_predictions as f64 / total_predictions as f64,
            average_fee_efficiency: fee_efficiency_sum / total_predictions as f64,
            total_predictions,
        }
    }
}
```

## Configuration

### Fee Estimation Settings

```yaml
fee_estimation:
  enabled: true
  update_interval: 30  # seconds
  
  algorithms:
    historical_analysis:
      enabled: true
      lookback_blocks: 144  # ~24 hours
      weight_decay: 0.95
    
    ml_prediction:
      enabled: true
      model_update_frequency: 3600  # seconds
      feature_window: 720  # data points
    
    mempool_analysis:
      enabled: true
      sampling_interval: 10  # seconds
      
  targets:
    default: [1, 2, 3, 6, 12, 24]
    priority_mapping:
      urgent: 1
      high: 2
      medium: 6
      low: 12
      economy: 24
      
  rbf:
    enabled: true
    bump_threshold: 1.1  # 10% fee increase minimum
    max_replacements: 3
```

## See Also

- [Advanced Analytics](README.md)
- [Transaction Management](../../../docs/bitcoin/docs/features/transaction-management.md)
- [Network Performance](network-performance.md)
- [Price Analysis](price-analysis.md)

---

*This documentation is part of the Anya Enterprise Analytics suite.*
