// Bonnie mostly follows a strategy of integration testing to mimc real usage
// This also significantly reduces the brittleness of tests
// Note that the commands specified in testing WILL ACTUALLY BE RUN, so change things here carefully!
// Commands epcified should `echo` their name so we trace them back and `exit` with some exit code
// This file handles manually-coded known cases

// All these tests are Linux-specific due to their OS-specific testing/shells (sorry!), they are marked as such for conditional compilation

use lib::{Config, BONNIE_VERSION};

// A testing utility that represents all Bonnie returns as the promise of an exit code
// This is modelled off the code in `main.rs` that actually runs Bonnie
// This takes an output, which will be a simple vector in testing
#[cfg(test)]
fn run_e2e_test(
    cfg_str: &str,
    prog_args: Vec<String>,
    version: &str,
    output: &mut impl std::io::Write,
) -> Result<i32, String> {
    let cfg = Config::new(cfg_str)?.to_final(version, output)?;
    let (command_to_run, command_name, relevant_args) = cfg.get_command_for_args(&prog_args)?;
    let bone = command_to_run.prepare(&command_name, &relevant_args, &cfg.default_shell, output)?;
    let exit_code = bone.run(&command_name, output)?;

    Ok(exit_code)
}

// A testing utility macro that allows us to expect an exit code to be returned
// This returns the output of the execution (warnings, command info, etc.) as a vector of lines
// The config string given here does not have to contain any version tag, that will be added
#[cfg(test)]
macro_rules! expect_exit_code {
    ($exit_code:literal, $raw_cfg_str:expr, $version:expr, [ $($arg:expr),+ ]) => {
        {
            // We define a vector that warnings and command information will be printed to
            let mut output = Vec::new();
            let prog_args = vec![$($arg.to_string()), +];
            let cfg_str = "version = \"".to_string() + $version + "\"\n" + $raw_cfg_str;
            let res = run_e2e_test(&cfg_str, prog_args, $version, &mut output);
            assert_eq!(res, Ok($exit_code));
            // We know this will only be filled with `u8` bytes, so we can safely call `.unwrap()`
            let output_string = String::from_utf8(output).unwrap();
            let output_lines: Vec<String> = output_string.lines().map(|x| x.to_string()).collect();
            output_lines
        };
    }
}

// A testing utility macro that allows us to expect some error to be returned
// This returns the output of the execution (warnings, command info, etc.) as a vector of lines
// The config string given here does not have to contain any version tag, that will be added
// TODO after `error_chain` migration, test for specific errors here
#[cfg(test)]
macro_rules! expect_error {
    ($raw_cfg_str:expr, $version:expr, [ $($arg:expr),+ ]) => {
        {
            // We define a vector that warnings and command information will be printed to
            let mut output = Vec::new();
            let prog_args = vec![$($arg.to_string()), +];
            let cfg_str = "version = \"".to_string() + $version + "\"\n" + $raw_cfg_str;
            let res = run_e2e_test(&cfg_str, prog_args, $version, &mut output);
            println!("{:#?}", res);
            assert!(matches!(res, Err(_)));
            // We know this will only be filled with `u8` bytes, so we can safely call `.unwrap()`
            let output_string = String::from_utf8(output).unwrap();
            let output_lines: Vec<String> = output_string.lines().map(|x| x.to_string()).collect();
            output_lines
        }
    }
}

// A utility testing macro that asserts the ordered presence of a series of elements in a vector of strings
#[cfg(test)]
macro_rules! assert_contains_ordered {
    ($vec:expr, [ $($elem:expr),+ ]) => {
        {
            // Concatenate everything so we can easily assert order
            let concat_vec = $vec.join(" | ");
            let concat_checks = vec![$($elem.to_string()), +].join(" | ");

            assert!(concat_vec.contains(&concat_checks))
        }
    }
}

// A utility testing macro that asserts the unordered presence of a series of elements in a vector of strings
#[cfg(test)]
macro_rules! assert_contains {
    ($vec:expr, [ $($elem:expr),+ ]) => {
        {
            let checks = vec![$($elem.to_string()), +];
            let mut contains = false;
            for check in checks.iter() {
                if $vec.contains(check) {
                    // We only need
                    contains = true;
                }
            }

            assert!(contains)
        }
    }
}

