use serde::Serialize;

#[derive(Serialize)]
pub struct Cpu {
    pub physical_cores: Option<usize>,
    pub logical_processors: i32,
    pub base_speed_ghz: i32,
    pub current_speed_ghz: i32,
    pub cpu_usage_percent: i32,
    pub uptime_seconds: i32,
}

#[derive(Serialize)]
pub struct Memory {
    pub total_memory_gb: f64,
    pub available_memory_gb: f64,
    pub swap_used_gb: f64,
}

#[derive(Serialize)]
pub struct SystemStats {
    pub os: Option<String>,
    pub cpu: Cpu,
    pub memory: Memory,
}

#[derive(Serialize)]
pub struct Stats {
    pub execution_time_ms: u128,
    pub memory_used_bytes: usize,

    pub system_stat: SystemStats,
}
