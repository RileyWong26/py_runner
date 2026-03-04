use crate::utils;

// Return only files and directories, (no dot files)
pub fn file_check(name: String) -> Option<String> {
    // Remove dot files
    if name.chars().nth(2)? == '.' {
        return None;
    }
    // Check proper file format
    else if name[name.len() - 2..name.len()].to_string() == "py" {
        return Some(name[2..name.len()].to_string());
    }
    // Check if a directory
    else if name[name.len() - 1..name.len()].to_string() == "/" {
        if utils::directory_check(&name) {
            return Some(name[2..name.len()].to_string());
        }
        return None;
    }
    // Return none if none of the above conditions match
    return None;
}
