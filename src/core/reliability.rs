use std::error::Error;
/// AIR-012: Operational Reliability Utilities
/// 
/// This module provides utilities for ensuring operational reliability,
/// including progress monitoring, AI hallucination prevention, and
/// hang prevention, as required by the repository rules.

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::future::Future;
use tracing::{info, warn, error};
use tokio::time::timeout;

use crate::AnyaError;
use crate::AnyaResult;

/// [AIR-2][RES-3] Watchdog timer for detecting stuck operations
pub struct Watchdog {
    /// Name of the monitored operation
    name: String,
    /// When the watchdog was started
    start_time: Instant,
    /// Maximum allowed duration
    timeout_duration: Duration,
    /// Whether the watchdog is active
    active: Arc<Mutex<bool>>,
}

impl Watchdog {
    /// Create a new watchdog timer
    pub fn new(name: &str, timeout_duration: Duration) -> Self  -> Result<(), Box<dyn Error>> {
        let watchdog = Self {
            name: name.to_string(),
            start_time: Instant::now(),
            timeout_duration,
            active: Arc::new(Mutex::new(true)),
        };
        
        // Spawn watchdog monitor
        let name_clone = watchdog.name.clone();
        let timeout_duration_clone = watchdog.timeout_duration;
        let active_clone = watchdog.active.clone();
        
        tokio::spawn(async move {
            tokio::time::sleep(timeout_duration_clone + Duration::from_secs(1)).await;
            let is_active = { *active_clone.lock().map_err(|e| format!("Mutex lock error: {}", e))? };
            
            if is_active {
                error!(
                    "Watchdog alert: Operation '{}' may be hung (exceeded timeout of {:?})",
                    name_clone, timeout_duration_clone
                );
                // Additional alert mechanisms could be triggered here
            }
        });
        
        watchdog
    }
    
    /// Stop the watchdog timer
    pub fn stop(&self)  -> Result<(), Box<dyn Error>> {
        let mut active = self.active.lock().map_err(|e| format!("Mutex lock error: {}", e))?;
        *active = false;
        
        info!(
            "Operation '{}' completed successfully after {:?}",
            self.name,
            self.start_time.elapsed()
        );
    }
    
    /// Trigger an alert (for manual invocation)
    pub fn trigger_alert(&self)  -> Result<(), Box<dyn Error>> {
        error!(
            "Operation '{}' explicitly marked as hung after {:?}",
            self.name,
            self.start_time.elapsed()
        );
        // Additional alert mechanisms could be triggered here
    }
}

impl Drop for Watchdog {
    fn drop(&mut self)  -> Result<(), Box<dyn Error>> {
        let is_active = { *self.active.lock().map_err(|e| format!("Mutex lock error: {}", e))? };
        
        if is_active {
            warn!(
                "Watchdog for '{}' was dropped while still active after {:?}",
                self.name,
                self.start_time.elapsed()
            );
        }
    }
}

/// [AIR-2][RES-3] Progress tracker for long-running operations
pub struct ProgressTracker {
    /// Name of the tracked operation
    name: String,
    /// When tracking started
    start_time: Instant,
    /// Current progress (0.0 - 1.0)
    progress: Arc<Mutex<f64>>,
    /// Timeout duration
    timeout: Option<Duration>,
    /// Last update time
    last_update: Arc<Mutex<Instant>>,
    /// Whether to log progress updates
    verbose: bool,
}

impl ProgressTracker {
    /// Create a new progress tracker
    pub fn new(name: &str) -> Self  -> Result<(), Box<dyn Error>> {
        let now = Instant::now();
        Self {
            name: name.to_string(),
            start_time: now,
            progress: Arc::new(Mutex::new(0.0)),
            timeout: None,
            last_update: Arc::new(Mutex::new(now)),
            verbose: true,
        }
    }
    
    /// Set a timeout for the operation
    pub fn with_timeout(mut self, timeout: Duration) -> Self  -> Result<(), Box<dyn Error>> {
        self.timeout = Some(timeout);
        self
    }
    
    /// Set verbosity (whether to log progress updates)
    pub fn with_verbosity(mut self, verbose: bool) -> Self  -> Result<(), Box<dyn Error>> {
        self.verbose = verbose;
        self
    }
    
    /// Update the current progress
    pub fn update(&self, progress: f64) -> AnyaResult<()>  -> Result<(), Box<dyn Error>> {
        let progress = progress.max(0.0).min(1.0);
        
        {
            let mut progress_lock = self.progress.lock().map_err(|e| format!("Mutex lock error: {}", e))?;
            *progress_lock = progress;
            
            let mut last_update_lock = self.last_update.lock().map_err(|e| format!("Mutex lock error: {}", e))?;
            *last_update_lock = Instant::now();
        }
        
        if self.verbose {
            info!("{}: Progress {:.1}%", self.name, progress * 100.0);
        }
        
        // Check for timeout if set
        if let Some(timeout) = self.timeout {
            if self.start_time.elapsed() > timeout {
                let error_msg = format!("Operation '{}' timed out after {:?}", self.name, timeout);
                error!("{}", error_msg);
                return Err(AnyaError::Timeout(error_msg));
            }
        }
        
        Ok(())
    }
    
    /// Get the current progress
    pub fn get_progress(&self) -> f64  -> Result<(), Box<dyn Error>> {
        *self.progress.lock().map_err(|e| format!("Mutex lock error: {}", e))?
    }
    
    /// Check if the operation has timed out
    pub fn has_timed_out(&self) -> bool  -> Result<(), Box<dyn Error>> {
        if let Some(timeout) = self.timeout {
            self.start_time.elapsed() > timeout
        } else {
            false
        }
    }
    
