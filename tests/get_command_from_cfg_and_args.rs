use bonnie_lib::get_command_from_cfg_and_args;

#[test]
fn returns_correct_command() {
	let prog_args = vec!["".to_string(), "test".to_string(), "Name".to_string()];
	let conf = String::from("
		[scripts]
		test.cmd = \"echo %name\"
		test.args = [
			\"name\"
		]
	");
	let command_with_args = get_command_from_cfg_and_args(conf, prog_args);

	assert_eq!(command_with_args, Ok(String::from("echo Name")))
}
#[test]
fn returns_correct_command_with_shorthand() {
	let prog_args = vec!["".to_string(), "test".to_string()];
	let conf = String::from("
		[scripts]
		test = \"echo Name\"
	");
	let command_with_args = get_command_from_cfg_and_args(conf, prog_args);

	assert_eq!(command_with_args, Ok(String::from("echo Name")))
}
#[test]
fn returns_correct_command_with_appending() {
	let prog_args = vec!["".to_string(), "test".to_string(), "foo".to_string(), "bar".to_string()];
	let conf = String::from("
		[scripts]
		test = \"echo %%\"
	");
	let command_with_args = get_command_from_cfg_and_args(conf, prog_args);

	assert_eq!(command_with_args, Ok(String::from("echo foo bar")))
}
#[test]
fn returns_error_on_invalid_conf() {
	let prog_args = vec!["".to_string(), "test".to_string(), "Name".to_string()];
	let conf = String::from("
		[scripts]
		test.cnd = \"echo %name\" # Misspelt this line
		test.args = [
			\"name\"
		]
	");
	let command_with_args = get_command_from_cfg_and_args(conf, prog_args);
	if command_with_args.is_ok() {
		panic!("Didn't return an error on invalid config.");
	}
}
#[test]
fn returns_error_on_no_command() {
	let prog_args = vec!["".to_string()];
	let conf = String::from("
		[scripts]
		test.cmd = \"echo %name\"
		test.args = [
			\"name\"
		]
	");
	let command_with_args = get_command_from_cfg_and_args(conf, prog_args);
	if command_with_args.is_ok() {
		panic!("Didn't return an error on no command given.");
	}
}
#[test]
fn returns_error_on_invalid_command() {
	let prog_args = vec!["".to_string(), "trst".to_string(), "Name".to_string()]; // Misspelt command name here
	let conf = String::from("
		[scripts]
		test.cmd = \"echo %name\"
		test.args = [
			\"name\"
		]
	");
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
	let conf = String::from("
		[scripts]
		test.cmd = \"echo %name\"
		test.args = [
			\"name\"
		]
	");
	let command_with_args = get_command_from_cfg_and_args(conf, prog_args);
	if command_with_args.is_ok() {
		panic!("Didn't return an error on too few given arguments.");
	}
}
#[test]
fn returns_error_on_argument_not_inserted() {
	let prog_args = vec!["".to_string(), "test".to_string(), "Name".to_string()];
	let conf = String::from("
		[scripts]
		test.cmd = \"echo %name\"
		test.args = [
			\"namr\" # Argument misspelt
		]
	");
	let command_with_args = get_command_from_cfg_and_args(conf, prog_args);
	if command_with_args.is_ok() {
		panic!("Didn't return an error on argument not inserted.");
	}
}