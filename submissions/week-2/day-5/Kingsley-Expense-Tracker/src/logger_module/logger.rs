use chrono::Local;
use std::io::Write;
use std::{fs::OpenOptions, io::Result};

pub fn log(operation: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs.txt")?;

    let timestamp = Local::now().format("%Y-%M-%D %H:%M:%S");

    if let Err(e) = writeln!(file, "{timestamp} {operation}") {
        println!("Logging to file failed: {e}")
    }

    Ok(())
}
