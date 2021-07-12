use lib::{
    cache, cache_exists, get_cfg, get_template_path, help, init, load_from_cache, Config,
    BONNIE_VERSION,
};
use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command as OsCommand;

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
    // Check for special arguments
    let mut should_cache = false;
    if matches!(prog_args.get(0), Some(_)) {
        if prog_args[0] == "-v" || prog_args[0] == "--version" {
            writeln!(stdout, "You are currently running Bonnie v{}! You can see the latest release at https://github.com/arctic-hen7/bonnie/releases.", BONNIE_VERSION).expect("Failed to write version.");
            return Ok(0);
        } else if prog_args[0] == "-i" || prog_args[0] == "--init" {
            print!("Initialising...");

            init(
                // See if a template was provided with the `--template`/`-t` flag
                match prog_args.get(1).as_ref() {
                    Some(arg) if &**arg == "-t" || &**arg == "--template" => {
                        prog_args.get(2).map(|x| x.to_string())
                    }
                    _ => None,
                },
            )?;

            println!("    Done!");

            return Ok(0);
        } else if prog_args[0] == "-h" || prog_args[0] == "--help" {
            help(stdout);
            return Ok(0);
        } else if prog_args[0] == "-c" || prog_args[0] == "--cache" {
            should_cache = true;
        } else if prog_args[0] == "-e" || prog_args[0] == "--edit-template" {
            let template_path: String = match get_template_path() {
                Ok(path) => Ok(path.to_str().unwrap().to_string()),
                Err(err) => Err(format!(
                    "Failed to get template path with the following error: {:#?}",
                    err
                )),
            }?;

            let child;

            if cfg!(target_os = "windows") {
                // We need to spawn a `powershell` process to make `start` available.
                child = OsCommand::new("powershell")
                    .arg(format!("start '{}'", template_path))
                    .spawn()
                    .map(|mut x| x.wait());
            } else {
                let editor = PathBuf::from(env::var("EDITOR").unwrap_or("nano".to_string()));

                let safe_editor = editor.to_str().ok_or(
                    "The value given in the 'EDITOR' environment variable couldn't be parsed as a valid path.",
                )?;

                child = OsCommand::new(safe_editor)
                    .arg(template_path)
                    .spawn()
                    .map(|mut x| x.wait());
            }

            let result = match child {
                Ok(_) => {
                    println!("Opening template file...");
                    Ok(0)
                }
                Err(err) => Err(format!(
                    "Your editor failed to start with the following error: {:#?}",
                    err
                )),
            };

            return result;
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

    // Determine which command we're actually running
    let (command_to_run, command_name, relevant_args) = cfg.get_command_for_args(&prog_args)?;
    // Get the Bone (item in Bones execution runtime)
    let bone = command_to_run.prepare(&command_name, &relevant_args, &cfg.default_shell)?;
    // Execute the Bone, getting its final exit code
    // We parse in `stdout` as the place to write command information, but that will only be done in testing
    let exit_code = bone.run(&command_name, stdout)?;

    Ok(exit_code)
}
