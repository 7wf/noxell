use std::io::{self, Write};

/**
 * Prints the λoxell delimiter.
 */
fn print_delimiter() {
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
