use sysinfo::{System, SystemExt, CpuExt, DiskExt};
use anyhow::Result;

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

    pub fn analyze_performance(&self) -> Result<PerformanceMetrics> {
        let cpu_usage = self.system.cpus().iter()
            .map(|cpu| cpu.cpu_usage())
            .sum::<f32>() / self.system.cpus().len() as f32;

        let ram_usage = (self.system.used_memory() as f64 / self.system.total_memory() as f64) * 100.0;

        Ok(PerformanceMetrics {
            cpu_usage,
            ram_usage,
            disk_io: self.measure_disk_io()?,
            network_bandwidth: self.measure_network_bandwidth()?,
        })
    }

    fn measure_disk_io(&self) -> Result<f64> {
        // Implement disk I/O measurement
        Ok(0.0)
    }

    fn measure_network_bandwidth(&self) -> Result<f64> {
        // Implement network bandwidth measurement
        Ok(0.0)
    }
}

#[derive(Debug)]
pub struct PerformanceMetrics {
    pub cpu_usage: f32,
    pub ram_usage: f64,
    pub disk_io: f64,
    pub network_bandwidth: f64,
}
