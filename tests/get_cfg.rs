use bonnie_lib::get_cfg;

#[test]
fn returns_cfg_string() {
	let cfg_path = "src/bonnie.toml";
	let cfg = get_cfg(cfg_path);
	if cfg.is_err() {
		// This is a super simple function, but it depends on a file existing, tell the user so they can create it if necessary!
		panic!("Returned error on valid config file path. Please ensure 'src/bonnie.toml' still exists.");
	}
}
#[test]
fn returns_error_on_invalid_path() {
	let cfg_path = "nonexistent";
	let cfg = get_cfg(cfg_path);
	if cfg.is_ok() {
		// This is a super simple function, but it depends on a file existing, tell the user so they can create it if necessary!
		panic!("Didn't return an error on invalid file path. Please ensure the file 'nonexistent' doesn't exist.");
	}
}