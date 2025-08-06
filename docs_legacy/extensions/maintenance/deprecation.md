# Deprecation Management Guide [AIR-3][AIS-3][AIT-3][RES-3]

Comprehensive deprecation management for Anya-core extensions, ensuring smooth transitions while maintaining Bitcoin BIP compliance, Web5 protocol compatibility, and ML system stability.

## Overview

The Anya-core deprecation system provides structured pathways for phasing out outdated functionality while maintaining backward compatibility and providing clear migration paths. All deprecation follows semantic versioning and includes extensive developer notifications.

## Deprecation Policy

### Deprecation Timeline

- **Announcement**: 6 months before removal
- **Warning Phase**: 3 months of runtime warnings
- **Final Notice**: 1 month of error-level warnings
- **Removal**: Complete removal in next major version

### Deprecation Categories

- **API Deprecation**: Function, method, and interface removal
- **Feature Deprecation**: Complete feature or module removal
- **Protocol Deprecation**: Outdated Bitcoin or Web5 protocol support
- **Configuration Deprecation**: Settings and parameter changes
- **Dependency Deprecation**: Third-party library replacements

## Deprecation Management System

### Deprecation Tracker

```rust
use chrono::{DateTime, Utc, Duration};
use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeprecationInfo {
    pub item_type: DeprecationType,
    pub item_name: String,
    pub deprecated_in_version: Version,
    pub removal_target_version: Version,
    pub deprecation_date: DateTime<Utc>,
    pub removal_date: DateTime<Utc>,
    pub reason: String,
    pub migration_guide: String,
    pub alternatives: Vec<Alternative>,
    pub impact_level: ImpactLevel,
    pub affects_bip_compliance: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeprecationType {
    Function,
    Method,
    Class,
    Module,
    Configuration,
    Protocol,
    Feature,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactLevel {
    Low,     // Internal implementation detail
    Medium,  // Public API with alternatives
    High,    // Core functionality change
    Critical,// Security or compliance related
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alternative {
    pub name: String,
    pub description: String,
    pub migration_complexity: MigrationComplexity,
    pub available_since: Version,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MigrationComplexity {
    Trivial,    // Drop-in replacement
    Simple,     // Minor API changes
    Moderate,   // Structural changes required
    Complex,    // Significant refactoring needed
}

pub struct DeprecationManager {
    deprecations: Vec<DeprecationInfo>,
    notification_service: NotificationService,
    migration_analyzer: MigrationAnalyzer,
}

impl DeprecationManager {
    pub fn new() -> Self {
        Self {
            deprecations: load_deprecation_registry(),
            notification_service: NotificationService::new(),
            migration_analyzer: MigrationAnalyzer::new(),
        }
    }
    
    pub fn deprecate_item(&mut self, deprecation: DeprecationInfo) -> Result<(), DeprecationError> {
        // Validate deprecation timeline
        self.validate_deprecation_timeline(&deprecation)?;
        
        // Check for conflicts with existing deprecations
        self.check_deprecation_conflicts(&deprecation)?;
        
        // Add to registry
        self.deprecations.push(deprecation.clone());
        
        // Generate migration guide
        let migration_guide = self.migration_analyzer.generate_migration_guide(&deprecation)?;
        
        // Send notifications
        self.notification_service.notify_deprecation(&deprecation, &migration_guide)?;
        
        // Update documentation
        self.update_deprecation_documentation(&deprecation)?;
        
        Ok(())
    }
    
    pub fn get_active_deprecations(&self) -> Vec<&DeprecationInfo> {
        let now = Utc::now();
        self.deprecations.iter()
            .filter(|dep| dep.deprecation_date <= now && dep.removal_date > now)
            .collect()
    }
    
    pub fn get_deprecations_by_impact(&self, impact: ImpactLevel) -> Vec<&DeprecationInfo> {
        self.deprecations.iter()
            .filter(|dep| std::mem::discriminant(&dep.impact_level) == std::mem::discriminant(&impact))
            .collect()
    }
}
```

### Deprecation Annotations

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};

