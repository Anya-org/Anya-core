# Session Management

Enterprise-grade session management for secure user authentication and authorization.

## Overview

The session management system provides comprehensive session handling capabilities including secure session creation, validation, timeout management, and cleanup for Anya Enterprise deployments.

## Features

### Session Creation

```rust
use anya_security::session::{SessionManager, SessionConfig};
use uuid::Uuid;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Session {
    pub id: Uuid,
    pub user_id: String,
    pub roles: Vec<String>,
    pub created_at: SystemTime,
    pub last_activity: SystemTime,
    pub expires_at: SystemTime,
    pub ip_address: IpAddr,
    pub user_agent: String,
}

impl SessionManager {
    pub async fn create_session(
        &self,
        user_id: &str,
        ip_address: IpAddr,
        user_agent: String,
    ) -> Result<Session, SessionError> {
        let session_id = Uuid::new_v4();
        let now = SystemTime::now();
        let expires_at = now + self.config.session_timeout;
        
        let session = Session {
            id: session_id,
            user_id: user_id.to_string(),
            roles: self.get_user_roles(user_id).await?,
            created_at: now,
            last_activity: now,
            expires_at,
            ip_address,
            user_agent,
        };
        
        self.store_session(&session).await?;
        Ok(session)
    }
}
```

### Session Validation

```typescript
interface SessionValidationResult {
  valid: boolean;
  session?: Session;
  reason?: string;
}

class SessionValidator {
  async validateSession(sessionId: string, ip: string): Promise<SessionValidationResult> {
    const session = await this.getSession(sessionId);
    
    if (!session) {
      return { valid: false, reason: 'Session not found' };
    }
    
    if (session.expires_at < new Date()) {
      await this.cleanupSession(sessionId);
      return { valid: false, reason: 'Session expired' };
    }
    
    if (this.config.strictIpValidation && session.ip_address !== ip) {
      return { valid: false, reason: 'IP address mismatch' };
    }
    
    // Update last activity
    await this.updateLastActivity(sessionId);
    
    return { valid: true, session };
  }
}
```

### Session Security Features

#### Secure Session IDs

- **Cryptographically Strong**: Using secure random number generation
- **Sufficient Length**: 128-bit session identifiers
- **Non-predictable**: No sequential or pattern-based IDs
- **Unique**: Collision-resistant generation

#### Session Fixation Protection

```rust
impl SessionManager {
    pub async fn regenerate_session_id(&self, old_session_id: Uuid) -> Result<Uuid, SessionError> {
        let session = self.get_session(old_session_id).await?;
        
        // Generate new session ID
        let new_session_id = Uuid::new_v4();
        
        // Update session with new ID
        let mut updated_session = session;
        updated_session.id = new_session_id;
        
        // Store updated session
        self.store_session(&updated_session).await?;
        
        // Remove old session
        self.remove_session(old_session_id).await?;
        
        Ok(new_session_id)
    }
}
```

#### Session Timeout Management

```yaml
session_config:
  timeout:
    idle_timeout: "30m"          # 30 minutes of inactivity
    absolute_timeout: "8h"       # Maximum session duration
    warning_time: "5m"           # Warning before expiration
  
  security:
    strict_ip_validation: true
    secure_cookies: true
    http_only: true
    same_site: "strict"
  
  cleanup:
    cleanup_interval: "5m"       # How often to run cleanup
    batch_size: 100             # Sessions to process per batch
```

### Multi-Factor Authentication Integration

```typescript
interface MFASession extends Session {
  mfa_verified: boolean;
  mfa_required: boolean;
  mfa_methods: string[];
}

class MFASessionManager extends SessionManager {
  async requireMFA(sessionId: string): Promise<void> {
    const session = await this.getSession(sessionId) as MFASession;
    session.mfa_required = true;
    session.mfa_verified = false;
    await this.updateSession(session);
  }
  
  async verifyMFA(sessionId: string, mfaToken: string): Promise<boolean> {
    const session = await this.getSession(sessionId) as MFASession;
    
    if (await this.mfaProvider.verify(session.user_id, mfaToken)) {
      session.mfa_verified = true;
      await this.updateSession(session);
      return true;
    }
    
    return false;
  }
}
```

## Storage Backends

### Redis Backend

