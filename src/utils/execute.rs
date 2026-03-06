use std::os::unix::process::CommandExt;
use std::process::Command;

// Python execute a file
pub fn execute(file: &str) {
    println!("Running: {}", file);
    let _output = Command::new("python").arg(file).exec();
}
