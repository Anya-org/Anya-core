pub fn register_metrics() {
    register_counter!(
        "mcp_tx_processed_total", 
        "Total transactions processed"
    ).unwrap();
    
    register_histogram!(
        "mcp_validation_duration_seconds",
        "Transaction validation time distribution"
    ).unwrap();
} 