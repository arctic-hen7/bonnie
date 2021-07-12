use home::home_dir;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command as OsCommand;

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

pub fn edit() -> Result<(), String> {
    let template_path: String = match get_template_path() {
        Ok(path) => Ok(path.to_str().unwrap().to_string()),
        Err(err) => Err(format!(
            "Failed to get template path with the following error: {:#?}",
            err
        )),
    }?;

    let child;

    if cfg!(target_os = "windows") {
        // We need to spawn a `powershell` process to make `start` available.
        child = OsCommand::new("powershell")
            .arg(format!("start '{}'", template_path))
            .spawn()
            .map(|mut x| x.wait());
    } else {
        let editor = PathBuf::from(env::var("EDITOR").unwrap_or("nano".to_string()));

        let safe_editor = editor.to_str().ok_or(
            "The value given in the 'EDITOR' environment variable couldn't be parsed as a valid path.",
        )?;

        child = OsCommand::new(safe_editor)
            .arg(template_path)
            .spawn()
            .map(|mut x| x.wait());
    }

    let result = match child {
        Ok(_) => Ok(()),
        Err(err) => Err(format!(
            "Your editor failed to start with the following error: {:#?}",
            err
        )),
    };

    return result;
}
