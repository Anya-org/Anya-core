#![feature(edition2021)]
// Web5 DID Resolution Module with Caching Support
// This module provides DID resolution for various methods with a caching layer

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use reqwest::Client;
use std::str::FromStr;
use url::Url;

/// DID Document as specified in W3C DID Core
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidDocument {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<Vec<VerificationMethod>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assertion_method: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_agreement: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_invocation: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_delegation: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Vec<Service>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub also_known_as: Option<Vec<String>>,
}

/// Verification Method as specified in DID Core
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationMethod {
    pub id: String,
    pub type_: String,
    pub controller: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_jwk: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_multibase: Option<String>,
}

/// Service as specified in DID Core
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    pub type_: String,
    pub service_endpoint: ServiceEndpoint,
}

/// Service Endpoint which can be a string or array
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServiceEndpoint {
    String(String),
    Array(Vec<String>),
    Map(HashMap<String, String>),
}

/// Resolution result with metadata
#[derive(Debug, Clone)]
pub struct DidResolutionResult {
    pub did_document: Option<DidDocument>,
    pub metadata: DidResolutionMetadata,
}

/// Resolution metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidResolutionMetadata {
    pub content_type: Option<String>,
    pub error: Option<String>,
    pub cached: bool,
    pub retrieved_time: Option<u64>,
}

/// Cache entry for DID resolution
struct CacheEntry {
    did_document: DidDocument,
    timestamp: Instant,
}

/// Resolver for DIDs
#[async_trait]
pub trait DidResolver: Send + Sync {
    async fn resolve(&self, did: &str) -> Result<DidResolutionResult>;
    fn supports_method(&self, method: &str) -> bool;
}

/// Web resolver that uses ION network
pub struct IonResolver {
    client: Client,
    endpoint: String,
}

impl IonResolver {
    pub fn new(endpoint: Option<&str>) -> Self {
        Self {
            client: Client::new(),
            endpoint: endpoint.unwrap_or("https://ion.tbddev.org/").to_string(),
        }
    }
}

#[async_trait]
impl DidResolver for IonResolver {
    async fn resolve(&self, did: &str) -> Result<DidResolutionResult> {
        // Validate DID
        if !did.starts_with("did:ion:") {
            return Err(anyhow!("Not an ION DID: {}", did));
        }
        
        // Construct endpoint
        let url = format!("{}{}", self.endpoint, did);
        
        // Fetch DID Document
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Ok(DidResolutionResult {
                did_document: None,
                metadata: DidResolutionMetadata {
                    content_type: Some("application/json".to_string()),
                    error: Some(format!("Failed to resolve DID: {}", response.status())),
                    cached: false,
                    retrieved_time: Some(chrono::Utc::now().timestamp() as u64),
                },
            });
        }
        
        // Parse response
        let did_document = response.json::<DidDocument>().await?;
        
        Ok(DidResolutionResult {
            did_document: Some(did_document),
            metadata: DidResolutionMetadata {
                content_type: Some("application/json".to_string()),
                error: None,
                cached: false,
                retrieved_time: Some(chrono::Utc::now().timestamp() as u64),
            },
        })
    }
    
    fn supports_method(&self, method: &str) -> bool {
        method == "ion"
    }
}

/// DID:KEY resolver for simple DIDs
pub struct KeyResolver;

impl KeyResolver {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl DidResolver for KeyResolver {
    async fn resolve(&self, did: &str) -> Result<DidResolutionResult> {
        // Validate DID
        if !did.starts_with("did:key:") {
            return Err(anyhow!("Not a key DID: {}", did));
        }
        
        // Extract key material
        let key_material = did.replace("did:key:", "");
        
        // Create a simple DID document
        let verification_method = VerificationMethod {
            id: format!("{}#0", did),
            type_: "Ed25519VerificationKey2020".to_string(),
            controller: did.to_string(),
            public_key_multibase: Some(key_material.to_string()),
            public_key_jwk: None,
        };
        
        let did_document = DidDocument {
            id: did.to_string(),
            controller: Some(vec![did.to_string()]),
            verification_method: Some(vec![verification_method.clone()]),
            authentication: Some(vec![format!("{}#0", did)]),
            assertion_method: Some(vec![format!("{}#0", did)]),
            key_agreement: None,
            capability_invocation: None,
            capability_delegation: None,
            service: None,
            also_known_as: None,
        };
        
        Ok(DidResolutionResult {
            did_document: Some(did_document),
            metadata: DidResolutionMetadata {
                content_type: Some("application/json".to_string()),
                error: None,
                cached: false,
                retrieved_time: Some(chrono::Utc::now().timestamp() as u64),
            },
        })
    }
    
