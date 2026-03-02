use std::io;
use std::process::Command;
use std::process::Stdio;

pub fn execute(file: String) -> io::Result<()> {
    let command_string: String = String::from("python ") + &file;
    let output = Command::new("python").arg(file).output();

    let binding = output.unwrap();
    let text = str::from_utf8(&binding.stdout).expect("Not valid output");
    println!("{}", text);
    Ok(())
}
