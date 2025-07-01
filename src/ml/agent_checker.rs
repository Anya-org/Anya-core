// [AIR-3][AIS-3][BPC-3][RES-3] Removed unused import: std::error::Error
// AIP-002: Agent Checker System Implementation
// Priority: CRITICAL - ML-based system analyzer with in-memory auto-save

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Status threshold constants for system readiness
const DEVELOPMENT_THRESHOLD: f64 = 0.60;
const PRODUCTION_THRESHOLD: f64 = 0.90;
const RELEASE_THRESHOLD: f64 = 0.99;

/// Environment stage enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SystemStage {
    Development,
    Production,
    Release,
    Unavailable,
}

/// Component readiness status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentStatus {
    /// Component name
    pub name: String,
    /// Component status (0.0 to 1.0)
    pub status: f64,
    /// Last check time as timestamp (u64)
    pub last_check: u64,
    /// Additional metrics
    pub metrics: HashMap<String, f64>,
    /// List of issues
    pub issues: Vec<String>,
}

impl ComponentStatus {
    /// Create a new component status
    pub fn new(
        name: String,
        status: f64,
        metrics: HashMap<String, f64>,
        issues: Vec<String>,
    ) -> Self {
        Self {
            name,
            status,
            last_check: chrono::Utc::now().timestamp() as u64,
            metrics,
            issues,
        }
    }
}

/// System health metrics
#[derive(Debug, Clone)]
pub struct SystemHealth {
    overall_status: f64,
    stage: SystemStage,
    components: HashMap<String, ComponentStatus>,
    last_update: Instant,
}

/// Agent Checker main system
pub struct AgentChecker {
    health: Arc<Mutex<SystemHealth>>,
    input_buffer: Arc<Mutex<Vec<String>>>,
    input_counter: Arc<Mutex<usize>>,
    auto_save_frequency: usize,
    last_save: Arc<Mutex<Instant>>,
}

impl AgentChecker {
    /// Create a new agent checker with specified auto-save frequency
    pub fn new(auto_save_frequency: usize) -> Self {
        let health = SystemHealth {
            overall_status: 0.0,
            stage: SystemStage::Unavailable,
            components: HashMap::new(),
            last_update: Instant::now(),
        };

        Self {
            health: Arc::new(Mutex::new(health)),
            input_buffer: Arc::new(Mutex::new(Vec::new())),
            input_counter: Arc::new(Mutex::new(0)),
            auto_save_frequency,
            last_save: Arc::new(Mutex::new(Instant::now())),
        }
    }

    /// Process input and auto-save every Nth input
    pub fn process_input(&self, input: &str) -> Result<(), String> {
        // Add input to buffer
        {
            let mut buffer = match self.input_buffer.lock() {
                Ok(guard) => guard,
                Err(e) => return Err(format!("Failed to lock input buffer: {e}")),
            };
            buffer.push(input.to_string());
        }

        // Increment counter and check for auto-save
        let should_save = {
            let mut counter = match self.input_counter.lock() {
                Ok(guard) => guard,
                Err(e) => return Err(format!("Failed to lock input counter: {e}")),
            };
            *counter += 1;
            *counter % self.auto_save_frequency == 0
        };

        // Auto-save every Nth input (e.g., every 20th input)
        if should_save {
            self.save_state_to_memory();
            println!("Auto-saved state after processing input");
        }

        // Process the input for agent checking
        self.analyze_input(input)
    }

    /// Save the current state to memory (no file writing)
    fn save_state_to_memory(&self) {
        // In a real implementation, this would create a checkpoint of the current state
        match self.last_save.lock() {
            Ok(mut last_save) => {
                *last_save = Instant::now();
            }
            Err(e) => {
                log::error!("Failed to update last save time: {e}");
            }
        }
    }

    /// Analyze input for agent checking
    fn analyze_input(&self, input: &str) -> Result<(), String> {
        // Simplified implementation for demo purposes
        let mut health = match self.health.lock() {
            Ok(guard) => guard,
            Err(e) => return Err(format!("Failed to acquire health lock: {e}")),
        };

        // Update overall system health based on input
        // This is a placeholder for actual ML-based analysis
        if input.contains("error") {
            health.overall_status = (health.overall_status - 0.05).max(0.0);
        } else if input.contains("success") {
            health.overall_status = (health.overall_status + 0.03).min(1.0);
        }

        // Update system stage based on health
        health.stage = if health.overall_status >= RELEASE_THRESHOLD {
            SystemStage::Release
        } else if health.overall_status >= PRODUCTION_THRESHOLD {
            SystemStage::Production
        } else if health.overall_status >= DEVELOPMENT_THRESHOLD {
            SystemStage::Development
        } else {
            SystemStage::Unavailable
        };

        health.last_update = Instant::now();
        Ok(())
    }

    /// Get current system stage
    pub fn get_system_stage(&self) -> SystemStage {
        match self.health.lock() {
            Ok(health) => health.stage,
            Err(_) => SystemStage::Unavailable,
        }
    }

