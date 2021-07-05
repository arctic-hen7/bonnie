// This file contains logic to get the actual configuration itself

use std::fs;
use std::env;

// This can be changed by the user with the `BONNIE_CONF` environment variable
pub const DEFAULT_BONNIE_CFG_PATH: &str = "./bonnie.toml";

// Extracts the config from the TOML file at the given path
pub fn get_cfg() -> Result<String, String> {
    // Get the path of the config
    let path = get_cfg_path()?;
    let cfg_string = fs::read_to_string(&path);
    match cfg_string {
		Ok(cfg_string) => Ok(cfg_string),
		Err(_) => Err(format!("Error reading Bonnie configuration file at '{}', make sure the file is present in this directory and you have the permissions to read it.", path))
	}
}

// Gets the path to the config file based on given environment variables
// This will return an error if the `BONNIE_CONF` environment variable is set, but is invalid
fn get_cfg_path() -> Result<String, String> {
    // Get the `BONNIE_CONF` variable
    let given_path = env::var("BONNIE_CONF");
    match given_path {
        Ok(path) => Ok(path),
        Err(env::VarError::NotUnicode(_)) => Err(String::from("The path to your Bonnie configuration file given in the 'BONNIE_CONF' environment variable contained invalid characters. Please make sure it only contains valid Unicode.")),
        Err(env::VarError::NotPresent) => Ok(DEFAULT_BONNIE_CFG_PATH.to_string()) // If the env var wasn't found, then use the default
    }
}
