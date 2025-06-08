/// Sectional Test Utilities
/// 
/// Provides utilities for running specific sections of tests in isolation

use std::collections::HashMap;

/// Test section configuration
#[derive(Debug, Clone)]
pub struct TestSection {
    pub name: String,
    pub enabled: bool,
    pub timeout_seconds: u64,
    pub dependencies: Vec<String>,
}

impl Default for TestSection {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            enabled: true,
            timeout_seconds: 300, // 5 minutes
            dependencies: Vec::new(),
        }
    }
}

/// Test section runner
pub struct SectionRunner {
    sections: HashMap<String, TestSection>,
}

impl SectionRunner {
    pub fn new() -> Self {
        Self {
            sections: HashMap::new(),
        }
    }
    
    pub fn add_section(&mut self, section: TestSection) {
        self.sections.insert(section.name.clone(), section);
    }
    
    pub fn enable_section(&mut self, name: &str) {
        if let Some(section) = self.sections.get_mut(name) {
            section.enabled = true;
        }
    }
    
    pub fn disable_section(&mut self, name: &str) {
        if let Some(section) = self.sections.get_mut(name) {
            section.enabled = false;
        }
    }
    
    pub fn run_section(&self, name: &str) -> Result<(), String> {
        if let Some(section) = self.sections.get(name) {
            if !section.enabled {
                return Err(format!("Section '{}' is disabled", name));
            }
            
            // Check dependencies
            for dep in &section.dependencies {
                if !self.sections.contains_key(dep) {
                    return Err(format!("Missing dependency '{}' for section '{}'", dep, name));
                }
            }
            
            println!("Running test section: {}", section.name);
            // Actual test execution would go here
            Ok(())
        } else {
            Err(format!("Section '{}' not found", name))
        }
    }
    
    pub fn list_sections(&self) -> Vec<&str> {
        self.sections.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for SectionRunner {
    fn default() -> Self {
        Self::new()
    }
}