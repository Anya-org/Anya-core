#![feature(edition2021)]
//! Cache performance testing

use crate::testing::performance::{
    PerformanceTestable, TestConfig, TestResult, PerfTestError, Result, Timer, MetricType
};
use std::collections::{HashMap, VecDeque};
use rand::{thread_rng, Rng, distributions::{Distribution, Zipf}};
use std::time::Duration;

/// Cache algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheAlgorithm {
    /// Least Recently Used
    LRU,
    
    /// First In First Out
    FIFO,
    
    /// Random Replacement
    Random,
}

/// Cache access pattern
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessPattern {
    /// Uniform random access
    Uniform,
    
    /// Zipfian distribution (skewed access)
    Zipfian,
    
    /// Sequential access
    Sequential,
    
    /// Repeated access to a small set
    Repeated,
}

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Cache size in items
    pub size: usize,
    
    /// Cache algorithm
    pub algorithm: CacheAlgorithm,
    
    /// Access pattern
    pub access_pattern: AccessPattern,
    
    /// Key space size
    pub key_space_size: usize,
    
    /// Zipfian parameter (if using Zipfian distribution)
    pub zipf_param: f64,
    
    /// Repeated set size (if using repeated pattern)
    pub repeated_set_size: usize,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            size: 1000,
            algorithm: CacheAlgorithm::LRU,
            access_pattern: AccessPattern::Zipfian,
            key_space_size: 10000,
            zipf_param: 1.07,  // Typical web traffic
            repeated_set_size: 100,
        }
    }
}

/// Simple cache implementation
#[derive(Debug)]
pub struct SimpleCache<K, V> {
    /// Cache algorithm
    algorithm: CacheAlgorithm,
    
    /// Maximum size
    max_size: usize,
    
    /// Current size
    current_size: usize,
    
    /// Data
    data: HashMap<K, V>,
    
    /// Access order for LRU
    access_order: VecDeque<K>,
    
    /// Insertion order for FIFO
    insertion_order: VecDeque<K>,
    
    /// Stats
    stats: CacheStats,
}

/// Cache statistics
#[derive(Debug, Default, Clone)]
pub struct CacheStats {
    /// Cache hits
    pub hits: usize,
    
    /// Cache misses
    pub misses: usize,
    
    /// Evictions
    pub evictions: usize,
    
    /// Insertions
    pub insertions: usize,
    
    /// Read operations
    pub reads: usize,
    
    /// Write operations
    pub writes: usize,
    
    /// Total time spent in read operations (ms)
    pub read_time_ms: u64,
    
    /// Total time spent in write operations (ms)
    pub write_time_ms: u64,
}

impl<K: Clone + Eq + std::hash::Hash, V: Clone> SimpleCache<K, V> {
    /// Create a new simple cache
    pub fn new(algorithm: CacheAlgorithm, max_size: usize) -> Self {
        Self {
            algorithm,
            max_size,
            current_size: 0,
            data: HashMap::new(),
            access_order: VecDeque::new(),
            insertion_order: VecDeque::new(),
            stats: CacheStats::default(),
        }
    }
    
    /// Get an item from the cache
    pub fn get(&mut self, key: &K) -> Option<V> {
        let mut timer = Timer::new();
        timer.start();
        
        let result = self.data.get(key).cloned();
        
        if result.is_some() {
            self.stats.hits += 1;
            
            // Update access order for LRU
            if self.algorithm == CacheAlgorithm::LRU {
                // Remove from current position
                if let Some(pos) = self.access_order.iter().position(|k| k == key) {
                    self.access_order.remove(pos);
                }
                // Add to the end
                self.access_order.push_back(key.clone());
            }
        } else {
            self.stats.misses += 1;
        }
        
        self.stats.reads += 1;
        
        timer.stop();
        if let Ok(elapsed) = timer.elapsed_ms() {
            self.stats.read_time_ms += elapsed;
        }
        
        result
    }
    
    /// Put an item in the cache
    pub fn put(&mut self, key: K, value: V) {
        let mut timer = Timer::new();
        timer.start();
        
        // Check if key already exists
        let is_new = !self.data.contains_key(&key);
        
        // Add to data
        self.data.insert(key.clone(), value);
        
        if is_new {
            self.stats.insertions += 1;
            
            // Update size
            self.current_size += 1;
            
            // Update insertion order for FIFO
            self.insertion_order.push_back(key.clone());
            
            // Update access order for LRU
            if self.algorithm == CacheAlgorithm::LRU {
                self.access_order.push_back(key);
            }
            
            // Evict if necessary
            self.evict_if_needed();
        } else {
            // Update access order for LRU
            if self.algorithm == CacheAlgorithm::LRU {
                // Remove from current position
                if let Some(pos) = self.access_order.iter().position(|k| k == &key) {
                    self.access_order.remove(pos);
                }
                // Add to the end
                self.access_order.push_back(key);
            }
        }
        
        self.stats.writes += 1;
        
        timer.stop();
        if let Ok(elapsed) = timer.elapsed_ms() {
            self.stats.write_time_ms += elapsed;
        }
    }
    
