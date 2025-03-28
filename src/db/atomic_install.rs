use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::Path;
use std::fs;

// Database state tracking for atomic installation
pub struct DatabaseStateManager {
    pool: PgPool,
    backup_dir: String,
    installation_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseState {
    timestamp: u64,
    installation_id: String,
    phase: String,
    db_snapshot: String,
    can_rollback: bool,
}

impl DatabaseStateManager {
    pub async fn new(connection_str: &str, backup_dir: &str) -> Result<Self> {
        // Create a connection pool
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(connection_str)
            .await
            .context("Failed to connect to database")?;
            
        // Create installation ID
        let installation_id = format!("install_{}", 
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .context("Time went backwards")?
                .as_secs()
        );
        
        // Ensure backup directory exists
        let backup_path = Path::new(backup_dir);
        if !backup_path.exists() {
            fs::create_dir_all(backup_path)
                .context("Failed to create backup directory")?;
        }
        
        // Create the state manager
        let manager = Self {
            pool,
            backup_dir: backup_dir.to_string(),
            installation_id,
        };
        
        // Initialize the state tracking table if it doesn't exist
        manager.initialize_state_tracking().await?;
        
        Ok(manager)
    }
    
    async fn initialize_state_tracking(&self) -> Result<()> {
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS installation_states (
                id SERIAL PRIMARY KEY,
                timestamp BIGINT NOT NULL,
                installation_id VARCHAR(255) NOT NULL,
                phase VARCHAR(255) NOT NULL,
                db_snapshot VARCHAR(255) NOT NULL,
                can_rollback BOOLEAN NOT NULL DEFAULT TRUE
            );
        "#)
        .execute(&self.pool)
        .await
        .context("Failed to create installation_states table")?;
        
        Ok(())
    }
    
    pub async fn save_state(&self, phase_name: &str) -> Result<()> {
        // Create snapshot filename
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("Time went backwards")?
            .as_secs();
            
        let snapshot_file = format!("{}/{}_{}.sql", 
            self.backup_dir, self.installation_id, phase_name);
            
        // Create database snapshot using pg_dump
        let output = std::process::Command::new("pg_dump")
            .args(["-f", &snapshot_file])
            .output()
            .context("Failed to execute pg_dump")?;
            
        if !output.status.success() {
            return Err(anyhow!("pg_dump failed: {}", 
                String::from_utf8_lossy(&output.stderr)));
        }
        
        // Record this state in the tracking table
        sqlx::query(r#"
            INSERT INTO installation_states 
            (timestamp, installation_id, phase, db_snapshot, can_rollback)
            VALUES ($1, $2, $3, $4, $5)
        "#)
        .bind(timestamp as i64)
        .bind(&self.installation_id)
        .bind(phase_name)
        .bind(&snapshot_file)
        .bind(true)
        .execute(&self.pool)
        .await
        .context("Failed to record installation state")?;
        
        Ok(())
    }
    
    pub async fn rollback_to_phase(&self, phase_name: &str) -> Result<()> {
        // Find the snapshot for the given phase
        let state = sqlx::query_as!(
            DatabaseState,
            r#"
            SELECT 
                timestamp, installation_id, phase, 
                db_snapshot, can_rollback
            FROM installation_states 
            WHERE installation_id = $1 AND phase = $2
            "#,
            self.installation_id,
            phase_name
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to query installation state")?;
        
        let state = match state {
            Some(s) => s,
            None => return Err(anyhow!("No saved state found for phase: {}", phase_name)),
        };
        
        if !state.can_rollback {
            return Err(anyhow!("Rollback is not allowed for phase: {}", phase_name));
        }
        
        // Execute the rollback using psql
        let output = std::process::Command::new("psql")
            .args(["-f", &state.db_snapshot])
            .output()
            .context("Failed to execute psql for rollback")?;
            
        if !output.status.success() {
            return Err(anyhow!("Database rollback failed: {}", 
                String::from_utf8_lossy(&output.stderr)));
        }
        
        // Record that we've rolled back
        println!("Successfully rolled back to phase: {}", phase_name);
        
        Ok(())
    }
    
    pub async fn complete_installation(&self) -> Result<()> {
        // Mark all states as non-rollbackable once installation is complete
        sqlx::query(r#"
            UPDATE installation_states
            SET can_rollback = false
            WHERE installation_id = $1
        "#)
        .bind(&self.installation_id)
        .execute(&self.pool)
        .await
        .context("Failed to mark installation as complete")?;
        
        Ok(())
    }
    
    pub async fn cleanup_old_snapshots(&self, days_to_keep: u64) -> Result<()> {
        // Find installation states older than the specified days
        let cutoff_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("Time went backwards")?
            .as_secs() - (days_to_keep * 86400);
            
        let old_states = sqlx::query_as!(
            DatabaseState,
            r#"
            SELECT 
                timestamp, installation_id, phase, 
                db_snapshot, can_rollback
            FROM installation_states 
            WHERE timestamp < $1
            "#,
            cutoff_timestamp as i64
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to query old installation states")?;
        
        // Remove old snapshots
        for state in old_states {
            // Delete the snapshot file
            if let Err(e) = fs::remove_file(&state.db_snapshot) {
                println!("Warning: Could not delete old snapshot {}: {}", 
                    state.db_snapshot, e);
            }
            
            // Remove the record
            sqlx::query(r#"
                DELETE FROM installation_states
                WHERE installation_id = $1 AND phase = $2
            "#)
            .bind(&state.installation_id)
            .bind(&state.phase)
            .execute(&self.pool)
            .await
            .context("Failed to delete old installation state record")?;
        }
        
        Ok(())
    }
} 