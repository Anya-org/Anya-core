# Multi-Factor Authentication (MFA)

Comprehensive multi-factor authentication implementation for enhanced security.

## Overview

Multi-Factor Authentication (MFA) adds an additional layer of security by requiring users to provide multiple forms of verification before accessing sensitive resources.

## Supported Authentication Factors

### 1. Something You Know (Knowledge)

- **Passwords**: Strong password requirements
- **PINs**: Numeric personal identification numbers
- **Security Questions**: Customizable security questions

### 2. Something You Have (Possession)

- **TOTP Tokens**: Time-based One-Time Passwords
- **Hardware Tokens**: Physical security keys (FIDO2/WebAuthn)
- **SMS Tokens**: SMS-based verification codes
- **Mobile Apps**: Authenticator mobile applications

### 3. Something You Are (Inherence)

- **Biometric Authentication**: Fingerprint, facial recognition
- **Hardware Security Modules**: HSM-based authentication

## Implementation

### TOTP Authentication

```rust
use totp_rs::{Algorithm, TOTP, Secret};
use qrcode::QrCode;

pub struct MFAManager {
    secret_store: SecretStore,
}

impl MFAManager {
    pub fn generate_totp_secret(&self, user_id: &str) -> Result<TOTPSecret, MFAError> {
        let secret = Secret::generate_secret();
        let totp = TOTP::new(
            Algorithm::SHA1,
            6, // digits
            1, // skew
            30, // step (seconds)
            secret.to_bytes().unwrap(),
        )?;
        
        // Store secret securely
        self.secret_store.store(user_id, &secret)?;
        
        Ok(TOTPSecret {
            secret: secret.to_encoded(),
            qr_code: self.generate_qr_code(&totp, user_id)?,
            backup_codes: self.generate_backup_codes(user_id)?,
        })
    }
    
    pub fn verify_totp(&self, user_id: &str, token: &str) -> Result<bool, MFAError> {
        let secret = self.secret_store.get(user_id)?;
        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            secret.to_bytes().unwrap(),
        )?;
        
        Ok(totp.check_current(token)?)
    }
    
    fn generate_qr_code(&self, totp: &TOTP, user_id: &str) -> Result<String, MFAError> {
        let url = totp.get_url("Anya Core", user_id);
        let qr = QrCode::new(&url)?;
        Ok(qr.to_string(false, 3))
    }
}
```

### Hardware Security Keys

```rust
use webauthn_rs::prelude::*;

pub struct WebAuthnMFA {
    webauthn: Webauthn,
}

impl WebAuthnMFA {
    pub fn new() -> Self {
        let rp_id = "anya-core.org";
        let rp_origin = Url::parse("https://anya-core.org").unwrap();
        let webauthn = WebauthnBuilder::new(rp_id, &rp_origin)
            .unwrap()
            .build()
            .unwrap();
        
        Self { webauthn }
    }
    
    pub fn start_registration(&self, user_id: &str) -> Result<(CreationChallengeResponse, PasskeyRegistration), WebauthnError> {
        let user_unique_id = Uuid::new_v4();
        let (ccr, reg_state) = self.webauthn.start_passkey_registration(
            user_unique_id,
            user_id,
            user_id,
            None,
        )?;
        
        Ok((ccr, reg_state))
    }
    
    pub fn finish_registration(
        &self,
        reg: &RegisterPublicKeyCredential,
        reg_state: &PasskeyRegistration,
    ) -> Result<Passkey, WebauthnError> {
        self.webauthn.finish_passkey_registration(reg, reg_state)
    }
}
```

## Configuration

### MFA Policy Configuration

```yaml
mfa:
  enabled: true
  required_for:
    - admin_access
    - financial_operations
    - sensitive_data_access
  
  methods:
    totp:
      enabled: true
      issuer: "Anya Core"
      backup_codes: 10
    
    webauthn:
      enabled: true
      user_verification: "preferred"
      authenticator_attachment: "cross-platform"
    
    sms:
      enabled: false  # Not recommended for production
      provider: "twilio"
    
  policies:
    grace_period: 7200  # seconds
    max_attempts: 3
    lockout_duration: 900  # seconds
    remember_device: true
    remember_duration: 2592000  # 30 days
```

### User Experience

```typescript
interface MFASetupFlow {
  userId: string;
  method: 'totp' | 'webauthn' | 'sms';
  step: 'init' | 'verify' | 'complete';
}

class MFASetupComponent extends React.Component<MFASetupFlow> {
  async setupTOTP() {
    const response = await fetch('/api/mfa/totp/setup', {
      method: 'POST',
      headers: { 'Authorization': `Bearer ${this.token}` }
    });
    
    const { secret, qrCode, backupCodes } = await response.json();
    
    this.setState({
      qrCode,
      backupCodes,
      step: 'verify'
    });
  }
  
  async verifyTOTP(token: string) {
    const response = await fetch('/api/mfa/totp/verify', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ token })
    });
    
    if (response.ok) {
      this.setState({ step: 'complete' });
    }
  }
}
```

