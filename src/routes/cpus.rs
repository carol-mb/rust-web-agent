use rocket::{get, serde::json::Json};
use sysinfo::{System, SystemExt};

use super::cpu_common::{create_cpu_response, CpuResponse};

#[get("/cpus")]
pub fn cpus() -> Json<Vec<CpuResponse>> {
    Json(
        System::new_all()
            .cpus()
            .iter()
            .map(|cpu| create_cpu_response(cpu))
            .collect(),
    )
}
