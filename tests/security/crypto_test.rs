#[test]
fn verify_schnorr_implementation() {
    use rand::thread_rng;
    use secp256k1::hashes::sha256;
    use secp256k1::{Keypair, Message, Secp256k1, XOnlyPublicKey};

    let secp = Secp256k1::new();
    let mut rng = thread_rng();
    let key_pair = Keypair::new(&secp, &mut rng);
    let (xonly, _) = XOnlyPublicKey::from_keypair(&key_pair);

    let msg = Message::from_hashed_data::<sha256::Hash>(b"test");
    let sig = secp.sign_schnorr(&msg, &key_pair);

    assert!(
        secp.verify_schnorr(&sig, &msg, &xonly).is_ok(),
        "Schnorr signature verification failed"
    );
}
