use std::{process::Command, sync::mpsc};

pub fn run_shell(command: &str) -> Result<String, Box<dyn std::error::Error>>  {
    // let (cx, rx) = mpsc::channel::<String>();
     let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).into())
    }
}
