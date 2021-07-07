// This file contains the final schema into which all Bonnie configurations are parsed
// This does not reflect the actual syntax used in the configuration files themselves (see `raw_schema.rs`)

use crate::bones::{Bone, BonesCommand, BonesCore, BonesDirective};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub default_shell: DefaultShell,
    pub scripts: Scripts,
    // These last two properties are required for loading the config if it's cached
    pub env_files: Vec<String>,
    pub version: String,
}
impl Config {
    // Gets the command requested by the given vector of arguments
    // The given arguments are expected not to include the first program argument (`bonnie` or the like)
    // Returns the command itself, its name, and the arguments relevant thereto
    pub fn get_command_for_args(
        &self,
        args: &[String],
    ) -> Result<(&Command, String, Vec<String>), String> {
        // We do everything in here for recursion
        // We need to know if this is the first time so we know to say 'command' or 'subcommand' in error messages
        fn get_command_for_scripts_and_args<'a>(
            scripts: &'a Scripts,
            args: &[String],
            first_time: bool,
        ) -> Result<(&'a Command, String, Vec<String>), String> {
            // Get the name of the command
            let command_name = args.get(0);
            let command_name = match command_name {
                Some(command_name) => command_name,
                None => {
                    return Err(match first_time {
                        true => String::from("Please provide a command to run."),
                        false => String::from("Please provide a subcommand to run."),
                    })
                }
            };
            // Try to find it among those we know
            let command = scripts.get(command_name);
            let command = match command {
                Some(command) => command,
                None => {
                    return Err(match first_time {
                        true => format!("Unknown command '{}'.", command_name),
                        false => format!("Unknown subcommand '{}'.", command_name),
                    })
                }
            };
            // We found it, check if it has any unordered subcommands or a root-level command
            let final_command_and_relevant_args = match &command.subcommands {
                // It has a root-level command (which can't take arguments) and no more arguments are present, this is the command we want
                Some(_) if matches!(command.cmd, Some(_)) && args.len() == 1 => {
                    (command, command_name.to_string(), {
                        // We get the arguments to the program, excluding the name of this command, these are the arguments to be inteprolated
                        let mut args_for_interpolation = args.to_vec();
                        args_for_interpolation.remove(0);
                        args_for_interpolation
                    })
                }
                // It does, recurse on them
                Some(subcommands) if matches!(command.order, None) => {
                    // We remove the first argument, which is the name of this, the parent command
                    let mut args_without_this = args.to_vec();
                    args_without_this.remove(0);
                    get_command_for_scripts_and_args(&subcommands, &args_without_this, false)?
                    // It's no longer the first time obviously
                }
                // They're ordered and so individually uninvocable, this is the command we want
                Some(_) => (command, command_name.to_string(), {
                    // We get the arguments to the program, excluding the name of this command, these are the arguments to be inteprolated
                    let mut args_for_interpolation = args.to_vec();
                    args_for_interpolation.remove(0);
                    args_for_interpolation
                }),
                // It doesn't, this is the command we want
                None => (command, command_name.to_string(), {
                    // We get the arguments to the program, excluding the name of this command, these are the arguments to be inteprolated
                    let mut args_for_interpolation = args.to_vec();
                    args_for_interpolation.remove(0);
                    args_for_interpolation
                }),
            };

            Ok(final_command_and_relevant_args)
        }

        // Begin the recursion on the global scripts with the given arguments
        let data = get_command_for_scripts_and_args(&self.scripts, args, true)?;

