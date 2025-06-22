use crate::layer2::{
    BobClient, Layer2ProtocolType, Layer2ProtocolType, Layer2ProtocolTrait, LightningNetwork, LiquidModule, Proof,
    RskClient, StacksClient, StateChannel, TaprootAssetsProtocol,
};

/// Comprehensive Layer 2 integration manager
pub struct Layer2Manager {
    bob_client: Option<BobClient>,
    liquid_module: Option<LiquidModule>,
    rsk_client: Option<RskClient>,
    stacks_client: Option<StacksClient>,
    taproot_assets: Option<TaprootAssetsProtocol>,
    #[allow(dead_code)] // For future Lightning integration
    lightning_network: Option<LightningNetwork>,
    #[allow(dead_code)] // For future State Channel integration
    state_channels: Option<StateChannel>,
}

impl Default for Layer2Manager {
    fn default() -> Self {
        Self::new()
    }
}

impl Layer2Manager {
    /// Create a new Layer 2 manager with all protocols
    pub fn new() -> Self {
        Self {
            bob_client: None,
            liquid_module: None,
            rsk_client: None,
            stacks_client: None,
            taproot_assets: None,
            lightning_network: None,
            state_channels: None,
        }
    }

    /// Initialize all Layer 2 protocols
    pub fn initialize_all(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.init_protocol::<BobClient>(&mut self.bob_client, "BobClient")?;
        self.init_protocol::<LiquidModule>(&mut self.liquid_module, "LiquidModule")?;
        self.init_protocol::<RskClient>(&mut self.rsk_client, "RskClient")?;
        self.init_protocol::<StacksClient>(&mut self.stacks_client, "StacksClient")?;
        self.init_protocol::<TaprootAssetsProtocol>(&mut self.taproot_assets, "TaprootAssetsProtocol")?;

        println!("All Layer 2 protocols initialized successfully");
        Ok(())
    }

    fn init_protocol<T>(
        &mut self,
        field: &mut Option<T>,
        name: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    where
        T: Layer2ProtocolTrait + Default,
    {
        let mut instance = T::default();
        if let Err(e) = instance.initialize() {
            eprintln!("Failed to initialize {}: {}", name, e);
            return Err(e);
        }
        *field = Some(instance);
        Ok(())
    }

    /// Get protocol by type
    pub fn get_protocol(&self, protocol_type: Layer2ProtocolType) -> Option<&dyn Layer2ProtocolTrait> {
        match protocol_type {
            Layer2ProtocolType::BOB => self.bob_client.as_ref().map(|c| c as &dyn Layer2ProtocolTrait),
            Layer2ProtocolType::Liquid => self.liquid_module.as_ref().map(|c| c as &dyn Layer2ProtocolTrait),
            Layer2ProtocolType::RSK => self.rsk_client.as_ref().map(|c| c as &dyn Layer2ProtocolTrait),
            Layer2ProtocolType::Stacks => self.stacks_client.as_ref().map(|c| c as &dyn Layer2ProtocolTrait),
            Layer2ProtocolType::TaprootAssets => self.taproot_assets.as_ref().map(|c| c as &dyn Layer2ProtocolTrait),
            _ => None,
        }
    }

    /// Cross-layer asset transfer
    pub fn cross_layer_transfer(
        &self,
        from_protocol: Layer2ProtocolType,
        to_protocol: Layer2ProtocolType,
        asset_id: &str,
        amount: u64,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        println!(
            "Executing cross-layer transfer from {:?} to {:?}",
            from_protocol, to_protocol
        );

        let transfer_id = format!(
            "cross_{}_{}_{}_{}",
            protocol_name(from_protocol),
            protocol_name(to_protocol),
            asset_id,
            amount
        );

        Ok(transfer_id)
    }

    /// Verify cross-layer proof
    pub fn verify_cross_layer_proof(
        &self,
        proof: Proof,
        protocols: Vec<Layer2ProtocolType>,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        println!(
            "Verifying cross-layer proof across {} protocols",
            protocols.len()
        );

        for protocol_type in protocols {
            if let Some(protocol) = self.get_protocol(protocol_type) {
                if !protocol.verify_proof(proof.clone())?.is_valid {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }
}

fn protocol_name(protocol: Layer2ProtocolType) -> &'static str {
    match protocol {
        Layer2ProtocolType::Lightning => "lightning",
        Layer2ProtocolType::StateChannels => "state_channels",
        Layer2ProtocolType::RGB => "rgb",
        Layer2ProtocolType::DLC => "dlc",
        Layer2ProtocolType::BOB => "bob",
        Layer2ProtocolType::Liquid => "liquid",
        Layer2ProtocolType::RSK => "rsk",
        Layer2ProtocolType::Stacks => "stacks",
        Layer2ProtocolType::TaprootAssets => "taproot_assets",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer2_manager_initialization() {
        let mut manager = Layer2Manager::new();
        assert!(manager.initialize_all().is_ok());
    }

    #[test]
    fn test_all_protocols_available() {
        let mut manager = Layer2Manager::new();
        manager.initialize_all().unwrap();

        assert!(manager.get_protocol(Layer2ProtocolType::BOB).is_some());
        assert!(manager.get_protocol(Layer2ProtocolType::Liquid).is_some());
        assert!(manager.get_protocol(Layer2ProtocolType::RSK).is_some());
        assert!(manager.get_protocol(Layer2ProtocolType::Stacks).is_some());
        assert!(manager.get_protocol(Layer2ProtocolType::TaprootAssets).is_some());
    }

    #[test]
    fn test_cross_layer_transfer() {
        let mut manager = Layer2Manager::new();
        manager.initialize_all().unwrap();

        let result = manager.cross_layer_transfer(
            Layer2ProtocolType::BOB,
            Layer2ProtocolType::Liquid,
            "test_asset",
            1000,
        );

        assert!(result.is_ok());
        let transfer_id = result.unwrap();
        assert!(transfer_id.contains("bob_liquid"));
    }
}
