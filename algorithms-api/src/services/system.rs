use crate::{
    models::stats::{CpuStats, MemoryStats, SystemStats},
    services::converter::Converter,
};
use sysinfo::System;

impl SystemStats {
    pub fn collect() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        SystemStats {
            kernel: System::kernel_long_version(),
            cpus: CpuStats {
                physical_cores: System::physical_core_count(),
                logical_processors: sys.cpus().len(),
                uptime: Converter::seconds_to_format(System::uptime()),
            },
            memory: MemoryStats {
                total_memory_gb: Converter::byte_to_gb(sys.total_memory()),
                available_memory_gb: Converter::byte_to_gb(sys.available_memory()),
            },
        }
    }
}
