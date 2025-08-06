# Authorization Guide

Comprehensive authorization system for controlling access to resources and operations in Anya Enterprise.

## Overview

The authorization system provides fine-grained access control based on user roles, permissions, and contextual factors. It integrates with the authentication system to ensure users can only access resources they're authorized to use.

## Core Concepts

### Role-Based Access Control (RBAC)

```typescript
interface Role {
  id: string;
  name: string;
  description: string;
  permissions: Permission[];
  inherits?: string[];  // Role inheritance
}

interface Permission {
  id: string;
  resource: string;
  action: string;
  conditions?: PermissionCondition[];
}

interface PermissionCondition {
  field: string;
  operator: 'eq' | 'ne' | 'in' | 'contains' | 'gt' | 'lt';
  value: any;
}
```

### Attribute-Based Access Control (ABAC)

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationRequest {
    pub subject: Subject,
    pub resource: Resource,
    pub action: Action,
    pub context: Context,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subject {
    pub user_id: String,
    pub roles: Vec<String>,
    pub attributes: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resource {
    pub id: String,
    pub type_: String,
    pub owner: Option<String>,
    pub attributes: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Context {
    pub time: SystemTime,
    pub ip_address: IpAddr,
    pub location: Option<String>,
    pub attributes: HashMap<String, Value>,
}
```

## Authorization Engine

### Policy Evaluation

```rust
pub struct AuthorizationEngine {
    policies: Vec<Policy>,
    rbac_engine: RBACEngine,
    abac_engine: ABACEngine,
}

impl AuthorizationEngine {
    pub async fn authorize(&self, request: &AuthorizationRequest) -> AuthorizationResult {
        // First check RBAC permissions
        if let Some(rbac_result) = self.rbac_engine.evaluate(request).await? {
            if rbac_result.is_allowed() {
                return Ok(AuthorizationResult::Allow);
            }
        }
        
        // Then evaluate ABAC policies
        for policy in &self.policies {
            let result = self.abac_engine.evaluate_policy(policy, request).await?;
            match result {
                PolicyResult::Allow => return Ok(AuthorizationResult::Allow),
                PolicyResult::Deny => return Ok(AuthorizationResult::Deny),
                PolicyResult::NotApplicable => continue,
            }
        }
        
        // Default deny
        Ok(AuthorizationResult::Deny)
    }
}
```

### Policy Language

```yaml
# Example authorization policy
policies:
  - name: "wallet_access"
    description: "Control access to wallet operations"
    rules:
      - effect: "allow"
        subjects:
          - role: "wallet_owner"
        resources:
          - type: "wallet"
            attributes:
              owner: "{{ subject.user_id }}"
        actions: ["read", "send"]
        conditions:
          - field: "context.ip_address"
            operator: "in"
            value: ["trusted_networks"]
            
  - name: "admin_operations"
    description: "Administrative operations"
    rules:
      - effect: "allow"
        subjects:
          - role: "admin"
        resources:
          - type: "*"
        actions: ["*"]
        conditions:
          - field: "context.time"
            operator: "between"
            value: ["09:00", "17:00"]  # Business hours only
```

## Implementation Examples

### API Authorization Middleware

```typescript
import { Request, Response, NextFunction } from 'express';

interface AuthorizedRequest extends Request {
  user?: User;
  authorization?: AuthorizationResult;
}

export function authorize(resource: string, action: string) {
  return async (req: AuthorizedRequest, res: Response, next: NextFunction) => {
    if (!req.user) {
      return res.status(401).json({ error: 'Authentication required' });
    }
    
    const authRequest: AuthorizationRequest = {
      subject: {
        user_id: req.user.id,
        roles: req.user.roles,
        attributes: req.user.attributes
      },
      resource: {
        id: req.params.id || resource,
        type_: resource,
        owner: req.params.owner,
        attributes: {}
      },
      action: { name: action },
      context: {
        time: new Date(),
        ip_address: req.ip,
        attributes: {
          user_agent: req.get('User-Agent'),
          method: req.method
        }
      }
    };
    
    const result = await authorizationEngine.authorize(authRequest);
    
    if (result.is_allowed()) {
      req.authorization = result;
      next();
    } else {
      res.status(403).json({ 
        error: 'Insufficient permissions',
        required_permissions: result.required_permissions
      });
    }
  };
}

// Usage
app.get('/api/wallets/:id', 
  authenticate, 
  authorize('wallet', 'read'), 
  getWallet
);
```

### Database-Level Authorization

```sql
-- Row-level security policies
CREATE POLICY wallet_owner_policy ON wallets
  FOR ALL TO authenticated_users
  USING (owner_id = current_user_id());

CREATE POLICY admin_access_policy ON wallets
  FOR ALL TO authenticated_users
  USING (has_role('admin'));

-- Function to check permissions
CREATE OR REPLACE FUNCTION check_permission(
  user_id UUID,
  resource_type TEXT,
  resource_id UUID,
  action TEXT
) RETURNS BOOLEAN AS $$
BEGIN
  -- Check if user has required permissions
  RETURN EXISTS (
    SELECT 1 FROM user_permissions up
    JOIN permissions p ON up.permission_id = p.id
    WHERE up.user_id = $1
      AND p.resource_type = $2
      AND p.action = $4
      AND (p.resource_id IS NULL OR p.resource_id = $3)
  );
END;
$$ LANGUAGE plpgsql;
```

## Advanced Features

### Dynamic Permissions

```rust
use async_trait::async_trait;

#[async_trait]
pub trait DynamicPermissionProvider {
    async fn get_permissions(
        &self,
        user_id: &str,
        resource: &Resource,
        context: &Context,
    ) -> Result<Vec<Permission>, AuthorizationError>;
}

pub struct OwnershipPermissionProvider;

#[async_trait]
impl DynamicPermissionProvider for OwnershipPermissionProvider {
    async fn get_permissions(
        &self,
        user_id: &str,
        resource: &Resource,
        context: &Context,
    ) -> Result<Vec<Permission>, AuthorizationError> {
        let mut permissions = Vec::new();
        
        // Grant full permissions to resource owner
        if resource.owner.as_ref() == Some(&user_id.to_string()) {
            permissions.push(Permission {
                id: "owner_all".to_string(),
                resource: resource.type_.clone(),
                action: "*".to_string(),
                conditions: None,
            });
        }
        
        Ok(permissions)
    }
}
```

### Hierarchical Resources

```typescript
class HierarchicalAuthorization {
  async checkPermission(
    userId: string,
    resourcePath: string,
    action: string
  ): Promise<boolean> {
    // Check permission at current level
    if (await this.hasDirectPermission(userId, resourcePath, action)) {
      return true;
    }
    
    // Check inherited permissions from parent resources
    const parentPath = this.getParentPath(resourcePath);
    if (parentPath) {
      return this.checkPermission(userId, parentPath, action);
    }
    
    return false;
  }
  
  private getParentPath(path: string): string | null {
    const parts = path.split('/');
    if (parts.length <= 1) return null;
    
    parts.pop();
    return parts.join('/');
  }
}
```

### Time-Based Permissions

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct TemporalPermission {
    pub permission: Permission,
    pub valid_from: Option<SystemTime>,
    pub valid_until: Option<SystemTime>,
    pub schedule: Option<Schedule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Schedule {
    pub days_of_week: Vec<u8>,  // 1-7 (Monday-Sunday)
    pub hours: Option<(u8, u8)>,  // (start_hour, end_hour)
    pub timezone: String,
}

impl TemporalPermission {
    pub fn is_valid_at(&self, time: SystemTime) -> bool {
        if let Some(valid_from) = self.valid_from {
            if time < valid_from {
                return false;
            }
        }
        
        if let Some(valid_until) = self.valid_until {
            if time > valid_until {
                return false;
            }
        }
        
        if let Some(schedule) = &self.schedule {
            return self.matches_schedule(schedule, time);
        }
        
        true
    }
}
```

## Performance Optimization

### Permission Caching

```typescript
class PermissionCache {
  private cache = new Map<string, CacheEntry>();
  private ttl = 5 * 60 * 1000; // 5 minutes
  
  async getPermissions(userId: string, resourceId: string): Promise<Permission[]> {
    const key = `${userId}:${resourceId}`;
    const cached = this.cache.get(key);
    
    if (cached && cached.expires > Date.now()) {
      return cached.permissions;
    }
    
    const permissions = await this.fetchPermissions(userId, resourceId);
    
    this.cache.set(key, {
      permissions,
      expires: Date.now() + this.ttl
    });
    
    return permissions;
  }
  
  invalidate(userId?: string, resourceId?: string) {
    if (userId && resourceId) {
      this.cache.delete(`${userId}:${resourceId}`);
    } else if (userId) {
      // Clear all permissions for user
      for (const key of this.cache.keys()) {
        if (key.startsWith(`${userId}:`)) {
          this.cache.delete(key);
        }
      }
    } else {
      // Clear all cache
      this.cache.clear();
    }
  }
}
```

### Bulk Authorization

```rust
impl AuthorizationEngine {
    pub async fn authorize_bulk(
        &self,
        requests: &[AuthorizationRequest],
    ) -> Result<Vec<AuthorizationResult>, AuthorizationError> {
        // Group requests by user for efficient permission lookup
        let mut user_groups: HashMap<String, Vec<&AuthorizationRequest>> = HashMap::new();
        
        for request in requests {
            user_groups
                .entry(request.subject.user_id.clone())
                .or_default()
                .push(request);
        }
        
        let mut results = Vec::with_capacity(requests.len());
        
        for (user_id, user_requests) in user_groups {
            // Fetch all permissions for user once
            let permissions = self.get_user_permissions(&user_id).await?;
            
            for request in user_requests {
                let result = self.evaluate_with_permissions(request, &permissions).await?;
                results.push(result);
            }
        }
        
        Ok(results)
    }
}
```

## Monitoring and Auditing

### Authorization Events

```typescript
interface AuthorizationEvent {
  id: string;
  timestamp: Date;
  user_id: string;
  resource: string;
  action: string;
  result: 'allow' | 'deny';
  reason?: string;
  ip_address: string;
  user_agent: string;
}

class AuthorizationAuditor {
  async logEvent(event: AuthorizationEvent): Promise<void> {
    // Store in audit log
    await this.auditDb.insert('authorization_events', event);
    
    // Real-time monitoring
    if (event.result === 'deny') {
      await this.alertOnUnauthorizedAccess(event);
    }
    
    // Metrics collection
    this.metrics.increment(`authorization.${event.result}`, {
      resource: event.resource,
      action: event.action
    });
  }
}
```

### Security Analytics

```sql
-- Find users with unusual access patterns
SELECT 
  user_id,
  COUNT(*) as access_attempts,
  COUNT(CASE WHEN result = 'deny' THEN 1 END) as denied_attempts,
  ARRAY_AGG(DISTINCT resource) as accessed_resources
FROM authorization_events 
WHERE timestamp > NOW() - INTERVAL '1 hour'
GROUP BY user_id
HAVING COUNT(CASE WHEN result = 'deny' THEN 1 END) > 10;

-- Resource access frequency
SELECT 
  resource,
  action,
  COUNT(*) as access_count,
  COUNT(DISTINCT user_id) as unique_users
FROM authorization_events
WHERE timestamp > NOW() - INTERVAL '1 day'
GROUP BY resource, action
ORDER BY access_count DESC;
```

## Configuration

### Authorization Configuration

```yaml
authorization:
  engine: "hybrid"  # rbac, abac, or hybrid
  
  rbac:
    role_hierarchy: true
    role_inheritance: true
    
  abac:
    policy_language: "yaml"
    policy_directory: "/etc/anya/policies"
    
  caching:
    enabled: true
    ttl: 300  # seconds
    max_entries: 10000
    
  audit:
    enabled: true
    log_all_events: true
    alert_on_denials: true
    
  performance:
    bulk_evaluation: true
    parallel_processing: true
    max_concurrent: 100
```

## Testing

### Authorization Test Framework

```typescript
describe('Authorization System', () => {
  it('should allow wallet owner to read their wallet', async () => {
    const request = {
      subject: { user_id: 'user123', roles: ['user'] },
      resource: { id: 'wallet456', type_: 'wallet', owner: 'user123' },
      action: { name: 'read' },
      context: { time: new Date(), ip_address: '192.168.1.1' }
    };
    
    const result = await authEngine.authorize(request);
    expect(result.is_allowed()).toBe(true);
  });
  
  it('should deny access to other users wallets', async () => {
    const request = {
      subject: { user_id: 'user123', roles: ['user'] },
      resource: { id: 'wallet456', type_: 'wallet', owner: 'user999' },
      action: { name: 'read' },
      context: { time: new Date(), ip_address: '192.168.1.1' }
    };
    
    const result = await authEngine.authorize(request);
    expect(result.is_allowed()).toBe(false);
  });
});
```

## See Also

- [Role-Based Access Control](rbac.md)
- [Session Management](session-management.md)
- [Multi-Factor Authentication](mfa.md)
- [Security Monitoring](security-monitoring.md)

---

*This documentation is part of the Anya Enterprise Security suite.*
