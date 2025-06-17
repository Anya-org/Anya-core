# Security Guidelines

Comprehensive security guidelines for developing, deploying, and operating Anya Core systems.

## Overview

This document provides essential security guidelines and best practices for all aspects of the Anya Core ecosystem, from development to production deployment.

## Development Security

### Secure Coding Practices

#### Input Validation

```rust
use validator::{Validate, ValidationError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Validate)]
pub struct WalletRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    
    #[validate(custom = "validate_bitcoin_address")]
    pub address: String,
    
    #[validate(range(min = 0.0001, max = 21000000.0))]
    pub amount: f64,
}

fn validate_bitcoin_address(address: &str) -> Result<(), ValidationError> {
    use bitcoin::Address;
    
    match address.parse::<Address>() {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("invalid_bitcoin_address")),
    }
}
```

#### Error Handling

```rust
// Secure error handling - avoid information leakage
pub fn handle_authentication_error(error: AuthError) -> ApiResponse {
    match error {
        AuthError::InvalidCredentials => {
            // Don't reveal which part of credentials was wrong
            ApiResponse::error("Authentication failed", 401)
        }
        AuthError::AccountLocked => {
            ApiResponse::error("Account temporarily unavailable", 423)
        }
        AuthError::Internal(details) => {
            // Log detailed error internally but return generic message
            log::error!("Internal auth error: {}", details);
            ApiResponse::error("Authentication service unavailable", 500)
        }
    }
}
```

### Cryptographic Security

#### Key Management

```rust
use aes_gcm::{Aes256Gcm, Key, Nonce};
use rand::{rngs::OsRng, RngCore};

pub struct SecureKeyManager {
    master_key: Key<Aes256Gcm>,
    rng: OsRng,
}

impl SecureKeyManager {
    pub fn new() -> Result<Self, CryptoError> {
        let mut key_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut key_bytes);
        
        Ok(Self {
            master_key: Key::<Aes256Gcm>::from_slice(&key_bytes).clone(),
            rng: OsRng,
        })
    }
    
    pub fn encrypt_sensitive_data(&mut self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let cipher = Aes256Gcm::new(&self.master_key);
        let mut nonce_bytes = [0u8; 12];
        self.rng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let ciphertext = cipher.encrypt(nonce, data)
            .map_err(|_| CryptoError::EncryptionFailed)?;
        
        // Prepend nonce to ciphertext
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }
}
```

#### Secure Random Number Generation

```typescript
import crypto from 'crypto';

class SecureRandom {
  static generateSessionId(): string {
    return crypto.randomBytes(32).toString('hex');
  }
  
  static generateSalt(): Buffer {
    return crypto.randomBytes(16);
  }
  
  static generateApiKey(): string {
    const bytes = crypto.randomBytes(32);
    return bytes.toString('base64url');
  }
  
  // For cryptographic operations - use cryptographically secure randomness
  static generateCryptoKey(length: number = 32): Buffer {
    return crypto.randomBytes(length);
  }
}
```

## Network Security

### TLS Configuration

```yaml
# nginx.conf - Secure TLS configuration
server {
    listen 443 ssl http2;
    server_name api.anya-core.org;
    
    # TLS Configuration
    ssl_certificate /etc/ssl/certs/anya-core.crt;
    ssl_certificate_key /etc/ssl/private/anya-core.key;
    
    # Strong SSL Security
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-RSA-AES256-GCM-SHA512:DHE-RSA-AES256-GCM-SHA512:ECDHE-RSA-AES256-GCM-SHA384;
    ssl_ecdh_curve secp384r1;
    ssl_prefer_server_ciphers on;
    ssl_session_cache shared:SSL:10m;
    ssl_session_tickets off;
    
    # HSTS
    add_header Strict-Transport-Security "max-age=63072000; includeSubDomains; preload";
    
    # Security Headers
    add_header X-Frame-Options DENY;
    add_header X-Content-Type-Options nosniff;
    add_header X-XSS-Protection "1; mode=block";
    add_header Referrer-Policy "strict-origin-when-cross-origin";
    add_header Content-Security-Policy "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline';";
}
```

