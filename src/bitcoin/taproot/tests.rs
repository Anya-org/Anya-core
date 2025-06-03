use super::*;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::key::Keypair;
use bitcoin::taproot::TaprootBuilder;

#[test]
fn test_taproot_builder() -> Result<(), Box<dyn std::error::Error>> {
    // Test basic Taproot builder functionality
    let secp = Secp256k1::new();
    let keypair = KeyPair::new(&secp, &mut rand::thread_rng());
    let builder = TaprootBuilder::new();
    
    // Test that we can create a builder
    assert_eq!(builder.num_leaves(), 0, "New builder should have 0 leaves");
    
    Ok(())
}
