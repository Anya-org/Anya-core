use sysinfo::{System, SystemExt, CpuExt, DiskExt};
use anyhow::{Result, Context};
use std::time::{Duration, Instant};
use std::thread;
use std::path::PathBuf;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SystemCheckStatus {
    Pass,
    Warning,
    Fail,
    Pending,
}

impl fmt::Display for SystemCheckStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SystemCheckStatus::Pass => write!(f, "✅ Pass"),
            SystemCheckStatus::Warning => write!(f, "⚠️ Warning"),
            SystemCheckStatus::Fail => write!(f, "❌ Fail"),
            SystemCheckStatus::Pending => write!(f, "⏳ Pending"),
        }
    }
}

#[derive(Debug)]
pub struct SystemReport {
    pub cpu_check: SystemCheckStatus,
    pub cpu_cores: usize,
    pub cpu_usage: f32,
    
    pub memory_check: SystemCheckStatus,
    pub memory_total_gb: u64,
    pub memory_usage_percent: f64,
    
    pub disk_check: SystemCheckStatus,
    pub disk_free_gb: u64,
    pub disk_io_mbps: f64,
    
    pub network_check: SystemCheckStatus,
    pub network_bandwidth_mbps: f64,
    
    pub bitcoin_check: SystemCheckStatus,
    pub lightning_check: SystemCheckStatus,
    pub web5_check: SystemCheckStatus,
    
    pub overall_status: SystemCheckStatus,
    
    pub recommended_node_type: String,
    pub recommended_config: BitcoinConfig,
}

#[derive(Debug)]
pub struct BitcoinConfig {
    pub db_cache_mb: u64,
    pub max_connections: u32,
    pub pruning_enabled: bool,
    pub pruning_size_mb: u64,
    pub indexing_enabled: bool,
    pub tx_index_enabled: bool,
    pub compact_filters_enabled: bool,
    pub rbf_enabled: bool,
    pub lightning_enabled: bool,
}

pub struct HardwareAnalyzer<'a> {
    system: &'a System,
}

impl<'a> HardwareAnalyzer<'a> {
    pub fn new(sys: &'a System) -> Self {
        Self { system: sys }
    }

    pub fn get_ram_gb(&self) -> u64 {
        self.system.total_memory() / 1024 / 1024 / 1024
    }

    pub fn get_cpu_cores(&self) -> usize {
        self.system.cpus().len()
    }

    pub fn get_available_disk_gb(&self) -> u64 {
        self.system.disks().iter()
            .filter(|disk| disk.is_removable() == false)
            .map(|disk| disk.available_space() / 1024 / 1024 / 1024)
            .sum()
    }
    
    pub fn get_cpu_usage(&self) -> f32 {
        self.system.cpus().iter()
            .map(|cpu| cpu.cpu_usage())
            .sum::<f32>() / self.system.cpus().len() as f32
    }

    pub fn get_memory_usage_percent(&self) -> f64 {
        (self.system.used_memory() as f64 / self.system.total_memory() as f64) * 100.0
    }

    pub fn analyze_performance(&self) -> Result<PerformanceMetrics> {
        let cpu_usage = self.get_cpu_usage();
        let ram_usage = self.get_memory_usage_percent();

        Ok(PerformanceMetrics {
            cpu_usage,
            ram_usage,
            disk_io: self.measure_disk_io()?,
            network_bandwidth: self.measure_network_bandwidth()?,
        })
    }

    fn measure_disk_io(&self) -> Result<f64> {
        // Implement disk I/O measurement - Get a disk to measure
        let start_disk = self.system.disks().iter()
            .find(|disk| !disk.is_removable())
            .context("No non-removable disk found")?;

        let start_read = start_disk.total_read_bytes();
        let start_write = start_disk.total_written_bytes();
        
        // Sleep for a short time
        thread::sleep(Duration::from_millis(500));
        
        // We'd need to refresh the system to get updated disk stats
        // Since we have the system as a reference, we can't modify it here
        // In a real implementation, we would refresh it and measure again
        // For now, return a placeholder
        Ok(0.0)
    }

    fn measure_network_bandwidth(&self) -> Result<f64> {
        // Implement network bandwidth measurement
        // Similar constraints as above - would need to refresh system
        Ok(0.0)
    }
    
