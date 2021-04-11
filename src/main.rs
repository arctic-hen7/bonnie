use std::env;

mod command;
mod commands_registry;
mod read_cfg;
use crate::command::Command;
use crate::read_cfg::{get_cfg, get_commands_registry_from_cfg};

// TODO colorise output?

const DEFAULT_BONNIE_CFG_PATH: &str = "./bonnie.toml";

fn main() {
    // All error propagation is done with the string errors preformed, so we can print them directly from the match statements
    // As this is a CLI program, any errors are propagated to `main` and then printed with `eprintln!`

    let default_cfg_path = DEFAULT_BONNIE_CFG_PATH.to_string();
    let cfg_path = env::var("BONNIE_CONF").unwrap_or(default_cfg_path);
    let cfg = get_cfg(&cfg_path);
    let cfg = match cfg {
        Ok(cfg) => cfg,
        Err(err) => return eprintln!("{}", err)
    };

    let registry = get_commands_registry_from_cfg(&cfg);

    // Get the arguments to this program and extract the command the user wants to run and the arguments they're providing to it
    let prog_args: Vec<String> = env::args().collect();
    // When getting the command the suer wants to run, they may not have provided one, so we handle that
    let cmd = &prog_args.get(1); // The command the user wants to run
    let cmd = match cmd {
        Some(cmd) => cmd,
        None => return eprintln!("You must provide a command to run.")
    };
    let args = &prog_args[2..]; // Any arguments to that command the user has provided

    let command = registry.get(cmd);
    let command = match command {
        Ok(command) => command,
        Err(err) => return eprintln!("{}", err)
    };
    let command_with_args = command.insert_args(&args.to_vec());
    let command_with_args = match command_with_args {
        Ok(command_with_args) => command_with_args,
        Err(err) => return eprintln!("{}", err)
    };

    let cmd_output = Command::run(&command_with_args);
    match cmd_output {
        Ok(cmd_output) => cmd_output,
        Err(err) => return eprintln!("{}", err)
    };
}