/// Mark a function as deprecated with migration information
#[proc_macro_attribute]
pub fn deprecated_fn(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let args = parse_macro_input!(args as DeprecationArgs);
    
    let fn_name = &input_fn.sig.ident;
    let deprecated_since = &args.since;
    let removal_version = &args.removal;
    let alternative = &args.alternative;
    let migration_guide = &args.migration_guide;
    
    let expanded = quote! {
        #[deprecated(
            since = #deprecated_since,
            note = "This function will be removed in version {removal_version}. Use {alternative} instead. Migration guide: {migration_guide}"
        )]
        #input_fn
        
        // Runtime warning on first use
        static DEPRECATION_WARNING_SHOWN: std::sync::Once = std::sync::Once::new();
        
        impl Drop for #fn_name {
            fn drop(&mut self) {
                DEPRECATION_WARNING_SHOWN.call_once(|| {
                    eprintln!("⚠️  DEPRECATION WARNING: Function '{}' is deprecated since v{} and will be removed in v{}.", 
                             stringify!(#fn_name), #deprecated_since, #removal_version);
                    eprintln!("   Alternative: {}", #alternative);
                    eprintln!("   Migration guide: {}", #migration_guide);
                });
            }
        }
    };
    
    TokenStream::from(expanded)
}

/// Example usage of deprecation annotation
#[deprecated_fn(
    since = "2.5.0",
    removal = "3.0.0",
    alternative = "validate_transaction_v2",
    migration_guide = "https://docs.anya.org/migration/transaction-validation"
)]
pub fn validate_transaction(tx: &Transaction) -> Result<ValidationResult, ValidationError> {
    // Legacy implementation
    warn!("Using deprecated validate_transaction function. Migrate to validate_transaction_v2.");
    validate_transaction_legacy(tx)
}
```

## Bitcoin Protocol Deprecation

### BIP Deprecation Management

```rust
pub struct BipDeprecationManager {
    supported_bips: HashMap<u32, BipInfo>,
    deprecated_bips: HashMap<u32, BipDeprecationInfo>,
}

#[derive(Debug, Clone)]
pub struct BipInfo {
    pub number: u32,
    pub title: String,
    pub status: BipStatus,
    pub implementation_version: Version,
    pub deprecation_info: Option<BipDeprecationInfo>,
}

#[derive(Debug, Clone)]
pub struct BipDeprecationInfo {
    pub reason: String,
    pub superseded_by: Option<u32>,
    pub deprecation_date: DateTime<Utc>,
    pub removal_date: DateTime<Utc>,
    pub migration_complexity: MigrationComplexity,
}

impl BipDeprecationManager {
    pub fn deprecate_bip(&mut self, bip_number: u32, deprecation: BipDeprecationInfo) -> Result<(), BipError> {
        // Validate BIP exists and is currently supported
        let bip_info = self.supported_bips.get_mut(&bip_number)
            .ok_or(BipError::BipNotFound(bip_number))?;
        
        // Check if BIP is required for core functionality
        if self.is_core_bip(bip_number) {
            return Err(BipError::CannotDeprecateCoreBip(bip_number));
        }
        
        // Update BIP status
        bip_info.status = BipStatus::Deprecated;
        bip_info.deprecation_info = Some(deprecation.clone());
        
        // Add to deprecated BIPs registry
        self.deprecated_bips.insert(bip_number, deprecation);
        
        // Notify affected components
        self.notify_bip_deprecation(bip_number)?;
        
        Ok(())
    }
    
    pub fn get_migration_path(&self, deprecated_bip: u32) -> Result<MigrationPath, BipError> {
        let deprecation_info = self.deprecated_bips.get(&deprecated_bip)
            .ok_or(BipError::BipNotDeprecated(deprecated_bip))?;
        
        let migration_path = if let Some(superseded_by) = deprecation_info.superseded_by {
            MigrationPath::Replacement {
                from_bip: deprecated_bip,
                to_bip: superseded_by,
                complexity: deprecation_info.migration_complexity.clone(),
            }
        } else {
            MigrationPath::Removal {
                bip: deprecated_bip,
                alternative_approach: self.get_alternative_approach(deprecated_bip)?,
            }
        };
        
        Ok(migration_path)
    }
}
```

### Legacy Transaction Format Deprecation

```rust
/// Example: Deprecating legacy transaction serialization
pub struct TransactionSerializer {
    format_version: u32,
}

