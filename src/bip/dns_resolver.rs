// [AIR-3][AIS-3][BPC-3][AIT-3] BIP353 DNS Resolver Implementation

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use thiserror::Error;
use tokio::time::timeout;
use tracing::{debug, error};
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    proto::rr::Name,
    TokioAsyncResolver,
};

// BIP353 record format: _bitcoin._wallet.example.org
const BITCOIN_SERVICE: &str = "_bitcoin";
const WALLET_SERVICE: &str = "_wallet";
const DNS_TIMEOUT_SECS: u64 = 5;

/// DNS Resolver errors for BIP353
#[derive(Error, Debug)]
pub enum DnsResolverError {
    #[error("DNS resolution error: {0}")]
    Resolution(String),

    #[error("Timeout error: {0}")]
    Timeout(String),

    #[error("DNSSEC validation error: {0}")]
    DnssecValidation(String),

    #[error("Record format error: {0}")]
    RecordFormat(String),

    #[error("No valid TXT records found")]
    NoValidRecords,

    #[error("DNS resolver initialization error: {0}")]
    Initialization(String),
}

/// Result type for DNS operations
pub type DnsResult<T> = Result<T, DnsResolverError>;

/// Cache entry for DNS resolution
struct CacheEntry {
    pub txt_records: Vec<String>,
    pub expires_at: u64,
    pub is_secure: bool,
}

/// DNS Resolver for BIP353
pub struct DnsResolver {
    resolver: TokioAsyncResolver,
    cache: Arc<Mutex<HashMap<String, CacheEntry>>>,
    validate_dnssec: bool,
    cache_duration: u64, // in seconds
}

impl DnsResolver {
    /// Create a new DNS resolver
    pub async fn new(validate_dnssec: bool, cache_duration: u64) -> DnsResult<Self> {
        let mut opts = ResolverOpts::default();
        opts.validate = validate_dnssec;

        let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), opts);

        Ok(Self {
            resolver,
            cache: Arc::new(Mutex::new(HashMap::new())),
            validate_dnssec,
            cache_duration,
        })
    }

    /// Resolve a payment username
    pub async fn resolve_payment_username(
        &self,
        user: &str,
        domain: &str,
    ) -> DnsResult<Vec<String>> {
        // Check cache first
        let cache_key = format!("{}@{}", user, domain);

        // Check for cached entry
        if let Some(cached) = self.check_cache(&cache_key) {
            debug!("Using cached DNS records for {}", cache_key);
            return Ok(cached);
        }

        // Format the DNS name for BIP353
        let name = self.format_dns_name(user, domain)?;

        debug!("Resolving DNS TXT records for {}", name);

        // Query DNS with timeout
        let lookup_future = self.resolver.txt_lookup(name.clone());
        let lookup_result =
            match timeout(Duration::from_secs(DNS_TIMEOUT_SECS), lookup_future).await {
                Ok(result) => result.map_err(|e| DnsResolverError::Resolution(e.to_string())),
                Err(_) => Err(DnsResolverError::Timeout(format!(
                    "DNS lookup for {} timed out",
                    name
                ))),
            }?;

        // Process TXT records
        let mut txt_records = Vec::new();
        let mut is_secure = false;

        // Use trust-dns-resolver to get the TXT records
        for record in lookup_result.iter() {
            for txt in record.iter() {
                let txt_data = String::from_utf8_lossy(txt).to_string();
                debug!("Found TXT record: {}", txt_data);
                txt_records.push(txt_data);
            }
        }

        // Check for DNSSEC if validation required
        if self.validate_dnssec {
            // For DNSSEC validation, we assume the resolver handles it
            // The trust-dns-resolver will automatically validate if configured
            debug!("DNSSEC validation enabled for {}", cache_key);
            is_secure = true; // trust-dns handles DNSSEC internally when validate=true
        }

        if txt_records.is_empty() {
            return Err(DnsResolverError::NoValidRecords);
        }

        // Cache the results
        self.cache_result(&cache_key, txt_records.clone(), is_secure);

        Ok(txt_records)
    }

    /// Format the DNS name according to BIP353
    fn format_dns_name(&self, user: &str, domain: &str) -> DnsResult<Name> {
        // Format: _bitcoin._wallet.<user>.<domain>
        let dns_name = format!("{}.{}.{}.{}", BITCOIN_SERVICE, WALLET_SERVICE, user, domain);

        Name::from_ascii(&dns_name).map_err(|e| DnsResolverError::RecordFormat(e.to_string()))
    }

    /// Check the cache for existing results
    fn check_cache(&self, cache_key: &str) -> Option<Vec<String>> {
        let cache = self.cache.lock().unwrap();

        if let Some(entry) = cache.get(cache_key) {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            if entry.expires_at > now {
                return Some(entry.txt_records.clone());
            }
        }

        None
    }

    /// Cache the resolution result
    fn cache_result(&self, cache_key: &str, txt_records: Vec<String>, is_secure: bool) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let expires_at = now + self.cache_duration;

        let entry = CacheEntry {
            txt_records,
            expires_at,
            is_secure,
        };

        let mut cache = self.cache.lock().unwrap();
        cache.insert(cache_key.to_string(), entry);
    }

    /// Clear the cache
    pub fn clear_cache(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }

    /// Update resolver configuration
    pub fn update_config(&mut self, validate_dnssec: bool, cache_duration: u64) -> DnsResult<()> {
        // Only recreate resolver if DNSSEC setting changed
        if self.validate_dnssec != validate_dnssec {
            let mut opts = ResolverOpts::default();
            opts.validate = validate_dnssec;

            let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), opts);

            self.resolver = resolver;
            self.validate_dnssec = validate_dnssec;
        }

        self.cache_duration = cache_duration;

        // Clear the cache when configuration changes
        self.clear_cache();

        Ok(())
    }
}

