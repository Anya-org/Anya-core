# Role-Based Access Control (RBAC)

Comprehensive role-based access control system for enterprise security.

## Overview

RBAC provides a structured approach to managing user permissions by assigning roles to users and permissions to roles, ensuring secure and scalable access management.

## Core Concepts

### Users, Roles, and Permissions

```rust
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub roles: HashSet<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: String,
    pub permissions: HashSet<String>,
    pub is_system_role: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub id: String,
    pub resource: String,
    pub action: String,
    pub description: String,
}

pub struct RBACManager {
    users: HashMap<String, User>,
    roles: HashMap<String, Role>,
    permissions: HashMap<String, Permission>,
    role_hierarchy: RoleHierarchy,
}
```

### Permission Management

```rust
impl RBACManager {
    pub fn check_permission(&self, user_id: &str, resource: &str, action: &str) -> bool {
        let user = match self.users.get(user_id) {
            Some(user) if user.is_active => user,
            _ => return false,
        };
        
        // Check direct permissions through roles
        for role_id in &user.roles {
            if let Some(role) = self.roles.get(role_id) {
                let permission_key = format!("{}:{}", resource, action);
                if role.permissions.contains(&permission_key) {
                    return true;
                }
                
                // Check inherited permissions
                if self.check_inherited_permissions(role_id, &permission_key) {
                    return true;
                }
            }
        }
        
        false
    }
    
    pub fn assign_role(&mut self, user_id: &str, role_id: &str) -> Result<(), RBACError> {
        let user = self.users.get_mut(user_id)
            .ok_or(RBACError::UserNotFound)?;
        
        if !self.roles.contains_key(role_id) {
            return Err(RBACError::RoleNotFound);
        }
        
        user.roles.insert(role_id.to_string());
        self.audit_log_role_assignment(user_id, role_id);
        
        Ok(())
    }
    
    pub fn create_role(&mut self, role: Role) -> Result<(), RBACError> {
        // Validate permissions exist
        for permission in &role.permissions {
            if !self.permissions.contains_key(permission) {
                return Err(RBACError::InvalidPermission(permission.clone()));
            }
        }
        
        self.roles.insert(role.id.clone(), role);
        Ok(())
    }
}
```

## Pre-defined Roles

### System Roles

```rust
pub fn create_system_roles() -> Vec<Role> {
    vec![
        Role {
            id: "super_admin".to_string(),
            name: "Super Administrator".to_string(),
            description: "Full system access".to_string(),
            permissions: HashSet::from([
                "system:*".to_string(),
                "users:*".to_string(),
                "roles:*".to_string(),
            ]),
            is_system_role: true,
            created_at: Utc::now(),
        },
        
        Role {
            id: "admin".to_string(),
            name: "Administrator".to_string(),
            description: "Administrative access".to_string(),
            permissions: HashSet::from([
                "users:read".to_string(),
                "users:create".to_string(),
                "users:update".to_string(),
                "bitcoin:*".to_string(),
                "reports:read".to_string(),
            ]),
            is_system_role: true,
            created_at: Utc::now(),
        },
        
        Role {
            id: "trader".to_string(),
            name: "Trader".to_string(),
            description: "Trading operations access".to_string(),
            permissions: HashSet::from([
                "wallet:read".to_string(),
                "transactions:create".to_string(),
                "transactions:read".to_string(),
                "market:read".to_string(),
                "analytics:read".to_string(),
            ]),
            is_system_role: true,
            created_at: Utc::now(),
        },
        
        Role {
            id: "viewer".to_string(),
            name: "Viewer".to_string(),
            description: "Read-only access".to_string(),
            permissions: HashSet::from([
                "dashboard:read".to_string(),
                "reports:read".to_string(),
                "analytics:read".to_string(),
            ]),
            is_system_role: true,
            created_at: Utc::now(),
        },
    ]
}
```

## Role Hierarchy

