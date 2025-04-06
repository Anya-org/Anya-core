use anyhow::Result;
use bitcoin::secp256k1::PublicKey;
use std::sync::RwLock;

pub struct TokenProtocol {
    current_supply: RwLock<u64>,
    balances: RwLock<HashMap<PublicKey, u64>>,
    params: TokenomicsParams,
}

impl TokenProtocol {
    pub fn new() -> Result<Self> {
        Ok(Self {
            current_supply: RwLock::new(0),
            balances: RwLock::new(HashMap::new()),
            params: TokenomicsParams::default(),
        })
    }

    pub fn get_block_reward(&self, height: u32) -> u64 {
        let halvings = height / self.params.halving_interval;
        if halvings >= 64 { return 0; }
        
        // Bitcoin-style halving
        (self.params.initial_block_reward >> halvings) 
    }

    pub fn distribute_block_reward(&self, height: u32) -> Result<()> {
        let reward = self.get_block_reward(height);
        
        // Distribute according to allocation
        let dex_amount = (reward as f64 * self.params.dex_allocation) as u64;
        let dao_amount = (reward as f64 * self.params.dao_allocation) as u64;
        let dev_amount = (reward as f64 * self.params.dev_allocation) as u64;

        self.mint_to_dex(dex_amount)?;
        self.mint_to_dao(dao_amount)?;
        self.mint_to_dev(dev_amount)?;

        Ok(())
    }
}
