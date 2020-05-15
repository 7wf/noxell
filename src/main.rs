#[macro_use]
extern crate colour;

use std::process;

mod commander;
mod commands;
mod ui;

/**
 * Runs a command.
 */
fn run_command(command_name: &str, command_arguments: Vec<&str>) -> Result<bool, String> {
    match command_name {
        "exit" => process::exit(0),
        "cd" => match commands::change_directory::execute(command_arguments) {
            Ok(_) => Ok(true),
            Err(error) => Err(error),
        },
        command_name => {
            commander::run_command(command_name, command_arguments);
            Ok(true)
        },
    }
}

fn main() {
    loop {
        let input = ui::read_line();
        if input.is_empty() {
            continue;
        }

        let input_commands = input.split("&&");
        for input_command in input_commands {
            let (command_name, command_arguments) = commander::parse_command_request(&input_command);
            let command_arguments: Vec<&str> = command_arguments.collect();

            match run_command(command_name, command_arguments) {
                Ok(_) => continue,
                Err(error) => eprintln!("{}", error)
            }            
        }
    }
}
