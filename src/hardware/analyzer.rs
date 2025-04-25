use std::error::Error;
pub struct HardwareAnalyzer {
    system: System,
}

impl HardwareAnalyzer {
    pub fn new() -> Self  -> Result<(), Box<dyn Error>> {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self { system: sys }
    }

    pub fn cpu_cores(&self) -> usize  -> Result<(), Box<dyn Error>> {
        self.system.cpus().len()
    }

    pub fn memory_gb(&self) -> u64  -> Result<(), Box<dyn Error>> {
        self.system.total_memory() / 1_000_000_000
    }
} 
