//! Machine Learning Research Module
//! 
//! This module provides research capabilities for ML algorithms
//! and model development within the Anya ecosystem.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchModel {
    pub id: String,
    pub name: String,
    pub version: String,
    pub parameters: HashMap<String, f64>,
}

#[derive(Debug, Clone)]
pub struct ResearchEngine {
    models: Vec<ResearchModel>,
}

impl ResearchEngine {
    pub fn new() -> Self {
        Self {
            models: Vec::new(),
        }
    }

    pub fn add_model(&mut self, model: ResearchModel) {
        self.models.push(model);
    }

    pub fn get_models(&self) -> &[ResearchModel] {
        &self.models
    }
}

impl Default for ResearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ResearchManager {
    engine: ResearchEngine,
}

impl ResearchManager {
    pub fn new() -> Self {
        Self {
            engine: ResearchEngine::new(),
        }
    }

    pub fn add_model(&mut self, model: ResearchModel) {
        self.engine.add_model(model);
    }

    pub fn get_models(&self) -> &[ResearchModel] {
        self.engine.get_models()
    }
}
