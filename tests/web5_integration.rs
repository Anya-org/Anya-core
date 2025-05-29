#[test]
fn test_web5_did_rotation() {
    let mut did = Web5Did::new();
    let old_key = did.public_key.clone();

    did.rotate_keys().expect("Key rotation failed");

    assert_ne!(did.public_key, old_key, "DID keys not rotated");
}

#[test]
fn test_web5_credential_revocation() {
    let vc = VerifiableCredential::new()
        .set_id("vc:1")
        .add_revocation_list("https://revoke.anya");

    let status = vc.check_revocation().expect("Revocation check failed");

    assert!(!status.revoked, "Credential should not be revoked");
}
