// Define the missing ValidationLevel enum
#[derive(Debug, Clone, Copy)]
pub enum ValidationLevel {
    Strict,
    Moderate,
    None,
}

// Removed custom attributes that were causing build issues
#[derive(Debug)]
pub enum CodeTier {
    // Security critical (AIS-3)
    Core,
    // Production ready (AIP-3)
    Project,
    // Research code (AIR-3)
    Experimental,
}

impl CodeTier {
    pub fn validation_rules(&self) -> ValidationLevel {
        match self {
            Self::Core => ValidationLevel::Strict,
            Self::Project => ValidationLevel::Moderate,
            Self::Experimental => ValidationLevel::None,
        }
    }
} 