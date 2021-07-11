use home::home_dir;

use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    CouldNotGetHomeDirectory,
    Other(io::Error),
}

pub fn get_template_path() -> Result<PathBuf, Error> {
    let default_template_path = home_dir()
        .map(|x| x.join(".bonnie").join("template.toml"))
        .ok_or(Error::CouldNotGetHomeDirectory)?;

    Ok(env::var("BONNIE_TEMPLATE_PATH")
        .map(|x| PathBuf::from(x))
        .unwrap_or(default_template_path))
}

pub fn get_default() -> Result<String, Error> {
    let path = get_template_path()?;

    let template = fs::read_to_string(path);

    return template.map_err(|err| Error::Other(err));
}
