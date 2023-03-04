use serde::Serialize;
use sysinfo::{Cpu, CpuExt};

#[derive(Debug, Serialize)]
pub struct CpuResponse {
    model: String,
    manufacturer: String,
    speed: u64,
    usage: f32,
}

pub fn create_cpu_response(cpu: &Cpu) -> CpuResponse {
    CpuResponse {
        model: cpu.brand().to_string(),
        manufacturer: cpu.vendor_id().to_string(),
        speed: cpu.frequency(),
        usage: cpu.cpu_usage(),
    }
}