```rust
pub struct RoleHierarchy {
    parent_roles: HashMap<String, HashSet<String>>,
    child_roles: HashMap<String, HashSet<String>>,
}

impl RoleHierarchy {
    pub fn new() -> Self {
        let mut hierarchy = Self {
            parent_roles: HashMap::new(),
            child_roles: HashMap::new(),
        };
        
        // Set up default hierarchy
        hierarchy.add_inheritance("super_admin", "admin");
        hierarchy.add_inheritance("admin", "trader");
        hierarchy.add_inheritance("trader", "viewer");
        
        hierarchy
    }
    
    pub fn add_inheritance(&mut self, parent: &str, child: &str) {
        self.parent_roles
            .entry(child.to_string())
            .or_insert_with(HashSet::new)
            .insert(parent.to_string());
            
        self.child_roles
            .entry(parent.to_string())
            .or_insert_with(HashSet::new)
            .insert(child.to_string());
    }
    
    pub fn get_effective_permissions(&self, role_id: &str, roles: &HashMap<String, Role>) -> HashSet<String> {
        let mut permissions = HashSet::new();
        
        if let Some(role) = roles.get(role_id) {
            permissions.extend(role.permissions.clone());
        }
        
        // Add inherited permissions
        if let Some(parents) = self.parent_roles.get(role_id) {
            for parent_id in parents {
                permissions.extend(self.get_effective_permissions(parent_id, roles));
            }
        }
        
        permissions
    }
}
```

## Dynamic Permissions

### Context-Aware Permissions

```rust
#[derive(Debug, Clone)]
pub struct PermissionContext {
    pub user_id: String,
    pub resource_id: String,
    pub resource_owner: Option<String>,
    pub organization_id: Option<String>,
    pub environment: String, // "production", "staging", "development"
    pub time_restrictions: Option<TimeRestriction>,
}

#[derive(Debug, Clone)]
pub struct TimeRestriction {
    pub start_time: Time,
    pub end_time: Time,
    pub days_of_week: Vec<Weekday>,
    pub timezone: String,
}

impl RBACManager {
    pub fn check_permission_with_context(
        &self,
        context: &PermissionContext,
        action: &str,
    ) -> bool {
        // Basic permission check
        if !self.check_permission(&context.user_id, &context.resource_id, action) {
            return false;
        }
        
        // Resource ownership check
        if let Some(owner) = &context.resource_owner {
            if owner != &context.user_id && !self.check_permission(&context.user_id, "admin", "override") {
                return false;
            }
        }
        
        // Time-based restrictions
        if let Some(time_restriction) = &context.time_restrictions {
            if !self.check_time_restriction(time_restriction) {
                return false;
            }
        }
        
        // Environment-based restrictions
        if context.environment == "production" {
            return self.check_permission(&context.user_id, "production", action);
        }
        
        true
    }
}
```

## API Integration

### REST API Endpoints

```bash
# Get user roles
GET /api/v1/rbac/users/{user_id}/roles
Authorization: Bearer <token>

Response:
{
  "user_id": "user123",
  "roles": [
    {
      "id": "trader",
      "name": "Trader",
      "permissions": ["wallet:read", "transactions:create"]
    }
  ]
}

# Assign role to user
POST /api/v1/rbac/users/{user_id}/roles
Content-Type: application/json

{
  "role_id": "admin"
}

# Check permission
POST /api/v1/rbac/check-permission
Content-Type: application/json

{
  "user_id": "user123",
  "resource": "wallet",
  "action": "transfer"
}

Response:
{
  "allowed": true,
  "reason": "User has trader role with wallet:transfer permission"
}
```

### Middleware Integration

```typescript
import { Request, Response, NextFunction } from 'express';

interface AuthenticatedRequest extends Request {
  user?: {
    id: string;
    roles: string[];
  };
}

export const requirePermission = (resource: string, action: string) => {
  return async (req: AuthenticatedRequest, res: Response, next: NextFunction) => {
    if (!req.user) {
      return res.status(401).json({ error: 'Authentication required' });
    }
    
    const hasPermission = await rbacManager.checkPermission(
      req.user.id,
      resource,
      action
    );
    
    if (!hasPermission) {
      return res.status(403).json({ 
        error: 'Insufficient permissions',
        required: `${resource}:${action}`
      });
    }
    
    next();
  };
};

// Usage
app.post('/api/transactions', 
  authenticate(),
  requirePermission('transactions', 'create'),
  createTransaction
);
```