impl TransactionSerializer {
    #[deprecated_fn(
        since = "2.4.0",
        removal = "3.0.0",
        alternative = "serialize_witness_transaction",
        migration_guide = "https://docs.anya.org/migration/witness-transactions"
    )]
    pub fn serialize_legacy_transaction(&self, tx: &Transaction) -> Result<Vec<u8>, SerializationError> {
        warn!("Legacy transaction serialization is deprecated. Use witness serialization for BIP 141 compliance.");
        
        // Check if transaction has witness data
        if tx.has_witness() {
            return Err(SerializationError::WitnessDataInLegacyFormat);
        }
        
        // Legacy serialization logic
        self.serialize_legacy_format(tx)
    }
    
    pub fn serialize_witness_transaction(&self, tx: &Transaction) -> Result<Vec<u8>, SerializationError> {
        // Modern BIP 141 compliant serialization
        self.serialize_witness_format(tx)
    }
}
```

## Web5 Protocol Deprecation

### DID Method Deprecation

```rust
pub struct DidMethodManager {
    supported_methods: HashMap<String, DidMethodInfo>,
    deprecated_methods: HashMap<String, MethodDeprecationInfo>,
}

#[derive(Debug, Clone)]
pub struct MethodDeprecationInfo {
    pub method_name: String,
    pub reason: String,
    pub superseded_by: Option<String>,
    pub security_issues: Vec<SecurityIssue>,
    pub migration_deadline: DateTime<Utc>,
}

impl DidMethodManager {
    pub fn deprecate_did_method(&mut self, method: &str, deprecation: MethodDeprecationInfo) -> Result<(), DidError> {
        // Validate method exists
        if !self.supported_methods.contains_key(method) {
            return Err(DidError::MethodNotSupported(method.to_string()));
        }
        
        // Check for active DIDs using this method
        let active_dids = self.count_active_dids_for_method(method)?;
        if active_dids > 0 {
            warn!("Deprecating DID method '{}' with {} active DIDs", method, active_dids);
        }
        
        // Add to deprecated methods
        self.deprecated_methods.insert(method.to_string(), deprecation.clone());
        
        // Generate migration notices for affected DIDs
        self.notify_affected_did_holders(method, &deprecation)?;
        
        Ok(())
    }
    
    #[deprecated_fn(
        since = "1.8.0",
        removal = "2.0.0",
        alternative = "resolve_did_web5",
        migration_guide = "https://docs.web5.org/migration/did-methods"
    )]
    pub fn resolve_did_legacy(&self, did: &str) -> Result<DidDocument, DidError> {
        warn!("Legacy DID resolution is deprecated. Use Web5-native resolution.");
        
        // Check if using deprecated method
        if let Some(method) = self.extract_did_method(did) {
            if self.deprecated_methods.contains_key(&method) {
                let deprecation = &self.deprecated_methods[&method];
                warn!("DID method '{}' is deprecated: {}", method, deprecation.reason);
                
                if let Some(alternative) = &deprecation.superseded_by {
                    warn!("Consider migrating to method: {}", alternative);
                }
            }
        }
        
        self.resolve_legacy_format(did)
    }
}
```

## ML Model Deprecation

### Model Version Management

```rust
pub struct ModelVersionManager {
    models: HashMap<String, Vec<ModelVersion>>,
    deprecated_models: HashMap<String, ModelDeprecationInfo>,
}

#[derive(Debug, Clone)]
pub struct ModelVersion {
    pub version: Version,
    pub model_path: PathBuf,
    pub accuracy: f64,
    pub created_at: DateTime<Utc>,
    pub deprecation_info: Option<ModelDeprecationInfo>,
}

#[derive(Debug, Clone)]
pub struct ModelDeprecationInfo {
    pub reason: String,
    pub accuracy_threshold: f64,
    pub replacement_model: Option<String>,
    pub migration_script: Option<PathBuf>,
    pub deprecation_date: DateTime<Utc>,
}

