use std::env;
use std::process::Command as OsCommand;

pub struct Command<'a> {
    name: &'a str,
    args: Vec<String>, // Because converting a vector of &str to Strings is really annoying
    env_vars: Vec<String>,
    cmd: &'a str,
}
impl<'a> Command<'a> {
    pub fn new(
        name: &'a str,
        args: Vec<String>,
        env_vars: Vec<String>,
        cmd: &'a str,
    ) -> Command<'a> {
        Command {
            name,
            args,
            env_vars,
            cmd,
        }
    }
    pub fn run(command: &str) -> Result<(), String> {
        // Run the given command (accounting for architecture)
        let child;
        if cfg!(target_os = "windows") {
            child = OsCommand::new("cmd").args(&["/C", &command]).spawn();
        } else {
            child = OsCommand::new("sh").arg("-c").arg(&command).spawn();
        };

        // The child must be mutable so we can wait for it to finish later
        let mut child = match child {
            Ok(child) => child,
            Err(_) => return Err(
                format!(
                    "Command '{}' failed to run. This doesn't mean the command produced an error, but that the process couldn't even be initialised.",
                    &command
                )
            )
        };

        // If we don't wait on the child, any long-running commands will print into the prompt because the parent terminates first (try it yourself with the `long` command)
        let child = child.wait();
        match child {
            Ok(_) => Ok(()),
            Err(_) => return Err(
                format!(
                    "Command '{}' didn't run (parent unable to wait on child process). See the Bonnie documentation for more details on this problem.",
                    &command
                )
            )
        }
    }
    pub fn insert_args_and_env_vars(&self, arg_values: &[String]) -> Result<String, String> {
        // Check if this command ends with a double percent sign (meaning we should append all given arguments)
        let append_args = self.cmd.ends_with("%%");
        // Ensure the user hasn't tried to append all arguments and have custom ones as well (not yet implemented)
        if append_args && !self.args.is_empty() {
            return Err(
                format!(
                    "Command '{command}' requires all given arguments to be appended to it (ends with special characters '%%'), but also has custom arguments. Right now, only one of these features can be used at a time on a given command. Please alter your Bonnie configuration to reflect this.",
                    command=self.name
                )
            );
        }
        // Check if the correct number of arguments was provided (if we're appending, any number is valid)
        // Return an error if there are too few
        // Warn if there are too many
        if self.args.len() > arg_values.len() && !append_args {
            return Err(
                format!(
                    "The command '{command}' requires {num_required_args} argument(s), but {num_given_args} argument(s) were provided (too few). Please provide all the required arguments.",
                    command=&self.name,
                    num_required_args=&self.args.len(),
                    num_given_args=&arg_values.len()
                )
            );
        }
        // TEST this case
        if self.args.len() < arg_values.len() && !append_args {
            println!(
                "Warning: The command '{command}' only needs {num_required_args} argument(s), but {num_given_args} argument(s) were provided (too many). Your command will still run, this warning is just here to save time in debugging!",
                command=&self.name,
                num_required_args=&self.args.len(),
                num_given_args=&arg_values.len()
            );
        }
        let mut command_with_args_and_env_vars: String = self.cmd.to_string();

        if append_args {
            // Replace the special sequence '%%' with all the arguments joined together
            command_with_args_and_env_vars = self.cmd.replace("%%", &arg_values.join(" "));
        } else {
            // Loop through all the arguments the command takes, substituting in each one
            for (idx, arg) in self.args.iter().enumerate() {
                let arg_value = &arg_values[idx]; // The arrays are the same length, see above check
                                                  // All arguments are shown in the command string as `%name` or the like, so we get that whole string
                let arg_with_sign = "%".to_string() + arg;
                let new_command =
                    command_with_args_and_env_vars.replace(&arg_with_sign, &arg_value);
                // Run a quick check to make sure we've changed something (otherwise there's probably a typo in the command)
                // We return an error here because substituting '%arg_name' into the command may result in undefined behaviour
                if new_command == command_with_args_and_env_vars {
                    return Err(
                        format!(
                            "The argument '{arg_name}' could not be substituted into the command '{command}'. This probably means there's a typo somewhere in your command definition.",
                            arg_name=&arg,
                            command=&self.name
                        )
                    );
                }
                command_with_args_and_env_vars = new_command;
            }
        }

        // Now we interpolate environment variables in a similar way
        // Note that if the user defines an argument and environment variable with the same name, the argument will be evaluated first

        // Loop through all tghe environment variables the command needs
        for env_var_name in self.env_vars.iter() {
            // Load the environment variable
            let env_var = env::var(env_var_name);
            let env_var = match env_var {
                Ok(env_var) => env_var,
                Err(_) => return Err(format!("The environment variable '{}' couldn't be loaded. This means it either hasn't been defined (you may need to load another environment variable file) or contains invalid characters.", env_var_name))
            };
            // Interpolate it into the command itself
            let to_replace = "%".to_string() + env_var_name;
            let new_command = command_with_args_and_env_vars.replace(&to_replace, &env_var);
            // Run a quick check to make sure we've changed something (otherwise there's probably a typo in the command)
            // We return an error here because substituting '%ENV_VAR_NAME' into the command may result in undefined behaviour
            if new_command == command_with_args_and_env_vars {
                return Err(
                    format!(
                        "The environment variable '{env_var_name}' could not be substituted into the command '{command}'. This probably means there's a typo somewhere in your command definition.",
                        env_var_name=&env_var_name,
                        command=&self.name
                    )
                );
            }
            command_with_args_and_env_vars = new_command;
        }

        Ok(command_with_args_and_env_vars)
    }
}
