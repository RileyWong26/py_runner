use dialoguer::console::Term;
use dialoguer::{Select, theme::ColorfulTheme};
use std::env;

pub fn create_selection(mut valid_files: Vec<String>) -> String {
    // Get pwd
    let path = env::current_dir();
    if path.is_ok() {
        // Create the selection
        let thing = &path.unwrap();
        let dir: &str = thing.to_str().unwrap();
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(dir)
            .default(0)
            .items(&valid_files)
            .report(false)
            .interact_on_opt(&Term::stderr());
        // When something is selected
        return match selection {
            Ok(Some(index)) => valid_files.remove(index),
            Ok(None) => String::from("bad"),
            Err(_err) => "error".to_string(),
        };
    }

    return "".to_string();
}
