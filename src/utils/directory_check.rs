use crate::utils;
use std::io;

// Check if the current directory contains a py file
pub fn directory_check(file: &str) -> bool {
    if file[file.len() - 1..file.len()].to_string() != "/" {
        return false;
    } else {
        let valid_files: Result<Vec<String>, io::Error> = utils::read_directory(file);

        for file in valid_files.unwrap() {
            if file != "../" {
                if check_py(&file) {
                    return true;
                } else if directory_check(&file) {
                    return true;
                }
            }
        }

        return false;
    }
}

// Check if the file string is a py file
pub fn check_py(file: &str) -> bool {
    if file[file.len() - 2..file.len()].to_string() == "py" {
        return true;
    }
    return false;
}
