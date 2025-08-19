use serde::Serialize;

#[derive(Serialize)]
pub struct CpuStats {
    pub physical_cores: Option<usize>,
    pub logical_processors: usize,
    pub uptime: String,
}

#[derive(Serialize)]
pub struct MemoryStats {
    pub total_memory_gb: f64,
    pub available_memory_gb: f64,
}

#[derive(Serialize)]
pub struct SystemStats {
    pub kernel: String,
    pub cpus: CpuStats,
    pub memory: MemoryStats,
}

#[derive(Serialize)]
pub struct Stats {
    pub execution_time_us: u128,
    pub memory_used_bytes: usize,
}