### API Security

```typescript
import rateLimit from 'express-rate-limit';
import helmet from 'helmet';
import cors from 'cors';

// Rate limiting
const limiter = rateLimit({
  windowMs: 15 * 60 * 1000, // 15 minutes
  max: 100, // Limit each IP to 100 requests per windowMs
  message: 'Too many requests from this IP',
  standardHeaders: true,
  legacyHeaders: false,
});

// Security middleware
app.use(helmet({
  contentSecurityPolicy: {
    directives: {
      defaultSrc: ["'self'"],
      scriptSrc: ["'self'"],
      styleSrc: ["'self'", "'unsafe-inline'"],
      imgSrc: ["'self'", "data:", "https:"],
    },
  },
  hsts: {
    maxAge: 31536000,
    includeSubDomains: true,
    preload: true
  }
}));

// CORS configuration
app.use(cors({
  origin: process.env.ALLOWED_ORIGINS?.split(',') || ['https://app.anya-core.org'],
  credentials: true,
  methods: ['GET', 'POST', 'PUT', 'DELETE'],
  allowedHeaders: ['Content-Type', 'Authorization'],
}));

app.use('/api/', limiter);
```

## Infrastructure Security

### Container Security

```dockerfile
# Secure Dockerfile practices
FROM node:18-alpine AS builder

# Create non-root user
RUN addgroup -g 1001 -S nodejs
RUN adduser -S nextjs -u 1001

# Set working directory
WORKDIR /app

# Copy package files
COPY package*.json ./
RUN npm ci --only=production && npm cache clean --force

# Copy source code
COPY --chown=nextjs:nodejs . .

# Build application
RUN npm run build

# Production stage
FROM node:18-alpine AS runner
WORKDIR /app

ENV NODE_ENV production

# Create non-root user
RUN addgroup -g 1001 -S nodejs
RUN adduser -S nextjs -u 1001

# Copy built application
COPY --from=builder --chown=nextjs:nodejs /app/dist ./dist
COPY --from=builder --chown=nextjs:nodejs /app/node_modules ./node_modules
COPY --from=builder --chown=nextjs:nodejs /app/package.json ./package.json

# Switch to non-root user
USER nextjs

EXPOSE 3000

CMD ["npm", "start"]
```

### Kubernetes Security

```yaml
# Secure Kubernetes deployment
apiVersion: apps/v1
kind: Deployment
metadata:
  name: anya-core-api
spec:
  replicas: 3
  selector:
    matchLabels:
      app: anya-core-api
  template:
    metadata:
      labels:
        app: anya-core-api
    spec:
      serviceAccountName: anya-core-sa
      securityContext:
        runAsNonRoot: true
        runAsUser: 1001
        fsGroup: 1001
      containers:
      - name: api
        image: anya-core/api:latest
        securityContext:
          allowPrivilegeEscalation: false
          readOnlyRootFilesystem: true
          runAsNonRoot: true
          runAsUser: 1001
          capabilities:
            drop:
            - ALL
        resources:
          limits:
            memory: "512Mi"
            cpu: "500m"
          requests:
            memory: "256Mi"
            cpu: "250m"
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: database-secret
              key: url
        volumeMounts:
        - name: tmp
          mountPath: /tmp
        - name: cache
          mountPath: /app/cache
      volumes:
      - name: tmp
        emptyDir: {}
      - name: cache
        emptyDir: {}
```

## Data Security

### Database Security

```sql
-- Database security configuration
-- Enable SSL
ALTER SYSTEM SET ssl = on;
ALTER SYSTEM SET ssl_cert_file = '/etc/ssl/certs/server.crt';
ALTER SYSTEM SET ssl_key_file = '/etc/ssl/private/server.key';

-- Secure authentication
ALTER SYSTEM SET password_encryption = 'scram-sha-256';

-- Audit logging
ALTER SYSTEM SET log_statement = 'all';
ALTER SYSTEM SET log_connections = on;
ALTER SYSTEM SET log_disconnections = on;

-- Row-level security
ALTER TABLE wallets ENABLE ROW LEVEL SECURITY;

CREATE POLICY wallet_access_policy ON wallets
  FOR ALL TO authenticated_role
  USING (owner_id = current_user_id());
```

