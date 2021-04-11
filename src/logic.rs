use std::fs;
use std::env::VarError;

use crate::command::Command;
use crate::read_cfg::{parse_cfg, get_commands_registry_from_cfg};

const DEFAULT_BONNIE_CFG_PATH: &str = "./bonnie.toml";

// Performs most program logic with manipulable arguments for easier testing
// This only calls component functions that propagate pre-formed errors, so we can safely use `?`
// This function does not run the final command because that would produce side effects outside the testing environment
pub fn get_command_from_cfg_and_args(cfg_string: String, prog_args: Vec<String>) -> Result<String, String> {
    let cfg = parse_cfg(cfg_string)?;
    let registry = get_commands_registry_from_cfg(&cfg);

	// Extract the command the user wants to run and the arguments they're providing to it
    // When getting the command the user wants to run, they may not have provided one, so we handle that
    let cmd = &prog_args.get(1); // The command the user wants to run
    let cmd = match cmd {
        Some(cmd) => cmd,
        None => return Err(String::from("You must provide a command to run."))
    };
    let args = &prog_args[2..]; // Any arguments to that command the user has provided

    let command = registry.get(cmd)?;
    let command_with_args = command.insert_args(&args.to_vec())?;

	Ok(command_with_args)
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
pub fn get_cfg_path(env_var: Result<String, VarError>) -> String {
	let default_cfg_path = DEFAULT_BONNIE_CFG_PATH.to_string();

    env_var.unwrap_or(default_cfg_path)
}

// Runs a command (abstracted here to keep the call-only pattern in `main`)
pub fn run_cmd(cmd: String) -> Result<(), String> {
	Command::run(&cmd)?;

	Ok(())
}