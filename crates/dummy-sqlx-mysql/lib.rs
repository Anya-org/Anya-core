//! Stub sqlx-mysql crate
//! This crate intentionally provides no functionality. It exists solely to satisfy
//! a [patch.crates-io] override in the root Cargo.toml to block inclusion of the
//! real `sqlx-mysql` crate that currently pulls in a vulnerable `rsa` transitive dependency.
//!
//! DO NOT IMPLEMENT ANY REAL API HERE. If MySQL support is required in the future,
//! remove the patch override and audit the upstream dependency tree first.

#[allow(dead_code)]
pub const VERSION: &str = "0.0.0-stub";
