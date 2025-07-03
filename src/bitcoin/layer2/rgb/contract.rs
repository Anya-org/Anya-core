// RGB Contract implementation
// This file provides contract types for RGB assets

/// RGB Contract Type
#[derive(Debug, Clone)]
pub enum ContractType {
    Asset,
    Collectible,
    Identity,
    Custom(String),
}

/// RGB Contract
#[derive(Debug, Clone)]
pub struct Contract {
    pub id: String,
    pub contract_type: ContractType,
    pub script: String,
    pub witnesses: Vec<Witness>,
}

/// RGB Contract Builder
#[derive(Debug, Default)]
pub struct ContractBuilder {
    contract_type: Option<ContractType>,
    script: Option<String>,
    witnesses: Vec<Witness>,
}

/// RGB Witness
#[derive(Debug, Clone)]
pub struct Witness {
    pub public_key: String,
    pub signature: String,
}

impl Contract {
    /// Create a new contract
    pub fn new(id: &str, contract_type: ContractType, script: &str) -> Self {
        Self {
            id: id.to_string(),
            contract_type,
            script: script.to_string(),
            witnesses: Vec::new(),
        }
    }
    
    /// Add witness to contract
    pub fn add_witness(&mut self, witness: Witness) {
        self.witnesses.push(witness);
    }
    
    /// Verify contract
    pub fn verify(&self) -> bool {
        // Simple verification: must have at least one witness
        !self.witnesses.is_empty()
    }
}

impl ContractBuilder {
    /// Create a new contract builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set contract type
    pub fn contract_type(mut self, contract_type: ContractType) -> Self {
        self.contract_type = Some(contract_type);
        self
    }
    
    /// Set contract script
    pub fn script(mut self, script: &str) -> Self {
        self.script = Some(script.to_string());
        self
    }
    
    /// Add witness
    pub fn witness(mut self, witness: Witness) -> Self {
        self.witnesses.push(witness);
        self
    }
    
    /// Build contract
    pub fn build(self) -> Result<Contract, &'static str> {
        let contract_type = self.contract_type.ok_or("Contract type is required")?;
        let script = self.script.ok_or("Script is required")?;
        
        // Generate a random ID for now
        let id = format!("rgb:{:x}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs());
            
        let mut contract = Contract::new(&id, contract_type, &script);
        
        for witness in self.witnesses {
            contract.add_witness(witness);
        }
        
        Ok(contract)
    }
}
