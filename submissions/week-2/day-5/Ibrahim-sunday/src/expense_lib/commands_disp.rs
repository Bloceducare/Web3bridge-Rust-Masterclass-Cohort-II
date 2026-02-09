use std::io;

pub fn display_commands() {
    println!("\n\n...............Commands list.............\n");
    println!("add: Add an expense");
    println!("view_all: View all expenses");
    println!("view: View an expense");
    println!("view_debits: View all debit expenses");
    println!("view_credits: View all credit expenses");
    println!("update: Update an expense");
    println!("del: Delete an expense");
    println!("q: Quit/Exit the application");
    println!("........................................\n\n");
}

pub fn start_command() {
    let mut command = String::new();
    println!("\n\n............................\n");
    println!("Click c to continue");
    println!("........................................\n\n");

    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");

    let cmd_parsed = command.trim();

    if cmd_parsed == "c" {
        println!("\n ");
    } else {
        println!("Program ended");
    }
}
