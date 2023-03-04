use rocket::get;
use rocket::serde::json::Json;
use serde::Serialize;
use sysinfo::{Pid, ProcessExt, System, SystemExt};

#[derive(Serialize)]
pub struct Memory {
    vsz: u64,
    rss: u64,
}

#[derive(Serialize)]
pub struct Process {
    command: String,
    arguments: Vec<String>,
    memory: Memory,
}

#[get("/processes/<pid>")]
pub fn get_proc_by_pid(pid: i32) -> Json<Process> {
    let mut command: String = "".to_string();
    let mut arguments: Vec<String> = vec![];
    let mut rss: u64 = 0;
    let mut vsz: u64 = 0;

    let s = System::new_all();

    if let Some(process) = s.process(Pid::from(pid)) {
        command = process.name().to_string();

        for arg in process.cmd() {
            arguments.push(arg.to_string());
        }
        arguments.remove(0);

        rss = process.memory();
        vsz = process.virtual_memory();
    }

    Json(Process {
        command: command,
        arguments: arguments,
        memory: Memory { rss: rss, vsz: vsz },
    })
}
