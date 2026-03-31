#[derive(Debug, Clone, Default)]
pub struct ProcessInfo {
    pub pid: i32,
    pub name: String,
    pub cpu: f32,
    pub memory_mb: u64,
}

#[derive(Debug, Clone, Default)]
pub struct GpuInfo {
    pub index: u32,
    pub name: String,
    pub vram_used_mb: u64,
    pub vram_total_mb: u64,
    pub temperature_c: u32,
    pub utilization_gpu: u32,
}

#[derive(Debug, Clone)]
pub struct MetricsSnapshot {
    pub timestamp_ms: u64,
    pub cpu_per_core: Vec<f32>,
    pub cpu_avg: f32,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub net_rx_bytes_per_sec: u64,
    pub net_tx_bytes_per_sec: u64,
    pub top_processes: Vec<ProcessInfo>,
    pub gpus: Vec<GpuInfo>,
    pub nvml_available: bool,
}

impl Default for MetricsSnapshot {
    fn default() -> Self {
        Self {
            timestamp_ms: 0,
            cpu_per_core: Vec::new(),
            cpu_avg: 0.0,
            memory_used_mb: 0,
            memory_total_mb: 0,
            net_rx_bytes_per_sec: 0,
            net_tx_bytes_per_sec: 0,
            top_processes: Vec::new(),
            gpus: Vec::new(),
            nvml_available: false,
        }
    }
}
