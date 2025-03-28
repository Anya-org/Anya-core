#![feature(edition2021)]
//! Database access pattern performance testing

use crate::testing::performance::{
    PerformanceTestable, TestConfig, TestResult, PerfTestError, Result, Timer, MetricType
};
use std::collections::HashMap;
use rand::{thread_rng, Rng};

/// Database operation type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DbOperation {
    /// Read operation
    Read,
    
    /// Write operation
    Write,
    
    /// Update operation
    Update,
    
    /// Delete operation
    Delete,
}

/// Database configuration
#[derive(Debug, Clone)]
pub struct DbConfig {
    /// Database type
    pub db_type: String,
    
    /// Connection string
    pub connection_string: String,
    
    /// Batch size
    pub batch_size: usize,
    
    /// Use prepared statements
    pub use_prepared_statements: bool,
    
    /// Cache size in MB
    pub cache_size_mb: usize,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            db_type: "sqlite".to_string(),
            connection_string: ":memory:".to_string(),
            batch_size: 100,
            use_prepared_statements: true,
            cache_size_mb: 10,
        }
    }
}

/// Simple mock database for testing
#[derive(Debug)]
pub struct MockDatabase {
    /// Configuration
    config: DbConfig,
    
    /// Data
    data: HashMap<String, Vec<u8>>,
    
    /// Cache
    cache: HashMap<String, Vec<u8>>,
    
    /// Stats
    stats: DbStats,
}

/// Database statistics
#[derive(Debug, Default, Clone)]
pub struct DbStats {
    /// Read operations
    pub reads: usize,
    
    /// Write operations
    pub writes: usize,
    
    /// Update operations
    pub updates: usize,
    
    /// Delete operations
    pub deletes: usize,
    
    /// Cache hits
    pub cache_hits: usize,
    
    /// Cache misses
    pub cache_misses: usize,
    
    /// Total time spent in read operations (ms)
    pub read_time_ms: u64,
    
    /// Total time spent in write operations (ms)
    pub write_time_ms: u64,
    
    /// Total time spent in update operations (ms)
    pub update_time_ms: u64,
    
    /// Total time spent in delete operations (ms)
    pub delete_time_ms: u64,
}

impl MockDatabase {
    /// Create a new mock database
    pub fn new(config: DbConfig) -> Self {
        Self {
            config,
            data: HashMap::new(),
            cache: HashMap::new(),
            stats: DbStats::default(),
        }
    }
    
    /// Read data
    pub fn read(&mut self, key: &str) -> Option<Vec<u8>> {
        let mut timer = Timer::new();
        timer.start();
        
        // Check cache first
        if let Some(value) = self.cache.get(key) {
            self.stats.cache_hits += 1;
            timer.stop();
            if let Ok(elapsed) = timer.elapsed_ms() {
                self.stats.read_time_ms += elapsed;
            }
            self.stats.reads += 1;
            return Some(value.clone());
        }
        
        self.stats.cache_misses += 1;
        
        // Check data store
        let result = self.data.get(key).cloned();
        
        // Update cache if found
        if let Some(value) = &result {
            self.cache.insert(key.to_string(), value.clone());
            
            // Simple cache size management
            if self.cache.len() > self.config.cache_size_mb * 1024 * 1024 / 100 {
                // Remove a random entry to simulate eviction
                if let Some(key) = self.cache.keys().next().cloned() {
                    self.cache.remove(&key);
                }
            }
        }
        
        timer.stop();
        if let Ok(elapsed) = timer.elapsed_ms() {
            self.stats.read_time_ms += elapsed;
        }
        self.stats.reads += 1;
        
        result
    }
    
    /// Write data
    pub fn write(&mut self, key: &str, value: Vec<u8>) {
        let mut timer = Timer::new();
        timer.start();
        
        self.data.insert(key.to_string(), value.clone());
        self.cache.insert(key.to_string(), value);
        
        timer.stop();
        if let Ok(elapsed) = timer.elapsed_ms() {
            self.stats.write_time_ms += elapsed;
        }
        self.stats.writes += 1;
    }
    
    /// Update data
    pub fn update(&mut self, key: &str, value: Vec<u8>) -> bool {
        let mut timer = Timer::new();
        timer.start();
        
        let result = self.data.insert(key.to_string(), value.clone()).is_some();
        self.cache.insert(key.to_string(), value);
        
        timer.stop();
        if let Ok(elapsed) = timer.elapsed_ms() {
            self.stats.update_time_ms += elapsed;
        }
        self.stats.updates += 1;
        
        result
    }
    
    /// Delete data
    pub fn delete(&mut self, key: &str) -> bool {
        let mut timer = Timer::new();
        timer.start();
        
        let result = self.data.remove(key).is_some();
        self.cache.remove(key);
        
        timer.stop();
        if let Ok(elapsed) = timer.elapsed_ms() {
            self.stats.delete_time_ms += elapsed;
        }
        self.stats.deletes += 1;
        
        result
    }
    
    /// Get stats
    pub fn get_stats(&self) -> DbStats {
        self.stats.clone()
    }
    
    /// Reset stats
    pub fn reset_stats(&mut self) {
        self.stats = DbStats::default();
    }
}

/// Database access pattern test
pub struct DatabaseAccessTest {
    /// Mock database
    db: MockDatabase,
    
    /// Operations to test
    operations: Vec<DbOperation>,
    
    /// Key space size
    key_space_size: usize,
    
    /// Value size in bytes
    value_size: usize,
}

