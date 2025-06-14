use anya_core::{
    auth::{AuthCredentials, AuthManager},
    infrastructure::{Database, Monitoring},
    ml::{FileTracker, ModelTrainer},
};
use tokio;

mod auth;
mod infrastructure;
mod ml;

pub(crate) async fn setup_test_db() -> Database {
    let db_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:test@localhost:5432/anya_test".to_string());

    let db = Database::new(&db_url)
        .await
        .expect("Failed to connect to test database");
    db.run_migrations().await.expect("Failed to run migrations");
    db
}