## Database Schema

```sql
-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMP
);

-- Roles table
CREATE TABLE roles (
    id UUID PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    is_system_role BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Permissions table
CREATE TABLE permissions (
    id UUID PRIMARY KEY,
    resource VARCHAR(255) NOT NULL,
    action VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(resource, action)
);

-- User-Role assignments
CREATE TABLE user_roles (
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    role_id UUID REFERENCES roles(id) ON DELETE CASCADE,
    assigned_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    assigned_by UUID REFERENCES users(id),
    PRIMARY KEY (user_id, role_id)
);

-- Role-Permission assignments
CREATE TABLE role_permissions (
    role_id UUID REFERENCES roles(id) ON DELETE CASCADE,
    permission_id UUID REFERENCES permissions(id) ON DELETE CASCADE,
    PRIMARY KEY (role_id, permission_id)
);

-- Role hierarchy
CREATE TABLE role_hierarchy (
    parent_role_id UUID REFERENCES roles(id) ON DELETE CASCADE,
    child_role_id UUID REFERENCES roles(id) ON DELETE CASCADE,
    PRIMARY KEY (parent_role_id, child_role_id)
);
```

## Security Considerations

### Audit Logging

```rust
#[derive(Debug, Serialize)]
pub struct RBACEvent {
    pub event_type: RBACEventType,
    pub user_id: String,
    pub target_user_id: Option<String>,
    pub role_id: Option<String>,
    pub permission: Option<String>,
    pub success: bool,
    pub timestamp: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Serialize)]
pub enum RBACEventType {
    RoleAssigned,
    RoleRevoked,
    PermissionChecked,
    RoleCreated,
    RoleDeleted,
    PermissionGranted,
    PermissionDenied,
}

impl RBACManager {
    fn audit_log_role_assignment(&self, user_id: &str, role_id: &str) {
        let event = RBACEvent {
            event_type: RBACEventType::RoleAssigned,
            user_id: self.current_user_id.clone(),
            target_user_id: Some(user_id.to_string()),
            role_id: Some(role_id.to_string()),
            permission: None,
            success: true,
            timestamp: Utc::now(),
            ip_address: self.current_ip.clone(),
            user_agent: self.current_user_agent.clone(),
        };
        
        self.audit_logger.log(event);
    }
}
```

### Best Practices

1. **Principle of Least Privilege**: Grant minimum necessary permissions
2. **Regular Audits**: Periodic review of role assignments
3. **Separation of Duties**: No single role should have all permissions
4. **Role Rotation**: Regular rotation of sensitive roles
5. **Monitoring**: Continuous monitoring of permission usage

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_permission_check() {
        let mut rbac = RBACManager::new();
        
        // Create test user and role
        let user = User {
            id: "test_user".to_string(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            roles: HashSet::from(["trader".to_string()]),
            is_active: true,
            created_at: Utc::now(),
            last_login: None,
        };
        
        let role = Role {
            id: "trader".to_string(),
            name: "Trader".to_string(),
            description: "Trading role".to_string(),
            permissions: HashSet::from(["wallet:read".to_string()]),
            is_system_role: false,
            created_at: Utc::now(),
        };
        
        rbac.users.insert(user.id.clone(), user);
        rbac.roles.insert(role.id.clone(), role);
        
        assert!(rbac.check_permission("test_user", "wallet", "read"));
        assert!(!rbac.check_permission("test_user", "wallet", "write"));
    }
    
    #[test]
    fn test_role_hierarchy() {
        let mut rbac = RBACManager::new();
        rbac.setup_default_hierarchy();
        
        // Admin should inherit trader permissions
        let admin_permissions = rbac.role_hierarchy
            .get_effective_permissions("admin", &rbac.roles);
        
        assert!(admin_permissions.contains("wallet:read"));
        assert!(admin_permissions.contains("analytics:read"));
    }
}
```

## See Also

- [Security Features Overview](README.md)
- [Multi-Factor Authentication](mfa.md)
- [Session Management](session-management.md)
- [Authorization Guide](authorization.md)
- [Security Monitoring](security-monitoring.md)

---

*This documentation is part of the Anya Enterprise Security suite.*
