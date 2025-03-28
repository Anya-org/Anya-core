pub fn validate_bip_check() -> Result<()> {
    let repo_path = Path::new("dependencies/bips");
    BipValidator::new()
        .check_commit("7c4b4c1d91b7d3dc6d7862ad5a8c1472332e6d84")?
        .verify_directory_structure()?
        .check_checksums();
    Ok(())
}
