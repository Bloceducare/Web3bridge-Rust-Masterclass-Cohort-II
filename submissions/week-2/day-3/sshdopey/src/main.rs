use std::env;
use std::io::{self, Write};
use std::os::unix::process::CommandExt;
use std::process::{Command, exit};
use std::thread::sleep;
use std::time::Duration;

enum Actions {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

fn loading(message: &str) {
    print!(" >> {}", message);
    let _ = io::stdout().flush();

    for _ in 0..3 {
        sleep(Duration::from_millis(500));
        print!(".");
        let _ = io::stdout().flush();
    }
    println!("");
}

fn sleep_mode(message: &str) {
    loading(message);
    println!(" >> System is asleep. Press Enter to wake up...");

    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    println!(" >> System awake!");
}
fn shutdown(message: &str) {
    loading(message);
    exit(0);
}

fn reboot(message: &str) {
    loading(message);
    if let Ok(current_exe) = env::current_exe() {
        let _ = Command::new(current_exe).exec();
    }
}

fn execute(action: Actions) {
    match action {
        Actions::Off => shutdown("Powering off"),
        Actions::Shutdown => shutdown("Shutting down"),
        Actions::Sleep => sleep_mode("Entering sleep mode"),
        Actions::Reboot => reboot("System rebooting"),
        Actions::Hibernate => sleep_mode("Hibernating system"),
    }
}

fn main() {
    println!("\n --- SYSTEM READY ---");
    println!(" >> Please enter a command (Off, Sleep, Reboot, Shutdown, Hibernate):");

    loop {
        print!(" >> ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        input = input.trim().to_lowercase();

        if input.is_empty() {
            continue;
        }

        match input.as_str() {
            "off" => execute(Actions::Off),
            "sleep" => execute(Actions::Sleep),
            "reboot" => execute(Actions::Reboot),
            "shutdown" => execute(Actions::Shutdown),
            "hibernate" => execute(Actions::Hibernate),
            _ => println!(" >> Error: '{}' is not a recognized command.\n", input),
        };
    }
}