/// Parse BIP353 encoded payment instruction
pub fn parse_payment_instruction(txt_record: &str) -> Option<String> {
    // According to BIP353, the format should be:
    // bitcoin=<payment-instruction>

    if txt_record.starts_with("bitcoin=") {
        Some(txt_record[8..].to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;

    // Mock DNS resolver for testing
    mock! {
        DnsResolver {
            async fn resolve_payment_username(&self, user: &str, domain: &str) -> DnsResult<Vec<String>>;
            fn format_dns_name(&self, user: &str, domain: &str) -> DnsResult<Name>;
            fn check_cache(&self, cache_key: &str) -> Option<Vec<String>>;
            fn cache_result(&self, cache_key: &str, txt_records: Vec<String>, is_secure: bool);
            fn clear_cache(&self);
            fn update_config(&mut self, validate_dnssec: bool, cache_duration: u64) -> DnsResult<()>;
        }
    }

    #[tokio::test]
    async fn test_parse_payment_instruction() {
        let valid_txt = "bitcoin=lnurl1dp68gurn8ghj7um9wfmxjcm99e3k7mf0v9cxj0m385ekvcenxc6r2c35xvukxefcv5mkvv34x5ekzd3ev56nyd3hxqurzepexejxxepnxscrvwfnv9nxzcn9xq6xyefhvgcxxcmyxymnserxfq5fns";
        let invalid_txt = "not_bitcoin=something";

        assert_eq!(
            parse_payment_instruction(valid_txt),
            Some("lnurl1dp68gurn8ghj7um9wfmxjcm99e3k7mf0v9cxj0m385ekvcenxc6r2c35xvukxefcv5mkvv34x5ekzd3ev56nyd3hxqurzepexejxxepnxscrvwfnv9nxzcn9xq6xyefhvgcxxcmyxymnserxfq5fns".to_string())
        );

        assert_eq!(parse_payment_instruction(invalid_txt), None);
    }

    #[tokio::test]
    async fn test_format_dns_name() {
        // We'll use the real implementation for this test
        let resolver = DnsResolver {
            resolver: TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default()),
            cache: Arc::new(Mutex::new(HashMap::new())),
            validate_dnssec: false,
            cache_duration: 3600,
        };

        let name = resolver.format_dns_name("alice", "example.org").unwrap();
        assert_eq!(name.to_string(), "_bitcoin._wallet.alice.example.org.");
    }
}
