
use std::io;
enum PowerStatus {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

fn format_power_status(input: &str) -> Option<PowerStatus> {
    match input.to_lowercase().as_str() {
        "off" => Some(PowerStatus::Off),
        "sleep" => Some(PowerStatus::Sleep),
        "reboot" => Some(PowerStatus::Reboot),
        "shutdown" => Some(PowerStatus::Shutdown),
        "hibernate" => Some(PowerStatus::Hibernate),
        _ => None,
    }
}


fn handle_power_status(status: PowerStatus) {
    match status {
        PowerStatus::Off => println!("Turning off the computer"),
        PowerStatus::Sleep => println!("Putting the computer to sleep"),
        PowerStatus::Reboot => println!("Rebooting the computer"),
        PowerStatus::Shutdown => println!("Shutting down the computer"),
        PowerStatus::Hibernate => println!("Hibernating the computer"),
    }
}



fn main() {
      println!("Enter power option (off, sleep, reboot, shutdown, hibernate):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input = input.trim();

    match format_power_status(input) {
        Some(status) => handle_power_status(status),
        None => println!("Error: Invalid power option"),
    }
}
    