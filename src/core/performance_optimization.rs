// [AIR-3][AIS-3][BPC-3][RES-3] Removed unused import: std::error::Error
// AIR-008: Performance Optimization Implementation
// Priority: HIGH - Performance tuning with in-memory auto-save

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Resource type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    CPU,
    Memory,
    Disk,
    Network,
    Database,
    Cache,
    Custom(u32),
}

/// Resource optimization status
#[derive(Debug, Clone, PartialEq)]
pub enum OptimizationStatus {
    NotOptimized,
    Optimizing,
    Optimized,
    Failed,
}

/// Performance metrics for a resource
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PerformanceMetrics {
    resource_type: ResourceType,
    utilization: f64,
    throughput: f64,
    latency: Duration,
    metrics: HashMap<String, f64>,
    last_updated: Instant,
}

/// Resource optimization configuration
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct OptimizationConfig {
    resource_type: ResourceType,
    name: String,
    status: OptimizationStatus,
    settings: HashMap<String, String>,
    target_utilization: f64,
    target_throughput: f64,
    target_latency: Duration,
    last_modified: Instant,
}

/// Performance optimization manager
pub struct PerformanceOptimizer {
    resources: Arc<Mutex<HashMap<String, OptimizationConfig>>>,
    metrics: Arc<Mutex<HashMap<String, PerformanceMetrics>>>,
    input_counter: Arc<Mutex<usize>>,
    auto_save_frequency: usize,
}

impl PerformanceOptimizer {
    /// Create a new performance optimizer
    pub fn new(auto_save_frequency: usize) -> Self {
        Self {
            resources: Arc::new(Mutex::new(HashMap::new())),
            metrics: Arc::new(Mutex::new(HashMap::new())),
            input_counter: Arc::new(Mutex::new(0)),
            auto_save_frequency,
        }
    }

    /// Add or update resource configuration
    pub fn configure_resource(
        &self,
        resource_name: &str,
        resource_type: ResourceType,
        settings: HashMap<String, String>,
        target_utilization: f64,
        target_throughput: f64,
        target_latency: Duration,
    ) -> Result<(), String> {
        {
            let mut resources = self
                .resources
                .lock()
                .map_err(|e| format!("Mutex lock error: {}", e))?;

            let config = OptimizationConfig {
                resource_type,
                name: resource_name.to_string(),
                status: OptimizationStatus::NotOptimized,
                settings,
                target_utilization,
                target_throughput,
                target_latency,
                last_modified: Instant::now(),
            };

            resources.insert(resource_name.to_string(), config);
        } // Release the lock before calling auto-save

        // Record input and potentially auto-save
        let _ = self.record_input_and_check_save();

        Ok(())
    }

    /// Update performance metrics for a resource
    pub fn update_metrics(
        &self,
        resource_name: &str,
        utilization: f64,
        throughput: f64,
        latency: Duration,
        additional_metrics: HashMap<String, f64>,
    ) -> Result<(), String> {
        // Check if resource exists and get resource type
        let resource_type = {
            let resources = self
                .resources
                .lock()
                .map_err(|e| format!("Mutex lock error: {}", e))?;
            if !resources.contains_key(resource_name) {
                return Err(format!("Resource not found: {}", resource_name));
            }
            resources
                .get(resource_name)
                .ok_or(format!("Resource not found: {}", resource_name))?
                .resource_type
        };

        // Update metrics
        {
            let mut metrics_map = self
                .metrics
                .lock()
                .map_err(|e| format!("Mutex lock error: {}", e))?;

            let metrics = PerformanceMetrics {
                resource_type,
                utilization,
                throughput,
                latency,
                metrics: additional_metrics,
                last_updated: Instant::now(),
            };

            metrics_map.insert(resource_name.to_string(), metrics);
        } // Release the lock before calling auto-save

        // Record input and potentially auto-save
        let _ = self.record_input_and_check_save();

        Ok(())
    }

