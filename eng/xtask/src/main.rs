use std::{env, fs, process::Command, str};

use serde::Deserialize;
use serde_json::Result;

#[derive(Deserialize)]
struct Response {
    packages: Vec<Package>,
}

#[derive(Deserialize)]
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

    let mut binding = Command::new(cargo);

    let command = binding
        .arg("metadata")
        .arg("--format-version")
        .arg("1")
        .arg("--no-deps");

    let output = command.output().expect("failed to execute process");

    let metadata_output = str::from_utf8(&output.stdout).unwrap();

    let packages = serde_json::from_str::<Response>(metadata_output)
        .unwrap()
        .packages;

    // print the current working directory
    let current_dir = env::current_dir().unwrap();
    eprintln!("Current directory: {:?}", current_dir);

    let toml_path = current_dir.join("Cargo.toml");

    let carto_toml = fs::read_to_string(&toml_path).unwrap();

    // parse cargo.toml
    let mut cargo_parsed = toml::from_str::<toml::Table>(&carto_toml).unwrap();

    // print the keys
    for key in cargo_parsed.keys() {
        eprintln!("Key: {}", key);
    }

    let dependencies = cargo_parsed["workspace"]["dependencies"]
        .as_table_mut()
        .unwrap();

    // for each package in packages, update the contents of the root Cargo.toml and update the package workspace dependency version
    for package in packages {
        let package_name = package.name;
        let package_version = package.version;
        let package_key = format!("workspace.dependencies.{}", package_name).to_string();

        // find the workspace dependency entry
        if !dependencies.contains_key(&package_name) {
            eprintln!("{} not found in Cargo.toml", package_key);
            continue;
        }

        eprintln!("Updating {} to version {}", package_key, package_version);
        let package = dependencies[&package_name].as_table_mut().unwrap();
        if package.contains_key("version") {
            package["version"] = toml::Value::String(package_version);
        } else {
            package.insert("version".to_string(), toml::Value::String(package_version));
        }
    }

    // write the updated Cargo.toml
    let updated_toml = toml::to_string_pretty(&cargo_parsed).unwrap();
    fs::write(toml_path, updated_toml).unwrap();

    Ok(())
}