    /// Get system health metrics
    pub fn get_system_health(&self) -> SystemHealth {
        match self.health.lock() {
            Ok(health) => health.clone(),
            Err(_) => SystemHealth {
                stage: SystemStage::Unavailable,
                components: HashMap::new(),
                overall_status: 0.0,
                last_update: Instant::now(),
            },
        }
    }

    /// Check component readiness
    pub fn check_component_status(&self, component_name: &str) -> Option<ComponentStatus> {
        match self.health.lock() {
            Ok(health) => health.components.get(component_name).cloned(),
            Err(_) => None,
        }
    }

    /// Update component status
    pub fn update_component_status(
        &self,
        component_name: &str,
        status: f64,
        metrics: HashMap<String, f64>,
        issues: Vec<String>,
    ) {
        let mut health = match self.health.lock() {
            Ok(health) => health,
            Err(e) => {
                log::error!("Failed to acquire health lock: {e}");
                return;
            }
        };

        let component = ComponentStatus {
            name: component_name.to_string(),
            status,
            last_check: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or(Duration::ZERO)
                .as_secs(),
            metrics,
            issues,
        };

        health
            .components
            .insert(component_name.to_string(), component);

        // Recalculate overall system health
        let component_count = health.components.len() as f64;
        let total_status: f64 = health.components.values().map(|c| c.status).sum();

        if component_count > 0.0 {
            health.overall_status = total_status / component_count;

            // Update system stage based on overall status
            health.stage = if health.overall_status >= RELEASE_THRESHOLD {
                SystemStage::Release
            } else if health.overall_status >= PRODUCTION_THRESHOLD {
                SystemStage::Production
            } else if health.overall_status >= DEVELOPMENT_THRESHOLD {
                SystemStage::Development
            } else {
                SystemStage::Unavailable
            };
        }
    }

    /// Validate system readiness against thresholds
    pub fn validate_system_readiness(&self) -> (bool, SystemStage, Vec<String>) {
        let health = match self.health.lock() {
            Ok(health) => health,
            Err(e) => {
                log::error!("Failed to acquire health lock: {e}");
                return (
                    false,
                    SystemStage::Unavailable,
                    vec![format!("Failed to acquire health lock: {}", e)],
                );
            }
        };

        let stage = health.stage;
        let mut issues = Vec::new();

        for (name, component) in &health.components {
            if component.status < DEVELOPMENT_THRESHOLD {
                issues.push(format!(
                    "Component {} is below minimum threshold: {:.2}",
                    name, component.status
                ));
            }
        }

        let is_ready = match stage {
            SystemStage::Development => health.overall_status >= DEVELOPMENT_THRESHOLD,
            SystemStage::Production => health.overall_status >= PRODUCTION_THRESHOLD,
            SystemStage::Release => health.overall_status >= RELEASE_THRESHOLD,
            SystemStage::Unavailable => false,
        };

        (is_ready, stage, issues)
    }

    /// Get input buffer stats
    pub fn get_input_stats(&self) -> (usize, usize, Duration) {
        let buffer = match self.input_buffer.lock() {
            Ok(guard) => guard,
            Err(_) => return (0, 0, Duration::from_secs(0)),
        };

        let counter = match self.input_counter.lock() {
            Ok(guard) => *guard,
            Err(_) => return (buffer.len(), 0, Duration::from_secs(0)),
        };

        let last_save = match self.last_save.lock() {
            Ok(guard) => guard.elapsed(),
            Err(_) => Duration::from_secs(0),
        };

        (buffer.len(), counter, last_save)
    }
}

// Tests for the AgentChecker
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_processing_with_auto_save() -> Result<(), Box<dyn std::error::Error>> {
        let checker = AgentChecker::new(20); // Auto-save every 20th input

        // Process 25 inputs
        for i in 0..25 {
            let input = if i % 5 == 0 {
                format!("success message {i}")
            } else {
                format!("normal message {i}")
            };

            checker
                .process_input(&input)
                .map_err(|e| format!("Failed to process input: {e}"))?;
        }

        // Check the stats
        let (buffer_size, counter, _) = checker.get_input_stats();
        assert_eq!(buffer_size, 25);
        assert_eq!(counter, 25);

        // Verify system state updated
        let health = checker.get_system_health();
        assert!(health.overall_status > 0.0);

        Ok(())
    }

    #[test]
    fn test_system_stage_transitions() -> Result<(), Box<dyn std::error::Error>> {
        let checker = AgentChecker::new(10);

        // Initially at Unavailable
        assert_eq!(checker.get_system_stage(), SystemStage::Unavailable);

        // Update component to reach Development stage
        let mut metrics = HashMap::new();
        metrics.insert("memory".to_string(), 0.70);
        metrics.insert("cpu".to_string(), 0.65);

        checker.update_component_status("core", 0.62, metrics, vec![]);

        // Should be at Development stage now
        assert_eq!(checker.get_system_stage(), SystemStage::Development);

        Ok(())
    }
}
