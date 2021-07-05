use lib::{get_cfg, raw_schema, BONNIE_VERSION};
use std::env;

// All this does is run the program and terminate with the acquired exit code
fn main() {
    let exit_code = real_main();
    std::process::exit(exit_code)
}

// This manages error handling and returns a definite exit code to terminate Bonnie with
fn real_main() -> i32 {
    let res = core();
    match res {
        // If it worked, we pass the executed command's exit code through
        Ok(exit_code) => exit_code,
        // If something failed, we print the error to stderr and return a failure exit code
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    }
}

// This performs the actual logic, separated for deduplication of error handling and destructor control
// This returns the exit code of the executed command, which we should return from Bonnie itself
fn core() -> Result<i32, String> {
    // Get the config as a string
    let cfg_str = get_cfg()?;
    // Create a raw config object and parse it fully
    // TODO this takes meaningful millseconds for complex configs, so we should be able to cache its results in `.bonnie.cache.json` for speed in extreme cases
    let cfg = raw_schema::Config::new(&cfg_str)?.to_final(BONNIE_VERSION)?;
    // Get the arguments to this program, removing the first one (something like `bonnie`)
    let mut prog_args: Vec<String> = env::args().collect();
    let _executable_name = prog_args.remove(0); // This will panic if the first argument is not found (which is probably someone trying to fuzz us)
                                                // TODO add a checker for the executable that offers to install Bonnie if it isn't already?
                                                // Determine which command we're actually running
    let (command_to_run, command_name, relevant_args) = cfg.get_command_for_args(&prog_args)?;
    // Get the Bone (item in Bones execution runtime)
    let bone = command_to_run.prepare(&command_name, &relevant_args, &cfg.default_shell)?;
    // Execute the Bone, getting its final exit code
    let exit_code = bone.run(&command_name)?;

    Ok(exit_code)
}
