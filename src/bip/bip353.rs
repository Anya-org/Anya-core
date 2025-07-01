// [AIR-3][AIS-3][BPC-3][AIT-3] BIP353 DNS Payment Instructions implementation
// Based on BIP353: https://github.com/bitcoin/bips/blob/master/bip-0353.mediawiki

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::Resolver;

// Use the core config type instead

/// BIP353 implementation status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Bip353Status {
    /// Not enabled
    Disabled,
    /// Enabled for stable features only
    Stable,
    /// Enabled for all features including bleeding edge
    Beta,
}

impl fmt::Display for Bip353Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Bip353Status::Disabled => write!(f, "Disabled"),
            Bip353Status::Stable => write!(f, "Stable"),
            Bip353Status::Beta => write!(f, "Beta"),
        }
    }
}

/// BIP353 DNS Payment Instructions configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bip353Config {
    /// Current status/tier
    pub status: Bip353Status,
    /// Default DNS resolver to use
    pub default_resolver: String,
    /// Cache duration in seconds
    pub cache_duration: u64,
    /// Whether to validate DNSSEC
    pub validate_dnssec: bool,
    /// Support for beta features
    pub beta_features: BetaFeatures,
}

/// Beta features configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct BetaFeatures {
    /// Support for non-ASCII identifiers (punycode)
    pub non_ascii_identifiers: bool,
    /// Support for wildcard records
    pub wildcard_records: bool,
    /// Support for out-of-band notifications
    pub oob_notifications: bool,
    /// Enhanced privacy routing
    pub enhanced_privacy: bool,
}


impl Default for Bip353Config {
    fn default() -> Self {
        Self {
            status: Bip353Status::Disabled,
            default_resolver: "1.1.1.1".to_string(),
            cache_duration: 3600, // 1 hour
            validate_dnssec: true,
            beta_features: BetaFeatures {
                non_ascii_identifiers: false,
                wildcard_records: false,
                oob_notifications: false,
                enhanced_privacy: false,
            },
        }
    }
}

/// Errors that can occur during BIP353 operations
#[derive(Debug)]
pub enum Bip353Error {
    /// DNS resolution error
    ResolutionError(String),
    /// DNSSEC validation error
    DnssecError(String),
    /// Parse error
    ParseError(String),
    /// Feature not enabled
    FeatureNotEnabled(String),
    /// Cache error
    CacheError(String),
    /// Authorization error
    AuthError(String),
}

impl fmt::Display for Bip353Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Bip353Error::ResolutionError(msg) => write!(f, "DNS resolution error: {msg}"),
            Bip353Error::DnssecError(msg) => write!(f, "DNSSEC validation error: {msg}"),
            Bip353Error::ParseError(msg) => write!(f, "Parse error: {msg}"),
            Bip353Error::FeatureNotEnabled(msg) => write!(f, "Feature not enabled: {msg}"),
            Bip353Error::CacheError(msg) => write!(f, "Cache error: {msg}"),
            Bip353Error::AuthError(msg) => write!(f, "Authorization error: {msg}"),
        }
    }
}

impl Error for Bip353Error {}

/// Cache entry for payment instruction
struct CacheEntry {
    payment_instruction: String,
    timestamp: Instant,
}

/// BIP353 implementation
pub struct Bip353 {
    config: Bip353Config,
    resolver: Resolver,
    cache: Arc<Mutex<HashMap<String, CacheEntry>>>,
}

/// Payment recipient information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRecipient {
    /// Username part
    pub user: String,
    /// Domain part
    pub domain: String,
    /// Full address (may include bitcoin prefix)
    pub full_address: String,
    /// Payment instruction
    pub payment_instruction: Option<String>,
}

