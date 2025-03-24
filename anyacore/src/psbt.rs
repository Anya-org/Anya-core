#[bip370]
pub fn validate_fee_rate(psbt: &Psbt) -> Result<()> {
    let expected_fee = psbt.fee_rate() * psbt.vsize();
    ensure!(
        psbt.fee() >= expected_fee,
        "Fee below minimum rate: {} < {}",
        psbt.fee(),
        expected_fee
    );
    Ok(())
} 