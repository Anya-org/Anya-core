// [AIR-3][AIS-3][BPC-3][AIT-3] Bitcoin Improvement Proposal implementations

pub mod bip353;
pub mod bip353_auth;
pub mod dns_resolver;
pub mod health;
pub mod validation;

pub use bip353::{BetaFeatures, Bip353, Bip353Config, Bip353Error, Bip353Status, PaymentRecipient};
pub use bip353_auth::{
    AuthSession, BetaAccessConfig, BetaAccessError, BetaAccessManager, BetaAuthToken,
};
pub use health::{BipDetail, BipHealthChecker, BipHealthReport};
pub use validation::{BipValidator, ComplianceStatus};
