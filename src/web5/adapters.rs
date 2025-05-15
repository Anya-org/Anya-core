// Web5 adapter implementation
use std::sync::Arc;
use crate::config::Web5Config;
use crate::core::AnyaResult;

/// Adapter for Web5 DID and DWN functionality
pub struct Web5Adapter {
    config: Web5Config,
}

impl Web5Adapter {
    pub async fn build(config: Web5Config) -> AnyaResult<Arc<Self>> {
        Ok(Arc::new(Self { config }))
    }
}
