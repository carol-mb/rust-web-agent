use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use sysinfo::{Pid, ProcessExt, System, SystemExt};

#[derive(Serialize)]
pub struct Response {
    status: String,
    error: u8,
}

#[get("/processes/kill/<process_pid>")]
pub fn kill_pid(process_pid: i32) -> Json<Response> {
    let s = System::new_all();
    if let Some(process) = s.process(Pid::from(process_pid)) {
        let result = process.kill();
        Json(Response {
            status: if result {
                "ok".to_string()
            } else {
                "error".to_string()
            },
            error: if result { 0 } else { 2 },
        })
    } else {
        Json(Response {
            status: "error".to_string(),
            error: 1,
        })
    }
}
