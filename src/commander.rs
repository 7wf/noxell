use std::process::Command;
use std::str::SplitWhitespace;

/**
 * Parses a command request.
 */
pub fn parse_command_request<'a>(input: &'a str) -> (&str, SplitWhitespace<'_>) {
    let mut arguments = input.split_whitespace();
    let command = &arguments.next().unwrap();

    (command, arguments)
}

/**
 * Runs a command from the command name and the arguments.
 */
pub fn run_command(command_name: &str, command_arguments: Vec<&str>) {
    let command_name = command_name.to_owned();
    let mut command = create_command(&command_name, command_arguments);

    match command.spawn() {
        Err(error) => eprintln!("Failed to spawn \"{}\". {}", command_name, error),
        Ok(mut child) => match child.wait() {
            Ok(result) => drop(result),
            Err(error) => eprintln!("Failed to wait for \"{}\". {}", command_name, error),
        },
    };
}

/**
 * Creates a command from a executable name and the provided arguments.
 */
fn create_command(executable: &String, arguments: Vec<&str>) -> Command {
    let mut command = Command::new(&executable);
    command.args(arguments);

    command
}