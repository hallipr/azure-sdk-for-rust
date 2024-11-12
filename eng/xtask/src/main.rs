use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use serde_json::{Result, Value};

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<()> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("ws-versions") => set_workspace_versions()?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:

ws-versions            set the versions for workspace crate dependencies
"
    )
}

fn set_workspace_versions() -> Result<()> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    let json = Command::new(cargo)
        .current_dir(project_root())
        .arg("metadata")
        .arg("--format-version")
        .arg("1")
        .arg("--no-deps")
        .arg("--manifest-path")
        .arg();

    let metadata_output = cargo.run_always().run_capture_stdout(build).stdout();
    let Output { packages, .. } = t!(serde_json::from_str(&metadata_output));
    
    packages
}
