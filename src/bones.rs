// Bones is Bonnie's command execution runtime, which mainly handles ordered subcommands

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command as OsCommand;

// This enables recursion of ordered subcommands (which would be the most complex use-case of Bonnie thus far)
// This really represents (from Bonnie's perspective) a future for an exit code
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Bone {
    Simple(Vec<BonesCore>),
    Complex(BonesCommand),
}
impl Bone {
    // Executes this command, returning its exit code
    // This takes an optional buffer to write data about the command being executed in testing
    pub fn run(&self, name: &str, output: &mut impl std::io::Write) -> Result<i32, String> {
        match self {
            Bone::Simple(cores) => {
                // Execute each simple command core
                // There is no logic to move between these, they're executed sequentially
                // If one fails, we terminate with its exit code
                let mut exit_code = 0;
                for core in cores {
                    let core_exit_code = core.execute(name, output)?;
                    match core_exit_code {
                        // If it succeeded, we continue onto the next command
                        0 => continue,
                        // If it failed, set the exit code to terminate with and break (we don't run the remaining commands)
                        _ => {
                            exit_code = core_exit_code;
                            break;
                        }
                    }
                }

                // Return the exit code of the command sequence
                Ok(exit_code)
            }
            Bone::Complex(command) => {
                // If it's complex and thus recursive, we depend on the Bones language parser
                command.run(output)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BonesCommand {
    // A HashMap of command names to vectors of raw commands to be executed
    // The commands to run are expected to have interpolation and target/shell resolution already done
    cmds: HashMap<String, Bone>,
    // The directive from of how to run the commands (written as per Bones' specification)
    directive: BonesDirective,
}
impl BonesCommand {
    // This creates a full Bones command
    // This is used when actual logic is given by the user (ordered subcommands)
    pub fn new(directive: &BonesDirective, cmds: HashMap<String, Bone>) -> Self {
        Self {
            directive: directive.clone(),
            cmds,
        }
    }
    // Runs a Bones command by evaluating the directive itself and calling commands in sequence recursively
    // Currently, the logic of the Bones language lives here
    fn run(&self, output: &mut impl std::io::Write) -> Result<i32, String> {
        // This system is highly recursive, so everything is done in this function for progressively less complex directives
        fn run_for_directive(
            directive: &BonesDirective,
            cmds: &HashMap<String, Bone>,
            output: &mut impl std::io::Write,
        ) -> Result<i32, String> {
            // Get the token, which names the command we'll be running
            let command_name = &directive.0;
            // Now get the corresponding Bone if it exists
            let bone = cmds.get(command_name);
            let bone = match bone {
                Some(bone) => bone,
                None => return Err(format!("Error in executing Bones directive: subcommand '{}' not found. This is probably a typo in your Bonnie configuration.", command_name)),
            };
            // Now execute it and get the exit code (this may recursively call this function if ordered subcommands are nested, but that dcoesn't matter)
            // Bonnie treats all command cores as futures for an exit code, we don't care about any side effects (printing, server execution, etc.)
            let exit_code = bone.run(command_name, output)?;
            // Iterate over the conditions given and check if any of them match that exit code
            // We'll run the first one that does (even if more do after that)
            // TODO document the above behaviour
            let mut final_exit_code = exit_code;
            for (operator, directive) in directive.1.iter() {
                if operator.matches(&exit_code) {
                    // An operator has matched, check if it has an associated directive
                    final_exit_code = match directive {
                        // If it does, run that and get its exit code
                        Some(directive) => run_for_directive(directive, cmds, output)?,
                        // If not, return the exit code we just got above
                        None => exit_code,
                    };
                }
            }

            // All nestings have resolved to one exit code, we return it
            Ok(final_exit_code)
        }

        // Begin the recursion on this top-level directive
        // This will eventually return the exit code from the lowest level of recursion, which we return
        let exit_code = run_for_directive(&self.directive, &self.cmds, output)?;
        Ok(exit_code)
    }
}

// A directive telling the Bones engine how to progress between ordered subcommands
// This maps the command to run to a set of conditions as to how to proceed based on its exit code
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BonesDirective(String, HashMap<BonesOperator, Option<BonesDirective>>);
// This is used for direct parsing, before we've had a chance to handle the operators
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct RawBonesDirective(String, HashMap<String, Option<RawBonesDirective>>);
impl RawBonesDirective {
    // This converts to a `BonesDirective` by parsing the operator strings into full operators
    fn convert_to_proper(&self) -> Result<BonesDirective, String> {
        // Parse the conditions `HashMap`
        let mut parsed_conditions: HashMap<BonesOperator, Option<BonesDirective>> = HashMap::new();
        for (raw_operator, raw_directive) in &self.1 {
            let operator = BonesOperator::parse_str(&raw_operator)?;
            // Parse the directive recursively
            // We need to use a full `match` statement for `?`
            let directive = match raw_directive {
                Some(raw_directive) => Some(raw_directive.convert_to_proper()?),
                None => None,
            };
            parsed_conditions.insert(operator, directive);
        }

        Ok(
            // We don't need to do any parsing on the command name, just the conditions
            BonesDirective(self.0.to_string(), parsed_conditions),
        )
    }
}
// Bones operators can be more than just exit codes, this defines their possibilities
// For deserialization, this is left tagged (we pre-parse)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, std::hash::Hash)]
pub enum BonesOperator {
    // A simple exit code comparison
    ExitCode(i32),
    // A negative exit code comparison ('anything except ...')
    NotExitCode(i32),
    // An operator that will match no matter what its command returned
    Any,
    // An operator that will never match no matter what its command returned
    None,
    // The requirement for command success (an alias for `ExitCode(0)`)
    Success,
    // The requirement for command failure (an alias for `NotExitCode(0)`)
    Failure,
    // Matches if any contained operators match (or statement)
    Union(Vec<BonesOperator>),
    // Matches if all contained operators match (and statement)
    // No it shouldn't be possible to have multiple exit codes match simultaneously but this is here anyway for potential future additions
    Intersection(Vec<BonesOperator>),
}
impl BonesOperator {
    // Checks if the given exit code matches this operator
    fn matches(&self, exit_code: &i32) -> bool {
        // This can be recursive due to the `Union` an d`Intersection` variants
        fn matches(exit_code: &i32, variant: &BonesOperator) -> bool {
            // Go through each different type of operator possible
            match variant {
                BonesOperator::Success => *exit_code == 0,
                BonesOperator::Failure => *exit_code != 0,
                BonesOperator::ExitCode(comparison) => exit_code == comparison,
                BonesOperator::NotExitCode(comparison) => exit_code != comparison,
                BonesOperator::Any => true,
                BonesOperator::None => false,
                BonesOperator::Union(operators) => {
                    let mut is_match = false;
                    for operator in operators {
                        let op_matches = operator.matches(exit_code);
                        // We only need one of them to be true
                        if op_matches {
                            is_match = true;
                            break;
                        }
                    }
                    is_match
                }
                BonesOperator::Intersection(operators) => {
                    let mut is_match = false;
                    for operator in operators {
                        let op_matches = operator.matches(exit_code);
                        // We only need one of them to be false (aka. all of them have to be true)
                        is_match = op_matches;
                        if !op_matches {
                            break;
                        }
                    }
                    is_match
                }
            }
        }

        matches(exit_code, self)
    }
    // Parses a string operator given in a directive string into a fully-fledged variant
    fn parse_str(raw_operator: &str) -> Result<Self, String> {
        // Attempt to parse it as an exit code integer (we'll use that twice)
        let exit_code = raw_operator.parse::<i32>();
        let operator = match raw_operator {
            _ if exit_code.is_ok() => BonesOperator::ExitCode(exit_code.unwrap()),
            _ if raw_operator.starts_with('!') => {
                let exit_code_str = raw_operator.get(1..);
                let exit_code = match exit_code_str {
                    Some(exit_code) => match exit_code.parse::<i32>() {
                        Ok(exit_code) => exit_code,
                        Err(_) => return Err(format!("Couldn't parse exit code as 32-bit integer from `NotExitCode` operator invocation '{}'.", raw_operator))
                    },
                    None => return Err(format!("Couldn't extract exit code from `NotExitCode` operator invocation '{}'.", raw_operator))
                };
                BonesOperator::NotExitCode(exit_code)
            }
            // The next four are simple because they have no attached data
            "Any" => BonesOperator::Any,
            "None" => BonesOperator::None,
            "Success" => BonesOperator::Success,
            "Failure" => BonesOperator::Failure,
            // These require recursion
            _ if raw_operator.contains('|') => {
                let parts: Vec<&str> = raw_operator.split('|').collect();
                let mut operators: Vec<BonesOperator> = Vec::new();
                // Recursively parse each operator
                for part in parts {
                    operators.push(BonesOperator::parse_str(part)?)
                }
                BonesOperator::Union(operators)
            }
            _ if raw_operator.contains('+') => {
                let parts: Vec<&str> = raw_operator.split('+').collect();
                let mut operators: Vec<BonesOperator> = Vec::new();
                // Recursively parse each operator
                for part in parts {
                    operators.push(BonesOperator::parse_str(part)?)
                }
                BonesOperator::Intersection(operators)
            }
            _ => {
                return Err(format!(
                    "Unrecognized operator '{}' in Bones directive.",
                    raw_operator
                ))
            }
        };

        Ok(operator)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BonesCore {
    pub cmd: String,
    pub shell: Vec<String>, // Vector of executable and arguments thereto
}
impl BonesCore {
    fn execute(&self, name: &str, output: &mut impl std::io::Write) -> Result<i32, String> {
        // Interpolate the command into the given shell
        // The shell might inteprolate it multiple times or not at all, we don't particularly care (shells can be as weird as they like)
        let cmd_parts: Vec<String> = self
            .shell
            .iter()
            .map(|part| part.replace("{COMMAND}", &self.cmd))
            .collect();
        // Get the executable from that vector (the first element)
        let executable = cmd_parts.get(0);
        let executable = match executable {
            Some(executable) => executable,
            None => return Err(String::from("An empty shell was provided. Shells must have at least one element as an executable to be invoked."))
        };
        // Get the rest of the arguments to that executable
        // This won't panic (we handled if the zero index doesn't exist in the above `match` statement)
        let args: Vec<String> = cmd_parts[1..].to_vec();
        // If we're in debug, write details about the command to the given output
        // TODO only do this in testing
        if cfg!(debug_assertions) {
            writeln!(output, "{}, {:?}", executable, args).expect("Failed to write warning.");
        }

        // Prepare the child process
        let child = OsCommand::new(&executable).args(&args).spawn();

        // The child must be mutable so we can wait for it to finish later
        let mut child = match child {
            Ok(child) => child,
            Err(_) => return Err(
                format!(
                    "Command '{}' failed to run. This doesn't mean the command produced an error, but that the process couldn't even be initialised.",
                    &name
                )
            )
        };
        // If we don't wait on the child, any long-running commands will print into the prompt because the parent terminates first (try it yourself with the `long` command)
        let child = child.wait();
        let exit_status = match child {
            Ok(exit_status) => exit_status,
            Err(_) => return Err(
                format!(
                    "Command '{}' didn't run (parent unable to wait on child process). See the Bonnie documentation for more details on this problem.",
                    &name
                )
            )
        };

        // We now need to pass that exit code through so Bonnie can terminate with it (otherwise `&&` chaining doesn't work as expected, etc.)
        // This will work on both Unix and Windows (and so theoretically any other weird OSes that make any sense at all)
        Ok(match exit_status.code() {
            Some(exit_code) => exit_code,       // If we have an exit code, use it
            None if exit_status.success() => 0, // If we don't, but we know the command succeeded, return 0 (success code)
            None => 1, // If we don't know an exit code but we know that the command failed, return 1 (general error code)
        })
    }
}

// This parses a directive string into a `BonesDirective` that can be executed
// The logic of parsing and executing is made separate so we can cache the parsed form for large configuration files
// This function basically interprets a miniature programming language
// Right now, this is quite slow due to its extensive use of RegEx, any ideas to speed it up would be greatly appreciated!
pub fn parse_directive_str(directive_str: &str) -> Result<BonesDirective, String> {
    let directive_json: String;
    // Check if we have the alternative super-simple form (just one command, rare but easy to parse)
    if !directive_str.contains('{') {
        directive_json = "[\"".to_string() + directive_str + "\", {}]"
    } else {
        // We transform the directive string into compliant JSON with a series of substitutions
        // Execute non-regex substitutions
        let stage1 = directive_str.replace("}", "}]");
        // We can unwrap all the RegExps because we know they're valid
        // Please refer to the Bones specification to understand how these work
        let re1 = Regex::new(r"(?m)^(\s*)(.+) => (.+)\b \{").unwrap();
        let sub1 = "$1\"$2\": [\"$3\", {";
        let re2 = Regex::new(r"(?m)^(\s*)(.+) => (.+)\b").unwrap();
        let sub2 = "$1\"$2\": [\"$3\", {}]";
        let re3 = Regex::new(r"^\s*\b(.+) \{").unwrap();
        let sub3 = "[\"$1\", {";
        // Execute each of those substitutions
        let stage2 = re1.replace_all(&stage1, sub1);
        let stage3 = re2.replace_all(&stage2, sub2);
        directive_json = re3.replace_all(&stage3, sub3).to_string();
    }
    // Now we can deserialize that directly using Serde
    let raw_directive = serde_json::from_str::<RawBonesDirective>(&directive_json);
    let raw_directive = match raw_directive {
        Ok(raw_directive) => raw_directive,
        Err(err) => return Err(format!("The following error occurred while parsing a Bones directive: '{}'. Please note that your code is transformed in several ways before this step, so you may need to refer to the documentation on Bones directives.", err))
    };
    // Now we handle the operators
    let directive = raw_directive.convert_to_proper()?;

    Ok(directive)
}
