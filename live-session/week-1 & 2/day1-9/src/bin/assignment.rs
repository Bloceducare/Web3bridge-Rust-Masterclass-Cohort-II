use std::io;

enum PowerOptions {
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerOptions {
    fn power_action(state: &str) -> Option<PowerOptions> {
        let state = state.trim().to_lowercase();
        match state.as_str() {
            "sleep" => Some(PowerOptions::Sleep),
            "reboot" => Some(PowerOptions::Reboot),
            "shutdown" => Some(PowerOptions::Shutdown),
            "hibernate" => Some(PowerOptions::Hibernate),
            _ => None,
        }
    }
}

fn print_state(action: PowerOptions) {
    match action {
        PowerOptions::Sleep => println!("Sleeping"),
        PowerOptions::Reboot => println!("Rebooting"),
        PowerOptions::Shutdown => println!("Shutting Down"),
        PowerOptions::Hibernate => println!("Hibernating"),
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter power option");

    let user_input = io::stdin().read_line(&mut input);

    if user_input.is_ok() {
        match PowerOptions::power_action(&input) {
            Some(result) => print_state(result),
            None => println!("Invalid power state"),
        }
    }
}
