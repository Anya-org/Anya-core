#![feature(edition2021)]
pub struct HardwareAnalyzer {
    system: System,
}

impl HardwareAnalyzer {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self { system: sys }
    }

    pub fn cpu_cores(&self) -> usize {
        self.system.cpus().len()
    }

    pub fn memory_gb(&self) -> u64 {
        self.system.total_memory() / 1_000_000_000
    }
} 