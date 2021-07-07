// This file contains the schema that all Bonnie configuration files are deserialised with
// They will then be parsed into the schema defined in `schema.rs` using the logic in the methods on this schema
// The use of `#[serde(untagged)]` on all `enum`s simply ensures that Serde doesn't require them to be labelled as to their variant
// This raw schema will also derive the `Arbitrary` trait for fuzzing when that feature is enabled

use crate::bones::parse_directive_str;
use crate::default_shells::get_default_shells;
use crate::schema;
use crate::version::{get_version_parts, VersionCompatibility, VersionDifference, BONNIE_VERSION};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    version: String,                // This will be used to confirm compatibility
    env_files: Option<Vec<String>>, // Files specified here have their environment variables loaded into Bonnie
    default_shell: Option<DefaultShell>,
    scripts: Scripts,
}
impl Config {
    pub fn new(cfg_string: &str) -> Result<Self, String> {
        let cfg: Result<Self, toml::de::Error> = toml::from_str(cfg_string);
        let cfg = match cfg {
            Ok(cfg) => cfg,
            // We explicitly handle the missing version for better backward-compatibility before 0.2.0 and because it's an easy mistake to make
            Err(err) if err.to_string().starts_with("missing field `version`") => return Err("Your Bonnie configuration file appears to be missing a 'version' key. From Bonnie 0.2.0 onwards, this key is mandatory for compatibility reasons. Please add `version = \"".to_string() + BONNIE_VERSION + "\"` to the top of your Bonnie configuration file."),
            Err(err) => return Err(format!("Invalid Bonnie configuration file. Error: '{}'", err))
        };

        Ok(cfg)
    }
    // Runs all the necessary methods to fully parse the config, consuming `self`
    // Takes the current version of Bonnie (extracted for testing purposes)
    // This accepts an output for warnings (extracted for testing)
    pub fn to_final(
        &self,
        bonnie_version_str: &str,
        output: &mut impl std::io::Write,
    ) -> Result<schema::Config, String> {
        // These two are run for their side-effects
        self.parse_version_against_current(bonnie_version_str, output)?;
        self.load_env_files()?;
        // And then we get the final config
        let cfg = self.parse()?;

        Ok(cfg)
    }
    // Parses the version of the config to check for compatibility issues, consuming `self`
    // We extract the version of Bonnie itself for testing purposes
    fn parse_version_against_current(
        &self,
        bonnie_version_str: &str,
        output: &mut impl std::io::Write,
    ) -> Result<(), String> {
        // Split the program and config file versions into their components
        let bonnie_version = get_version_parts(bonnie_version_str)?;
        let cfg_version = get_version_parts(&self.version)?;
        // Compare the two and warn/error appropriately
        let compat = bonnie_version.is_compatible_with(&cfg_version);
        match compat {
            VersionCompatibility::DifferentBetaVersion(version_difference) => return Err("The provided configuration file is incompatible with this version of Bonnie. You are running Bonnie v".to_string() + bonnie_version_str + ", but the configuration file expects Bonnie v" + &self.version + ". " + match version_difference {
                VersionDifference::TooNew => "This issue can be fixed by updating Bonnie to the appropriate version, which can be done at https://github.com/arctic-hen7/bonnie/releases.",
                VersionDifference::TooOld => "This issue can be fixed by updating the configuration file, which may require changing some of its syntax (see https://github.com/arctic-hen7/bonnie for how to do so). Alternatively, you can download an older version of Bonnie from https://github.com/arctic-hen7/bonnie/releases (not recommended)."
            }),
            VersionCompatibility::DifferentMajor(version_difference) => return Err("The provided configuration file is incompatible with this version of Bonnie. You are running Bonnie v".to_string() + bonnie_version_str + ", but the configuration file expects Bonnie v" + &self.version + ". " + match version_difference {
                VersionDifference::TooNew => "This issue can be fixed by updating Bonnie to the appropriate version, which can be done at https://github.com/arctic-hen7/bonnie/releases.",
                VersionDifference::TooOld => "This issue can be fixed by updating the configuration file, which may require changing some of its syntax (see https://github.com/arctic-hen7/bonnie for how to do so). Alternatively, you can download an older version of Bonnie from https://github.com/arctic-hen7/bonnie/releases (not recommended)."
            }),
            // These next two are just warnings, not errors
            VersionCompatibility::DifferentMinor(version_difference) => writeln!(output, "{}", "The provided configuration file is compatible with this version of Bonnie, but has a different minor version. You are running Bonnie v".to_string() + bonnie_version_str + ", but the configuration file expects Bonnie v" + &self.version + ". " + match version_difference {
                VersionDifference::TooNew => "This issue can be fixed by updating Bonnie to the appropriate version, which can be done at https://github.com/arctic-hen7/bonnie/releases.",
                VersionDifference::TooOld => "This issue can be fixed by updating the configuration file, which may require changing some of its syntax (see https://github.com/arctic-hen7/bonnie for how to do so). Alternatively, you can download an older version of Bonnie from https://github.com/arctic-hen7/bonnie/releases (not recommended)."
            }).expect("Failed to write warning."),
            VersionCompatibility::DifferentPatch(version_difference) => writeln!(output, "{}", "The provided configuration file is compatible with this version of Bonnie, but has a different patch version. You are running Bonnie v".to_string() + bonnie_version_str + ", but the configuration file expects Bonnie v" + &self.version + ". " + match version_difference {
                VersionDifference::TooNew => "You may want to update Bonnie to the appropriate version, which can be done at https://github.com/arctic-hen7/bonnie/releases.",
                VersionDifference::TooOld => "You may want to update the configuration file (which shouldn't require any syntax changes)."
            }).expect("Failed to write warning."),
            _ => ()
        };

        // If we haven't returned an error yet, the version is valid (and warnings have been emitted as necessary)
        Ok(())
    }
    // Loads the environment variable files requested in the config
    fn load_env_files(&self) -> Result<(), String> {
        let env_files = match self.env_files.clone() {
            Some(env_files) => env_files,
            None => Vec::new(),
        };
        // Parse each of the requested environment variable files
        for env_file in env_files.iter() {
            // Load the file
            // This will be loaded for the Bonnie program, which allows us to interpolate them into commands
            let res = dotenv::from_filename(&env_file);
            if res.is_err() {
                return Err(format!("Requested environment variable file '{}' could not be loaded. Either the file doesn't exist, Bonnie doesn't have the permissions necessary to access it, or something inside it can't be processed.", &env_file));
            }
        }

        Ok(())
    }
    // Parses the rest of the config into the final form, consuming `self`
    // A very large portion of Bonnie's logic lives here or is called here (spec transformation)
    fn parse(&self) -> Result<schema::Config, String> {
        // Parse the default shell
        let default_shell = match &self.default_shell {
            // If we're just given a shell string, use it as the generic shell
            Some(DefaultShell::Simple(generic)) => schema::DefaultShell {
                generic: generic.to_vec(),
                targets: HashMap::new(),
            },
            // If we have all the information we need, just transform it
            Some(DefaultShell::Complex { generic, targets }) => schema::DefaultShell {
                generic: generic.to_vec(),
                targets: match targets {
                    Some(raw_targets) => {
                        // This is just transformation logic
                        let mut targets = HashMap::new();
                        for (target_name, shell) in raw_targets.iter() {
                            targets.insert(target_name.to_string(), shell.to_vec());
                        }
                        targets
                    }
                    None => HashMap::new(), // We'll just use the generic if we don't have anything else
                },
            },
            // If no default shell is provided, we'll use the default paradigm (see `default_shells.rs`)
            None => get_default_shells(),
        };
        // Parse the scripts (brace yourself!)
        // We do this inside a function because it's recursive
        // Unfortunately we can't define methods on type aliases, so this goes here
        // This involves validation logic to ensure invalid property combinations aren't specified, so we need to know whether or not `order` is specified if this is parsing subcommands
        fn parse_scripts(
            raw_scripts: &Scripts,
            is_order_defined: bool,
        ) -> Result<schema::Scripts, String> {
            let mut scripts: schema::Scripts = HashMap::new();
            for (script_name, raw_command) in raw_scripts.iter() {
                let command = match raw_command {
                    Command::Simple(raw_command_wrapper) => schema::Command {
                        args: Vec::new(),
                        env_vars: Vec::new(),
                        subcommands: None,
                        order: None,
                        cmd: Some(raw_command_wrapper.parse()) // In the simple form, a command must be given (no subcommands can be specified)
                    },
                    Command::Complex {
                        args,
                        env_vars,
                        subcommands,
                        order,
                        cmd
                    } => schema::Command {
                        // If `order` is defined at the level above, we can't interpolate environment variables from here (has to be done at the level `order` was specified)
                        args: match is_order_defined {
                            // Unordered subcommands can't take arguments in any case of upper-level `order` definition
                            _ if matches!(subcommands, Some(_)) && matches!(order, None) && matches!(args, Some(_)) => return Err(format!("Error in parsing Bonnie configuration file: if `subcommands` is specified without `order`, `args` cannot be specified. This error occurred in in the '{}' script/subscript.", script_name)),
                            // If it was and `args` is specified, return an error
                            true if matches!(args, Some(_)) => return Err(format!("Error in parsing Bonnie configuration file: if `order` is specified, subscripts cannot specify `args`, as no environment variables can be provided to them. Environment variables to be interpolated in ordered subcommands must be set at the top-level. This error occurred in the '{}' script/subscript.", script_name)),
                            // If it was but args` isn't specified, it doesn't matter and we just give an empty vector instead
                            true => Vec::new(),
                            // If it wasn't, no validation needed
                            false => args.as_ref().unwrap_or(&Vec::new()).to_vec()
                        },
                        // This doesn't need any transformation, just a simple alternative if it's `None`
                        env_vars: env_vars.as_ref().unwrap_or(&Vec::new()).to_vec(),
                        // The subcommands are parsed recursively as scripts using this very function
                        // We parse through whether or not `order` is defined (has validation implications)
                        subcommands: match subcommands {
                            // We can't use `.map()` for this because we need support for `?`
                            Some(subcommands) => Some(
                                parse_scripts(&subcommands, matches!(order, Some(_)))?
                            ),
                            None => None
                        },
                        // If `order` is defined at the level above and `subcommands` is defined here, `order` must be defined here too
                        order: match is_order_defined {
                            true if matches!(subcommands, Some(_)) => match order {
                                // If it was required and was given, no problem
                                Some(order) => Some(parse_directive_str(order)?),
                                // If it was required but not given, return an error
                                None => return Err(format!("Error in parsing Bonnie configuration file: if `order` is specified, all further nested subsubcommands must also specify `order`. This occurred in the '{}' script/subscript.", script_name))
                            }
                            // If it wasn't required, no validation needed
                            true | false => match order {
                                Some(order) => Some(parse_directive_str(order)?),
                                None => None
                            }
                        },
                        // If subcommands were specified, this is optional, otherwise we return an error
                        cmd: match cmd {
                            // It was given, but there are also ordered subcommands here, so execution will be ambiguous, return an error
                            Some(_) if matches!(order, Some(_)) => return Err(format!("Error in parsing Bonnie configuration file: both `cmd` and `order` were specified. This would lead to problems of ambiguous execution, so commands can have either the top-level `cmd` property or ordered subcommands, the two are mutually exclusive. This error occurred in in the '{}' script/subscript.", script_name)),
                            // It's optional
                            _ if matches!(subcommands, Some(_)) => cmd.as_ref().map(|cmd| cmd.parse()),
                            // It's mandatory and given
                            Some(cmd) => Some(cmd.parse()),
                            // It's mandatory and not given
                            None => return Err(format!("Error in parsing Bonnie configuration file: if `subcommands` is not specified, `cmd` is mandatory. This error occurred in in the '{}' script/subscript.", script_name))
                        }
                    },
                };
                scripts.insert(script_name.to_string(), command);
            }

            Ok(scripts)
        }

        let scripts = parse_scripts(&self.scripts, false)?;

        Ok(schema::Config {
            default_shell,
            scripts,
        })
    }
}
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum DefaultShell {
    Simple(Shell), // Just a generic shell
    Complex {
        generic: Shell, // A generic shell must be given
        targets: Option<HashMap<String, Shell>>,
    },
}
type Shell = Vec<String>; // A vector of the executable followed by raw arguments thereto, the location for command interpolation is specified with '{COMMAND}'
type TargetString = String; // A target like `linux` or `x86_64-unknown-linux-musl` (see `rustup` targets)
type Scripts = HashMap<String, Command>;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum Command {
    Simple(CommandWrapper), // Might be just a string command to run on the default generic shell
    Complex {
        args: Option<Vec<String>>,
        env_vars: Option<Vec<String>>,
        subcommands: Option<Scripts>, // Subcommands are fully-fledged  commands (mostly)
        order: Option<OrderString>, // If this is specified,subcomands must not specify the `args` property, it may be specified at the top-level of this script as a sibling of `order`
        cmd: Option<CommandWrapper>, // This is optional if subcommands are specified
    },
}
type OrderString = String; // A string of as yet undefined syntax that defines the progression between subcommands
                           // This wraps the complexities of having different shell logic for each command in a multi-stage context
                           // subcommands are specified above this level (see `Command::Complex`)
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum CommandWrapper {
    Universal(CommandCore), // Just a given command
    Specific {
        generic: CommandCore,
        targets: Option<HashMap<TargetString, CommandCore>>,
    },
}
impl CommandWrapper {
    // Parses `self` into its final form (`schema::CommandWrapper`)
    fn parse(&self) -> schema::CommandWrapper {
        match self {
            // If it's universal to all targets, just provide a generic
            CommandWrapper::Universal(raw_command_core) => schema::CommandWrapper {
                generic: raw_command_core.parse(),
                targets: HashMap::new(),
            },
            // If no targets were given in specific form, the expansion is basically the same as if it were universal
            CommandWrapper::Specific {
                generic,
                targets: None,
            } => schema::CommandWrapper {
                generic: generic.parse(),
                targets: HashMap::new(),
            },
            CommandWrapper::Specific {
                generic,
                targets: Some(targets),
            } => {
                let parsed_generic = generic.parse();
                let mut parsed_targets: HashMap<schema::TargetString, schema::CommandCore> =
                    HashMap::new();
                for (target_name, raw_command_core) in targets.iter() {
                    parsed_targets.insert(target_name.to_string(), raw_command_core.parse());
                }
                schema::CommandWrapper {
                    generic: parsed_generic,
                    targets: parsed_targets,
                }
            }
        }
    }
}
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum CommandCore {
    Simple(CommandBox), // No shell configuration
    WithShell {
        exec: CommandBox, // We can't call this `cmd` because otherwise we'd have a collision with the higher-level `cmd`, which leads to misinterpretation
        shell: Option<Shell>,
    },
}
impl CommandCore {
    // Parses `self` into its final form (`schema::CommandCore`)
    fn parse(&self) -> schema::CommandCore {
        match self {
            CommandCore::Simple(exec) => schema::CommandCore {
                exec: exec.parse(),
                shell: None,
            },
            CommandCore::WithShell {
                exec,
                shell: Some(shell),
            } => schema::CommandCore {
                exec: exec.parse(),
                shell: Some(shell.to_vec()),
            },
            // If no shell was given in the complex form, the expansion is the same as the simple form
            CommandCore::WithShell { exec, shell: None } => schema::CommandCore {
                exec: exec.parse(),
                shell: None,
            },
        }
    }
}
// This represents the possibility of a vector or string at the lowest level
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum CommandBox {
    Simple(String),
    MultiStage(Vec<String>),
}
impl CommandBox {
    // Parses `self` into its final form (`Vec<schema::CommandWrapper>`)
    fn parse(&self) -> Vec<String> {
        match self {
            // In fully parsed form, all command wrappers are inside vectors for simplicity
            CommandBox::Simple(cmd_str) => vec![cmd_str.to_string()],
            CommandBox::MultiStage(cmd_strs) => {
                cmd_strs.iter().map(|cmd_str| cmd_str.to_string()).collect()
            }
        }
    }
}
