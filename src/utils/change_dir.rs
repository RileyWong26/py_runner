use std::env;
use std::path::Path;

// cd into a path
pub fn go_child(child: &str) {
    let new_dir = Path::new(child);
    if env::set_current_dir(&new_dir).is_ok() {}
}