    /// Evict an item from the cache if needed
    fn evict_if_needed(&mut self) {
        if self.current_size <= self.max_size {
            return;
        }
        
        match self.algorithm {
            CacheAlgorithm::LRU => {
                // Evict least recently used
                if let Some(key) = self.access_order.pop_front() {
                    self.data.remove(&key);
                    self.current_size -= 1;
                    self.stats.evictions += 1;
                }
            }
            CacheAlgorithm::FIFO => {
                // Evict first in
                if let Some(key) = self.insertion_order.pop_front() {
                    self.data.remove(&key);
                    self.current_size -= 1;
                    self.stats.evictions += 1;
                }
            }
            CacheAlgorithm::Random => {
                // Evict random item
                let mut rng = thread_rng();
                if !self.data.is_empty() {
                    let keys: Vec<K> = self.data.keys().cloned().collect();
                    let idx = rng.gen_range(0..keys.len());
                    let key = &keys[idx];
                    self.data.remove(key);
                    self.current_size -= 1;
                    self.stats.evictions += 1;
                    
                    // Remove from access and insertion orders
                    if let Some(pos) = self.access_order.iter().position(|k| k == key) {
                        self.access_order.remove(pos);
                    }
                    if let Some(pos) = self.insertion_order.iter().position(|k| k == key) {
                        self.insertion_order.remove(pos);
                    }
                }
            }
        }
    }
    
    /// Get stats
    pub fn get_stats(&self) -> CacheStats {
        self.stats.clone()
    }
    
    /// Reset stats
    pub fn reset_stats(&mut self) {
        self.stats = CacheStats::default();
    }
}

/// Cache performance test
pub struct CachePerformanceTest {
    /// Cache configuration
    config: CacheConfig,
    
    /// Access keys (for sequential and repeated patterns)
    keys: Vec<String>,
}

impl CachePerformanceTest {
    /// Create a new cache performance test
    pub fn new(config: CacheConfig) -> Self {
        let mut keys = Vec::with_capacity(config.key_space_size);
        
        for i in 0..config.key_space_size {
            keys.push(format!("key_{}", i));
        }
        
        Self {
            config,
            keys,
        }
    }
    
    /// Generate a key based on the access pattern
    fn generate_key(&self, iteration: usize) -> String {
        match self.config.access_pattern {
            AccessPattern::Uniform => {
                let mut rng = thread_rng();
                let idx = rng.gen_range(0..self.config.key_space_size);
                self.keys[idx].clone()
            }
            AccessPattern::Zipfian => {
                let mut rng = thread_rng();
                let zipf = Zipf::new(self.config.key_space_size as u64, self.config.zipf_param)
                    .expect("Invalid Zipf distribution parameters");
                let idx = zipf.sample(&mut rng) as usize - 1;
                self.keys[idx].clone()
            }
            AccessPattern::Sequential => {
                let idx = iteration % self.config.key_space_size;
                self.keys[idx].clone()
            }
            AccessPattern::Repeated => {
                let mut rng = thread_rng();
                let set_size = self.config.repeated_set_size.min(self.config.key_space_size);
                let idx = rng.gen_range(0..set_size);
                self.keys[idx].clone()
            }
        }
    }
    
    /// Generate a random value
    fn generate_value(&self) -> String {
        let mut rng = thread_rng();
        let size = rng.gen_range(10..100);
        let mut value = String::with_capacity(size);
        
        for _ in 0..size {
            let c = rng.gen_range(0..26) as u8 + b'a';
            value.push(c as char);
        }
        
        value
    }
    
    /// Run a test with a specific algorithm and access pattern
    fn run_algorithm_test(&self, iterations: usize) -> Result<CacheStats> {
        let mut cache = SimpleCache::<String, String>::new(
            self.config.algorithm,
            self.config.size
        );
        
        // Run test iterations
        for i in 0..iterations {
            let key = self.generate_key(i);
            
            // 80% of operations are reads, 20% are writes
            let mut rng = thread_rng();
            let is_read = rng.gen_range(0..100) < 80;
            
            if is_read {
                let _ = cache.get(&key);
            } else {
                let value = self.generate_value();
                cache.put(key, value);
            }
        }
        
        Ok(cache.get_stats())
    }
}

