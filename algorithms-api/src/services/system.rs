// use crate::models::stats::{Cpu, Memory, SystemStats};
// use sysinfo::System;

// const BYTES_TO_GB: u64 = 1_073_741_824;

// impl SystemStats {
//     pub fn collect() -> Self {
//         let mut sys = System::new_all();
//         sys.refresh_all();

//         SystemStats {
//             kernel: System::kernel_long_version(),
//             cpu: Cpu {
//                 physical_cores: System::physical_core_count(),
//                 logical_processors: System::processors_count(),
//                 base_speed_ghz: None,
//                 current_speed_ghz: sys.global_cpu_info().frequency() as f64 / 1000.0,
//                 cpu_usage_percent: sys.global_cpu_info().cpu_usage(),
//                 uptime_seconds: System::uptime(),
//             },
//             memory: Memory {
//                 total_memory_gb: (sys.total_memory() as f64) / (BYTES_TO_GB as f64),
//                 available_memory_gb: (sys.available_memory() as f64) / (BYTES_TO_GB as f64),
//                 swap_used_gb: (sys.used_swap() as f64) / (BYTES_TO_GB as f64),
//             },
//         }
//     }
// }
