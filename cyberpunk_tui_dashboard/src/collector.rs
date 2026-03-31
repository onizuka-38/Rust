use crate::model::{GpuInfo, MetricsSnapshot, ProcessInfo};
use nvml_wrapper::{enum_wrappers::device::TemperatureSensor, Nvml};
use std::cmp::Ordering;
use std::time::{SystemTime, UNIX_EPOCH};
use sysinfo::{CpuExt, NetworkExt, NetworksExt, PidExt, ProcessExt, System, SystemExt};
use tokio::sync::{oneshot, watch};
use tokio::time::{self, Duration};

pub async fn run_collector(
    tx: watch::Sender<MetricsSnapshot>,
    mut shutdown: oneshot::Receiver<()>,
    sample_ms: u64,
) {
    let mut sys = System::new_all();
    let nvml = Nvml::init().ok();

    sys.refresh_cpu();
    sys.refresh_memory();
    sys.refresh_networks();
    sys.refresh_processes();

    let mut last_rx_total = sum_network_rx(&sys);
    let mut last_tx_total = sum_network_tx(&sys);
    let mut interval = time::interval(Duration::from_millis(sample_ms));

    loop {
        tokio::select! {
            _ = &mut shutdown => break,
            _ = interval.tick() => {
                sys.refresh_cpu();
                sys.refresh_memory();
                sys.refresh_networks();
                sys.refresh_processes();

                let now_rx_total = sum_network_rx(&sys);
                let now_tx_total = sum_network_tx(&sys);

                let rx_delta = now_rx_total.saturating_sub(last_rx_total);
                let tx_delta = now_tx_total.saturating_sub(last_tx_total);

                last_rx_total = now_rx_total;
                last_tx_total = now_tx_total;

                let cpu_per_core: Vec<f32> = sys.cpus().iter().map(|c| c.cpu_usage()).collect();
                let cpu_avg = if cpu_per_core.is_empty() {
                    0.0
                } else {
                    cpu_per_core.iter().sum::<f32>() / cpu_per_core.len() as f32
                };

                let memory_total_mb = kib_to_mib(sys.total_memory());
                let memory_used_mb = kib_to_mib(sys.used_memory());

                let mut top_processes = sys
                    .processes()
                    .values()
                    .map(|p| ProcessInfo {
                        pid: p.pid().as_u32() as i32,
                        name: p.name().to_string(),
                        cpu: p.cpu_usage(),
                        memory_mb: kib_to_mib(p.memory()),
                    })
                    .collect::<Vec<_>>();

                top_processes.sort_by(|a, b| {
                    b.cpu
                        .partial_cmp(&a.cpu)
                        .unwrap_or(Ordering::Equal)
                        .then_with(|| b.memory_mb.cmp(&a.memory_mb))
                });
                top_processes.truncate(8);

                let (gpus, nvml_available) = collect_gpu(&nvml);

                let snap = MetricsSnapshot {
                    timestamp_ms: now_ms(),
                    cpu_per_core,
                    cpu_avg,
                    memory_used_mb,
                    memory_total_mb,
                    net_rx_bytes_per_sec: (rx_delta * 1000) / sample_ms.max(1),
                    net_tx_bytes_per_sec: (tx_delta * 1000) / sample_ms.max(1),
                    top_processes,
                    gpus,
                    nvml_available,
                };

                if tx.send(snap).is_err() {
                    break;
                }
            }
        }
    }
}

fn collect_gpu(nvml: &Option<Nvml>) -> (Vec<GpuInfo>, bool) {
    let Some(nvml) = nvml else {
        return (Vec::new(), false);
    };

    let Ok(count) = nvml.device_count() else {
        return (Vec::new(), false);
    };

    let mut out = Vec::new();
    for i in 0..count {
        let Ok(dev) = nvml.device_by_index(i) else {
            continue;
        };

        let name = dev.name().unwrap_or_else(|_| format!("GPU-{i}"));
        let mem = dev.memory_info().ok();
        let temp = dev.temperature(TemperatureSensor::Gpu).unwrap_or(0);
        let util = dev.utilization_rates().map(|u| u.gpu).unwrap_or(0);

        let (vram_used_mb, vram_total_mb) = if let Some(m) = mem {
            (bytes_to_mib(m.used), bytes_to_mib(m.total))
        } else {
            (0, 0)
        };

        out.push(GpuInfo {
            index: i,
            name,
            vram_used_mb,
            vram_total_mb,
            temperature_c: temp,
            utilization_gpu: util,
        });
    }

    (out, true)
}

fn sum_network_rx(sys: &System) -> u64 {
    sys.networks().iter().map(|(_, n)| n.received()).sum()
}

fn sum_network_tx(sys: &System) -> u64 {
    sys.networks().iter().map(|(_, n)| n.transmitted()).sum()
}

fn kib_to_mib(kib: u64) -> u64 {
    kib / 1024
}

fn bytes_to_mib(bytes: u64) -> u64 {
    bytes / 1024 / 1024
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