    fn supports_method(&self, method: &str) -> bool {
        method == "key"
    }
}

/// Web5 resolver that supports web5 DIDs
pub struct Web5Resolver {
    client: Client,
}

impl Web5Resolver {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait]
impl DidResolver for Web5Resolver {
    async fn resolve(&self, did: &str) -> Result<DidResolutionResult> {
        // Validate DID
        if !did.starts_with("did:web5:") {
            return Err(anyhow!("Not a Web5 DID: {}", did));
        }
        
        // In a real implementation, this would contact the appropriate DWN
        // For now, we create a simple document
        let verification_method = VerificationMethod {
            id: format!("{}#0", did),
            type_: "JsonWebKey2020".to_string(),
            controller: did.to_string(),
            public_key_multibase: None,
            public_key_jwk: Some(serde_json::from_str(r#"{"kty":"EC","crv":"P-256","x":"example","y":"example"}"#)?),
        };
        
        let service = Service {
            id: format!("{}#dwn", did),
            type_: "DecentralizedWebNode".to_string(),
            service_endpoint: ServiceEndpoint::String("https://dwn.tbddev.org/".to_string()),
        };
        
        let did_document = DidDocument {
            id: did.to_string(),
            controller: Some(vec![did.to_string()]),
            verification_method: Some(vec![verification_method.clone()]),
            authentication: Some(vec![format!("{}#0", did)]),
            assertion_method: Some(vec![format!("{}#0", did)]),
            key_agreement: None,
            capability_invocation: None,
            capability_delegation: None,
            service: Some(vec![service]),
            also_known_as: None,
        };
        
        Ok(DidResolutionResult {
            did_document: Some(did_document),
            metadata: DidResolutionMetadata {
                content_type: Some("application/json".to_string()),
                error: None,
                cached: false,
                retrieved_time: Some(chrono::Utc::now().timestamp() as u64),
            },
        })
    }
    
    fn supports_method(&self, method: &str) -> bool {
        method == "web5"
    }
}

/// Universal resolver that supports multiple methods with caching
pub struct UniversalResolver {
    resolvers: Vec<Box<dyn DidResolver>>,
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    max_cache_size: usize,
    cache_ttl: Duration,
}

impl UniversalResolver {
    pub fn new() -> Self {
        Self {
            resolvers: Vec::new(),
            cache: Arc::new(RwLock::new(HashMap::new())),
            max_cache_size: 1000,
            cache_ttl: Duration::from_secs(3600), // 1 hour default
        }
    }
    
    pub fn with_cache_config(mut self, max_size: usize, ttl_seconds: u64) -> Self {
        self.max_cache_size = max_size;
        self.cache_ttl = Duration::from_secs(ttl_seconds);
        self
    }
    
    pub fn add_resolver(&mut self, resolver: Box<dyn DidResolver>) {
        self.resolvers.push(resolver);
    }
    
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }
    
    pub async fn is_cached(&self, did: &str) -> bool {
        let cache = self.cache.read().await;
        cache.contains_key(did)
    }
    
    pub async fn get_cache_size(&self) -> usize {
        let cache = self.cache.read().await;
        cache.len()
    }
    
    async fn cleanup_cache(&self) {
        let mut cache = self.cache.write().await;
        
        // Remove expired entries
        let now = Instant::now();
        cache.retain(|_, entry| now.duration_since(entry.timestamp) < self.cache_ttl);
        
        // If still too large, remove oldest entries
        if cache.len() > self.max_cache_size {
            let mut entries: Vec<_> = cache.iter().collect();
            entries.sort_by_key(|(_, entry)| entry.timestamp);
            
            let to_remove = entries.len() - self.max_cache_size;
            for i in 0..to_remove {
                if i < entries.len() {
                    cache.remove(entries[i].0);
                }
            }
        }
    }
    
    pub async fn resolve(&self, did: &str) -> Result<DidResolutionResult> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(entry) = cache.get(did) {
                let now = Instant::now();
                if now.duration_since(entry.timestamp) < self.cache_ttl {
                    return Ok(DidResolutionResult {
                        did_document: Some(entry.did_document.clone()),
                        metadata: DidResolutionMetadata {
                            content_type: Some("application/json".to_string()),
                            error: None,
                            cached: true,
                            retrieved_time: Some(chrono::Utc::now().timestamp() as u64),
                        },
                    });
                }
            }
        }
        
