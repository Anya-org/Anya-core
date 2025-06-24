use std::env;

fn main() {
    // Detect OS and set appropriate feature flags
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/monitoring/blockchain_metrics.rs");
    println!("cargo:rerun-if-changed=src/monitoring/blockchain_alerts.rs");
    println!("cargo:rerun-if-changed=src/monitoring/metrics_service.rs");
    println!("cargo:rerun-if-changed=src/monitoring/metrics_api.rs");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_OS");
    println!("cargo:rerun-if-env-changed=ANYA_METRICS_COLLECTION_INTERVAL_MS");
    println!("cargo:rerun-if-env-changed=ANYA_METRICS_PORT");

    match target_os.as_str() {
        "linux" => {
            println!("cargo:rustc-cfg=feature=\"secure-storage-linux\"");
        }
        "windows" => {
            println!("cargo:rustc-cfg=feature=\"secure-storage-windows\"");
        }
        "macos" => {
            println!("cargo:rustc-cfg=feature=\"secure-storage-macos\"");
        }
        _ => {
            println!("cargo:rustc-cfg=feature=\"secure-storage-fallback\"");
        }
    }
}
