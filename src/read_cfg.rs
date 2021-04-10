use std::fs;
use std::collections::HashMap;
use serde::Deserialize;
use crate::commands_registry::CommandsRegistry;
use crate::command::Command;

#[derive(Deserialize, Debug)]
pub struct Config {
	scripts: HashMap<String, CommandWithArgs>
}

#[derive(Deserialize, Debug)]
pub struct RawConfig {
	scripts: HashMap<String, Script>
}

#[derive(Deserialize, Debug)]
pub struct CommandWithArgs {
	args: Vec<String>,
	cmd: String
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Script {
	WithArgs {
		args: Vec<String>,
		cmd: String
	},
	NoArgs(String) // This variant is shorthand when a command has no arguments
}

// Extracts the config from the TOML file and parses it
pub fn get_cfg(path: &str) -> Config {
	let cfg_string = fs::read_to_string(path).expect("Error reading bonnie.toml, make sure the file is present in this directory and you have the permissions to read it.");
	let raw_cfg: RawConfig = toml::from_str(&cfg_string).expect("Invalid Bonnie configuration file.");

	// Parse the scripts (resolving the enum to a single value)
	let mut parsed_scripts: HashMap<String, CommandWithArgs> = HashMap::new();
	for (name, script) in raw_cfg.scripts {
		parsed_scripts.insert(name, parse_script(&script));
	}

	Config {
		scripts: parsed_scripts
	}
}

fn parse_script(unparsed_script: &Script) -> CommandWithArgs {
	match unparsed_script {
		Script::WithArgs { args, cmd } => CommandWithArgs {
			args: args.to_vec(),
			cmd: cmd.to_string()
		},
		Script::NoArgs(cmd) => CommandWithArgs {
			args: Vec::new(),
			cmd: cmd.to_string()
		}
	}
}

pub fn get_commands_registry_from_cfg(cfg: &Config) -> CommandsRegistry {
	let mut commands_registry = CommandsRegistry::new();

	for (name, script) in cfg.scripts.iter() {
		commands_registry.add(name, Command::new(name, script.args.to_vec(), &script.cmd))
	}

	commands_registry
}