impl ModelVersionManager {
    pub fn deprecate_model_version(&mut self, model_name: &str, version: &Version, deprecation: ModelDeprecationInfo) -> Result<(), ModelError> {
        // Find the model version
        let model_versions = self.models.get_mut(model_name)
            .ok_or(ModelError::ModelNotFound(model_name.to_string()))?;
        
        let model_version = model_versions.iter_mut()
            .find(|v| v.version == *version)
            .ok_or(ModelError::VersionNotFound(version.clone()))?;
        
        // Check if this is the only version
        if model_versions.len() == 1 {
            return Err(ModelError::CannotDeprecateOnlyVersion);
        }
        
        // Mark as deprecated
        model_version.deprecation_info = Some(deprecation.clone());
        
        // Add to deprecated models registry
        let key = format!("{}:{}", model_name, version);
        self.deprecated_models.insert(key, deprecation);
        
        // Notify users of this model
        self.notify_model_users(model_name, version)?;
        
        Ok(())
    }
    
    #[deprecated_fn(
        since = "1.5.0",
        removal = "2.0.0",
        alternative = "predict_with_model_v2",
        migration_guide = "https://docs.anya.org/migration/ml-models"
    )]
    pub fn predict_with_legacy_model(&self, model_name: &str, input: &[f64]) -> Result<Prediction, ModelError> {
        warn!("Legacy model prediction interface is deprecated. Use v2 interface for better performance.");
        
        // Check if model version is deprecated
        if let Some(deprecation) = self.get_model_deprecation(model_name) {
            warn!("Model '{}' is deprecated: {}", model_name, deprecation.reason);
            
            if let Some(replacement) = &deprecation.replacement_model {
                warn!("Consider migrating to model: {}", replacement);
            }
        }
        
        self.predict_legacy_format(model_name, input)
    }
}
```

## Migration Assistance

### Automated Migration Tools

```rust
pub struct MigrationAssistant {
    code_analyzer: CodeAnalyzer,
    migration_generator: MigrationGenerator,
    test_generator: TestGenerator,
}

impl MigrationAssistant {
    pub fn analyze_codebase_for_deprecations(&self, codebase_path: &Path) -> Result<MigrationReport, MigrationError> {
        let mut report = MigrationReport::new();
        
        // Scan for deprecated API usage
        let deprecated_usages = self.code_analyzer.find_deprecated_usages(codebase_path)?;
        
        for usage in deprecated_usages {
            let migration_strategy = self.generate_migration_strategy(&usage)?;
            report.add_migration_item(MigrationItem {
                file_path: usage.file_path,
                line_number: usage.line_number,
                deprecated_item: usage.item_name,
                migration_strategy,
                complexity: self.assess_migration_complexity(&usage)?,
                estimated_effort: self.estimate_migration_effort(&usage)?,
            });
        }
        
        // Generate automated migration scripts
        let migration_scripts = self.migration_generator.generate_scripts(&report)?;
        report.set_migration_scripts(migration_scripts);
        
        // Generate migration tests
        let migration_tests = self.test_generator.generate_migration_tests(&report)?;
        report.set_migration_tests(migration_tests);
        
        Ok(report)
    }
    
