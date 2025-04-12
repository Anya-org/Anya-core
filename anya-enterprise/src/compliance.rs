pub struct ComplianceEngine {
    checks: Vec<ComplianceCheck>,
    hsm_validator: HsmValidator,
}

impl ComplianceEngine {
    pub fn new() -> Self {
        Self {
            checks: Vec::new(),
            hsm_validator: HsmValidator::new(FIDO2::v2_5()),
        }
    }

    pub fn check(&mut self, check: ComplianceCheck) -> &mut Self {
        self.checks.push(check);
        self
    }

    pub fn validate_hsm(&mut self) -> &mut Self {
        let status = self.hsm_validator.validate();
        self.check(ComplianceCheck::Hsm(status))
    }
}