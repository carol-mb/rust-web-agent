use rocket::{routes, Route};

mod cpus;
mod cpus_number;
mod processes;
mod processes_kill;
mod processes_pid;
mod processes_start;
mod status;

mod cpu_common;

pub fn get_all_routes() -> Vec<Route> {
    routes![
        cpus::cpus,
        cpus_number::cpus_number,
        processes::get_procs,
        processes_kill::kill_pid,
        processes_pid::get_proc_by_pid,
        processes_start::processes_start,
        status::status
    ]
}
