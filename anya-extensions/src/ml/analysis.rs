//! Analysis Module
//!
//! This module provides analysis capabilities for ML models
//! and system performance within the Anya ecosystem.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub id: String,
    pub timestamp: u64,
    pub metrics: HashMap<String, f64>,
    pub status: AnalysisStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalysisStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone)]
pub struct AnalysisEngine {
    results: Vec<AnalysisResult>,
}

impl AnalysisEngine {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    pub fn add_result(&mut self, result: AnalysisResult) {
        self.results.push(result);
    }

    pub fn get_results(&self) -> &[AnalysisResult] {
        &self.results
    }
}

impl Default for AnalysisEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct AnalysisManager {
    engine: AnalysisEngine,
}

impl AnalysisManager {
    pub fn new() -> Self {
        Self {
            engine: AnalysisEngine::new(),
        }
    }

    pub fn add_result(&mut self, result: AnalysisResult) {
        self.engine.add_result(result);
    }

    pub fn get_results(&self) -> &[AnalysisResult] {
        self.engine.get_results()
    }
}