```rust
use redis::{Client, Commands};

pub struct RedisSessionStore {
    client: Client,
    prefix: String,
}

impl SessionStore for RedisSessionStore {
    async fn store_session(&self, session: &Session) -> Result<(), SessionError> {
        let mut conn = self.client.get_connection()?;
        let key = format!("{}:session:{}", self.prefix, session.id);
        let session_data = serde_json::to_string(session)?;
        
        let ttl = session.expires_at.duration_since(SystemTime::now())?;
        
        conn.set_ex(&key, session_data, ttl.as_secs())?;
        Ok(())
    }
    
    async fn get_session(&self, session_id: Uuid) -> Result<Option<Session>, SessionError> {
        let mut conn = self.client.get_connection()?;
        let key = format!("{}:session:{}", self.prefix, session_id);
        
        let session_data: Option<String> = conn.get(&key)?;
        
        match session_data {
            Some(data) => {
                let session: Session = serde_json::from_str(&data)?;
                Ok(Some(session))
            }
            None => Ok(None),
        }
    }
}
```

### Database Backend

```sql
CREATE TABLE sessions (
    id UUID PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL,
    roles JSON,
    created_at TIMESTAMP NOT NULL,
    last_activity TIMESTAMP NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    ip_address INET,
    user_agent TEXT,
    data JSON,
    INDEX idx_user_id (user_id),
    INDEX idx_expires_at (expires_at)
);
```

## Session Monitoring

### Analytics Dashboard

```typescript
interface SessionMetrics {
  active_sessions: number;
  sessions_created_today: number;
  average_session_duration: number;
  expired_sessions_cleaned: number;
  failed_validations: number;
}

class SessionAnalytics {
  async getMetrics(): Promise<SessionMetrics> {
    return {
      active_sessions: await this.countActiveSessions(),
      sessions_created_today: await this.countTodaySessions(),
      average_session_duration: await this.calculateAverageDuration(),
      expired_sessions_cleaned: await this.countExpiredCleaned(),
      failed_validations: await this.countFailedValidations(),
    };
  }
}
```

### Security Monitoring

- **Concurrent Session Limits**: Prevent session abuse
- **Unusual Activity Detection**: Monitor for suspicious patterns
- **Session Hijacking Detection**: IP and user agent validation
- **Brute Force Protection**: Rate limiting and account lockout

## Configuration Examples

### Production Configuration

```toml
[session_management]
provider = "redis"
encryption_key = "${SESSION_ENCRYPTION_KEY}"

[session_management.timeouts]
idle_timeout = "30m"
absolute_timeout = "8h"
cleanup_interval = "5m"

[session_management.security]
strict_ip_validation = true
require_https = true
secure_cookies = true
http_only_cookies = true
same_site_policy = "strict"

[session_management.monitoring]
enable_analytics = true
log_session_events = true
alert_on_suspicious_activity = true
```

### Development Configuration

```toml
[session_management]
provider = "memory"

[session_management.timeouts]
idle_timeout = "4h"
absolute_timeout = "24h"

[session_management.security]
strict_ip_validation = false
require_https = false
```

## API Integration

### REST Endpoints

```bash
# Create session (login)
POST /api/v1/auth/sessions
Content-Type: application/json
{
  "username": "user@example.com",
  "password": "password",
  "mfa_token": "123456"
}

# Validate session
GET /api/v1/auth/sessions/{session_id}
Authorization: Bearer {session_id}

# Refresh session
PUT /api/v1/auth/sessions/{session_id}
Authorization: Bearer {session_id}

# Destroy session (logout)
DELETE /api/v1/auth/sessions/{session_id}
Authorization: Bearer {session_id}
```

### WebSocket Integration

```javascript
// Session validation for WebSocket connections
const ws = new WebSocket('wss://api.example.com/ws', [], {
  headers: {
    'Authorization': `Bearer ${sessionId}`
  }
});

ws.on('open', () => {
  console.log('WebSocket connection established');
});

ws.on('error', (error) => {
  if (error.code === 401) {
    // Session expired, redirect to login
    window.location.href = '/login';
  }
});
```

## See Also

- [Multi-Factor Authentication](mfa.md)
- [Role-Based Access Control](rbac.md)
- [Authorization Guide](authorization.md)
- [Security Monitoring](security-monitoring.md)

---

*This documentation is part of the Anya Enterprise Security suite.*
