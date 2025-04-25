use std::error::Error;
impl LabelValidator {
    pub fn validate_component(&self, component: &Component) -> Result<()> {
        let required_labels = match component.category {
            ComponentCategory::Consensus => vec!["BPC-3", "RES-3"],
            ComponentCategory::Network => vec!["AIS-3", "SCL-3", "BPC-3"],
            ComponentCategory::SmartContract => vec!["AIT-3", "BPC-3"],
            ComponentCategory::CrossChain => vec!["RES-3", "SCL-3"],
        };
        
        if !component.labels.contains_all(&required_labels) {
            return Err(LabelError::MissingRequirements(required_labels));
        }
        
        Ok(())
    }
} 