        // Extract method
        let parts: Vec<&str> = did.split(":").collect();
        if parts.len() < 3 || parts[0] != "did" {
            return Err(anyhow!("Invalid DID format: {}", did));
        }
        
        let method = parts[1];
        
        // Find resolver for this method
        for resolver in &self.resolvers {
            if resolver.supports_method(method) {
                match resolver.resolve(did).await {
                    Ok(result) => {
                        // Cache successful resolution
                        if let Some(doc) = &result.did_document {
                            let mut cache = self.cache.write().await;
                            cache.insert(did.to_string(), CacheEntry {
                                did_document: doc.clone(),
                                timestamp: Instant::now(),
                            });
                            
                            // Run cleanup asynchronously
                            let cache_ref = self.cache.clone();
                            let ttl = self.cache_ttl;
                            let max_size = self.max_cache_size;
                            tokio::spawn(async move {
                                let mut cache = cache_ref.write().await;
                                
                                // Remove expired entries
                                let now = Instant::now();
                                cache.retain(|_, entry| now.duration_since(entry.timestamp) < ttl);
                                
                                // If still too large, remove oldest entries
                                if cache.len() > max_size {
                                    let mut entries: Vec<_> = cache.iter().collect();
                                    entries.sort_by_key(|(_, entry)| entry.timestamp);
                                    
                                    let to_remove = entries.len() - max_size;
                                    for i in 0..to_remove {
                                        if i < entries.len() {
                                            cache.remove(entries[i].0);
                                        }
                                    }
                                }
                            });
                        }
                        
                        return Ok(result);
                    },
                    Err(e) => {
                        return Ok(DidResolutionResult {
                            did_document: None,
                            metadata: DidResolutionMetadata {
                                content_type: Some("application/json".to_string()),
                                error: Some(format!("Resolution error: {}", e)),
                                cached: false,
                                retrieved_time: Some(chrono::Utc::now().timestamp() as u64),
                            },
                        });
                    }
                }
            }
        }
        
        // No resolver found
        Ok(DidResolutionResult {
            did_document: None,
            metadata: DidResolutionMetadata {
                content_type: Some("application/json".to_string()),
                error: Some(format!("Unsupported DID method: {}", method)),
                cached: false,
                retrieved_time: Some(chrono::Utc::now().timestamp() as u64),
            },
        })
    }
}

// Helper function to create a default universal resolver
pub fn create_default_resolver() -> UniversalResolver {
    let mut resolver = UniversalResolver::new().with_cache_config(5000, 7200); // 2 hour cache, 5000 entries
    resolver.add_resolver(Box::new(IonResolver::new(None)));
    resolver.add_resolver(Box::new(KeyResolver::new()));
    resolver.add_resolver(Box::new(Web5Resolver::new()));
    resolver
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_key_resolver() {
        let resolver = KeyResolver::new();
        let test_did = "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK";
        
        let result = resolver.resolve(test_did).await.unwrap();
        
        assert!(result.did_document.is_some());
        let doc = result.did_document.unwrap();
        assert_eq!(doc.id, test_did);
        assert!(doc.verification_method.is_some());
    }
    
    #[tokio::test]
    async fn test_caching() {
        let resolver = create_default_resolver();
        let test_did = "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK";
        
        // First resolution
        let result = resolver.resolve(test_did).await.unwrap();
        assert!(!result.metadata.cached);
        
        // Second resolution should be cached
        let result2 = resolver.resolve(test_did).await.unwrap();
        assert!(result2.metadata.cached);
    }
}
