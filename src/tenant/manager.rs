use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use tokio::sync::RwLock;

pub struct TenantManager {
    tenants: RwLock<HashMap<String, Arc<Tenant>>>,
    config: TenantConfig,
}

pub struct TenantConfig {
    max_tenants: usize,
    storage_quota: u64,
    rate_limits: RateLimits,
}

pub struct RateLimits {
    requests_per_minute: u32,
    max_concurrent_requests: u32,
}

pub struct Tenant {
    id: String,
    name: String,
    config: TenantConfig,
    resources: TenantResources,
    security: Arc<TenantSecurity>,
}

pub struct TenantResources {
    storage_used: u64,
    active_connections: u32,
    request_count: u32,
}

pub struct TenantSecurity {
    encryption_keys: HashMap<String, Vec<u8>>,
    access_policies: HashMap<String, AccessPolicy>,
}

pub struct AccessPolicy {
    allowed_ips: Vec<String>,
    rate_limit: RateLimit,
    permissions: Vec<String>,
}

pub struct RateLimit {
    requests: u32,
    period: chrono::Duration,
}

impl TenantManager {
    pub fn new(config: TenantConfig) -> Self {
        Self {
            tenants: RwLock::new(HashMap::new()),
            config,
        }
    }

    pub async fn create_tenant(&self, name: &str) -> Result<Arc<Tenant>> {
        let mut tenants = self.tenants.write().await;
        if tenants.len() >= self.config.max_tenants {
            return Err(anyhow::anyhow!("Maximum number of tenants reached"));
        }

        let id = self.generate_tenant_id();
        let tenant = Arc::new(Tenant {
            id: id.clone(),
            name: name.to_string(),
            config: self.config.clone(),
            resources: TenantResources {
                storage_used: 0,
                active_connections: 0,
                request_count: 0,
            },
            security: Arc::new(TenantSecurity::new()),
        });

        tenants.insert(id, tenant.clone());
        Ok(tenant)
    }

    pub async fn get_tenant(&self, tenant_id: &str) -> Result<Arc<Tenant>> {
        let tenants = self.tenants.read().await;
        tenants.get(tenant_id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Tenant not found"))
    }

    pub async fn update_resources(&self, tenant_id: &str, usage: ResourceUsage) -> Result<()> {
        let tenant = self.get_tenant(tenant_id).await?;
        let mut resources = tenant.resources;
        
        resources.storage_used += usage.storage;
        resources.active_connections += usage.connections;
        resources.request_count += usage.requests;
        
        Ok(())
    }

    pub async fn check_access(&self, tenant_id: &str, ip: &str, permission: &str) -> Result<bool> {
        let tenant = self.get_tenant(tenant_id).await?;
        tenant.security.check_access(ip, permission)
    }
}

impl Tenant {
    pub fn new(id: String, name: String, config: TenantConfig) -> Self {
        Self {
            id,
            name,
            config,
            resources: TenantResources {
                storage_used: 0,
                active_connections: 0,
                request_count: 0,
            },
            security: Arc::new(TenantSecurity::new()),
        }
    }

    pub fn get_resources(&self) -> TenantResources {
        self.resources.clone()
    }
}

impl TenantSecurity {
    pub fn new() -> Self {
        Self {
            encryption_keys: HashMap::new(),
            access_policies: HashMap::new(),
        }
    }

    pub fn check_access(&self, ip: &str, permission: &str) -> Result<bool> {
        for policy in self.access_policies.values() {
            if policy.allowed_ips.contains(ip) && policy.permissions.contains(permission) {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn add_policy(&mut self, policy: AccessPolicy) {
        self.access_policies.insert(policy.permissions[0].clone(), policy);
    }
}
