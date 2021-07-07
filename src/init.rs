use std::fs;
use crate::version::BONNIE_VERSION;

// Creates a new Bonnie configuration file using a template, or from the default
pub fn init(template: Option<String>) -> Result<(), String> {
    // Check if there's already a config file in this directory
    if fs::metadata("./bonnie.toml").is_ok() {
        Err(String::from("A Bonnie configuration file already exists in this directory. If you want to create a new one, please delete the old one first."))
    } else {
        // Check if a template has been given
        let output;
        if matches!(template, Some(_)) && fs::metadata(template.as_ref().unwrap()).is_ok() {
            let template_path = template.unwrap();
            // We have a valid template file
            let contents = fs::read_to_string(&template_path);
            let contents = match contents {
                Ok(contents) => contents,
                Err(_) => return Err(format!("An error occurred while attempting to read the given template file '{}'. Please make sure the file exists and you have the permissions necessary to read from it.", &template_path))
            };
            output = fs::write("./bonnie.toml", contents);
        } else if matches!(template, Some(_)) && fs::metadata(template.as_ref().unwrap()).is_err() {
            // We have a template file that doesn't exist
            return Err(format!("The given template file at '{}' does not exist or can't be read. Please make sure the file exists and you have the permissions necessary to read from it.", template.as_ref().unwrap()))
        } else {
            // Create a new `bonnie.toml` file using the default
            // TODO read the default from `~/.bonnie/template.toml` if it exists
            output = fs::write(
                "./bonnie.toml",
                format!(
                "version=\"{version}\"

[scripts]
start = \"echo \\\"No start script yet!\\\"\"
                ",
                    version=BONNIE_VERSION
                )
            );
        }

        match output {
    		Ok(_) => Ok(()),
    		Err(_) => Err(String::from("Error creating new bonnie.toml, make sure you have the permissions to write to this directory."))
    	}
    }
}