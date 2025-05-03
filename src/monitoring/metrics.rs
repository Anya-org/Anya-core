use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

// Global metrics registry
lazy_static! {
    static ref METRICS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

/// Exports all metrics in Prometheus format
pub fn export_metrics() -> String {
    let metrics = METRICS.lock().unwrap();
    let mut output = String::new();
    
    // Add BIP compliance metrics
    output.push_str("# HELP bip_compliance Bitcoin Improvement Proposal compliance status\n");
    output.push_str("# TYPE bip_compliance gauge\n");
    
    // Add all metrics in Prometheus format
    for (key, value) in metrics.iter() {
        output.push_str(&format!("{} {}\n", key, value));
    }
    
    output
}

/// Registers BIP compliance status
pub fn register_bip_compliance(bip: &str, compliant: bool) {
    let mut metrics = METRICS.lock().unwrap();
    metrics.insert(
        format!("bip_compliance{{type=\"{}\"}}", bip),
        if compliant { "1".to_string() } else { "0".to_string() }
    );
}

/// Registers mempool size
pub fn register_mempool_size(size: usize) {
    let mut metrics = METRICS.lock().unwrap();
    metrics.insert("mempool_size".to_string(), size.to_string());
}

/// Registers taproot usage percentage
pub fn register_taproot_usage(percentage: f64) {
    let mut metrics = METRICS.lock().unwrap();
    metrics.insert("taproot_usage".to_string(), percentage.to_string());
}

/// Registers block propagation time in milliseconds
pub fn register_block_propagation_time(milliseconds: u64) {
    let mut metrics = METRICS.lock().unwrap();
    metrics.insert("block_propagation_time_ms".to_string(), milliseconds.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_registration() {
        // Register some metrics
        register_bip_compliance("341", true);
        register_bip_compliance("342", true);
        register_bip_compliance("174", true);
        register_mempool_size(15000);
        register_taproot_usage(78.5);
        
        // Export and verify
        let exported = export_metrics();
        assert!(exported.contains("bip_compliance{type=\"341\"} 1"));
        assert!(exported.contains("bip_compliance{type=\"342\"} 1"));
        assert!(exported.contains("bip_compliance{type=\"174\"} 1"));
        assert!(exported.contains("mempool_size 15000"));
        assert!(exported.contains("taproot_usage 78.5"));
    }
} 