#![feature(edition2021)]
#[derive(Clone, Debug)]
pub struct TaprootCompliance {
    pub version: u8,
    pub internal_key: PublicKey,
    pub merkle_root: Option<TapLeafHash>,
}

impl TaprootCompliance {
    pub fn verify_tapscript(&self, script: &Script) -> Result<(), MCPError> {
        let leaf_version = self.version | 0x50;
        let tap_leaf = TapLeaf::from_script(script, leaf_version)?;
        // ... existing merkle proof verification ...
        
        // NEW: Miniscript policy enforcement
        miniscript::policy::check_basic_consensus(tap_leaf.as_script())?;
    }
} 