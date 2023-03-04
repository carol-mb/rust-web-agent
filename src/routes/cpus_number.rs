use rocket::{get, serde::json::Json};
use sysinfo::{System, SystemExt};

use super::cpu_common::{create_cpu_response, CpuResponse};

#[get("/cpus/<cpu_number>")]
pub fn cpus_number(cpu_number: usize) -> Json<Option<CpuResponse>> {
    Json(
        System::new_all()
            .cpus()
            .iter()
            .nth(cpu_number)
            .map(|cpu| create_cpu_response(cpu)),
    )
}
