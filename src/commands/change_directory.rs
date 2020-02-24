use std::env;
use std::path::Path;

/**
 * Executes the `cd` (change directory) command.
 */
pub fn execute(arguments: Vec<&str>) -> Result<bool, String> {
    let target_directory = &arguments.join("/");
    let path = Path::new(target_directory);

    let result = if !path.exists() {
        Err("The given path does not exists.".to_owned())
    } else if !path.is_dir() {
        Err("The given path is not a directory.".to_owned())
    } else {
        env::set_current_dir(path).expect("Occourred a error while changing the directory.");
        Ok(true)
    };

    result
}
