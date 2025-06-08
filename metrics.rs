pub fn collect_metrics() {
    let propagation_time = block_propagation_timer.elapsed();
    metrics.record("block_propagation_ms", propagation_time.as_millis());
    
    let segwit_rate = calculate_segwit_adoption();
    metrics.record("segwit_adoption_percent", segwit_rate);
} 