## API Endpoints

### Setup Endpoints

```bash
# Initialize TOTP setup
POST /api/v1/mfa/totp/setup
Authorization: Bearer <token>

Response:
{
  "secret": "JBSWY3DPEHPK3PXP",
  "qr_code": "data:image/png;base64,...",
  "backup_codes": ["12345678", "87654321", ...]
}

# Verify TOTP setup
POST /api/v1/mfa/totp/verify
Content-Type: application/json

{
  "token": "123456"
}
```

### Authentication Endpoints

```bash
# Authenticate with MFA
POST /api/v1/auth/mfa/authenticate
Content-Type: application/json

{
  "user_id": "user123",
  "method": "totp",
  "token": "123456"
}

Response:
{
  "success": true,
  "session_token": "...",
  "expires_at": "2025-06-17T12:00:00Z"
}
```

## Security Considerations

### Best Practices

1. **Secret Storage**: Store TOTP secrets encrypted at rest
2. **Time Synchronization**: Ensure server time is synchronized
3. **Rate Limiting**: Implement rate limiting for MFA attempts
4. **Backup Codes**: Provide secure backup authentication methods
5. **Audit Logging**: Log all MFA events for security monitoring

### Threat Mitigation

```rust
pub struct MFASecurityControls {
    rate_limiter: RateLimiter,
    attempt_tracker: AttemptTracker,
    audit_logger: AuditLogger,
}

impl MFASecurityControls {
    pub async fn validate_attempt(&self, user_id: &str, ip: &str) -> Result<(), SecurityError> {
        // Rate limiting
        if !self.rate_limiter.check_rate(ip, Duration::from_secs(60), 5) {
            return Err(SecurityError::RateLimited);
        }
        
        // Check for suspicious patterns
        if self.attempt_tracker.is_suspicious(user_id, ip) {
            self.audit_logger.log_suspicious_activity(user_id, ip).await;
            return Err(SecurityError::SuspiciousActivity);
        }
        
        Ok(())
    }
}
```

## Integration Examples

### Express.js Middleware

```javascript
const mfaMiddleware = async (req, res, next) => {
  const { user, mfaToken } = req.body;
  
  try {
    const isValid = await mfaService.verify(user.id, mfaToken);
    if (!isValid) {
      return res.status(401).json({ error: 'Invalid MFA token' });
    }
    
    req.mfaVerified = true;
    next();
  } catch (error) {
    res.status(500).json({ error: 'MFA verification failed' });
  }
};

app.post('/api/sensitive-operation', mfaMiddleware, (req, res) => {
  // Perform sensitive operation
});
```

### Database Schema

```sql
CREATE TABLE mfa_secrets (
    user_id VARCHAR(255) PRIMARY KEY,
    secret_encrypted BLOB NOT NULL,
    method VARCHAR(50) NOT NULL,
    backup_codes JSON,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_used TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE
);

CREATE TABLE mfa_attempts (
    id UUID PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL,
    ip_address INET,
    method VARCHAR(50),
    success BOOLEAN,
    attempted_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_user_attempts (user_id, attempted_at),
    INDEX idx_ip_attempts (ip_address, attempted_at)
);
```

## Monitoring and Analytics

### MFA Metrics

- **Adoption Rate**: Percentage of users with MFA enabled
- **Success Rate**: Successful MFA authentications
- **Method Distribution**: Usage of different MFA methods
- **Failed Attempts**: Failed authentication patterns

### Dashboard Integration

```typescript
interface MFAMetrics {
  totalUsers: number;
  mfaEnabledUsers: number;
  successRate: number;
  methodDistribution: {
    totp: number;
    webauthn: number;
    sms: number;
  };
}

export const MFADashboard: React.FC = () => {
  const [metrics, setMetrics] = useState<MFAMetrics>();
  
  useEffect(() => {
    fetchMFAMetrics().then(setMetrics);
  }, []);
  
  return (
    <div className="mfa-dashboard">
      <MetricCard title="MFA Adoption" value={`${metrics?.adoptionRate}%`} />
      <MetricCard title="Success Rate" value={`${metrics?.successRate}%`} />
      <MethodDistributionChart data={metrics?.methodDistribution} />
    </div>
  );
};
```

## See Also

- [Security Features Overview](../README.md)
- [Authentication Guide](authorization.md)
- [Session Management](session-management.md)
- [Security Monitoring](security-monitoring.md)

---

*This documentation is part of the Anya Enterprise Security suite.*
