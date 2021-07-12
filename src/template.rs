use home::home_dir;

use std::env;
use std::fs;
use std::path::PathBuf;

pub fn get_template_path() -> Result<PathBuf, String> {
    let default_template_path = home_dir()
        .map(|x| x.join(".bonnie").join("template.toml"))
        .ok_or(format!(
            "I could not find your home directory. {}",
            if cfg!(target_os = "windows") {
                "That is most odd."
            } else {
                "Is the `HOME` environment variable set?"
            }
        ))?;

    Ok(env::var("BONNIE_TEMPLATE")
        .map(|x| PathBuf::from(x))
        .unwrap_or(default_template_path))
}

pub fn get_default() -> Result<String, String> {
    let path = get_template_path()?;

    let template = fs::read_to_string(path);

    template.map_err(|err| err.to_string())
}