### Data Encryption

```rust
use sqlx::{Postgres, Row};
use aes_gcm::{Aes256Gcm, Key, Nonce};

pub struct EncryptedField<T> {
    encrypted_data: Vec<u8>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> EncryptedField<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    pub fn encrypt(value: &T, key: &Key<Aes256Gcm>) -> Result<Self, CryptoError> {
        let serialized = serde_json::to_vec(value)?;
        let encrypted_data = encrypt_data(&serialized, key)?;
        
        Ok(Self {
            encrypted_data,
            _phantom: std::marker::PhantomData,
        })
    }
    
    pub fn decrypt(&self, key: &Key<Aes256Gcm>) -> Result<T, CryptoError> {
        let decrypted_data = decrypt_data(&self.encrypted_data, key)?;
        let value = serde_json::from_slice(&decrypted_data)?;
        Ok(value)
    }
}
```

## Monitoring and Incident Response

### Security Monitoring

```typescript
interface SecurityEvent {
  timestamp: Date;
  event_type: 'authentication' | 'authorization' | 'data_access' | 'system';
  severity: 'low' | 'medium' | 'high' | 'critical';
  user_id?: string;
  ip_address: string;
  user_agent: string;
  details: Record<string, any>;
}

class SecurityMonitor {
  private alerts: AlertManager;
  private metrics: MetricsCollector;
  
  async logSecurityEvent(event: SecurityEvent): Promise<void> {
    // Store event
    await this.storeEvent(event);
    
    // Check for patterns
    if (await this.detectSuspiciousActivity(event)) {
      await this.alerts.sendAlert({
        type: 'suspicious_activity',
        severity: event.severity,
        details: event
      });
    }
    
    // Update metrics
    this.metrics.increment('security_events', {
      type: event.event_type,
      severity: event.severity
    });
  }
  
  private async detectSuspiciousActivity(event: SecurityEvent): Promise<boolean> {
    // Multiple failed login attempts
    if (event.event_type === 'authentication' && event.details.success === false) {
      const recentFailures = await this.countRecentFailures(event.ip_address, '5m');
      return recentFailures >= 5;
    }
    
    // Unusual access patterns
    if (event.event_type === 'data_access') {
      return this.isUnusualAccessPattern(event);
    }
    
    return false;
  }
}
```

### Incident Response

```yaml
# Incident response playbook
incident_response:
  severity_levels:
    critical:
      response_time: "15m"
      escalation: ["security_team", "cto", "ceo"]
      actions:
        - isolate_affected_systems
        - notify_stakeholders
        - activate_backup_systems
    
    high:
      response_time: "1h"
      escalation: ["security_team", "engineering_lead"]
      actions:
        - investigate_scope
        - implement_containment
        - document_incident
    
    medium:
      response_time: "4h"
      escalation: ["on_call_engineer"]
      actions:
        - analyze_logs
        - apply_mitigations
        - update_monitoring
    
    low:
      response_time: "24h"
      escalation: ["security_team"]
      actions:
        - review_and_document
        - update_procedures
```

## Compliance and Auditing

### SOC 2 Compliance

```typescript
interface SOC2Control {
  id: string;
  description: string;
  category: 'security' | 'availability' | 'processing_integrity' | 'confidentiality' | 'privacy';
  implemented: boolean;
  evidence: string[];
  last_review: Date;
}

class ComplianceManager {
  async evaluateSOC2Controls(): Promise<SOC2Report> {
    const controls = await this.getSOC2Controls();
    const results = [];
    
    for (const control of controls) {
      const result = await this.evaluateControl(control);
      results.push(result);
    }
    
    return {
      timestamp: new Date(),
      controls: results,
      overall_compliance: this.calculateComplianceScore(results),
      recommendations: this.generateRecommendations(results)
    };
  }
}
```

### Audit Logging