    pub fn apply_migration(&self, migration_item: &MigrationItem) -> Result<MigrationResult, MigrationError> {
        // Create backup
        let backup_path = self.create_backup(&migration_item.file_path)?;
        
        // Apply migration
        match self.apply_migration_strategy(&migration_item.migration_strategy) {
            Ok(result) => {
                // Validate migration
                if self.validate_migration(&migration_item.file_path)? {
                    Ok(result)
                } else {
                    // Restore from backup
                    self.restore_backup(&backup_path, &migration_item.file_path)?;
                    Err(MigrationError::ValidationFailed)
                }
            }
            Err(e) => {
                // Restore from backup on error
                self.restore_backup(&backup_path, &migration_item.file_path)?;
                Err(e)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct MigrationStrategy {
    pub strategy_type: MigrationStrategyType,
    pub steps: Vec<MigrationStep>,
    pub rollback_steps: Vec<MigrationStep>,
}

#[derive(Debug, Clone)]
pub enum MigrationStrategyType {
    DirectReplacement,
    RefactorRequired,
    FeatureRemoval,
    ProtocolUpgrade,
}

#[derive(Debug, Clone)]
pub struct MigrationStep {
    pub description: String,
    pub action: MigrationAction,
    pub validation: Option<ValidationRule>,
}
```

### Migration Documentation Generator

```rust
pub struct MigrationDocGenerator {
    template_engine: TemplateEngine,
    example_generator: ExampleGenerator,
}

impl MigrationDocGenerator {
    pub fn generate_migration_guide(&self, deprecation: &DeprecationInfo) -> Result<String, DocumentationError> {
        let template = match deprecation.item_type {
            DeprecationType::Function => "function_migration.md.template",
            DeprecationType::Module => "module_migration.md.template",
            DeprecationType::Protocol => "protocol_migration.md.template",
            _ => "generic_migration.md.template",
        };
        
        let context = MigrationContext {
            item_name: &deprecation.item_name,
            deprecated_version: &deprecation.deprecated_in_version,
            removal_version: &deprecation.removal_target_version,
            reason: &deprecation.reason,
            alternatives: &deprecation.alternatives,
            examples: self.generate_examples(deprecation)?,
        };
        
        self.template_engine.render(template, &context)
    }
    
    fn generate_examples(&self, deprecation: &DeprecationInfo) -> Result<Vec<MigrationExample>, DocumentationError> {
        let mut examples = Vec::new();
        
        for alternative in &deprecation.alternatives {
            let before_example = self.example_generator.generate_before_example(deprecation)?;
            let after_example = self.example_generator.generate_after_example(alternative)?;
            
            examples.push(MigrationExample {
                title: format!("Migrating to {}", alternative.name),
                before: before_example,
                after: after_example,
                explanation: alternative.description.clone(),
            });
        }
        
        Ok(examples)
    }
}
```

## Deprecation Monitoring

### Usage Tracking

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;

pub struct DeprecationTracker {
    usage_counters: HashMap<String, AtomicUsize>,
    first_usage_times: HashMap<String, DateTime<Utc>>,
    last_usage_times: HashMap<String, DateTime<Utc>>,
}

impl DeprecationTracker {
    pub fn track_usage(&self, deprecated_item: &str) {
        let counter = self.usage_counters.get(deprecated_item)
            .unwrap_or(&AtomicUsize::new(0));
        
        counter.fetch_add(1, Ordering::Relaxed);
        
        let now = Utc::now();
        
        // Track first usage
        if !self.first_usage_times.contains_key(deprecated_item) {
            self.first_usage_times.insert(deprecated_item.to_string(), now);
        }
        
        // Update last usage
        self.last_usage_times.insert(deprecated_item.to_string(), now);
        
        // Log usage for monitoring
        debug!("Deprecated item '{}' used", deprecated_item);
    }
    
    pub fn generate_usage_report(&self) -> DeprecationUsageReport {
        let mut report = DeprecationUsageReport::new();
        
        for (item, counter) in &self.usage_counters {
            let usage_count = counter.load(Ordering::Relaxed);
            if usage_count > 0 {
                report.add_usage_stats(DeprecationUsageStats {
                    item_name: item.clone(),
                    usage_count,
                    first_used: self.first_usage_times.get(item).copied(),
                    last_used: self.last_usage_times.get(item).copied(),
                });
            }
        }
        
        report
    }
}

/// Macro for tracking deprecated function usage
macro_rules! track_deprecated_usage {
    ($tracker:expr, $item:expr) => {
        $tracker.track_usage($item);
        warn!("Using deprecated item: {}", $item);
    };
}
```

## Notification System

### Deprecation Alerts

```rust
pub struct DeprecationNotificationService {
    alert_channels: Vec<AlertChannel>,
    notification_scheduler: NotificationScheduler,
}

#[derive(Debug, Clone)]
pub enum AlertChannel {
    Email { recipients: Vec<String> },
    Slack { webhook_url: String },
    GitHub { issue_tracker: String },
    Documentation { update_path: PathBuf },
}

impl DeprecationNotificationService {
    pub async fn notify_deprecation(&self, deprecation: &DeprecationInfo) -> Result<(), NotificationError> {
        let notification = self.create_deprecation_notification(deprecation);
        
        for channel in &self.alert_channels {
            match channel {
                AlertChannel::Email { recipients } => {
                    self.send_email_notification(recipients, &notification).await?;
                }
                AlertChannel::Slack { webhook_url } => {
                    self.send_slack_notification(webhook_url, &notification).await?;
                }
                AlertChannel::GitHub { issue_tracker } => {
                    self.create_github_issue(issue_tracker, &notification).await?;
                }
                AlertChannel::Documentation { update_path } => {
                    self.update_documentation(update_path, &notification).await?;
                }
            }
        }
        
        // Schedule follow-up notifications
        self.schedule_followup_notifications(deprecation).await?;
        
        Ok(())
    }
    
    fn create_deprecation_notification(&self, deprecation: &DeprecationInfo) -> DeprecationNotification {
        DeprecationNotification {
            title: format!("🔔 Deprecation Notice: {}", deprecation.item_name),
            message: format!(
                "The {} '{}' has been deprecated in version {} and will be removed in version {}.\n\n\
                 Reason: {}\n\
                 Removal Date: {}\n\
                 Migration Guide: {}\n\n\
                 Alternative options:\n{}",
                deprecation.item_type.to_string().to_lowercase(),
                deprecation.item_name,
                deprecation.deprecated_in_version,
                deprecation.removal_target_version,
                deprecation.reason,
                deprecation.removal_date.format("%Y-%m-%d"),
                deprecation.migration_guide,
                deprecation.alternatives.iter()
                    .map(|alt| format!("  • {} - {}", alt.name, alt.description))
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
            severity: self.determine_notification_severity(deprecation),
            deprecation_info: deprecation.clone(),
        }
    }
}
```

## Best Practices

### Deprecation Guidelines

1. **Clear Communication**: Provide detailed reasoning and alternatives
2. **Sufficient Notice**: Follow the 6-month minimum timeline
3. **Migration Support**: Offer tools and documentation for migration
4. **Backward Compatibility**: Maintain compatibility during deprecation period
5. **Security Priority**: Fast-track security-related deprecations

### Deprecation Checklist

```rust
pub struct DeprecationChecklist {
    items: Vec<ChecklistItem>,
}

#[derive(Debug, Clone)]
pub struct ChecklistItem {
    pub task: String,
    pub completed: bool,
    pub required: bool,
    pub deadline: Option<DateTime<Utc>>,
}

impl DeprecationChecklist {
    pub fn create_for_deprecation(deprecation: &DeprecationInfo) -> Self {
        let mut checklist = Self { items: Vec::new() };
        
        checklist.add_item("Document deprecation reason", true, None);
        checklist.add_item("Identify alternatives", true, None);
        checklist.add_item("Create migration guide", true, None);
        checklist.add_item("Update API documentation", true, None);
        checklist.add_item("Add deprecation annotations", true, None);
        checklist.add_item("Notify stakeholders", true, Some(deprecation.deprecation_date));
        checklist.add_item("Create migration tools", false, Some(deprecation.deprecation_date + Duration::weeks(4)));
        checklist.add_item("Update examples and tutorials", false, Some(deprecation.deprecation_date + Duration::weeks(8)));
        checklist.add_item("Final removal implementation", true, Some(deprecation.removal_date));
        
        checklist
    }
    
    pub fn check_readiness(&self) -> Result<(), DeprecationError> {
        let missing_required: Vec<_> = self.items.iter()
            .filter(|item| item.required && !item.completed)
            .collect();
        
        if !missing_required.is_empty() {
            return Err(DeprecationError::IncompleteChecklist(
                missing_required.iter().map(|item| item.task.clone()).collect()
            ));
        }
        
        Ok(())
    }
}
```

## Resources

- [Semantic Versioning](https://semver.org/)
- [Rust Deprecation Guidelines](https://rust-lang.github.io/api-guidelines/future-proofing.html#public-dependencies-of-a-stable-crate-are-stable)
- [Bitcoin Core Deprecation Process](https://github.com/bitcoin/bitcoin/blob/master/doc/developer-notes.md#deprecating-rpcs)
- [Web5 Protocol Evolution](https://github.com/TBD54566975/web5-spec)
- [Update Management Guide](./updates.md)
- [Version Control Guide](./version-control.md)
- [Maintenance Overview](./README.md)

*Last updated: June 7, 2025*
