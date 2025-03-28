#![feature(edition2021)]
/// Updated Tapscript validation per BIP-342 final spec
fn verify_tapscript(
    script: &Script,
    tx: &Transaction,
    input_index: usize,
    value: Amount,
    sig_version: SigVersion,
) -> Result<(), Error> {
    // Add BIP-342 test vectors validation
    let test_vectors = Bip342Vectors::load()?;
    test_vectors.execute_all()?;

    // Enhanced signature validation
    let ctx = TaprootScriptContext::new(tx, input_index, value)?;
    Interpreter::verify_taproot_script(script, &ctx, sig_version)
        .context("BIP-342 validation failed")?;

    // Add audit trail
    audit_log!("BIP342", "Tapscript validated for tx {}", tx.txid());
    Ok(())
} 