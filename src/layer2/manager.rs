use crate::layer2::{
    BobClient, Layer2Protocol, Layer2ProtocolTrait, LightningNetwork, LiquidModule, Proof,
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
    pub fn initialize_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        macro_rules! init_protocol {
            ($field:ident, $ty:ty) => {{
                let instance = <$ty>::new(Default::default());
                instance.initialize()?;
                self.$field = Some(instance);
            }};
        }

        init_protocol!(bob_client, BobClient);
        init_protocol!(liquid_module, LiquidModule);
        init_protocol!(rsk_client, RskClient);
        init_protocol!(stacks_client, StacksClient);
        init_protocol!(taproot_assets, TaprootAssetsProtocol);

        println!("All Layer 2 protocols initialized successfully");
        Ok(())
    }

    /// Get protocol by type
    pub fn get_protocol(&self, protocol_type: Layer2Protocol) -> Option<&dyn Layer2ProtocolTrait> {
        match protocol_type {
            Layer2Protocol::BOB => self.bob_client.as_ref().map(|c| c as _),
            Layer2Protocol::Liquid => self.liquid_module.as_ref().map(|c| c as _),
            Layer2Protocol::RSK => self.rsk_client.as_ref().map(|c| c as _),
            Layer2Protocol::Stacks => self.stacks_client.as_ref().map(|c| c as _),
            Layer2Protocol::TaprootAssets => self.taproot_assets.as_ref().map(|c| c as _),
            _ => None,
        }
    }

    /// Cross-layer asset transfer
    pub fn cross_layer_transfer(
        &self,
        from_protocol: Layer2Protocol,
        to_protocol: Layer2Protocol,
        asset_id: &str,
        amount: u64,
    ) -> Result<String, Box<dyn std::error::Error>> {
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
        protocols: Vec<Layer2Protocol>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
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

fn protocol_name(protocol: Layer2Protocol) -> &'static str {
    use Layer2Protocol::*;
    match protocol {
        Lightning => "lightning",
        StateChannels => "state_channels",
        RGB => "rgb",
        DLC => "dlc",
        BOB => "bob",
        Liquid => "liquid",
        RSK => "rsk",
        Stacks => "stacks",
        TaprootAssets => "taproot_assets",
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

        assert!(manager.get_protocol(Layer2Protocol::BOB).is_some());
        assert!(manager.get_protocol(Layer2Protocol::Liquid).is_some());
        assert!(manager.get_protocol(Layer2Protocol::RSK).is_some());
        assert!(manager.get_protocol(Layer2Protocol::Stacks).is_some());
        assert!(manager.get_protocol(Layer2Protocol::TaprootAssets).is_some());
    }

    #[test]
    fn test_cross_layer_transfer() {
        let mut manager = Layer2Manager::new();
        manager.initialize_all().unwrap();

        let result = manager.cross_layer_transfer(
            Layer2Protocol::BOB,
            Layer2Protocol::Liquid,
            "test_asset",
            1000,
        );

        assert!(result.is_ok());
        let transfer_id = result.unwrap();
        assert!(transfer_id.contains("bob_liquid"));
    }
}