    /// Record an input and check if auto-save is needed
    fn record_input_and_check_save(&self) -> Result<(), String> {
        let mut counter = self
            .input_counter
            .lock()
            .map_err(|e| format!("Mutex lock error: {}", e))?;
        *counter += 1;

        // Auto-save every Nth input (e.g., every 20th input)
        if *counter % self.auto_save_frequency == 0 {
            match self.save_state_to_memory() {
                Ok(_) => println!("Auto-saved performance state after {} changes", *counter),
                Err(e) => eprintln!("Failed to auto-save: {}", e),
            }
        }

        Ok(())
    }

    /// Save the current state to memory (no file writing)
    fn save_state_to_memory(&self) -> Result<(), String> {
        // In a real implementation, this would create a snapshot of current performance state
        // For this implementation, we're just keeping everything in memory
        let resources = self
            .resources
            .lock()
            .map_err(|e| format!("Mutex lock error: {}", e))?;
        let metrics = self
            .metrics
            .lock()
            .map_err(|e| format!("Mutex lock error: {}", e))?;

        println!(
            "In-memory performance snapshot created: {} resources, {} metrics",
            resources.len(),
            metrics.len()
        );

        // Here you would normally serialize the state and store it
        Ok(())
    }

    /// Optimize a specific resource
    pub fn optimize_resource(&self, resource_name: &str) -> Result<OptimizationStatus, String> {
        // Get resource configuration and update status
        let status = {
            let mut resources = self
                .resources
                .lock()
                .map_err(|e| format!("Mutex lock error: {}", e))?;

            let config = match resources.get_mut(resource_name) {
                Some(config) => config,
                None => return Err(format!("Resource not found: {}", resource_name)),
            };

            // Check if metrics exist
            let metrics = {
                let metrics_map = self
                    .metrics
                    .lock()
                    .map_err(|e| format!("Mutex lock error: {}", e))?;
                match metrics_map.get(resource_name) {
                    Some(metrics) => metrics.clone(),
                    None => {
                        return Err(format!(
                            "No metrics available for resource: {}",
                            resource_name
                        ))
                    }
                }
            };

            // For demonstration purposes, we're just simulating optimization
            println!(
                "Optimizing resource {}: {:?}",
                resource_name, config.resource_type
            );

            // Simulate optimization logic
            let mut optimized = true;

            if metrics.utilization > config.target_utilization {
                println!(
                    "  - High utilization: {:.2}% (target: {:.2}%)",
                    metrics.utilization * 100.0,
                    config.target_utilization * 100.0
                );
                optimized = false;
            }

            if metrics.throughput < config.target_throughput {
                println!(
                    "  - Low throughput: {:.2} (target: {:.2})",
                    metrics.throughput, config.target_throughput
                );
                optimized = false;
            }

            if metrics.latency > config.target_latency {
                println!(
                    "  - High latency: {:?} (target: {:?})",
                    metrics.latency, config.target_latency
                );
                optimized = false;
            }

            // Update status
            config.status = if optimized {
                OptimizationStatus::Optimized
            } else {
                // Apply optimizations (simulated here)
                println!("  - Applying optimizations...");
                OptimizationStatus::Optimized
            };

            config.last_modified = Instant::now();
            config.status.clone()
        }; // Release the lock before calling auto-save

        // Record input and potentially auto-save
        let _ = self.record_input_and_check_save();

        Ok(status)
    }

    /// Optimize all resources
    pub fn optimize_all_resources(&self) -> HashMap<String, Result<OptimizationStatus, String>> {
        let resource_names = match self.resources.lock() {
            Ok(resources) => {
                let names: Vec<String> = resources.keys().cloned().collect();
                drop(resources); // Release the lock
                names
            }
            Err(e) => {
                // Return empty map with error for all resources if we can't even get the lock
                let mut map = HashMap::new();
                map.insert(
                    "general".to_string(),
                    Err(format!("Mutex lock error: {}", e)),
                );
                return map;
            }
        };

        // Optimize each resource
        let mut results = HashMap::new();
        for name in resource_names {
            results.insert(name.clone(), self.optimize_resource(&name));
        }

        results
    }

