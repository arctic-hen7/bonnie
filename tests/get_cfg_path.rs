use bonnie_lib::{get_cfg_path, DEFAULT_BONNIE_CFG_PATH};

use std::env::VarError;

#[test]
fn returns_default_path_if_env_not_present() {
    let cfg_path = get_cfg_path(Err(VarError::NotPresent));
    assert_eq!(cfg_path, DEFAULT_BONNIE_CFG_PATH);
}
#[test]
fn returns_default_path_if_env_not_valid() {
    use std::ffi::{OsString};
    let os_string = OsString::from("foo");

    let cfg_path = get_cfg_path(Err(VarError::NotUnicode(os_string)));
    assert_eq!(cfg_path, DEFAULT_BONNIE_CFG_PATH);
}
#[test]
fn returns_given_path() {
    let test_path = String::from("test");
    let cfg_path = get_cfg_path(Ok(String::from(&test_path)));
    assert_eq!(cfg_path, test_path);
}