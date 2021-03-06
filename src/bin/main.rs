use lib::{cache, cache_exists, get_cfg, help, init, load_from_cache, Config, BONNIE_VERSION};
use std::env;
use std::io::Write;

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
// Bonnie prints warnings using the `writeln!` macro, which allows the parsing of `stdout` in production or a vector in testing
// If at any point a warning can't be printed, the program will panic
fn core() -> Result<i32, String> {
    // Get `stdout` so we can write warnings appropriately
    let stdout = &mut std::io::stdout();
    // Get the arguments to this program, removing the first one (something like `bonnie`)
    let mut prog_args: Vec<String> = env::args().collect();
    // This will panic if the first argument is not found (which is probably someone trying to fuzz us)
    // TODO add a checker for the executable that offers to install Bonnie if it isn't already?
    let _executable_name = prog_args.remove(0);
    // Get the file we'll be using
    let cfg_path = env::var("BONNIE_CONF").unwrap_or_else(|_| "./bonnie.toml".to_string());
    // Check for special arguments
    let mut should_cache = false;
    let mut verbose = false;
    let mut document = false;
    if matches!(prog_args.get(0), Some(_)) {
        if prog_args[0] == "-v" || prog_args[0] == "--version" {
            writeln!(stdout, "You are currently running Bonnie v{}! You can see the latest release at https://github.com/arctic-hen7/bonnie/releases.", BONNIE_VERSION).expect("Failed to write version.");
            return Ok(0);
        } else if prog_args[0] == "-i" || prog_args[0] == "--init" {
            init(
                // See if a template was provided with the `--template`/`-t` flag
                match prog_args.get(1).as_ref() {
                    Some(arg) if &**arg == "-t" || &**arg == "--template" => {
                        prog_args.get(2).map(|s| s.to_string())
                    }
                    _ => None,
                },
                &cfg_path,
            )?;

            println!(
                "A new Bonnie configuration file has been initialized at {}!",
                &cfg_path
            );

            return Ok(0);
        } else if prog_args[0] == "-h" || prog_args[0] == "--help" {
            help(stdout);
            return Ok(0);
        } else if prog_args[0] == "-c" || prog_args[0] == "--cache" {
            should_cache = true;
        } else if prog_args[0] == "-d" || prog_args[0] == "--debug" {
            // This can be specified with a command following
            verbose = true;
            prog_args.remove(0);
        }
        // Check if the user wants the configuration's help page (self-documenting)
        // TODO 'doc' instead/as well?
        else if prog_args[0] == "help" {
            document = true;
        }
    }
    // Check if there's a cache we should read from
    // If there is but we're explicitly recaching, we should of course read directly from the source file
    let cfg;
    if cache_exists()? && !should_cache {
        cfg = load_from_cache(stdout, None)?;
    } else {
        // Get the config as a string
        let cfg_str = get_cfg()?;
        // Create a raw config object and parse it fully
        // We use `stdout` for printing warnings
        cfg = Config::new(&cfg_str)?.to_final(BONNIE_VERSION, stdout)?;
    }

    // Check if we're caching
    if should_cache {
        cache(&cfg, stdout, None)?;
        return Ok(0);
    }

    if document {
        // Handle individual commands
        let msg = cfg.document(prog_args.get(1).cloned())?;
        writeln!(stdout, "{}", msg).expect("Failed to write configuration help.");
        return Ok(0);
    }

    // Determine which command we're actually running
    let (command_to_run, command_name, relevant_args) = cfg.get_command_for_args(&prog_args)?;
    // Get the Bone (item in Bones execution runtime)
    let bone = command_to_run.prepare(&command_name, &relevant_args, &cfg.default_shell)?;
    // Execute the Bone, getting its final exit code
    // We parse in `stdout` as the place to write command information, but that will only be done in testing
    let exit_code = bone.run(&command_name, verbose, stdout)?;

    Ok(exit_code)
}
