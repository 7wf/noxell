#[macro_use]
extern crate colour;

mod commander;
mod ui;

fn main() {
    loop {
        let input = ui::read_line();
        if input.is_empty() {
            continue;
        }

        let (command_name, command_arguments) = commander::parse_command_request(&input);

        match command_name {
            "exit" => return,
            "cd" => unimplemented!(),
            command_name => commander::run_command(command_name, command_arguments),
        }
    }
}
