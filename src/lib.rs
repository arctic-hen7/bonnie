use std::env::VarError;
use std::fs;

mod command;
mod commands_registry;
mod help_page;
mod install;
mod read_cfg;
use crate::command::Command;
use crate::help_page::BONNIE_HELP_PAGE;
use crate::install::{
    download_package, get_dependencies_and_dev_dependencies, get_latest_version,
    get_tarball_download_link_and_name,
};
use crate::read_cfg::{
    get_commands_registry_from_cfg, parse_cfg, parse_dependencies, Dependencies,
};

pub const DEFAULT_BONNIE_CFG_PATH: &str = "./bonnie.toml";
// Performs most program logic with manipulable arguments for easier testing
// This only calls component functions that propagate pre-formed errors, so we can safely use `?`
// This function does not run the final command because that would produce side effects outside the testing environment
pub fn get_command_from_cfg_and_args(
    cfg_string: String,
    prog_args: Vec<String>,
) -> Result<String, String> {
    let cfg = parse_cfg(cfg_string)?;
    let registry = get_commands_registry_from_cfg(&cfg);

    // Extract the command the user wants to run and the arguments they're providing to it
    // When getting the command the user wants to run, they may not have provided one, so we handle that
    let cmd = &prog_args.get(1); // The command the user wants to run
    let cmd = match cmd {
        Some(cmd) => cmd,
        None => return Err(String::from("You must provide a command to run.")),
    };
    let args = &prog_args[2..]; // Any arguments to that command the user has provided

    let command = registry.get(cmd)?;
    let command_with_args = command.insert_args(&args.to_vec())?;

    Ok(command_with_args)
}

pub fn install_dependencie_from_toml(value: String) -> Result<Dependencies, String> {
    Ok(parse_dependencies(value)?)
}

pub async fn install_dependencie_from_arg(args: &[std::string::String]) {
    for dependency in args {
        let (package, version) = get_latest_version(dependency).await.unwrap();
        let link = get_tarball_download_link_and_name(package, &version)
            .await
            .unwrap();
        download_package(link).await.unwrap();
        // println!("link {}", link);
        let dep = get_dependencies_and_dev_dependencies(package, &version)
            .await
            .unwrap();
        println!("dependencies {:?}", dep)
    }
}
// Extracts the config from the TOML file at the given path
pub fn get_cfg(path: &str) -> Result<String, String> {
    let cfg_string = fs::read_to_string(path);
    match cfg_string {
		Ok(cfg_string) => Ok(cfg_string),
		Err(_) => Err(String::from("Error reading bonnie.toml, make sure the file is present in this directory and you have the permissions to read it."))
	}
}

// Gets the path to the config file based on given environment variables
// TODO if non-unicode variable given, print a warning to explain it (right now this behaviour needs to be documented)
pub fn get_cfg_path(env_var: Result<String, VarError>) -> String {
    let default_cfg_path = DEFAULT_BONNIE_CFG_PATH.to_string();

    env_var.unwrap_or(default_cfg_path)
}

// Runs a command (abstracted here to keep the call-only pattern in `main`)
pub fn run_cmd(cmd: String) -> Result<(), String> {
    Command::run(&cmd)?;

    Ok(())
}

// Functions for reserved commands
pub fn init() -> Result<(), String> {
    // Check if there's already a config file in this directory
    if fs::metadata("./bonnie.toml").is_ok() {
        Err(String::from("A Bonnie configuration file already exists in this directory. If you want to create a new one, please delete the old one first."))
    } else {
        // Create a new `bonnie.toml` file
        let output = fs::write(
            "./bonnie.toml",
            "[scripts]
start = \"echo \\\"No start script yet.\\\"\"
",
        );

        match output {
    		Ok(_) => Ok(()),
    		Err(_) => Err(String::from("Error creating new bonnie.toml, make sure you have the permissions to write to this directory."))
    	}
    }
}
pub fn help() {
    println!("{}", BONNIE_HELP_PAGE);
}
