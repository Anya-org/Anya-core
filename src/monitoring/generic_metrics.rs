use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

// Generic metrics registry for metrics not covered by specialized systems
lazy_static! {
    static ref GENERIC_METRICS: Mutex<HashMap<String, f64>> = Mutex::new(HashMap::new());
}

/// Register a generic metric
pub fn register_metric(name: &str, value: f64) {
    let mut metrics = GENERIC_METRICS.lock().unwrap();
    metrics.insert(name.to_string(), value);
}

/// Get all generic metrics
pub fn get_generic_metrics() -> HashMap<String, f64> {
    let metrics = GENERIC_METRICS.lock().unwrap();
    metrics.clone()
}

/// Include generic metrics in Prometheus output
pub fn include_generic_metrics(output: &mut String) {
    let metrics = GENERIC_METRICS.lock().unwrap();
    
    // Add all generic metrics
    for (key, value) in metrics.iter() {
        output.push_str(&format!("{} {}\n", key, value));
    }
}
