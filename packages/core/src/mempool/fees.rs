//! Fee estimation for Bitcoin transactions
//! This module implements fee estimation algorithms for Bitcoin transactions

use log::info;

/// Estimate fee for a transaction based on current network conditions
pub fn estimate_fee(tx_vsize: usize, priority: FeePriority) -> u64 {
    let fee_rate = match priority {
        FeePriority::Low => 1.0,    // 1 sat/vB
        FeePriority::Medium => 5.0, // 5 sat/vB
        FeePriority::High => 10.0,  // 10 sat/vB
    };
    
    info!("Estimating fee with rate {} sat/vB", fee_rate);
    (tx_vsize as f64 * fee_rate).round() as u64
}

/// Transaction fee priority levels
#[derive(Debug, Clone, Copy)]
pub enum FeePriority {
    /// Low priority (slower confirmation)
    Low,
    /// Medium priority (average confirmation time)
    Medium,
    /// High priority (faster confirmation)
    High,
}
