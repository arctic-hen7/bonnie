// When loading environment variable files in these tests, we assume we're running from the root and so add `src/`

use bonnie_lib::get_command_from_cfg_and_args;

#[test]
fn returns_correct_command() {
    let prog_args = vec!["".to_string(), "test".to_string(), "Name".to_string()];
    let conf = String::from(
        "
		[scripts]
		test.cmd = \"echo %name\"
		test.args = [
			\"name\"
		]
	",
    );
    let command_with_args = get_command_from_cfg_and_args(conf, prog_args);

    assert_eq!(command_with_args, Ok(String::from("echo Name")))
}
#[test]
fn returns_correct_command_with_env_var_file() {
    let prog_args = vec!["".to_string(), "test".to_string()];
    let conf = String::from(
        // Note that a space is needed after `env_files`
        // TODO document the above
        "
        env_files = [
            \"src/.env\"
        ]

		[scripts]
		test.cmd = \"echo %GREETING\"
		test.env_vars = [
            \"GREETING\"
        ]
	",
    );
    let command_with_args = get_command_from_cfg_and_args(conf, prog_args);

    assert_eq!(command_with_args, Ok(String::from("echo Hello, my dear friend")))
}
#[test]
fn returns_correct_command_with_args_and_env_vars() {
    let prog_args = vec!["".to_string(), "test".to_string(), "Name".to_string()];
    let conf = String::from(
        "
        env_files = [
            \"src/.env\"
        ]

		[scripts]
		test.cmd = \"echo %GREETING %name!\"
        test.args = [
            \"name\"
        ]
		test.env_vars = [
            \"GREETING\"
        ]
	",
    );
    let command_with_args = get_command_from_cfg_and_args(conf, prog_args);

    assert_eq!(command_with_args, Ok(String::from("echo Hello, my dear friend Name!")))
}
#[test]
fn returns_correct_command_with_shorthand() {
    let prog_args = vec!["".to_string(), "test".to_string()];
    let conf = String::from(
        "
		[scripts]
		test = \"echo Name\"
	",
    );
    let command_with_args = get_command_from_cfg_and_args(conf, prog_args);

    assert_eq!(command_with_args, Ok(String::from("echo Name")))
}
#[test]
fn returns_correct_command_with_appending() {
    let prog_args = vec![
        "".to_string(),
        "test".to_string(),
        "foo".to_string(),
        "bar".to_string(),
    ];
    let conf = String::from(
        "
		[scripts]
		test = \"echo %%\"
	",
    );
    let command_with_args = get_command_from_cfg_and_args(conf, prog_args);

    assert_eq!(command_with_args, Ok(String::from("echo foo bar")))
}
#[test]
fn returns_error_on_invalid_conf() {
    let prog_args = vec!["".to_string(), "test".to_string(), "Name".to_string()];
    let conf = String::from(
        "
		[scripts]
		test.cnd = \"echo %name\" # Misspelt this line
		test.args = [
			\"name\"
		]
	",
    );
    let command_with_args = get_command_from_cfg_and_args(conf, prog_args);
    if command_with_args.is_ok() {
        panic!("Didn't return an error on invalid config.");
    }
}
#[test]
fn returns_error_on_no_command() {
    let prog_args = vec!["".to_string()];
    let conf = String::from(
        "
		[scripts]
		test.cmd = \"echo %name\"
		test.args = [
			\"name\"
		]
	",
    );
    let command_with_args = get_command_from_cfg_and_args(conf, prog_args);
    if command_with_args.is_ok() {
        panic!("Didn't return an error on no command given.");
    }
}
#[test]
fn returns_error_on_invalid_command() {
    let prog_args = vec!["".to_string(), "trst".to_string(), "Name".to_string()]; // Misspelt command name here
    let conf = String::from(
        "
		[scripts]
		test.cmd = \"echo %name\"
		test.args = [
			\"name\"
		]
	",
    );
    let command_with_args = get_command_from_cfg_and_args(conf, prog_args);
    if command_with_args.is_ok() {
        panic!("Didn't return an error on invalid command.");
    }
}
#[test]
fn returns_error_on_command_with_appending_and_args() {
    let prog_args = vec!["".to_string(), "test".to_string(), "Name".to_string()];
    let conf = String::from("
		[scripts]
		test.cmd = \"echo %name %%\" # Appending all arguments as well as having custom arguments is not allowed
		test.args = [
			\"name\"
		]
	");
    let command_with_args = get_command_from_cfg_and_args(conf, prog_args);
    if command_with_args.is_ok() {
        panic!("Didn't return an error on command that appends arguments and has custom ones.");
    }
}
#[test]
fn returns_error_on_too_few_args() {
    let prog_args = vec!["".to_string(), "test".to_string()]; // Didn't give a name
    let conf = String::from(
        "
		[scripts]
		test.cmd = \"echo %name\"
		test.args = [
			\"name\"
		]
	",
    );
    let command_with_args = get_command_from_cfg_and_args(conf, prog_args);
    if command_with_args.is_ok() {
        panic!("Didn't return an error on too few given arguments.");
    }
}
#[test]
fn returns_error_on_argument_not_inserted() {
    let prog_args = vec!["".to_string(), "test".to_string(), "Name".to_string()];
    let conf = String::from(
        "
		[scripts]
		test.cmd = \"echo %name\"
		test.args = [
			\"namr\" # Argument misspelt
		]
	",
    );
    let command_with_args = get_command_from_cfg_and_args(conf, prog_args);
    if command_with_args.is_ok() {
        panic!("Didn't return an error on argument not inserted.");
    }
}
#[test]
fn returns_error_on_env_var_not_inserted() {
    let prog_args = vec!["".to_string(), "test".to_string()];
    let conf = String::from(
        "
		env_files = [
            \"src/.env\"
        ]

		[scripts]
		test.cmd = \"echo %GREEETING\" # Misspelt this line
		test.env_vars = [
            \"GREETING\"
        ]
	",
    );
    let command_with_args = get_command_from_cfg_and_args(conf, prog_args);
    if command_with_args.is_ok() {
        panic!("Didn't return an error on environment variable not inserted.");
    }
}
#[test]
fn returns_error_on_attempted_insertion_of_unrequested_env_var() {
    let prog_args = vec!["".to_string(), "test".to_string()];
    let conf = String::from(
        "
		env_files = [
            \"src/.env\"
        ]

		[scripts]
		test.cmd = \"echo %SHORTGREETING\" # Tried to interpolate unrequested variable that does exist
		test.env_vars = [
            \"GREETING\"
        ]
	",
    );
    let command_with_args = get_command_from_cfg_and_args(conf, prog_args);
    if command_with_args.is_ok() {
        panic!("Didn't return an error on attempted insertion of unrequested environment variable.");
    }
}
#[test]
fn returns_error_on_env_file_not_found() {
    let prog_args = vec!["".to_string(), "test".to_string()];
    let conf = String::from(
        "
        env_files = [
            \"src/.envv\" # Misspelt this line
        ]

		[scripts]
		test.cmd = \"echo %GREETING\"
		test.env_vars = [
            \"GREETING\"
        ]
	",
    );
    let command_with_args = get_command_from_cfg_and_args(conf, prog_args);
    if command_with_args.is_ok() {
        panic!("Didn't return an error on environment variable file not found.");
    }
}
#[test]
fn returns_error_on_invalid_env_file() {
    let prog_args = vec!["".to_string(), "test".to_string()];
    let conf = String::from(
        "
        env_files = [
            \"src/.invalidenv\" # This file exists, but contains invalid characters
        ]

		[scripts]
		test.cmd = \"echo %GREETING\"
		test.env_vars = [
            \"GREETING\"
        ]
	",
    );
    let command_with_args = get_command_from_cfg_and_args(conf, prog_args);
    if command_with_args.is_ok() {
        panic!("Didn't return an error on invalid environment variable file.");
    }
}
#[test]
fn returns_error_on_env_var_not_found() {
    let prog_args = vec!["".to_string(), "test".to_string()];
    let conf = String::from(
        "
        env_files = [
            \"src/.env\"
        ]

		[scripts]
		test.cmd = \"echo %GREETING\"
		test.env_vars = [
            \"GREEETING\" # Misspelt this line
        ]
	",
    );
    let command_with_args = get_command_from_cfg_and_args(conf, prog_args);
    if command_with_args.is_ok() {
        panic!("Didn't return an error on environment variable not found (did you define $GREEETING at some point?).");
    }
}