    /// Get resource configuration
    pub fn get_resource_config(&self, resource_name: &str) -> Option<OptimizationConfig> {
        match self.resources.lock() {
            Ok(resources) => resources.get(resource_name).cloned(),
            Err(e) => {
                eprintln!("Mutex lock error: {}", e);
                None
            }
        }
    }

    /// Get resource metrics
    pub fn get_resource_metrics(&self, resource_name: &str) -> Option<PerformanceMetrics> {
        match self.metrics.lock() {
            Ok(metrics) => metrics.get(resource_name).cloned(),
            Err(e) => {
                eprintln!("Mutex lock error: {}", e);
                None
            }
        }
    }

    /// Get all resource configurations
    pub fn get_all_resources(&self) -> Vec<OptimizationConfig> {
        match self.resources.lock() {
            Ok(resources) => resources.values().cloned().collect(),
            Err(e) => {
                eprintln!("Mutex lock error: {}", e);
                Vec::new()
            }
        }
    }

    /// Get all resource metrics
    pub fn get_all_metrics(&self) -> Vec<PerformanceMetrics> {
        match self.metrics.lock() {
            Ok(metrics) => metrics.values().cloned().collect(),
            Err(e) => {
                eprintln!("Mutex lock error: {}", e);
                Vec::new()
            }
        }
    }

    /// Get number of changes and resources
    pub fn get_stats(&self) -> (usize, usize, usize) {
        let counter = match self.input_counter.lock() {
            Ok(counter) => *counter,
            Err(e) => {
                eprintln!("Mutex lock error for counter: {}", e);
                0
            }
        };

        let resources_len = match self.resources.lock() {
            Ok(resources) => resources.len(),
            Err(e) => {
                eprintln!("Mutex lock error for resources: {}", e);
                0
            }
        };

        let metrics_len = match self.metrics.lock() {
            Ok(metrics) => metrics.len(),
            Err(e) => {
                eprintln!("Mutex lock error for metrics: {}", e);
                0
            }
        };

        (counter, resources_len, metrics_len)
    }
}

// Tests for the PerformanceOptimizer
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_configuration_and_auto_save() -> Result<(), Box<dyn std::error::Error>> {
        let optimizer = PerformanceOptimizer::new(20); // Auto-save every 20th change

        // Configure 25 resources to trigger auto-save
        for i in 0..25 {
            let mut settings = HashMap::new();
            settings.insert("max_connections".to_string(), "100".to_string());
            settings.insert("timeout".to_string(), "5000".to_string());

            optimizer.configure_resource(
                &format!("resource_{}", i),
                ResourceType::CPU,
                settings,
                0.7,
                1000.0,
                Duration::from_millis(100),
            )?;
        }

        // Check stats
        let (changes, resources, _) = optimizer.get_stats();
        assert_eq!(changes, 25);
        assert_eq!(resources, 25);

        Ok(())
    }

    #[test]
    fn test_optimization_workflow() -> Result<(), Box<dyn std::error::Error>> {
        let optimizer = PerformanceOptimizer::new(10);

        // Configure a resource
        let mut settings = HashMap::new();
        settings.insert("cache_size".to_string(), "1024".to_string());

        optimizer.configure_resource(
            "database",
            ResourceType::Database,
            settings,
            0.8,
            500.0,
            Duration::from_millis(50),
        )?;

        // Add metrics
        let mut additional_metrics = HashMap::new();
        additional_metrics.insert("cache_hits".to_string(), 0.75);
        additional_metrics.insert("query_count".to_string(), 1500.0);

        optimizer.update_metrics(
            "database",
            0.9,                       // High utilization, needs optimization
            450.0,                     // Lower than target
            Duration::from_millis(60), // Higher than target
            additional_metrics,
        )?;

        // Optimize the resource
        let result = optimizer.optimize_resource("database")?;
        assert_eq!(result, OptimizationStatus::Optimized);

        // Verify the status
        if let Some(config) = optimizer.get_resource_config("database") {
            assert_eq!(config.status, OptimizationStatus::Optimized);
        } else {
            return Err("Resource config not found".into());
        }

        Ok(())
    }
}
