mod handlers;
mod models;
mod utils;

use crate::utils::read_input;
use handlers::{handle_add, handle_delete, handle_quit, handle_update, handle_view};
use models::ExpenseTracker;

fn main() {
    let mut tracker = ExpenseTracker::load();
    println!("--- Expense Tracker CLI ---");

    loop {
        println!("\nOPTIONS: [1] Add  [2] View  [3] Update  [4] Delete  [q] Quit");
        let choice = read_input("> ", None).unwrap();

        match choice.as_str() {
            "1" => handle_add(&mut tracker),
            "2" => handle_view(&tracker),
            "3" => handle_update(&mut tracker),
            "4" => handle_delete(&mut tracker),
            "q" => handle_quit(),
            _ => println!("Invalid option. Please try again."),
        }
    }
}