// This test suite tests all the major syntactic feature of Bonnie
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn succeeds_with_kv_syntax() {
    let output = expect_exit_code!(
        0,
        r#"
        [scripts]
        basic = "exit 0"
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
    println!("{:#?}", output);
    assert_contains!(output, ["sh, [\"-c\", \"exit 0\"]"]);
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux (uses the `USER` environment variable, the feature itself should be fine)
fn succeeds_with_env_var_interpolation() {
    let output = expect_exit_code!(
        0,
        r#"
        [scripts]
        basic.cmd = "echo %USER && exit 0"
        basic.env_vars = ["USER"]
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
    assert_contains_ordered!(output, ["sh, [\"-c\", \"echo ".to_string() + &std::env::var("USER").unwrap() + " && exit 0\"]"]);
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn succeeds_with_arg_interpolation() {
    let output = expect_exit_code!(
        0,
        r#"
        [scripts]
        basic.cmd = "echo %name && exit 0"
        basic.args = ["name"]
        "#,
        BONNIE_VERSION,
        ["basic", "Name"]
    );
    assert_contains_ordered!(output, ["sh, [\"-c\", \"echo Name && exit 0\"]"]);
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn returns_error_on_too_few_args() {
    expect_error!(
        r#"
        [scripts]
        basic.cmd = "echo %name && exit 0"
        basic.args = ["name"]
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn succeeds_with_mass_arg_interpolation_and_no_args() {
    let output = expect_exit_code!(
        0,
        r#"
        [scripts]
        basic = "echo %% && exit 0"
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
    println!("{:?}", output);
    assert_contains_ordered!(output, ["sh, [\"-c\", \"echo  && exit 0\"]"]); // Note the extra space from concatenation
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn succeeds_with_mass_arg_interpolation_and_one_arg() {
    let output = expect_exit_code!(
        0,
        r#"
        [scripts]
        basic = "echo %% && exit 0"
        "#,
        BONNIE_VERSION,
        ["basic", "Test"]
    );
    assert_contains_ordered!(output, ["sh, [\"-c\", \"echo Test && exit 0\"]"]);
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn succeeds_with_mass_arg_interpolation_and_many_args() {
    let output = expect_exit_code!(
        0,
        r#"
        [scripts]
        basic = "echo %% && exit 0"
        "#,
        BONNIE_VERSION,
        ["basic", "foo", "bar"]
    );
    assert_contains_ordered!(output, ["sh, [\"-c\", \"echo foo bar && exit 0\"]"]);
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn succeeds_with_mass_arg_interpolation_and_escaping() {
    let output = expect_exit_code!(
        0,
        r#"
        [scripts]
        basic = "echo %% \\%% && exit 0"
        "#,
        BONNIE_VERSION,
        ["basic", "foo", "bar"]
    );
    assert_contains_ordered!(output, ["sh, [\"-c\", \"echo foo bar %% && exit 0\"]"]);
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn succeeds_with_mass_arg_interpolation_and_specific_arg_interpolation() {
    let output = expect_exit_code!(
        0,
        r#"
        [scripts]
        basic.cmd = "echo %name %% && exit 0"
        basic.args = ["name"]
        "#,
        BONNIE_VERSION,
        ["basic", "Name", "foo", "bar"]
    );
    assert_contains_ordered!(output, ["sh, [\"-c\", \"echo Name foo bar && exit 0\"]"]);
}
// This test is dependent on the contents of `.env`
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn loads_env_files() {
    let output = expect_exit_code!(
        0,
        r#"
        env_files = ["src/.env"]
        [scripts]
        basic.cmd = "echo %SHORTGREETING && exit 0"
        basic.env_vars = ["SHORTGREETING"]
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
    assert_contains_ordered!(output, ["sh, [\"-c\", \"echo Hello && exit 0\"]"]);
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn returns_error_on_nonexistent_env_file() {
    expect_error!(
        r#"
        env_files = ["src/.ennv"] # Misspelt this line
        [scripts]
        basic.cmd = "echo %SHORTGREETING && exit 0"
        basic.env_vars = ["SHORTGREETING"]
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn returns_error_on_invalid_env_file() {
    expect_error!(
        r#"
        env_files = ["src/.env.invalid"] # This file contains an uninclosed ' ', and is thus invalid
        [scripts]
        basic.cmd = "echo %INVALID_VAR && exit 0"
        basic.env_vars = ["INVALID_VAR"]
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn succeeds_with_full_interpolation() {
    let output = expect_exit_code!(
        0,
        r#"
        env_files = ["src/.env"]
        [scripts]
        basic.cmd = "echo \"%SHORTGREETING %name %%\" && exit 0"
        basic.args = ["name"]
        basic.env_vars = ["SHORTGREETING"]
        "#,
        BONNIE_VERSION,
        ["basic", "Name", "(extra stuff)"]
    );
    assert_contains_ordered!(output, ["sh, [\"-c\", \"echo \\\"Hello Name (extra stuff)\\\" && exit 0\"]"]);
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn succeeds_with_multistage() {
    let output = expect_exit_code!(
        1,
        r#"
        [scripts]
        basic = ["exit 0", "exit 1"]
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
    assert_contains_ordered!(output, ["sh, [\"-c\", \"exit 0\"]", "sh, [\"-c\", \"exit 1\"]"]);
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn succeeds_with_multistage_with_interpolation() {
    let output = expect_exit_code!(
        1,
        r#"
        env_files = ["src/.env"]
        [scripts]
        basic.cmd = [
            "echo %SHORTGREETING %% && exit 0",
            "echo %name && exit 1"
        ]
        basic.args = ["name"]
        basic.env_vars = ["SHORTGREETING"]
        "#,
        BONNIE_VERSION,
        ["basic", "Name", "foo", "bar"]
    );
    assert_contains_ordered!(output, ["sh, [\"-c\", \"echo Hello foo bar && exit 0\"]", "sh, [\"-c\", \"echo Name && exit 1\"]"]);
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn succeeds_with_kv_unordered_subcommands() {
    let cfg = r#"
    [scripts]
    basic.subcommands.test = "exit 0"
    basic.subcommands.other = "exit 1"
    "#;
    let output1 = expect_exit_code!(
        0,
        cfg,
        BONNIE_VERSION,
        ["basic", "test"]
    );
    assert_contains_ordered!(output1, ["sh, [\"-c\", \"exit 0\"]"]);
    let output2 = expect_exit_code!(
        1,
        cfg,
        BONNIE_VERSION,
        ["basic", "other"]
    );
    assert_contains_ordered!(output2, ["sh, [\"-c\", \"exit 1\"]"]);
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn succeeds_with_multistage_and_interpolation_unordered_subcommands() {
    let cfg = r#"
    env_files = ["src/.env"]
    [scripts]
    basic.subcommands.test.cmd = [
        "echo %SHORTGREETING %% && exit 0",
        "echo %name && exit 1"
    ]
    basic.subcommands.test.args = ["name"]
    basic.subcommands.test.env_vars = ["SHORTGREETING"]
    basic.subcommands.other = "exit 1"
    "#;
    let output1 = expect_exit_code!(
        1,
        cfg,
        BONNIE_VERSION,
        ["basic", "test", "Name", "foo bar"]
    );
    assert_contains_ordered!(output1, ["sh, [\"-c\", \"echo Hello foo bar && exit 0\"]", "sh, [\"-c\", \"echo Name && exit 1\"]"]);
    let output2 = expect_exit_code!(
        1,
        cfg,
        BONNIE_VERSION,
        ["basic", "other"]
    );
    assert_contains_ordered!(output2, ["sh, [\"-c\", \"exit 1\"]"]);
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn succeeds_with_root_cmd_for_unordered_subcommands() {
    let cfg = r#"
    [scripts]
    basic.cmd = "exit 0"
    basic.subcommands.test = "exit 1"
    basic.subcommands.other = "exit 2"
    "#;
    let root_output = expect_exit_code!(
        0,
        cfg,
        BONNIE_VERSION,
        ["basic"]
    );
    assert_contains_ordered!(root_output, ["sh, [\"-c\", \"exit 0\"]"]);
    let output1 = expect_exit_code!(
        1,
        cfg,
        BONNIE_VERSION,
        ["basic", "test"]
    );
    assert_contains_ordered!(output1, ["sh, [\"-c\", \"exit 1\"]"]);
    let output2 = expect_exit_code!(
        2,
        cfg,
        BONNIE_VERSION,
        ["basic", "other"]
    );
    assert_contains_ordered!(output2, ["sh, [\"-c\", \"exit 2\"]"]);
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn returns_error_on_missing_cmd() {
    expect_error!(
        r#"
        [scripts]
        basic.args = ["name"]
        "#,
        BONNIE_VERSION,
        ["basic", "Name"]
    );
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn succeeds_with_os_specific_kv_cmd() {
    let output = expect_exit_code!(
        0,
        r#"
        [scripts]
        basic.cmd.generic = "exit 1"
        basic.cmd.targets.linux = "exit 0"
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
    assert_contains_ordered!(output, ["sh, [\"-c\", \"exit 0\"]"]);
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn succeeds_with_os_specific_multistage_and_interpolation_cmd() {
    let output = expect_exit_code!(
        1,
        r#"
        env_files = ["src/.env"]
        [scripts]
        basic.cmd.generic = "exit 2"
        basic.cmd.targets.linux = [
            "echo %SHORTGREETING %% && exit 0",
            "echo %name && exit 1"
        ]
        basic.args = ["name"]
        basic.env_vars = ["SHORTGREETING"]
        "#,
        BONNIE_VERSION,
        ["basic", "Name", "foo", "bar"]
    );
    println!("{:?}", output);
    assert_contains_ordered!(output, ["sh, [\"-c\", \"echo Hello foo bar && exit 0\"]", "sh, [\"-c\", \"echo Name && exit 1\"]"]);
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn succeeds_with_custom_shell() {
    let output = expect_exit_code!(
        0,
        r#"
        [scripts]
        basic.cmd.exec = "exit 0"
        basic.cmd.shell = ["bash", "-c", "{COMMAND}"]
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
    assert_contains_ordered!(output, ["bash, [\"-c\", \"exit 0\"]"]);
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn succeeds_with_custom_shell_and_os_specificity_and_multistage_and_interpolation() {
    let output = expect_exit_code!(
        1,
        r#"
        env_files = ["src/.env"]
        [scripts]
        basic.cmd.generic = "exit 2"
        basic.cmd.targets.linux.exec = [
            "echo %SHORTGREETING %% && exit 0",
            "echo %name && exit 1"
        ]
        basic.cmd.targets.linux.shell = ["bash", "-c", "{COMMAND}"]
        basic.args = ["name"]
        basic.env_vars = ["SHORTGREETING"]
        "#,
        BONNIE_VERSION,
        ["basic", "Name", "foo", "bar"]
    );
    println!("{:?}", output);
    assert_contains_ordered!(output, ["bash, [\"-c\", \"echo Hello foo bar && exit 0\"]", "bash, [\"-c\", \"echo Name && exit 1\"]"]);
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn returns_error_if_generic_os_specifier_not_given() {
    expect_error!(
        r#"
        [scripts]
        basic.cmd.targets.linux = "exit 0"
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn uses_simple_default_shell() {
    let output = expect_exit_code!(
        0,
        r#"
        default_shell = ["bash", "-c", "{COMMAND}"]
        [scripts]
        basic = "exit 0"
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
    assert_contains_ordered!(output, ["bash, [\"-c\", \"exit 0\"]"]);
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn uses_generic_default_shell() {
    let output = expect_exit_code!(
        0,
        r#"
        default_shell.generic = ["bash", "-c", "{COMMAND}"]
        [scripts]
        basic = "exit 0"
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
    assert_contains_ordered!(output, ["bash, [\"-c\", \"exit 0\"]"]);
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn uses_os_specific_default_shell() {
    let output = expect_exit_code!(
        0,
        r#"
        default_shell.generic = ["sh", "-c", "{COMMAND}"]
        default_shell.targets.linux = ["bash", "-c", "{COMMAND}"]
        [scripts]
        basic = "exit 0"
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
    assert_contains_ordered!(output, ["bash, [\"-c\", \"exit 0\"]"]);
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn succeeds_with_kv_simple_ordered_subcommands() {
    let output = expect_exit_code!(
        0,
        r#"
        [scripts]
        basic.subcommands.test = "exit 0"
        basic.subcommands.other = "exit 1"
        basic.order = "test"
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
    assert_contains_ordered!(output, ["sh, [\"-c\", \"exit 0\"]"]);
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn succeeds_with_kv_complex_ordered_subcommands() {
    let output = expect_exit_code!(
        1,
        r#"
        [scripts]
        basic.subcommands.test = "exit 0"
        basic.subcommands.other = "exit 1"
        basic.order = """
        test {
            Any => other
        }
        """
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
    assert_contains_ordered!(output, ["sh, [\"-c\", \"exit 0\"]", "sh, [\"-c\", \"exit 1\"]"]);
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn returns_error_on_non_global_args_for_ordered_subcommands() {
    expect_error!(
        r#"
        [scripts]
        basic.subcommands.test = "echo %name && exit 0"
        basic.subcommands.test.args = ["name"] # This has to be `basic.args` instead
        basic.subcommands.other = "exit 1"
        basic.order = """
        test {
            Any => other
        }
        """
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn returns_error_on_unordered_nesting_in_order() {
    expect_error!(
        r#"
        [scripts]
        basic.subcommands.test = "echo %name && exit 0"
        basic.subcommands.test.args = ["name"] # This has to be `basic.args` instead
        basic.subcommands.other = "exit 1"
        basic.subcommands.nested.subcommands.test = "exit 0"
        basic.subcommands.nested.subcommands.other = "exit 1"
        basic.order = """
        test {
            Any => other
        }
        """
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn returns_error_on_cmd_and_ordered_subcommands() {
    expect_error!(
        r#"
        [scripts]
        basic.cmd = "exit 0"
        basic.subcommands.test = "exit 0"
        basic.subcommands.other = "exit 1"
        basic.order = """
        test {
            Any => other
        }
        """
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
}
// This test should basically represent the most complex use-case of Bonnie in terms of syntax
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn succeeds_with_everything() {
    let output = expect_exit_code!(
        1,
        r#"
        env_files = ["src/.env"]
        default_env.generic = ["sh", "-c", "{COMMAND}"]
        default_env.targets.linux = ["bash", "-c", "{COMMAND}"]
        [scripts]
        basic.subcommands.test.cmd.generic = "exit 5"
        basic.subcommands.test.cmd.targets.linux.exec = [
            "echo %SHORTGREETING %% && exit 0",
            "echo %name && exit 1"
        ]
        basic.subcommands.test.env_vars = ["SHORTGREETING"]
        basic.subcommands.test.cmd.targets.linux.shell = ["sh", "-c", "{COMMAND}"]
        basic.subcommands.nested.subcommands.test = "exit 2"
        basic.subcommands.nested.subcommands.other = "exit 3"
        basic.subcommands.nested.order = """
        test {
            Any => other
        }
        """
        basic.args = ["name"]
        basic.order = """
        test {
            Any => nested {
                Any => test
            }
        }
        """
        "#,
        BONNIE_VERSION,
        ["basic", "Name", "foo", "bar"]
    );
    println!("{:?}", output);
    assert_contains_ordered!(
        output,
        [
            "sh, [\"-c\", \"echo Hello foo bar && exit 0\"]",
            "sh, [\"-c\", \"echo Name && exit 1\"]",
            "sh, [\"-c\", \"exit 2\"]",
            "sh, [\"-c\", \"exit 3\"]",
            "sh, [\"-c\", \"echo Hello foo bar && exit 0\"]",
            "sh, [\"-c\", \"echo Name && exit 1\"]"
        ]
    );
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn succeeds_with_success_failure_order_control() {
    let output1 = expect_exit_code!(
        1,
        r#"
        [scripts]
        basic.subcommands.test = "exit 0"
        basic.subcommands.other = "exit 1"
        basic.order = """
        test {
            Success => other
        }
        """
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
    assert_contains_ordered!(output1, ["sh, [\"-c\", \"exit 0\"]", "sh, [\"-c\", \"exit 1\"]"]);
    let output2 = expect_exit_code!(
        0,
        r#"
        [scripts]
        basic.subcommands.test = "exit 1"
        basic.subcommands.other = "exit 0"
        basic.order = """
        test {
            Failure => other
        }
        """
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
    assert_contains_ordered!(output2, ["sh, [\"-c\", \"exit 1\"]", "sh, [\"-c\", \"exit 0\"]"]);
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn succeeds_with_exit_code_order_control() {
    let output1 = expect_exit_code!(
        1,
        r#"
        [scripts]
        basic.subcommands.test = "exit 0"
        basic.subcommands.other = "exit 1"
        basic.order = """
        test {
            0 => other
        }
        """
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
    assert_contains_ordered!(output1, ["sh, [\"-c\", \"exit 0\"]", "sh, [\"-c\", \"exit 1\"]"]);
    let output2 = expect_exit_code!(
        0,
        r#"
        [scripts]
        basic.subcommands.test = "exit 1"
        basic.subcommands.other = "exit 0"
        basic.order = """
        test {
            1 => other
        }
        """
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
    assert_contains_ordered!(output2, ["sh, [\"-c\", \"exit 1\"]", "sh, [\"-c\", \"exit 0\"]"]);
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn succeeds_with_not_exit_code_order_control() {
    let output1 = expect_exit_code!(
        1,
        r#"
        [scripts]
        basic.subcommands.test = "exit 0"
        basic.subcommands.other = "exit 1"
        basic.order = """
        test {
            !1 => other
        }
        """
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
    assert_contains_ordered!(output1, ["sh, [\"-c\", \"exit 0\"]", "sh, [\"-c\", \"exit 1\"]"]);
    let output2 = expect_exit_code!(
        0,
        r#"
        [scripts]
        basic.subcommands.test = "exit 1"
        basic.subcommands.other = "exit 0"
        basic.order = """
        test {
            !0 => other
        }
        """
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
    assert_contains_ordered!(output2, ["sh, [\"-c\", \"exit 1\"]", "sh, [\"-c\", \"exit 0\"]"]);
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn succeeds_with_any_none_order_control() {
    let output1 = expect_exit_code!(
        1,
        r#"
        [scripts]
        basic.subcommands.test = "exit 0"
        basic.subcommands.other = "exit 1"
        basic.order = """
        test {
            Any => other
        }
        """
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
    assert_contains_ordered!(output1, ["sh, [\"-c\", \"exit 0\"]", "sh, [\"-c\", \"exit 1\"]"]);
    let output2 = expect_exit_code!(
        1,
        r#"
        [scripts]
        basic.subcommands.test = "exit 1"
        basic.subcommands.other = "exit 0"
        basic.order = """
        test {
            None => other
        }
        """
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
    assert_contains_ordered!(output2, ["sh, [\"-c\", \"exit 1\"]"]);
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn succeeds_with_union_order_control() {
    let output = expect_exit_code!(
        1,
        r#"
        [scripts]
        basic.subcommands.test = "exit 0"
        basic.subcommands.other = "exit 1"
        basic.order = """
        test {
            0|Success|2 => other
        }
        """
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
    assert_contains_ordered!(output, ["sh, [\"-c\", \"exit 0\"]", "sh, [\"-c\", \"exit 1\"]"]);
}
#[test]
#[cfg(target_os = "linux")] // This test will only work on Linux
fn succeeds_with_intersection_order_control() {
    let output = expect_exit_code!(
        1,
        r#"
        [scripts]
        basic.subcommands.test = "exit 0"
        basic.subcommands.other = "exit 1"
        basic.order = """
        test {
            0+Success => other
        }
        """
        "#,
        BONNIE_VERSION,
        ["basic"]
    );
    assert_contains_ordered!(output, ["sh, [\"-c\", \"exit 0\"]", "sh, [\"-c\", \"exit 1\"]"]);
}
