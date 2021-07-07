use lib::{cache, load_from_cache, Config, FinalConfig, BONNIE_VERSION};
use std::env;

// Each test in this file MUST have a separate temporary file to write to, otherwise undefined conflicts occur!

// This is one of the most syntactically complex use-cases of Bonnie (the most complex that's tested certainly)
const CFG_STR: &str = r#"
env_files = ["src/.env"]
default_env.generic = ["sh", "-c", "{COMMAND}"]
default_env.targets.linux = ["bash", "-c", "{COMMAND}"]
[scripts]
basic.subcommands.test.cmd.generic = "exit 5"
basic.subcommands.test.cmd.targets.linux.exec = [
    "echo %SHORTGREETING %% && exit 0",
    "echo %name && exit 1"
]
basic.subcommands.test.env_vars = ["SHORTGREETING"]
basic.subcommands.test.cmd.targets.linux.shell = ["sh", "-c", "{COMMAND}"]
basic.subcommands.nested.subcommands.test = "exit 2"
basic.subcommands.nested.subcommands.other = "exit 3"
basic.subcommands.nested.order = """
test {
    Any => other
}
"""
basic.args = ["name"]
basic.order = """
test {
    Any => nested {
        Any => test
    }
}
"""
"#;

// A testing utility to build a config to cache
#[cfg(test)]
fn get_cfg(version: &str) -> FinalConfig {
    let cfg_str = "version = \"".to_string() + version + "\"\n" + CFG_STR;
    // We don't care about the output of the config creation here, so we just parse a nameless vector in as the write buffer
    Config::new(&cfg_str)
        .unwrap()
        .to_final(version, &mut Vec::new())
        .unwrap()
}

#[test]
fn cache_works() {
    let tmp_path = "/tmp/bonnie_test_0.cache.json".to_string();
    let cfg = get_cfg(BONNIE_VERSION);
    let mut output = Vec::new();
    let res = cache(&cfg, &mut output, Some(&tmp_path));
    assert_eq!(res, Ok(()));
    let cfg_extracted = load_from_cache(&mut output, Some(&tmp_path));
    assert_eq!(cfg_extracted, Ok(cfg));
}
// That config loads `.env`, so we should be able to access `SHORTGREETING`
#[test]
fn loads_env_files() {
    let tmp_path = "/tmp/bonnie_test_1.cache.json".to_string();
    // After getting the config, we need to explicitly unset that environment variable, because it will be loaded in that process and then we aren't testing anything new
    let cfg = get_cfg(BONNIE_VERSION);
    env::remove_var("SHORTGREETING");
    let mut output = Vec::new();
    cache(&cfg, &mut output, Some(&tmp_path)).unwrap();
    load_from_cache(&mut output, Some(&tmp_path)).unwrap();
    assert_eq!(env::var("SHORTGREETING"), Ok("Hello".to_string()))
}
#[test]
fn returns_error_on_bad_version() {
    let tmp_path = "/tmp/bonnie_test_2.cache.json".to_string();
    let mut cfg = get_cfg(BONNIE_VERSION);
    let mut output = Vec::new();
    cfg.version = "0.1.0".to_string(); // No matter what, this version is incompatible (we're past it now)
    cache(&cfg, &mut output, Some(&tmp_path)).unwrap();
    let cfg_extracted = load_from_cache(&mut output, Some(&tmp_path));
    assert!(matches!(cfg_extracted, Err(_)));
}
