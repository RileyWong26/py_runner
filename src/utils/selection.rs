use dialoguer::console::Term;
use dialoguer::{Select, theme::ColorfulTheme};
use std::io;

pub fn create_selection(valid_files: Vec<String>) -> Result<String, String> {
    // Create the selection
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a file")
        .default(0)
        .items(&valid_files[..])
        .interact_on_opt(&Term::stderr());
    // When something is selected
    match selection {
        Ok(Some(index)) => Ok(valid_files[index].clone()),
        Ok(None) => Err(String::from("bad")),
        Err(Error) => Err(String::from("bad")),
    }
}
