use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use sysinfo::{CpuExt, SystemExt};

#[derive(Serialize)]
pub struct StatusInfo {
    cpus: usize,
    memory_total: u64,
    memory_used: u64,
    uptime: u64,
    usage: f32,
}

#[get("/status")]
pub fn status() -> Json<StatusInfo> {
    let mem_info = sysinfo::System::new();

    let mut usage: f32 = 0.0;
    for cpu in mem_info.cpus() {
        usage += cpu.cpu_usage();
    }

    Json(StatusInfo {
        cpus: (mem_info.cpus()).len(),
        memory_total: mem_info.total_memory(),
        memory_used: mem_info.used_memory(),
        uptime: mem_info.uptime(),
        usage: usage,
    })
}