impl PerformanceTestable for CachePerformanceTest {
    fn run_test(&self, config: &TestConfig) -> Result<TestResult> {
        let iterations = config.iterations;
        let warmup_iterations = config.warmup_iterations;
        
        // Parameters
        let mut parameters = HashMap::new();
        parameters.insert("cache_size".to_string(), self.config.size.to_string());
        parameters.insert("algorithm".to_string(), format!("{:?}", self.config.algorithm));
        parameters.insert("access_pattern".to_string(), format!("{:?}", self.config.access_pattern));
        parameters.insert("key_space_size".to_string(), self.config.key_space_size.to_string());
        
        if self.config.access_pattern == AccessPattern::Zipfian {
            parameters.insert("zipf_param".to_string(), self.config.zipf_param.to_string());
        }
        
        if self.config.access_pattern == AccessPattern::Repeated {
            parameters.insert("repeated_set_size".to_string(), self.config.repeated_set_size.to_string());
        }
        
        // Warmup (this initializes the system and JIT compiler)
        println!("Warming up cache for {} iterations...", warmup_iterations);
        if warmup_iterations > 0 {
            let _ = self.run_algorithm_test(warmup_iterations);
        }
        
        // Actual test
        println!("Running cache test for {} iterations with {:?} algorithm and {:?} access pattern...", 
                iterations, self.config.algorithm, self.config.access_pattern);
        
        let mut timer = Timer::new();
        timer.start();
        
        let stats = self.run_algorithm_test(iterations)?;
        
        timer.stop();
        
        // Calculate results
        let duration_ms = timer.elapsed_ms()?;
        
        // Calculate metrics
        let mut metrics = HashMap::new();
        let mut metric_types = HashMap::new();
        
        // Cache hit rate
        let total_reads = stats.hits + stats.misses;
        let cache_hit_rate = if total_reads > 0 {
            (stats.hits as f64) / (total_reads as f64) * 100.0
        } else {
            0.0
        };
        
        metrics.insert("cache_hit_rate".to_string(), cache_hit_rate);
        metric_types.insert("cache_hit_rate".to_string(), MetricType::CacheHitRate);
        
        // Operations per second
        let total_ops = stats.reads + stats.writes;
        let ops_per_second = (total_ops as f64) / (duration_ms as f64 / 1000.0);
        
        metrics.insert("operations_per_second".to_string(), ops_per_second);
        metric_types.insert("operations_per_second".to_string(), MetricType::DbOpsPerSecond);
        
        // Average read latency
        if stats.reads > 0 {
            let avg_read_ms = (stats.read_time_ms as f64) / (stats.reads as f64);
            metrics.insert("avg_read_latency_ms".to_string(), avg_read_ms);
            metric_types.insert("avg_read_latency_ms".to_string(), MetricType::LatencyMs);
        }
        
        // Average write latency
        if stats.writes > 0 {
            let avg_write_ms = (stats.write_time_ms as f64) / (stats.writes as f64);
            metrics.insert("avg_write_latency_ms".to_string(), avg_write_ms);
            metric_types.insert("avg_write_latency_ms".to_string(), MetricType::LatencyMs);
        }
        
        // Eviction rate
        let eviction_rate = (stats.evictions as f64) / (stats.insertions as f64) * 100.0;
        metrics.insert("eviction_rate".to_string(), eviction_rate);
        metric_types.insert("eviction_rate".to_string(), MetricType::CacheHitRate);
        
        Ok(TestResult {
            name: format!("{}_{:?}_{:?}", self.name(), 
                         self.config.algorithm, 
                         self.config.access_pattern),
            timestamp: chrono::Utc::now().to_rfc3339(),
            duration_ms,
            metrics,
            metric_types,
            parameters,
        })
    }
    
    fn name(&self) -> &str {
        "cache_performance"
    }
}

/// Create a standard set of cache performance tests
pub fn create_standard_cache_tests() -> Vec<Box<dyn PerformanceTestable>> {
    let mut tests = Vec::new();
    
    // LRU tests with different access patterns
    tests.push(Box::new(CachePerformanceTest::new(CacheConfig {
        algorithm: CacheAlgorithm::LRU,
        access_pattern: AccessPattern::Uniform,
        ..CacheConfig::default()
    })));
    
    tests.push(Box::new(CachePerformanceTest::new(CacheConfig {
        algorithm: CacheAlgorithm::LRU,
        access_pattern: AccessPattern::Zipfian,
        ..CacheConfig::default()
    })));
    
    tests.push(Box::new(CachePerformanceTest::new(CacheConfig {
        algorithm: CacheAlgorithm::LRU,
        access_pattern: AccessPattern::Sequential,
        ..CacheConfig::default()
    })));
    
    tests.push(Box::new(CachePerformanceTest::new(CacheConfig {
        algorithm: CacheAlgorithm::LRU,
        access_pattern: AccessPattern::Repeated,
        ..CacheConfig::default()
    })));
    
    // FIFO tests with different access patterns
    tests.push(Box::new(CachePerformanceTest::new(CacheConfig {
        algorithm: CacheAlgorithm::FIFO,
        access_pattern: AccessPattern::Uniform,
        ..CacheConfig::default()
    })));
    
    tests.push(Box::new(CachePerformanceTest::new(CacheConfig {
        algorithm: CacheAlgorithm::FIFO,
        access_pattern: AccessPattern::Zipfian,
        ..CacheConfig::default()
    })));
    
    // Random tests with different access patterns
    tests.push(Box::new(CachePerformanceTest::new(CacheConfig {
        algorithm: CacheAlgorithm::Random,
        access_pattern: AccessPattern::Uniform,
        ..CacheConfig::default()
    })));
    
    tests.push(Box::new(CachePerformanceTest::new(CacheConfig {
        algorithm: CacheAlgorithm::Random,
        access_pattern: AccessPattern::Zipfian,
        ..CacheConfig::default()
    })));
    
    tests
} 