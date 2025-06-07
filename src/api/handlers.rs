/// API handlers for Anya Core
use crate::AnyaResult;

pub mod auth;
pub mod system;
pub mod wallet;
pub mod identity;
pub mod dlc;

pub struct ApiHandlers;

impl ApiHandlers {
    pub fn new() -> Self {
        Self
    }
}