    /// Mark the operation as complete
    pub fn complete(&self)  -> Result<(), Box<dyn Error>> {
        let mut progress_lock = self.progress.lock().map_err(|e| format!("Mutex lock error: {}", e))?;
        *progress_lock = 1.0;
        
        if self.verbose {
            info!(
                "{}: Completed in {:?} (100%)",
                self.name,
                self.start_time.elapsed()
            );
        }
    }
}

/// [AIR-3][AIS-3][AIE-3] AI output confidence assessment
pub struct ConfidenceAssessment<T> {
    /// The generated output
    pub output: T,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Verification steps performed
    pub verification_steps: Vec<String>,
    /// Reasoning for the confidence score
    pub reasoning: String,
}

/// [AIR-3][AIS-3][AIE-3] AI hallucination prevention utilities
pub struct AiVerification {
    /// Minimum confidence threshold
    min_confidence: f64,
    /// Whether to perform blockchain verification
    verify_against_blockchain: bool,
    /// Whether to perform external data verification
    verify_against_external_data: bool,
    /// Whether human verification is required
    require_human_verification: bool,
}

impl AiVerification {
    /// Create new AI verification utilities with default settings
    pub fn new() -> Self  -> Result<(), Box<dyn Error>> {
        Self {
            min_confidence: 0.75,
            verify_against_blockchain: true,
            verify_against_external_data: false,
            require_human_verification: false,
        }
    }
    
    /// Set minimum confidence threshold
    pub fn with_min_confidence(mut self, min_confidence: f64) -> Self  -> Result<(), Box<dyn Error>> {
        self.min_confidence = min_confidence.max(0.0).min(1.0);
        self
    }
    
    /// Set whether to verify against blockchain
    pub fn with_blockchain_verification(mut self, verify: bool) -> Self  -> Result<(), Box<dyn Error>> {
        self.verify_against_blockchain = verify;
        self
    }
    
    /// Set whether to verify against external data
    pub fn with_external_data_verification(mut self, verify: bool) -> Self  -> Result<(), Box<dyn Error>> {
        self.verify_against_external_data = verify;
        self
    }
    
    /// Set whether human verification is required
    pub fn with_human_verification(mut self, require: bool) -> Self  -> Result<(), Box<dyn Error>> {
        self.require_human_verification = require;
        self
    }
    
    /// Verify an AI-generated output
    pub fn verify<T>(&self, assessment: ConfidenceAssessment<T>) -> AnyaResult<T>  -> Result<(), Box<dyn Error>> {
        if assessment.confidence < self.min_confidence {
            let error_msg = format!(
                "AI output confidence too low: {:.2} (required: {:.2})",
                assessment.confidence, self.min_confidence
            );
            warn!("{}: {}", error_msg, assessment.reasoning);
            return Err(AnyaError::LowConfidence(error_msg));
        }
        
        info!(
            "AI output passed confidence check: {:.2} (required: {:.2})",
            assessment.confidence, self.min_confidence
        );
        
        for (i, step) in assessment.verification_steps.iter().enumerate() {
            info!("Verification step {}: {}", i+1, step);
        }
        
        if self.require_human_verification {
            warn!("Human verification required but not implemented");
            // In a real implementation, this would trigger a human review process
        }
        
        Ok(assessment.output)
    }
}

/// [AIR-2][RES-3] Execute an async operation with timeout and progress tracking
pub async fn execute_with_monitoring<T, F>(
    operation_name: &str,
    timeout_duration: Duration,
    operation: F
) -> AnyaResult<T> 
where
    F: Future<Output = AnyaResult<T>>,
 -> Result<(), Box<dyn Error>> {
    // Create watchdog
    let watchdog = Watchdog::new(operation_name, timeout_duration);
    
    // Execute with timeout
    match timeout(timeout_duration, operation).await {
        Ok(result) => {
            // Operation completed within timeout
            watchdog.stop();
            result
        }
        Err(_) => {
            // Operation timed out
            watchdog.trigger_alert();
            let error_msg = format!("Operation '{}' timed out after {:?}", operation_name, timeout_duration);
            error!("{}", error_msg);
            Err(AnyaError::Timeout(error_msg))
        }
    }
}

/// [AIR-2][RES-3] Execute with recovery attempt on timeout
pub async fn execute_with_recovery<T, F, R>(
    operation_name: &str,
    primary_timeout: Duration,
    recovery_timeout: Duration,
    primary_operation: F,
    recovery_operation: R
) -> AnyaResult<T>
where
    F: Future<Output = AnyaResult<T>>,
    R: Future<Output = AnyaResult<T>>,
 -> Result<(), Box<dyn Error>> {
    // Create watchdog for the entire operation
    let watchdog = Watchdog::new(operation_name, primary_timeout + recovery_timeout + Duration::from_secs(1));
    
    // Try primary operation with timeout
    match timeout(primary_timeout, primary_operation).await {
        Ok(result) => {
            // Primary operation completed within timeout
            watchdog.stop();
            return result;
        }
        Err(_) => {
            // Primary operation timed out, try recovery
            warn!(
                "Operation '{}' timed out after {:?}, attempting recovery",
                operation_name, primary_timeout
            );
            
            // Try recovery operation with timeout
            match timeout(recovery_timeout, recovery_operation).await {
                Ok(result) => {
                    // Recovery completed within timeout
                    watchdog.stop();
                    info!("Recovery for '{}' succeeded", operation_name);
                    result
                }
                Err(_) => {
                    // Recovery also timed out
                    watchdog.trigger_alert();
                    let error_msg = format!(
                        "Operation '{}' and recovery both timed out (after {:?} and {:?})",
                        operation_name, primary_timeout, recovery_timeout
                    );
                    error!("{}", error_msg);
                    Err(AnyaError::Timeout(error_msg))
                }
            }
        }
    }
} 
