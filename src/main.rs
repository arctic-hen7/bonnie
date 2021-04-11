use std::env;

mod logic;
mod command;
mod commands_registry;
mod read_cfg;
use crate::logic::{get_command_from_cfg_and_args, get_cfg_path, get_cfg, run_cmd};

// TODO colorise output?

// This program follows the call-only pattern in `main`, so all logic is abstracted into `logic.rs`
// All error propagation is done with the string errors preformed, so we can print them directly from the match statements
// As this is a CLI program, any errors are propagated to `main` and then printed with `eprintln!`
fn main() {
    // Get the path of the configuration file
    let cfg_path = get_cfg_path(env::var("BONNIE_CONF"));
    // Get the arguments to this program
    let prog_args = env::args().collect();
    // Get the contents of the configuration file
    let cfg_string = get_cfg(&cfg_path);
    let cfg_string = match cfg_string {
        Ok(cfg_string) => cfg_string,
        Err(err) => return eprintln!("{}", err)
    };

    // Get the command to run from the arguments the user gave and the configuration file
    let command_with_args = get_command_from_cfg_and_args(cfg_string, prog_args);
    let command_with_args = match command_with_args {
        Ok(command_with_args) => command_with_args,
        Err(err) => return eprintln!("{}", err)
    };

    let cmd_output = run_cmd(command_with_args);
    match cmd_output {
        Ok(cmd_output) => cmd_output,
        Err(err) => return eprintln!("{}", err)
    };    
}
