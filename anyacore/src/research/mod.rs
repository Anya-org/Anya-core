#[derive(Debug, BPC3)]
pub enum CodeTier {
    #[ais3]
    Core,
    #[aip3]
    Project,
    #[air3]
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