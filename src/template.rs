use home::home_dir;

use crate::BONNIE_VERSION;
use std::env;
use std::fs;
use std::path::PathBuf;

// Gets the pre-programmed default template
fn get_inbuilt_default_template() -> String {
    format!(
        "version=\"{version}\"

[scripts]
start = \"echo \\\"No start script yet!\\\"\"",
        version = BONNIE_VERSION
    )
}

// Gets the path of a default template
// This will return `Ok(None)` if no default template is available
fn get_template_path() -> Option<PathBuf> {
    match env::var("BONNIE_TEMPLATE") {
        // We'll use the given template file path if provided
        Ok(path) => Some(PathBuf::from(path)),
        // If that variable isn't set properly or at all, we'll try the user's global template file
        // This will return `Ok(None)` if the home template couldn't be found
        Err(_) => {
            // This will return `None` if the user's home directory isn't found, we make it also do so if the global template isn't found
            home_dir()
                .map(|path| path.join(".bonnie").join("template.toml"))
                .map(|path| if path.exists() { Some(path) } else { None })
                .flatten()
        }
    }
}

// Gets the default template, from `BONNIE_TEMPLATE`, the global file, or the pre-programmed default
pub fn get_default_template() -> Result<String, String> {
    let path = get_template_path();
    if let Some(path) = path {
        let template = fs::read_to_string(path);
        match template {
            Ok(template) => Ok(template),
            Err(err) => Err(format!("Failed to get default template file. Please make sure any path in 'BONNIE_TEMPLATE' definitely exists. Error was '{}'.", err))
        }
    } else {
        // If no template files exist, use the pre-programmed default instead
        Ok(get_inbuilt_default_template())
    }
}
