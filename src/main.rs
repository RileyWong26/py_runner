use dialoguer::console::Term;
use dialoguer::{Select, theme::ColorfulTheme};
use std::fs;
use std::io;
fn main() -> io::Result<()> {
    // Pull everything in cucrent pwd into a vector of strings
    let paths: Vec<String> = fs::read_dir("./")?
        .map(|res| res.map(|e| e.path().to_string_lossy().into_owned()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select")
        .default(0)
        .items(&paths[..])
        .interact_on_opt(&Term::stderr())?;
    match selection {
        Some(index) => println!("User selected: {:?}", paths[index]),
        None => println!("User did not select anything"),
    }

    Ok(())
}
