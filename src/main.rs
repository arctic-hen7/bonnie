use std::env;

mod command;
mod commands_registry;
mod read_cfg;
use crate::command::Command;
use crate::read_cfg::{get_cfg, get_commands_registry_from_cfg};

const DEFAULT_BONNIE_CFG_PATH: &str = "./bonnie.toml";

fn main() {
    let default_cfg_path = DEFAULT_BONNIE_CFG_PATH.to_string();
    let cfg_path = env::var("BONNIE_CONF").unwrap_or(default_cfg_path);
    let cfg = get_cfg(&cfg_path);

    let registry = get_commands_registry_from_cfg(&cfg);

    // Get the arguments to this program and extract the command the user wants to run and the arguments they're providing to it
    let prog_args: Vec<String> = env::args().collect();
    // When getting the command the suer wants to run, they may not have provided one, so we handle that
    let cmd = &prog_args.get(1).expect("You must provide a command to run."); // The command the user wants to run
    let args = &prog_args[2..]; // Any arguments to that command the user has provided

    let command = registry.get(cmd);
    let command_with_args = command.insert_args(&args.to_vec());

    Command::run(&command_with_args);
}
