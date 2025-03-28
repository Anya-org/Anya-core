#![feature(edition2021)]
#[test]
fn verify_schnorr_implementation() {
    let secp = Secp256k1::new();
    let mut rng = rand::thread_rng();
    let key_pair = KeyPair::new(&secp, &mut rng);
    let (xonly, _) = XOnlyPublicKey::from_keypair(&key_pair);
    
    let msg = Message::from_hashed_data::<sha256::Hash>(b"test");
    let sig = secp.sign_schnorr(&msg, &key_pair);
    
    assert!(secp.verify_schnorr(&sig, &msg, &xonly).is_ok(),
        "Schnorr signature verification failed");
} 