impl DatabaseAccessTest {
    /// Create a new database access test
    pub fn new(config: DbConfig, operations: Vec<DbOperation>, key_space_size: usize, value_size: usize) -> Self {
        Self {
            db: MockDatabase::new(config),
            operations,
            key_space_size,
            value_size,
        }
    }
    
    /// Generate a random key
    fn random_key(&self) -> String {
        let mut rng = thread_rng();
        format!("key_{}", rng.gen_range(0..self.key_space_size))
    }
    
    /// Generate random data
    fn random_data(&self) -> Vec<u8> {
        let mut rng = thread_rng();
        let mut data = Vec::with_capacity(self.value_size);
        for _ in 0..self.value_size {
            data.push(rng.gen());
        }
        data
    }
    
    /// Run a test for a specific operation
    fn run_operation_test(&mut self, operation: DbOperation, iterations: usize) -> Result<f64> {
        self.db.reset_stats();
        
        let mut timer = Timer::new();
        timer.start();
        
        for _ in 0..iterations {
            match operation {
                DbOperation::Read => {
                    let key = self.random_key();
                    let _ = self.db.read(&key);
                }
                DbOperation::Write => {
                    let key = self.random_key();
                    let data = self.random_data();
                    self.db.write(&key, data);
                }
                DbOperation::Update => {
                    let key = self.random_key();
                    let data = self.random_data();
                    self.db.update(&key, data);
                }
                DbOperation::Delete => {
                    let key = self.random_key();
                    let _ = self.db.delete(&key);
                }
            }
        }
        
        timer.stop();
        
        let ops_per_second = (iterations as f64) / (timer.elapsed_secs()?);
        
        Ok(ops_per_second)
    }
}

impl PerformanceTestable for DatabaseAccessTest {
    fn run_test(&self, config: &TestConfig) -> Result<TestResult> {
        let iterations = config.iterations;
        let warmup_iterations = config.warmup_iterations;
        
        // Clone self to allow mutation during the test
        let mut test = DatabaseAccessTest {
            db: MockDatabase::new(self.db.config.clone()),
            operations: self.operations.clone(),
            key_space_size: self.key_space_size,
            value_size: self.value_size,
        };
        
        // Parameters
        let mut parameters = HashMap::new();
        parameters.insert("db_type".to_string(), test.db.config.db_type.clone());
        parameters.insert("batch_size".to_string(), test.db.config.batch_size.to_string());
        parameters.insert("use_prepared_statements".to_string(), test.db.config.use_prepared_statements.to_string());
        parameters.insert("cache_size_mb".to_string(), test.db.config.cache_size_mb.to_string());
        parameters.insert("key_space_size".to_string(), test.key_space_size.to_string());
        parameters.insert("value_size".to_string(), test.value_size.to_string());
        
        // Warmup
        println!("Warming up database for {} iterations...", warmup_iterations);
        // Populate some data for reads/updates/deletes
        for _ in 0..warmup_iterations / 10 {
            let key = test.random_key();
            let data = test.random_data();
            test.db.write(&key, data);
        }
        
        // Reset stats after warmup
        test.db.reset_stats();
        
        // Actual test
        println!("Running database test for {} iterations...", iterations);
        
        let mut timer = Timer::new();
        timer.start();
        
        let mut metrics = HashMap::new();
        let mut metric_types = HashMap::new();
        
        for operation in &test.operations {
            println!("Testing {:?} operations...", operation);
            let ops_per_second = test.run_operation_test(*operation, iterations / test.operations.len())?;
            
            let metric_name = match operation {
                DbOperation::Read => "read_ops_per_second",
                DbOperation::Write => "write_ops_per_second",
                DbOperation::Update => "update_ops_per_second",
                DbOperation::Delete => "delete_ops_per_second",
            };
            
            metrics.insert(metric_name.to_string(), ops_per_second);
            metric_types.insert(metric_name.to_string(), MetricType::DbOpsPerSecond);
        }
        
        // Overall metrics
        let stats = test.db.get_stats();
        
        // Cache hit rate
        let total_reads = stats.cache_hits + stats.cache_misses;
        let cache_hit_rate = if total_reads > 0 {
            (stats.cache_hits as f64) / (total_reads as f64) * 100.0
        } else {
            0.0
        };
        
        metrics.insert("cache_hit_rate".to_string(), cache_hit_rate);
        metric_types.insert("cache_hit_rate".to_string(), MetricType::CacheHitRate);
        
        // Average operation times
        if stats.reads > 0 {
            metrics.insert("avg_read_time_ms".to_string(), (stats.read_time_ms as f64) / (stats.reads as f64));
            metric_types.insert("avg_read_time_ms".to_string(), MetricType::LatencyMs);
        }
        
        if stats.writes > 0 {
            metrics.insert("avg_write_time_ms".to_string(), (stats.write_time_ms as f64) / (stats.writes as f64));
            metric_types.insert("avg_write_time_ms".to_string(), MetricType::LatencyMs);
        }
        
        if stats.updates > 0 {
            metrics.insert("avg_update_time_ms".to_string(), (stats.update_time_ms as f64) / (stats.updates as f64));
            metric_types.insert("avg_update_time_ms".to_string(), MetricType::LatencyMs);
        }
        
        if stats.deletes > 0 {
            metrics.insert("avg_delete_time_ms".to_string(), (stats.delete_time_ms as f64) / (stats.deletes as f64));
            metric_types.insert("avg_delete_time_ms".to_string(), MetricType::LatencyMs);
        }
        
        timer.stop();
        
        Ok(TestResult {
            name: self.name().to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            duration_ms: timer.elapsed_ms()?,
            metrics,
            metric_types,
            parameters,
        })
    }
    
    fn name(&self) -> &str {
        "database_access"
    }
} 