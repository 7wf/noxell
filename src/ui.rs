use std::io::{self, Write};
use std::env;

/**
 * Prints the λoxell delimiter.
 */
fn print_delimiter() {
    let directory = env::current_dir().expect("Cannot get current directory.");
    let directory = match directory.file_name() {
        None => "/",
        Some(path) => path.to_str().unwrap(),
    };

    yellow!("{} ", directory);
    cyan!("λ ");

    io::stdout().flush().unwrap();
}

/**
 * Reads the input line.
 */
pub fn read_line() -> String {
    print_delimiter();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_owned()
}
