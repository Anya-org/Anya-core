// [AIR-3][AIS-3][BPC-3][AIT-3] Bitcoin Improvement Proposal implementations

pub mod validation;
pub mod bip353;
pub mod bip353_auth;
pub mod health;
pub mod dns_resolver;

pub use validation::{BipValidator, ComplianceStatus};
pub use bip353::{Bip353, Bip353Config, Bip353Status, Bip353Error, PaymentRecipient, BetaFeatures};
pub use bip353_auth::{BetaAccessManager, BetaAccessConfig, AuthSession, BetaAuthToken, BetaAccessError};
pub use health::{BipHealthChecker, BipHealthReport, BipDetail}; 