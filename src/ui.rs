use std::io::{self, Write};
use std::path::Path;
use std::env;

/**
 * Returns the directory of a path.
 */
fn get_directory(path: &Path) -> &str {
    match &path.file_name() {
        None => "/",
        Some(directory) => directory.to_str().unwrap(),
    }
}

/**
 * Prints the λoxell delimiter.
 */
fn print_delimiter() {
    let directory = env::current_dir().expect("Cannot get current directory.");
    let directory = get_directory(&directory);

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