        Ok(data)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DefaultShell {
    pub generic: Shell,
    pub targets: HashMap<String, Shell>, // If the required target is not found, `generic` will be tried
}
// Shells are a series of values, the first being the executable and the rest being raw arguments
// One of those arguments must contain '{COMMAND}', where the command will be interpoalted
pub type Shell = Vec<String>;
pub type TargetString = String; // A target like `linux` or `x86_64-unknown-linux-musl` (see `rustup` targets)
pub type Scripts = HashMap<String, Command>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Command {
    pub args: Vec<String>,
    pub env_vars: Vec<String>,
    pub subcommands: Option<Scripts>, // Subcommands are fully-fledged commands (mostly)
    pub order: Option<BonesDirective>, // If this is specified, subcomands must not specify the `args` property, it may be specified at the top-level of this script as a sibling of `order`
    pub cmd: Option<CommandWrapper>,   // If subcommands are provided, a root command is optional
}
impl Command {
    // Prepares a command by interpolating everything and resolving shell/tagret logic
    // This requires the name of the command and the file's `DefaultShell` configuration
    // This interpolates arguments and environment variables
    // This returns a `BonesCommand` to be executed
    // This accepts an output for warnings (extracted for testing)
    pub fn prepare(
        &self,
        name: &str,
        prog_args: &[String],
        default_shell: &DefaultShell,
    ) -> Result<Bone, String> {
        let bone = self.prepare_internal(name, prog_args, &default_shell, None)?;

        Ok(bone)
    }
    // This is the internal command preparation logic, which is called recursively.
    // This also takes top-level arguments for recursing on ordered subcommands
    fn prepare_internal(
        &self,
        name: &str,
        prog_args: &[String],
        default_shell: &DefaultShell,
        top_level_args: Option<&[String]>,
    ) -> Result<Bone, String> {
        let args = match top_level_args {
            Some(args) => args,
            None => &self.args,
        };
        let at_top_level = top_level_args.is_none();
        if matches!(self.subcommands, None)
            || (matches!(self.subcommands, Some(_)) && matches!(self.cmd, Some(_)))
        {
            // We have either a direct command or a parent command that has irrelevant subcommands, either way we're interpolating into `cmd`
            // Get the vector of command wrappers
            let command_wrapper = self.cmd.as_ref().unwrap(); // Assuming the transformation logic works, an error can't occur here
                                                              // We have to do this in a for loop for `?`
            let mut cmd_strs: Vec<BonesCore> = Vec::new();
            let (cmds, shell) = command_wrapper.get_commands_and_shell(&default_shell);
            for cmd_str in cmds {
                let with_env_vars = Command::interpolate_env_vars(&cmd_str, &self.env_vars)?;
                let (with_args, remaining_args) =
                    Command::interpolate_specific_args(&with_env_vars, name, &args, prog_args)?;
                let ready_cmd =
                    Command::interpolate_remaining_arguments(&with_args, &remaining_args);
                cmd_strs.push(BonesCore {
                    cmd: ready_cmd,
                    shell: shell.to_vec(),
                });
            }

            Ok(
                // This does not contain recursive `BonesCommands`, so it's `Bone::Simple`
                Bone::Simple(cmd_strs),
            )
        } else if matches!(self.subcommands, Some(_)) && matches!(self.order, Some(_)) {
            // First, we resolve all the subcommands to vectors of strings to actually run
            let mut cmds: HashMap<String, Bone> = HashMap::new();
            // Now we run checks on whether the correct number of arguments have been provided if we're at the very top level
            // Otherwise error messages will relate to irrelevant subcommands
            // We don't check the case where too few arguments were provided because that's irrelevant (think about it)
            if at_top_level && args.len() > prog_args.len() {
                return Err(
                    format!(
                        "The command '{command}' requires {num_required_args} argument(s), but {num_given_args} argument(s) were provided (too few). Please provide all the required arguments.",
                        command=name,
                        num_required_args=args.len(),
                        num_given_args=&prog_args.len()
                    )
                );
            }
            // We `.unwrap()` here because we know more than the compiler
            for (subcommand_name, subcommand) in self.subcommands.as_ref().unwrap().iter() {
                // Parse the subcommand
                // We parse in the top-level arguments because ordered subcommands can't take their own, they inherit from this level (or the level this level inherits from, etc.)
                let cmd = subcommand.prepare_internal(
                    subcommand_name,
                    prog_args,
                    &default_shell,
                    Some(&args),
                )?;
                cmds.insert(subcommand_name.to_string(), cmd);
            }

            // Now we return a complex `Bone` (because it contains a `BonesCommand` with a directive)
            Ok(Bone::Complex(
                BonesCommand::new(self.order.as_ref().unwrap(), cmds), // We know more than the compiler by the check above
            ))
        } else {
            // This should not be possible!
            panic!("Critical logic failure in preparing command. You should report this as a bug.");
        }
    }
    // Interpolates specific arguments (doesn't handle `%%`)
    // This takes a string to interpolate into and doesn't take `self` so the order is open
    // This returns the readied command string and the remaining arguments or an error if an argument couldn't be substituted in
    // Errors for when the argument can't be interpolated can be silenced for ordered subcommands (which have a universal argument list for many subcommands)
    fn interpolate_specific_args(
        cmd_str: &str,
        name: &str,
        args: &[String],
        prog_args: &[String],
    ) -> Result<(String, Vec<String>), String> {
        // Check if the correct number of arguments was provided
        // Even if we're inserting the rest later, we still need the mandatory ones
        if args.len() > prog_args.len() {
            return Err(
                format!(
                    "The command '{command}' requires {num_required_args} argument(s), but {num_given_args} argument(s) were provided (too few). Please provide all the required arguments.",
                    command=name,
                    num_required_args=args.len(),
                    num_given_args=&prog_args.len()
                )
            );
        }
        // We don't warn if there are too many and we're not inserting the rest with `%%` later because that would mean checking every potential subcommand for `%%` as well if they exist
        let mut with_args = cmd_str.to_string();
        // We need to know the index so we can correlate to the index of the argument in `args`
        for (idx, arg) in args.iter().enumerate() {
            // The arrays are the same length, see above check
            // All arguments are shown in the command string as `%name` or the like, so we get that whole string
            let given_value = &prog_args[idx];
            let arg_with_sign = "%".to_string() + arg;
            let new_command = with_args.replace(&arg_with_sign, &given_value);
            // We don't check if we changed something because that doesn't work for multistage or ordered subcommands
            with_args = new_command;
        }
        // Get the program args after a certain point so they can be inserted with `%%` if necessary
        // We do this by getting the part of slice after the specific arguments
        let (_, remaining_args) = prog_args.split_at(args.len());

        Ok((with_args, remaining_args.to_vec())) // FIXME
    }
    // Interpolates environment variables
    // This takes a string to interpolate into, the environment variables to interpolate, and the name of the command
    // This doesn't take `self` so the order is open
    // This returns the readied command string only, or an error relating to environment variable loading
    fn interpolate_env_vars(cmd_str: &str, env_vars: &[String]) -> Result<String, String> {
        let mut with_env_vars = cmd_str.to_string();
        for env_var_name in env_vars.iter() {
            // Load the environment variable
            let env_var = env::var(env_var_name);
            let env_var = match env_var {
                Ok(env_var) => env_var,
                Err(_) => return Err(format!("The environment variable '{}' couldn't be loaded. This means it either hasn't been defined (you may need to load another environment variable file) or contains invalid characters.", env_var_name))
            };
            // Interpolate it into the command itself
            let to_replace = "%".to_string() + env_var_name;
            let new_command = with_env_vars.replace(&to_replace, &env_var);
            // We don't check if we changed something because that doesn't work for multistage or ordered subcommands
            with_env_vars = new_command;
        }

        Ok(with_env_vars)
    }
    // Interpolates all the given arguments at `%%` if it exists
    // This takes a string to interpolate into and doesn't take `self` so the order is open
    // This returns the readied command string only
    fn interpolate_remaining_arguments(cmd_str: &str, prog_args: &[String]) -> String {
        // This is just a simple `replace` operation for the operator `%%`
        // Split the command by the block insertion operator `%%`
        let mut interpolated = String::new();
        let split_on_operator: Vec<&str> = cmd_str.split("%%").collect();
        for (idx, part) in split_on_operator.iter().enumerate() {
            if idx == split_on_operator.len() - 1 {
                // This is the last element, there's no operator after this
                interpolated.push_str(part);
            } else if part.ends_with('\\') {
                // This part ends with `\`, meaning the operator was escaped
                // We just give the `%%` back
                // We only give back the part up until the escape character
                interpolated.push_str(&part[0..part.len() - 1]);
                interpolated.push_str("%%");
            } else {
                // There's a legitimate operator that should be at the end of this part
                // We push the program's arguments
                interpolated.push_str(part);
                interpolated.push_str(&prog_args.join(" "));
            }
        }

        interpolated
    }
}

// This defines how the command runs on different targets
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommandWrapper {
    pub generic: CommandCore,
    pub targets: HashMap<TargetString, CommandCore>, // If empty or target not found, `generic` will be used
}
impl CommandWrapper {
    // Gets the command to run, interpolated into a shell from the ambient OS information
    // This critically resolves which target we're running on
    fn get_commands_and_shell(&self, default_shell: &DefaultShell) -> (Vec<String>, Shell) {
        // Get the current target (unfortuantely we can't actually get the value out of `cfg!` yet...)
        // If the user needs to set custom commands based on target arch etc., they can write a script for it, this is exhaustive enough!
        let running_on = match true {
            _ if cfg!(target_os = "windows") => "windows",
            _ if cfg!(target_os = "macos") => "macos",
            _ if cfg!(target_os = "ios") => "ios",
            _ if cfg!(target_os = "linux") => "linux",
            _ if cfg!(target_os = "android") => "android",
            _ if cfg!(target_os = "freebsd") => "freebsd",
            _ if cfg!(target_os = "dragonfly") => "dragonfly",
            _ if cfg!(target_os = "openbsd") => "openbsd",
            _ if cfg!(target_os = "netbsd") => "netbsd",
            _ => "unknown", // If they want to, the user could actually specify something for this (like begging to be run somewhere that makes sense)
        };
        // See if that target is specified explicitly
        let target_specific_command_core = self.targets.get(running_on);
        let command_core = match target_specific_command_core {
            Some(command_core) => command_core,
            None => &self.generic,
        };
        // Get the commands as a vector ready for interpolation
        let cmd = &command_core.exec;
        // Get the shell, using the configured per-file default if it was undefined
        let shell = match &command_core.shell {
            Some(shell) => shell,
            None => {
                // If a particular shell has been configured for the current target, use that
                // Otherwise, use the generic
                // Remember that the schema transformation inserts program-level defaults if they aren't configured for the file by the user
                let target_specific_shell = default_shell.targets.get(running_on);
                match target_specific_shell {
                    Some(default_shell) => default_shell,
                    None => &default_shell.generic,
                }
            }
        };

        (cmd.to_vec(), shell.to_vec())
    }
}
// This is the lowest level of command specification, there is no more recursion allowed here (thus avoiding circularity)
// Actual command must be specified here are strings (with potential interpolation of arguments and environment variables)
// This can also define which shell the command will use
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommandCore {
    pub exec: Vec<String>, // These are the actual commands that will be run (named differently to avoid collisions)
    pub shell: Option<Shell>, // If given, this is the shell it will be run in, or the `default_shell` config for this target will be used
}
