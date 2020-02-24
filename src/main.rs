#[macro_use]
extern crate colour;

mod commander;
mod commands;
mod ui;

fn main() {
    loop {
        let input = ui::read_line();
        if input.is_empty() {
            continue;
        }

        let (command_name, command_arguments) = commander::parse_command_request(&input);
        let command_arguments: Vec<&str> = command_arguments.collect();

        match command_name {
            "exit" => return,
            "cd" => match commands::change_directory::execute(command_arguments) {
                Ok(_) => continue,
                Err(error) => eprintln!("{}", error),
            },
            command_name => commander::run_command(command_name, command_arguments),
        }
    }
}
