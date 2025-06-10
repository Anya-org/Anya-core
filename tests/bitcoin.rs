//! Bitcoin-related tests module
//! 
//! This module organizes all Bitcoin protocol, transaction, and layer tests.

// Individual test files
#[path = "bitcoin/cross_layer_tests.rs"]
pub mod cross_layer_tests;
#[path = "bitcoin/historical_compatibility_tests.rs"]
pub mod historical_compatibility_tests;
#[path = "bitcoin/layer3_tests.rs"]
pub mod layer3_tests;
#[path = "bitcoin/riscv_tests.rs"]
pub mod riscv_tests;
#[path = "bitcoin/riscv_vm_tests.rs"]
pub mod riscv_vm_tests;
#[path = "bitcoin/security_tests.rs"]
pub mod security_tests;
#[path = "bitcoin/validation_test.rs"]
pub mod validation_test;
#[path = "bitcoin/vm_layer_tests.rs"]
pub mod vm_layer_tests;

// Sub-modules from directories - explicit mod.rs paths
// NOTE: These modules have broken symlinks, commenting out for now
// #[path = "bitcoin/core/mod.rs"]
// pub mod core;
// #[path = "bitcoin/dlc/mod.rs"]
// pub mod dlc;
// #[path = "bitcoin/integration/mod.rs"]
// pub mod integration;
// #[path = "bitcoin/layer2/mod.rs"]
// pub mod layer2;
#[path = "bitcoin/protocol/mod.rs"]
pub mod protocol;
#[path = "bitcoin/riscv/mod.rs"]
pub mod riscv;
