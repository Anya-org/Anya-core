// Schnorr signature module (BIP-340)
// Bitcoin Development Framework v2.5

use super::SCHNORR_SIGNATURE_SIZE;
use secp256k1::hashes::{sha256, Hash};
use secp256k1::rand::rngs::OsRng;
use secp256k1::{schnorr::Signature, KeyPair, Message, PublicKey, Secp256k1, SecretKey};

/// Verifies a Schnorr signature according to BIP-340
///
/// # Arguments
/// * `message` - The message that was signed
/// * `signature` - The Schnorr signature (64 bytes)
/// * `public_key` - The public key (32 bytes)
///
/// # Returns
/// `true` if the signature is valid, `false` otherwise
pub fn verify_signature(message: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
    // Check signature size
    if signature.len() != SCHNORR_SIGNATURE_SIZE {
        return false;
    }

    // Check public key size
    if public_key.len() != 32 {
        return false;
    }

    let secp = Secp256k1::verification_only();

    // Parse the signature
    let sig = match Signature::from_slice(signature) {
        Ok(s) => s,
        Err(_) => return false,
    };

    // Parse the public key as x-only public key
    let xonly_pk = match secp256k1::XOnlyPublicKey::from_slice(public_key) {
        Ok(pk) => pk,
        Err(_) => return false,
    };

    // Hash the message using SHA256
    let msg_hash = sha256::Hash::hash(message);
    let msg = Message::from_digest_slice(&msg_hash[..]).expect("Hash is always valid");

    // Verify the signature
    secp.verify_schnorr(&sig, &msg, &xonly_pk).is_ok()
}

/// Signs a message using Schnorr signature algorithm (BIP-340)
///
/// # Arguments
/// * `message` - The message to sign
/// * `private_key` - The private key (32 bytes)
///
/// # Returns
/// The Schnorr signature (64 bytes) or None if signing fails
pub fn sign_message(message: &[u8], private_key: &[u8]) -> Option<Vec<u8>> {
    // Check private key size
    if private_key.len() != 32 {
        return None;
    }

    let secp = Secp256k1::signing_only();

    // Parse the private key
    let secret_key = match SecretKey::from_slice(private_key) {
        Ok(sk) => sk,
        Err(_) => return None,
    };

    // Create a key pair
    let key_pair = KeyPair::from_secret_key(&secp, &secret_key);

    // Hash the message using SHA256
    let msg_hash = sha256::Hash::hash(message);
    let msg = Message::from_digest_slice(&msg_hash[..]).expect("Hash is always valid");

    // Sign the message
    let signature = secp.sign_schnorr(&msg, &key_pair);

    Some(signature.as_ref().to_vec())
}

/// Batch verifies multiple Schnorr signatures
///
/// # Arguments
/// * `messages` - The messages that were signed
/// * `signatures` - The Schnorr signatures
/// * `public_keys` - The public keys
///
/// # Returns
/// `true` if all signatures are valid, `false` otherwise
pub fn batch_verify(messages: &[&[u8]], signatures: &[&[u8]], public_keys: &[&[u8]]) -> bool {
    // Check that we have the same number of messages, signatures and public keys
    if messages.len() != signatures.len() || messages.len() != public_keys.len() {
        return false;
    }

    // Verify each signature individually
    for i in 0..messages.len() {
        if !verify_signature(messages[i], signatures[i], public_keys[i]) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use secp256k1::rand::rngs::OsRng;
    use secp256k1::{KeyPair, Secp256k1, SecretKey};

    #[test]
    fn test_schnorr_signature() {
        let message = b"Test message";

        // Generate a real private key
        let secp = Secp256k1::new();
        let secret_key = SecretKey::new(&mut OsRng);
        let key_pair = KeyPair::from_secret_key(&secp, &secret_key);
        let (xonly_pk, _) = key_pair.x_only_public_key();

        // Sign the message
        let signature = sign_message(message, &secret_key.secret_bytes()).unwrap();

        // Verify the signature
        assert!(verify_signature(message, &signature, &xonly_pk.serialize()));

        // Test with wrong message
        assert!(!verify_signature(
            b"Wrong message",
            &signature,
            &xonly_pk.serialize()
        ));
    }

    #[test]
    fn test_batch_verification() {
        let message1 = b"Test message 1";
        let message2 = b"Test message 2";

        let secp = Secp256k1::new();

        // Generate real private keys
        let secret_key1 = SecretKey::new(&mut OsRng);
        let secret_key2 = SecretKey::new(&mut OsRng);

        let key_pair1 = KeyPair::from_secret_key(&secp, &secret_key1);
        let key_pair2 = KeyPair::from_secret_key(&secp, &secret_key2);

        let (xonly_pk1, _) = key_pair1.x_only_public_key();
        let (xonly_pk2, _) = key_pair2.x_only_public_key();

        // Sign the messages
        let signature1 = sign_message(message1, &secret_key1.secret_bytes()).unwrap();
        let signature2 = sign_message(message2, &secret_key2.secret_bytes()).unwrap();

        // Batch verify the signatures
        assert!(batch_verify(
            &[message1, message2],
            &[&signature1, &signature2],
            &[&xonly_pk1.serialize(), &xonly_pk2.serialize()]
        ));

        // Test with wrong signature
        assert!(!batch_verify(
            &[message1, message2],
            &[&signature2, &signature1], // Swapped signatures
            &[&xonly_pk1.serialize(), &xonly_pk2.serialize()]
        ));
    }
}