```sql
-- Comprehensive audit logging
CREATE TABLE audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    user_id UUID,
    session_id UUID,
    event_type VARCHAR(50) NOT NULL,
    resource_type VARCHAR(50),
    resource_id UUID,
    action VARCHAR(50) NOT NULL,
    outcome VARCHAR(20) NOT NULL, -- success, failure, error
    ip_address INET,
    user_agent TEXT,
    details JSONB,
    risk_score INTEGER DEFAULT 0,
    INDEX idx_timestamp (timestamp),
    INDEX idx_user_id (user_id),
    INDEX idx_event_type (event_type),
    INDEX idx_outcome (outcome)
);

-- Audit function
CREATE OR REPLACE FUNCTION audit_log_event(
    p_user_id UUID,
    p_session_id UUID,
    p_event_type VARCHAR(50),
    p_resource_type VARCHAR(50),
    p_resource_id UUID,
    p_action VARCHAR(50),
    p_outcome VARCHAR(20),
    p_ip_address INET,
    p_user_agent TEXT,
    p_details JSONB DEFAULT NULL
) RETURNS VOID AS $$
BEGIN
    INSERT INTO audit_log (
        user_id, session_id, event_type, resource_type, resource_id,
        action, outcome, ip_address, user_agent, details
    ) VALUES (
        p_user_id, p_session_id, p_event_type, p_resource_type, p_resource_id,
        p_action, p_outcome, p_ip_address, p_user_agent, p_details
    );
END;
$$ LANGUAGE plpgsql;
```

## Security Testing

### Automated Security Testing

```typescript
// Security test framework
describe('Security Tests', () => {
  describe('Authentication', () => {
    it('should reject weak passwords', async () => {
      const weakPasswords = ['123456', 'password', 'qwerty'];
      
      for (const password of weakPasswords) {
        const result = await authService.register({
          email: 'test@example.com',
          password
        });
        
        expect(result.success).toBe(false);
        expect(result.error).toContain('password strength');
      }
    });
    
    it('should implement rate limiting', async () => {
      const promises = [];
      
      // Attempt 20 rapid login attempts
      for (let i = 0; i < 20; i++) {
        promises.push(authService.login({
          email: 'test@example.com',
          password: 'wrongpassword'
        }));
      }
      
      const results = await Promise.all(promises);
      const rateLimitedCount = results.filter(r => 
        r.error && r.error.includes('rate limit')
      ).length;
      
      expect(rateLimitedCount).toBeGreaterThan(0);
    });
  });
  
  describe('Input Validation', () => {
    it('should sanitize SQL injection attempts', async () => {
      const maliciousInput = "'; DROP TABLE users; --";
      
      const result = await userService.updateProfile({
        name: maliciousInput
      });
      
      // Should either reject or sanitize the input
      expect(result.success).toBe(true);
      expect(result.data.name).not.toContain('DROP TABLE');
    });
  });
});
```

## Security Checklist

### Development Checklist

- [ ] All inputs validated and sanitized
- [ ] Sensitive data encrypted at rest and in transit
- [ ] Authentication implemented with strong password policies
- [ ] Authorization checks on all protected resources
- [ ] Error messages don't leak sensitive information
- [ ] Logging implemented for security events
- [ ] Dependencies regularly updated and scanned
- [ ] Security tests included in test suite

### Deployment Checklist

- [ ] TLS configured with strong ciphers
- [ ] Security headers implemented
- [ ] Rate limiting configured
- [ ] Monitoring and alerting set up
- [ ] Secrets properly managed (not in code)
- [ ] Container security policies applied
- [ ] Network security configured
- [ ] Backup and recovery procedures tested

### Operations Checklist

- [ ] Regular security assessments conducted
- [ ] Incident response plan updated
- [ ] Security patches applied promptly
- [ ] Access reviews conducted quarterly
- [ ] Audit logs reviewed regularly
- [ ] Compliance requirements met
- [ ] Security training completed
- [ ] Disaster recovery tested

## See Also

- [Authentication System](../api/auth.md)
- [Authorization Guide](../anya-enterprise/docs/security/authorization.md)
- [Encryption Standards](../security/encryption.md)
- [Compliance Framework](../compliance/framework.md)

---

*This document is part of the Anya Core Security Framework and should be reviewed quarterly.*
