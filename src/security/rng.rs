#![feature(edition2021)]
use rand_core::OsRng;

pub fn generate_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
} 