use rocket::{post, serde::json::Json};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, process::Command};

#[derive(Debug, Deserialize)]
pub struct ProcessStartArgs {
    command: String,
    arguments: Vec<String>,
    environment: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
pub struct ProcessStartResponse {
    status: String,
    error: i32,
}

#[post("/processes/start", data = "<args>")]
pub fn processes_start(args: Json<ProcessStartArgs>) -> Json<ProcessStartResponse> {
    let command = Command::new(args.command.clone())
        .args(args.arguments.clone())
        .envs(args.environment.clone())
        .spawn();

    Json(ProcessStartResponse {
        status: (if command.is_ok() { "ok" } else { "error" }).to_string(),
        error: command
            .err()
            .and_then(|err| err.raw_os_error())
            .unwrap_or(0),
    })
}
