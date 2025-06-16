pub mod auth;
pub mod dlc;
pub mod identity;
pub mod system;
pub mod wallet;

pub struct ApiHandlers;

impl ApiHandlers {
    pub fn new() -> Self {
        Self
    }
}
