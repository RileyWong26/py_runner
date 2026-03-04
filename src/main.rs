use std::io;

pub mod utils;
fn main() -> io::Result<()> {
    // Keep track of directory
    let file_string: String = String::from("./");

    while utils::directory_check(&file_string) {
        // Get all valid files (directory or .py)
        let valid_files: Result<Vec<String>, io::Error> = utils::read_directory(&file_string);

        if valid_files.is_ok() {
            // Create a selection in the cli
            let selected_file: String = utils::create_selection(valid_files?);

            // Check if can execute or it is a directory
            if !utils::check_py(&selected_file) {
                utils::go_child(&selected_file);
            } else {
                utils::execute(&selected_file);
            }
        }
    }
    Ok(())
}