    pub fn analyze_system(&self) -> Result<SystemReport> {
        // Create report
        let mut report = SystemReport {
            cpu_check: SystemCheckStatus::Pending,
            cpu_cores: self.get_cpu_cores(),
            cpu_usage: self.get_cpu_usage(),
            
            memory_check: SystemCheckStatus::Pending,
            memory_total_gb: self.get_ram_gb(),
            memory_usage_percent: self.get_memory_usage_percent(),
            
            disk_check: SystemCheckStatus::Pending,
            disk_free_gb: self.get_available_disk_gb(),
            disk_io_mbps: self.measure_disk_io()?,
            
            network_check: SystemCheckStatus::Pending,
            network_bandwidth_mbps: self.measure_network_bandwidth()?,
            
            bitcoin_check: SystemCheckStatus::Pending,
            lightning_check: SystemCheckStatus::Pending,
            web5_check: SystemCheckStatus::Pending,
            
            overall_status: SystemCheckStatus::Pending,
            
            recommended_node_type: String::new(),
            recommended_config: BitcoinConfig {
                db_cache_mb: 450,
                max_connections: 50,
                pruning_enabled: false,
                pruning_size_mb: 0,
                indexing_enabled: false,
                tx_index_enabled: false,
                compact_filters_enabled: false,
                rbf_enabled: true,
                lightning_enabled: false,
            },
        };
        
        // Check CPU
        if report.cpu_cores >= 8 {
            report.cpu_check = SystemCheckStatus::Pass;
        } else if report.cpu_cores >= 4 {
            report.cpu_check = SystemCheckStatus::Warning;
        } else {
            report.cpu_check = SystemCheckStatus::Fail;
        }
        
        // Check Memory
        if report.memory_total_gb >= 16 {
            report.memory_check = SystemCheckStatus::Pass;
        } else if report.memory_total_gb >= 8 {
            report.memory_check = SystemCheckStatus::Warning;
        } else {
            report.memory_check = SystemCheckStatus::Fail;
        }
        
        // Check Disk
        if report.disk_free_gb >= 500 {
            report.disk_check = SystemCheckStatus::Pass;
        } else if report.disk_free_gb >= 50 {
            report.disk_check = SystemCheckStatus::Warning;
        } else {
            report.disk_check = SystemCheckStatus::Fail;
        }
        
        // Network check would be determined by bandwidth measurement
        // For now, set based on a simple check
        report.network_check = SystemCheckStatus::Pass;
        
        // Determine overall status
        if report.cpu_check == SystemCheckStatus::Fail ||
           report.memory_check == SystemCheckStatus::Fail ||
           report.disk_check == SystemCheckStatus::Fail ||
           report.network_check == SystemCheckStatus::Fail {
            report.overall_status = SystemCheckStatus::Fail;
        } else if report.cpu_check == SystemCheckStatus::Warning ||
                  report.memory_check == SystemCheckStatus::Warning ||
                  report.disk_check == SystemCheckStatus::Warning ||
                  report.network_check == SystemCheckStatus::Warning {
            report.overall_status = SystemCheckStatus::Warning;
        } else {
            report.overall_status = SystemCheckStatus::Pass;
        }
        
        // Generate recommendations based on system specs
        report.recommended_config = self.generate_optimal_bitcoin_config(&report);
        report.recommended_node_type = self.determine_node_type(&report);
        
        Ok(report)
    }
    
    fn generate_optimal_bitcoin_config(&self, report: &SystemReport) -> BitcoinConfig {
        let mut config = BitcoinConfig {
            db_cache_mb: 450,
            max_connections: 50,
            pruning_enabled: false,
            pruning_size_mb: 0,
            indexing_enabled: false,
            tx_index_enabled: false,
            compact_filters_enabled: false,
            rbf_enabled: true,
            lightning_enabled: false,
        };
        
        // Set DB cache based on available memory
        if report.memory_total_gb >= 32 {
            config.db_cache_mb = 8192;
        } else if report.memory_total_gb >= 16 {
            config.db_cache_mb = 4096;
        } else if report.memory_total_gb >= 8 {
            config.db_cache_mb = 2048;
        } else if report.memory_total_gb >= 4 {
            config.db_cache_mb = 1024;
        }
        
        // Set max connections based on available CPU cores
        if report.cpu_cores >= 16 {
            config.max_connections = 256;
        } else if report.cpu_cores >= 8 {
            config.max_connections = 125;
        } else if report.cpu_cores >= 4 {
            config.max_connections = 75;
        }
        
        // Determine if pruning should be enabled based on available disk space
        if report.disk_free_gb < 500 {
            config.pruning_enabled = true;
            config.pruning_size_mb = 550;
        } else if report.disk_free_gb < 1000 {
            config.pruning_enabled = true;
            config.pruning_size_mb = 2000;
        } else {
            config.indexing_enabled = true;
            config.tx_index_enabled = true;
        }
        
        // Enable advanced features if system is capable
        if report.cpu_cores >= 8 && report.memory_total_gb >= 16 {
            config.compact_filters_enabled = true;
        }
        
        // Consider Lightning if system has sufficient resources
        if report.memory_total_gb >= 8 && report.cpu_cores >= 4 && report.disk_free_gb >= 500 {
            config.lightning_enabled = true;
        }
        
        config
    }
    
    fn determine_node_type(&self, report: &SystemReport) -> String {
        if report.overall_status == SystemCheckStatus::Pass {
            if report.memory_total_gb >= 16 && report.cpu_cores >= 8 && report.disk_free_gb >= 1000 {
                "Full Node with Indexing and Lightning".to_string()
            } else if report.memory_total_gb >= 8 && report.cpu_cores >= 4 && report.disk_free_gb >= 500 {
                "Full Node".to_string()
            } else {
                "Pruned Node".to_string()
            }
        } else if report.overall_status == SystemCheckStatus::Warning {
            if report.disk_free_gb < 50 {
                "Pruned Node (Limited)".to_string()
            } else {
                "Pruned Node".to_string()
            }
        } else {
            "SPV Client Recommended".to_string()
        }
    }
}

#[derive(Debug)]
pub struct PerformanceMetrics {
    pub cpu_usage: f32,
    pub ram_usage: f64,
    pub disk_io: f64,
    pub network_bandwidth: f64,
}
