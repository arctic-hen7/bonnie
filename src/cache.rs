use crate::version::BONNIE_VERSION;
use crate::{raw_schema, schema};
use std::fs;

// TODO add support for custom cache file paths with `BONNIE_CACHE`

// Serializes the given parsed configuration into a JSON string and write it to disk to speed up future execution
// This takes around 100ms on an old i7 for the testing file
pub fn cache(cfg: &schema::Config) -> Result<(), String> {
    let cache_str = serde_json::to_string(cfg);
    let cache_str = match cache_str {
        Ok(cache_str) => cache_str,
        Err(err) => return Err(format!("The following error occurred while attempting to cache your parsed Bonnie configuration: '{}'.", err))
    };
    let output = fs::write("./.bonnie.cache.json", cache_str);
    match output {
        Ok(_) => Ok(()),
        Err(err) => return Err(format!("The following error occurred while attempting to write your cached Bonnie configuration to './.bonnie.cache.json': '{}'.", err))
    }
}

pub fn cache_exists() -> bool {
    fs::metadata("./.bonnie.cache.json").is_ok()
}

// This does NOT attempt to check if the cache is out of date for performance
// The user must manually recache
pub fn load_from_cache(output: &mut impl std::io::Write) -> Result<schema::Config, String> {
    let cfg_str = fs::read_to_string("./.bonnie.cache.json");
    let cfg_str = match cfg_str {
        Ok(cfg_str) => cfg_str,
        Err(err) => return Err(format!("The following error occurred while attempting to read your cached Bonnie configuration: '{}'.", err))
    };

    let cfg = serde_json::from_str::<schema::Config>(&cfg_str);
    let cfg = match cfg {
        Ok(cfg) => cfg,
        Err(err) => return Err(format!("The following error occurred while attempting to parse your cached Bonnie configuration: '{}'. If this persists, you can recache with `bonnie -c`.", err))
    };
    // Check the version
    raw_schema::Config::parse_version_against_current(&cfg.version, BONNIE_VERSION, output)?;
    // Load the environment variable files
    raw_schema::Config::load_env_files(Some(cfg.env_files.clone()))?;

    Ok(cfg)
}
