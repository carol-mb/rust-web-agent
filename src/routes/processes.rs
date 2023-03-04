use rocket::{get, serde::json::Json};
use sysinfo::{PidExt, System, SystemExt};

#[get("/processes")]
pub fn get_procs() -> Json<Vec<u32>> {
    let s = System::new_all();

    let mut procs: Vec<u32> = Vec::new();

    for (pid, _) in s.processes() {
        procs.push(pid.as_u32());
    }

    Json(procs)
}