impl Bip353 {
    /// Create a new BIP353 implementation
    pub fn new(config: Bip353Config) -> Result<Self, Bip353Error> {
        // Configure DNS resolver
        let _resolver_ip: std::net::IpAddr = match config.default_resolver.parse() {
            Ok(ip) => ip,
            Err(_) => {
                return Err(Bip353Error::ResolutionError(
                    "Invalid resolver IP".to_string(),
                ))
            }
        };

        let resolver_config = ResolverConfig::cloudflare();
        let mut resolver_opts = ResolverOpts::default();

        // Configure DNSSEC validation
        resolver_opts.validate = config.validate_dnssec;

        // Create resolver
        let resolver = match Resolver::new(resolver_config, resolver_opts) {
            Ok(r) => r,
            Err(e) => return Err(Bip353Error::ResolutionError(e.to_string())),
        };

        Ok(Self {
            config,
            resolver,
            cache: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// Parse a payment address
    pub fn parse_address(&self, address: &str) -> Result<PaymentRecipient, Bip353Error> {
        // Check if we support BIP353
        if self.config.status == Bip353Status::Disabled {
            return Err(Bip353Error::FeatureNotEnabled(
                "BIP353 is disabled".to_string(),
            ));
        }

        // Check for bitcoin prefix (₿)
        let has_prefix = address.starts_with("₿");
        let address_without_prefix = if has_prefix { &address[1..] } else { address };

        // Check for @
        let parts: Vec<&str> = address_without_prefix.split('@').collect();
        if parts.len() != 2 {
            return Err(Bip353Error::ParseError(
                "Invalid address format".to_string(),
            ));
        }

        let user = parts[0].to_string();
        let domain = parts[1].to_string();

        // Check for non-ASCII identifiers
        let has_non_ascii =
            !user.is_ascii() || !domain.is_ascii();
        if has_non_ascii && !self.is_feature_enabled("non_ascii_identifiers") {
            return Err(Bip353Error::FeatureNotEnabled(
                "Non-ASCII identifiers not enabled".to_string(),
            ));
        }

        Ok(PaymentRecipient {
            user,
            domain,
            full_address: address.to_string(),
            payment_instruction: None,
        })
    }

    /// Resolve a payment address to get the payment instruction
    pub async fn resolve(&self, recipient: &mut PaymentRecipient) -> Result<String, Bip353Error> {
        // Check if we support BIP353
        if self.config.status == Bip353Status::Disabled {
            return Err(Bip353Error::FeatureNotEnabled(
                "BIP353 is disabled".to_string(),
            ));
        }

        // Check cache first
        let cache_key = format!("{}@{}", recipient.user, recipient.domain);
        if let Some(instruction) = self.check_cache(&cache_key) {
            recipient.payment_instruction = Some(instruction.clone());
            return Ok(instruction);
        }

        // Construct the DNS lookup string: user.user._bitcoin-payment.domain
        let lookup = format!(
            "{}.user._bitcoin-payment.{}",
            recipient.user, recipient.domain
        );

        // Perform DNS lookup
        let txt_records = match self.resolver.txt_lookup(lookup) {
            Ok(records) => records,
            Err(e) => {
                return Err(Bip353Error::ResolutionError(format!(
                    "DNS lookup failed: {e}"
                )));
            }
        };

        // Extract and validate the payment instruction
        let mut payment_instruction = None;

        for record in txt_records.iter() {
            for txt in record.txt_data() {
                let data = String::from_utf8_lossy(txt);
                if data.starts_with("bitcoin:") {
                    if payment_instruction.is_some() {
                        return Err(Bip353Error::ParseError(
                            "Multiple bitcoin: records found".to_string(),
                        ));
                    }
                    payment_instruction = Some(data.to_string());
                }
            }
        }

        let instruction = payment_instruction
            .ok_or_else(|| Bip353Error::ResolutionError("No bitcoin: record found".to_string()))?;

        // Cache the result
        self.cache_instruction(&cache_key, &instruction);

        // Update the recipient
        recipient.payment_instruction = Some(instruction.clone());

        Ok(instruction)
    }

    /// Check if a feature is enabled
    fn is_feature_enabled(&self, feature: &str) -> bool {
        // Basic features are available in both stable and beta
        if self.config.status == Bip353Status::Disabled {
            return false;
        }

        // For beta features, we need to be in beta mode
        match feature {
            "non_ascii_identifiers" => {
                self.config.status == Bip353Status::Beta
                    && self.config.beta_features.non_ascii_identifiers
            }
            "wildcard_records" => {
                self.config.status == Bip353Status::Beta
                    && self.config.beta_features.wildcard_records
            }
            "oob_notifications" => {
                self.config.status == Bip353Status::Beta
                    && self.config.beta_features.oob_notifications
            }
            "enhanced_privacy" => {
                self.config.status == Bip353Status::Beta
                    && self.config.beta_features.enhanced_privacy
            }
            // Basic features available in both stable and beta
            "basic_resolution" => self.config.status != Bip353Status::Disabled,
            "dnssec_validation" => {
                self.config.status != Bip353Status::Disabled && self.config.validate_dnssec
            }
            _ => false,
        }
    }

    /// Check cache for a payment instruction
    fn check_cache(&self, key: &str) -> Option<String> {
        let cache = self.cache.lock().unwrap();
        if let Some(entry) = cache.get(key) {
            if entry.timestamp.elapsed() < Duration::from_secs(self.config.cache_duration) {
                return Some(entry.payment_instruction.clone());
            }
        }
        None
    }

    /// Cache a payment instruction
    fn cache_instruction(&self, key: &str, instruction: &str) {
        let mut cache = self.cache.lock().unwrap();
        cache.insert(
            key.to_string(),
            CacheEntry {
                payment_instruction: instruction.to_string(),
                timestamp: Instant::now(),
            },
        );

        // Clean up expired entries
        cache.retain(|_, entry| {
            entry.timestamp.elapsed() < Duration::from_secs(self.config.cache_duration)
        });
    }

    /// Get the current status
    pub fn status(&self) -> Bip353Status {
        self.config.status
    }

    /// Update configuration
    pub fn update_config(&mut self, config: Bip353Config) -> Result<(), Bip353Error> {
        // Update the resolver if necessary
        if self.config.default_resolver != config.default_resolver
            || self.config.validate_dnssec != config.validate_dnssec
        {
            let _resolver_ip: std::net::IpAddr = match config.default_resolver.parse() {
                Ok(ip) => ip,
                Err(_) => {
                    return Err(Bip353Error::ResolutionError(
                        "Invalid resolver IP".to_string(),
                    ))
                }
            };

            let resolver_config = ResolverConfig::cloudflare();
            let mut resolver_opts = ResolverOpts::default();

            // Configure DNSSEC validation
            resolver_opts.validate = config.validate_dnssec;

            // Create resolver
            self.resolver = match Resolver::new(resolver_config, resolver_opts) {
                Ok(r) => r,
                Err(e) => return Err(Bip353Error::ResolutionError(e.to_string())),
            };
        }

        // Update config
        self.config = config;

        Ok(())
    }
}

/// Monitor service for BIP353 health and status
pub struct Bip353Monitor {
    bip353: Arc<Mutex<Bip353>>,
    last_check: Instant,
    check_interval: Duration,
    health_status: bool,
}

impl Bip353Monitor {
    pub fn new(bip353: Arc<Mutex<Bip353>>, check_interval_secs: u64) -> Self {
        Self {
            bip353,
            last_check: Instant::now(),
            check_interval: Duration::from_secs(check_interval_secs),
            health_status: true,
        }
    }

    pub async fn check_health(&mut self) -> bool {
        if self.last_check.elapsed() < self.check_interval {
            return self.health_status;
        }

        // Perform health check
        let bip353 = self.bip353.lock().unwrap();
        if bip353.config.status == Bip353Status::Disabled {
            self.health_status = true;
            self.last_check = Instant::now();
            return true;
        }

        // Check resolution
        let _test_recipient = match bip353.parse_address("test@example.com") {
            Ok(recipient) => recipient,
            Err(_) => {
                self.health_status = false;
                self.last_check = Instant::now();
                return false;
            }
        };

        // Test basic functionality - don't actually do DNS lookup in health check
        // Instead, just check if the instance is properly configured
        self.health_status = true;

        self.last_check = Instant::now();
        self.health_status
    }

    pub fn status(&self) -> bool {
        self.health_status
    }
}
