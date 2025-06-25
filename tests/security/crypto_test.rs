#[test]
fn verify_schnorr_implementation() {
    use rand::thread_rng;
    use secp256k1::hashes::sha256;
    // Import the Hash trait to use the hash method on sha256::Hash
    use secp256k1::hashes::Hash;
    use secp256k1::{Keypair, Message, Secp256k1, XOnlyPublicKey};

    let secp = Secp256k1::new();
    let mut rng = thread_rng();
    let key_pair = Keypair::new(&secp, &mut rng);
    let (xonly, _) = XOnlyPublicKey::from_keypair(&key_pair);

    // Use sha256::Hash directly to hash the data
    let digest = sha256::Hash::hash(b"test");
    let msg = Message::from_digest_slice(digest.as_ref()).expect("32 bytes");
    let sig = secp.sign_schnorr(&msg, &key_pair);

    assert!(
        secp.verify_schnorr(&sig, &msg, &xonly).is_ok(),
        "Schnorr signature verification failed"
    );
}
