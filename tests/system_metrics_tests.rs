use std::time::Duration;
use std::thread;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monitoring::system_metrics::SystemMetricsCollector;
    use crate::monitoring::generic_metrics;
    
    #[test] 
    fn test_system_metrics_collection() {
        // Create a system metrics collector
        let mut collector = SystemMetricsCollector::new();
        
        // Verify it's healthy
        assert!(collector.is_healthy());
        
        // Collect metrics
        collector.collect_system_metrics();
        
        // Give it a moment to collect
        thread::sleep(Duration::from_millis(100));
        
        // Verify metrics were collected
        let metrics = generic_metrics::get_generic_metrics();
        
        // Check that system metrics are present and reasonable
        assert!(metrics.contains_key("system_cpu_usage_percent"));
        assert!(metrics.contains_key("system_memory_usage_percent"));
        assert!(metrics.contains_key("system_memory_total_bytes"));
        
        // Verify CPU usage is in reasonable range (0-100%)
        let cpu_usage = metrics.get("system_cpu_usage_percent").unwrap();
        assert!(*cpu_usage >= 0.0 && *cpu_usage <= 100.0, "CPU usage should be 0-100%, got {}", cpu_usage);
        
        // Verify memory usage is in reasonable range (0-100%)  
        let memory_usage = metrics.get("system_memory_usage_percent").unwrap();
        assert!(*memory_usage >= 0.0 && *memory_usage <= 100.0, "Memory usage should be 0-100%, got {}", memory_usage);
        
        // Verify memory total is positive
        let memory_total = metrics.get("system_memory_total_bytes").unwrap();
        assert!(*memory_total > 0.0, "Total memory should be positive, got {}", memory_total);
        
        println!("✅ System metrics collection test passed");
        println!("   CPU Usage: {:.2}%", cpu_usage);
        println!("   Memory Usage: {:.2}%", memory_usage);
        println!("   Total Memory: {:.0} MB", memory_total / 1024.0 / 1024.0);
    }
    
    #[test]
    fn test_metrics_service_with_real_system_metrics() {
        use crate::monitoring::metrics_service::MetricsService;
        
        // Create metrics service with a very short interval for testing
        let service = MetricsService::new(Some(500)); // 500ms
        
        // Start the service
        service.start();
        
        // Let it collect metrics a few times
        thread::sleep(Duration::from_millis(1200));
        
        // Stop the service
        service.stop();
        
        // Verify that real system metrics were collected
        let metrics = generic_metrics::get_generic_metrics();
        
        // Should have system metrics from our new collector
        assert!(metrics.contains_key("system_cpu_usage_percent"));
        assert!(metrics.contains_key("system_memory_usage_percent"));
        
        println!("✅ Metrics service test with real system metrics passed");
        println!("   Collected {} metrics total", metrics.len());
    }
}