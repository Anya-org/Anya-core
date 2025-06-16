pub mod auth;
pub mod dlc;
pub mod identity;
pub mod system;
pub mod wallet;

pub struct ApiHandlers;

impl Default for ApiHandlers {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiHandlers {
    pub fn new() -> Self {
        Self
    }
}
