use std::{
    env,
    str,
    process::{Command},
};

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Response {
    packages: Vec<Package>,
}

#[derive(Serialize, Deserialize)]
struct Package {
    name: String,
    version: String,
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<()> {
    let task = env::args().nth(1);
    eprintln!("task: {:?}", task);
    match task.as_deref() {
        Some("versions") => set_workspace_versions()?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:

versions            set the versions for workspace crate dependencies
"
    )
}

fn set_workspace_versions() -> Result<()> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    let mut command = Command::new(cargo);

    let bound = command
        .arg("metadata")
        .arg("--format-version")
        .arg("1")
        .arg("--no-deps");

    let output = bound
        .output()
        .expect("failed to execute process");
    
    let metadata_output = str::from_utf8(&output.stdout).unwrap();

    let response: Response = serde_json::from_str(&metadata_output).unwrap();

    return Ok(());
}
