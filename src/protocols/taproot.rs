#![feature(edition2021)]
impl TaprootEngine {
    /// [BIP-341] Compliant Transaction Construction
    pub fn build_taproot_transaction(
        &self,
        inputs: Vec<TaprootInput>,
        outputs: Vec<TxOut>,
        fee_rate: FeeRate
    ) -> Result<Transaction> {
        let mut builder = self.network.taproot_builder();
        
        for input in inputs {
            builder = builder
                .add_leaf(input.leaf_version, &input.script)?
                .finalize(&self.secp, input.internal_key)?;
        }
        
        let psbt = Psbt::from_taproot_builder(builder)
            .set_fee_rate(fee_rate)?
            .finalize()?;
            
        psbt.extract_tx()
    }
} 