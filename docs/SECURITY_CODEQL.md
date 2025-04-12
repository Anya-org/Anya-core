// Mobile-specific security rules
import bitcoin.security-rules
import mobile.security.bitcoin
import mobile.security.hsm

// HSM Interface Validation
rule MobileHSMValidation {
  description: "Validate HSM interface standardization"
  severity: Warning
  override: "HSM 2.5 Standard"
  pattern: $HSM.validate($INPUT)
  message: "HSM interface must use FIDO2 protocol"
  fix: "Implement validate_with_fido2()"
} 