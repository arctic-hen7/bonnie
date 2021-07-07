use crate::version::BONNIE_VERSION;
use crate::{raw_schema, schema};
use std::env;
use std::fs;

// This can be changed by the user with the `BONNIE_CACHE` environment variable
pub const DEFAULT_BONNIE_CACHE_PATH: &str = "./.bonnie.cache.json";

// Gets the path to the cache file based on given environment variables
// This will return an error if the `BONNIE_CACHE` environment variable is set, but is invalid
fn get_cache_path() -> Result<String, String> {
    // Get the `BONNIE_CACHE` variable
    let given_path = env::var("BONNIE_CACHE");
    match given_path {
        Ok(path) => Ok(path),
        Err(env::VarError::NotUnicode(_)) => Err(String::from("The path to your Bonnie cache file given in the 'BONNIE_CACHE' environment variable contained invalid characters. Please make sure it only contains valid Unicode.")),
        Err(env::VarError::NotPresent) => Ok(DEFAULT_BONNIE_CACHE_PATH.to_string()) // If the env var wasn't found, then use the default
    }
}

// Serializes the given parsed configuration into a JSON string and write it to disk to speed up future execution
// This takes around 100ms on an old i7 for the testing file
pub fn cache(cfg: &schema::Config, output: &mut impl std::io::Write) -> Result<(), String> {
    let cache_path = get_cache_path()?;
    let cache_str = serde_json::to_string(cfg);
    let cache_str = match cache_str {
        Ok(cache_str) => cache_str,
        Err(err) => return Err(format!("The following error occurred while attempting to cache your parsed Bonnie configuration: '{}'.", err))
    };
    let res = fs::write(&cache_path, cache_str);
    if let Err(err) = res {
        return Err(format!("The following error occurred while attempting to write your cached Bonnie configuration to '{}': '{}'.", &cache_path, err));
    }

    writeln!(
        output,
        "Your Bonnie configuration has been successfully cached to '{}'! This will be used to speed up future execution. Please note that this cache will NOT be updated until you explicitly run `bonnie -c` again.",
        cache_path
    ).expect("Failed to write caching message.");
    Ok(())
}

pub fn cache_exists() -> Result<bool, String> {
    let exists = fs::metadata(get_cache_path()?).is_ok();
    Ok(exists)
}

// This does NOT attempt to check if the cache is out of date for performance
// The user must manually recache
pub fn load_from_cache(output: &mut impl std::io::Write) -> Result<schema::Config, String> {
    let cache_path = get_cache_path()?;
    let cfg_str = fs::read_to_string(&cache_path);
    let cfg_str = match cfg_str {
        Ok(cfg_str) => cfg_str,
        Err(err) => return Err(format!("The following error occurred while attempting to read your cached Bonnie configuration at '{}': '{}'.", &cache_path, err))
    };

    let cfg = serde_json::from_str::<schema::Config>(&cfg_str);
    let cfg = match cfg {
        Ok(cfg) => cfg,
        Err(err) => return Err(format!("The following error occurred while attempting to parse your cached Bonnie configuration at '{}': '{}'. If this persists, you can recache with `bonnie -c`.", &cache_path, err))
    };
    // Check the version
    raw_schema::Config::parse_version_against_current(&cfg.version, BONNIE_VERSION, output)?;
    // Load the environment variable files
    raw_schema::Config::load_env_files(Some(cfg.env_files.clone()))?;

    Ok(cfg)
}
