#![feature(edition2021)]
pub fn enforce_mobile_compliance() -> Result<()> {
    let report = validate_cross_implementation()?;

    if report.bip341 != ComplianceStatus::Full {
        anyhow::bail!("Mobile BIP-341 compliance incomplete");
    }

    if report.psbt_v2 != ComplianceStatus::Full {
        anyhow::bail!("PSBT v2 support missing in mobile");
    }

    Ok(())
}
