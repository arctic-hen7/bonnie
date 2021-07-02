use std::env;

use bonnie_lib::{
    get_cfg, get_cfg_path, get_command_from_cfg_and_args, help, init, run_cmd, BONNIE_VERSION,
};

// TODO colorise output?

// This program follows the call-only pattern in `main`, so all logic is abstracted into `lib.rs`
// All error propagation is done with the string errors preformed, so we can print them directly from the match statements
// As this is a CLI program, any errors are propagated to `main` and then printed with `eprintln!`
fn main() {
    // Get the path of the configuration file
    let cfg_path = get_cfg_path(env::var("BONNIE_CONF"));
    // Get the arguments to this program
    let prog_args: Vec<String> = env::args().collect();

    // Check if a special command is being run (config file has nothing to do with those)
    if prog_args.get(1) == Some(&String::from("help")) {
        // This just prints, we need no error handling whatsoever here
        help();
    } else if prog_args.get(1) == Some(&String::from("init")) {
        // As this creates a file, it can cause errors
        let output = init();
        match output {
            Ok(_) => {
                return println!("Bonnie configuration file created at './bonnie.toml'. Enjoy!")
            }
            Err(err) => return eprintln!("{}", err),
        }
    } else {
        // Get the contents of the configuration file
        let cfg_string = get_cfg(&cfg_path);
        let cfg_string = match cfg_string {
            Ok(cfg_string) => cfg_string,
            Err(err) => return eprintln!("{}", err),
        };

        // Get the command to run from the arguments the user gave and the configuration file
        // We parse the current version in directly here (only extracted as an argument for testing purposes)
        let command_with_args =
            get_command_from_cfg_and_args(cfg_string, prog_args, BONNIE_VERSION);
        let command_with_args = match command_with_args {
            Ok(command_with_args) => command_with_args,
            Err(err) => return eprintln!("{}", err),
        };

        let exit_code = run_cmd(command_with_args);
        match exit_code {
            Ok(exit_code) => std::process::exit(exit_code), // We exit with the same code as the command that was run
            Err(err) => return eprintln!("{}", err),
        };
    }
}
