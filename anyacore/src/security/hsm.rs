#![feature(edition2021)]
// [AIS-3] HSM audit logging
pub fn log_audit_trail(event: &str) -> Result<()> {
    hsm::execute_command(&format!("AUDIT:{}", event))?;
    Ok(())
} 