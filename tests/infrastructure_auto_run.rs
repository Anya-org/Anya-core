//! Auto-run tests for infrastructure module

use anya_core::infrastructure::{Database, Monitoring, MonitoringConfig};

#[tokio::test]
async fn test_database_new_and_migrations() {
    let db = Database::new("sqlite://memory").await.expect("Database creation failed");
    db.run_migrations().await.expect("Migrations failed");
}

#[tokio::test]
async fn test_monitoring_new_and_start() {
    let config = MonitoringConfig::default();
    let monitoring = Monitoring::new(config);
    monitoring.start().await.expect("Monitoring start failed");
}
