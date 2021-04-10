use std::env;

mod command;
mod commands_registry;
use crate::command::Command;
use crate::commands_registry::CommandsRegistry;

fn main() {
    let mut registry = CommandsRegistry::new();
    registry.add("test", Command::new("test", vec!["firstname", "lastname"], "echo \"Hello %firstname %lastname!\""));
    registry.add("cat", Command::new("cat", vec![], "cat hello.txt"));

    // Get the arguments to this program and extract the command the user wants to run and the arguments they're providing to it
    let prog_args: Vec<String> = env::args().collect();
    // When getting the command the suer wants to run, they may not have provided one, so we handle that
    let cmd = &prog_args.get(1).expect("You must provide a command to run."); // The command the user wants to run
    let args = &prog_args[2..]; // Any arguments to that command the user has provided

    let command = registry.get(cmd);
    let command_with_args = command.insert_args(&args.to_vec());

    Command::run(&command_with_args);
}
