// This file defines the default shells, which will be used if the user doesn't specify a default shell

use crate::schema;
use std::collections::HashMap;

// Gets the default shells
pub fn get_default_shells() -> schema::DefaultShell {
    let mut targets = HashMap::new();
    targets.insert(
        "windows".to_string(),
        schema::Shell {
            parts: vec![
                "powershell".to_string(),
                "-command".to_string(),
                "{COMMAND}".to_string(),
            ],
            delimiter: " ; ".to_string(),
        },
    );
    targets.insert(
        "macos".to_string(),
        schema::Shell {
            parts: vec!["sh".to_string(), "-c".to_string(), "{COMMAND}".to_string()],
            delimiter: " && ".to_string(),
        },
    );
    targets.insert(
        "linux".to_string(),
        schema::Shell {
            parts: vec!["sh".to_string(), "-c".to_string(), "{COMMAND}".to_string()],
            delimiter: " && ".to_string(),
        },
    );

    schema::DefaultShell {
        // If we have no idea where we're running, Linux Master Race
        generic: schema::Shell {
            parts: vec!["sh".to_string(), "-c".to_string(), "{COMMAND}".to_string()],
            delimiter: " && ".to_string(),
        },
        targets,
    }
}
