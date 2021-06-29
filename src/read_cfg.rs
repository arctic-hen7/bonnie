use serde::Deserialize;
use std::collections::HashMap;

use crate::version::{get_version_parts, VersionCompatibility, VersionDifference};
use crate::command::Command;
use crate::commands_registry::CommandsRegistry;

// Anything with `Raw` in front of it is deserialised directly into
// Anything without `Raw` in front of it is the final, parsed form

// The parsed config doesn't need to worry about environment variable files (they get loaded in the parsing process)
#[derive(Deserialize, Debug)]
pub struct Config {
    scripts: HashMap<String, Script>,
}

#[derive(Deserialize, Debug)]
pub struct RawConfig {
    version: String, // As of 0.2.0, a semantic version is required in all Bonnie configuration files to control compatibility
    env_files: Option<Vec<String>>,
    scripts: HashMap<String, RawScript>,
}

// The long-form notation for a command
// This isn't used directly in deserialisation, but all commands end up in this format
#[derive(Deserialize, Debug)]
pub struct Script {
    args: Vec<String>, // User-provided arguments
    cmd: String,
    env_vars: Vec<String>, // Environment variables to interpolate
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum RawScript {
    // Both arguments and environment variables to interpolate are optional here
    Complex {
        args: Option<Vec<String>>,
        cmd: String, // The user must always specify an actual command to run
        env_vars: Option<Vec<String>>,
    },
    Simple(String), // This variant is shorthand when a command has no arguments or interpolation
}

// Parses a given config string (extracted for testing purposes)
// The Bonnie version is also extracted for testing
pub fn parse_cfg(cfg_string: String, bonnie_version_str: &str) -> Result<Config, String> {
    // Deserialise the config file
    let raw_cfg: Result<RawConfig, toml::de::Error> = toml::from_str(&cfg_string);
    let raw_cfg = match raw_cfg {
        Ok(raw_cfg) => raw_cfg,
        // We explicitly handle the missing version for backward-compatibility before 0.2.0 and because it's an easy mistake to make
        Err(err) if err.to_string().starts_with("missing field `version`") => return Err("Your Bonnie configuration file appears to be missing a 'version' key. From Bonnie 0.2.0 onwards, this key is mandatory for compatibility reasons. Please add `version = \"".to_string() + bonnie_version_str + "\"` to the top of your Bonnie configuration file."),
        Err(_) => return Err(String::from("Invalid Bonnie configuration file."))
    };

    // Split the program and config file versions into their components
    let bonnie_version = get_version_parts(bonnie_version_str)?;
    let cfg_version = get_version_parts(&raw_cfg.version)?;
    // Compare the two and warn/error appropriately
    let compat = bonnie_version.is_compatible_with(&cfg_version);
    match compat {
        VersionCompatibility::DifferentBetaVersion(version_difference) => return Err("The provided configuration file is incompatible with this version of Bonnie. You are running Bonnie v".to_string() + bonnie_version_str + ", but the configuration file expects Bonnie v" + &raw_cfg.version + ". " + match version_difference {
            VersionDifference::TooNew => "This issue can be fixed by updating Bonnie to the appropriate version, which can be done at https://github.com/arctic-hen7/bonnie/releases.",
            VersionDifference::TooOld => "This issue can be fixed by updating the configuration file, which may require changing some of its syntax (see https://github.com/arctic-hen7/bonnie for how to do so). Alternatively, you can download an older version of Bonnie from https://github.com/arctic-hen7/bonnie/releases (not recommended)."
        }),
        VersionCompatibility::DifferentMajor(version_difference) => return Err("The provided configuration file is incompatible with this version of Bonnie. You are running Bonnie v".to_string() + bonnie_version_str + ", but the configuration file expects Bonnie v" + &raw_cfg.version + ". " + match version_difference {
            VersionDifference::TooNew => "This issue can be fixed by updating Bonnie to the appropriate version, which can be done at https://github.com/arctic-hen7/bonnie/releases.",
            VersionDifference::TooOld => "This issue can be fixed by updating the configuration file, which may require changing some of its syntax (see https://github.com/arctic-hen7/bonnie for how to do so). Alternatively, you can download an older version of Bonnie from https://github.com/arctic-hen7/bonnie/releases (not recommended)."
        }),
        // These next two are just warnings, not errors
        VersionCompatibility::DifferentMinor(version_difference) => println!("{}", "The provided configuration file is compatible with this version of Bonnie, but has a different minor version. You are running Bonnie v".to_string() + bonnie_version_str + ", but the configuration file expects Bonnie v" + &raw_cfg.version + ". " + match version_difference {
            VersionDifference::TooNew => "This issue can be fixed by updating Bonnie to the appropriate version, which can be done at https://github.com/arctic-hen7/bonnie/releases.",
            VersionDifference::TooOld => "This issue can be fixed by updating the configuration file, which may require changing some of its syntax (see https://github.com/arctic-hen7/bonnie for how to do so). Alternatively, you can download an older version of Bonnie from https://github.com/arctic-hen7/bonnie/releases (not recommended)."
        }),
        VersionCompatibility::DifferentPatch(version_difference) => println!("{}", "The provided configuration file is compatible with this version of Bonnie, but has a different patch version. You are running Bonnie v".to_string() + bonnie_version_str + ", but the configuration file expects Bonnie v" + &raw_cfg.version + ". " + match version_difference {
            VersionDifference::TooNew => "You may want to update Bonnie to the appropriate version, which can be done at https://github.com/arctic-hen7/bonnie/releases.",
            VersionDifference::TooOld => "You may want to update the configuration file (which shouldn't require any syntax changes)."
        }),
        _ => ()
    };

    // Parse the scripts (resolving the enum to a single value)
    let mut parsed_scripts: HashMap<String, Script> = HashMap::new();
    for (name, script) in raw_cfg.scripts {
        parsed_scripts.insert(name, parse_script(&script));
    }

    // If no environment variable files are being requested, we just make the array empty
    let env_files = match raw_cfg.env_files {
        Some(env_files) => env_files,
        None => Vec::new(),
    };

    // Parse each of the requested environment variable files
    for env_file in env_files {
        // Load the file
        // This will be loaded for the Bonnie program, which allows us to interpolate them into commands
        // TODO check how these paths are formed (relativity etc.)
        let res = dotenv::from_filename(&env_file);
        if res.is_err() {
            return Err(format!("Requested environment variable file '{}' could not be loaded. Either the file doesn't exist, Bonnie doesn't have the permissions necessary to access it, or something inside it can't be processed.", &env_file));
        }
    }

    Ok(Config {
        scripts: parsed_scripts,
    })
}

fn parse_script(unparsed_script: &RawScript) -> Script {
    match unparsed_script {
        // When processing a complex script, any missing values are added as empty
        RawScript::Complex {
            args,
            cmd,
            env_vars,
        } => Script {
            args: match args {
                Some(args) => args.to_vec(),
                None => Vec::new(),
            },
            cmd: cmd.to_string(),
            env_vars: match env_vars {
                Some(env_vars) => env_vars.to_vec(),
                None => Vec::new(),
            },
        },
        RawScript::Simple(cmd) => Script {
            args: Vec::new(), // A simple script can't specify these options, so they'll always be empty
            cmd: cmd.to_string(),
            env_vars: Vec::new(), // A simple script can't specify these options, so they'll always be empty
        },
    }
}

pub fn get_commands_registry_from_cfg(cfg: &Config) -> CommandsRegistry {
    let mut commands_registry = CommandsRegistry::new();

    for (name, script) in cfg.scripts.iter() {
        commands_registry.add(
            name,
            Command::new(
                name,
                script.args.to_vec(),
                script.env_vars.to_vec(),
                &script.cmd,
            ),
        )
    }

    commands_registry
}
