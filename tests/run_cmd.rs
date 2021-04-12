use bonnie_lib::run_cmd;

use std::env::VarError;

// TODO test error cases here

#[test]
fn returns_empty() {
	let cmd = "echo Test";
	let output = run_cmd(String::from(cmd));
	if !output.is_ok() {
		// This is a super simple function, but it depends on a file existing, tell the user so they can create it if necessary!
		panic!("Returned error for valid command '{}'.", cmd);
	}
}
#[test]
#[ignore] // This test takes 5 seconds to run
fn returns_empty_for_long_cmd() {
	let cmd = "sleep 5 && echo Test";
	let output = run_cmd(String::from(cmd));
	if !output.is_ok() {
		// This is a super simple function, but it depends on a file existing, tell the user so they can create it if necessary!
		panic!("Returned error for valid command '{}'.", cmd);
	}
}