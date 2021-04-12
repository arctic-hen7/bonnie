use crate::command::Command;
use crate::commands_registry::CommandsRegistry;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Config {
    scripts: HashMap<String, CommandWithArgs>,
}

#[derive(Deserialize, Debug)]
pub struct RawConfig {
    scripts: HashMap<String, Script>,
}

#[derive(Deserialize, Debug)]
pub struct CommandWithArgs {
    args: Vec<String>,
    cmd: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Script {
    WithArgs { args: Vec<String>, cmd: String },
    NoArgs(String), // This variant is shorthand when a command has no arguments
}

// Parses a given config string (extracted for testing purposes)
pub fn parse_cfg(cfg_string: String) -> Result<Config, String> {
    let raw_cfg: Result<RawConfig, toml::de::Error> = toml::from_str(&cfg_string);
    let raw_cfg = match raw_cfg {
        Ok(raw_cfg) => raw_cfg,
        Err(_) => return Err(String::from("Invalid Bonnie configuration file.")),
    };

    // Parse the scripts (resolving the enum to a single value)
    let mut parsed_scripts: HashMap<String, CommandWithArgs> = HashMap::new();
    for (name, script) in raw_cfg.scripts {
        parsed_scripts.insert(name, parse_script(&script));
    }

    Ok(Config {
        scripts: parsed_scripts,
    })
}

fn parse_script(unparsed_script: &Script) -> CommandWithArgs {
    match unparsed_script {
        Script::WithArgs { args, cmd } => CommandWithArgs {
            args: args.to_vec(),
            cmd: cmd.to_string(),
        },
        Script::NoArgs(cmd) => CommandWithArgs {
            args: Vec::new(),
            cmd: cmd.to_string(),
        },
    }
}

pub fn get_commands_registry_from_cfg(cfg: &Config) -> CommandsRegistry {
    let mut commands_registry = CommandsRegistry::new();

    for (name, script) in cfg.scripts.iter() {
        commands_registry.add(name, Command::new(name, script.args.to_vec(), &script.cmd))
    }

    commands_registry
}
