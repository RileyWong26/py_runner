use dialoguer::console::Term;
use dialoguer::{Select, theme::ColorfulTheme};

pub fn create_selection(mut valid_files: Vec<String>) -> String {
    // Create the selection
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a file")
        .default(0)
        .items(&valid_files)
        .interact_on_opt(&Term::stderr());
    // When something is selected
    match selection {
        Ok(Some(index)) => valid_files.remove(index),
        Ok(None) => String::from("bad"),
        Err(_err) => "error".to_string(),
    }
}
