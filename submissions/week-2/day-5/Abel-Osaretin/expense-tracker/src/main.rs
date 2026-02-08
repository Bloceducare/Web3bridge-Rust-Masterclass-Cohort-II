mod menu; 
mod modules;
mod tracker;

use menu::Menu;
use std::io;
use tracker::ExpenseTracker;

fn main() {
    let filename = "expenses.json";
    let mut tracker = ExpenseTracker::new();

    modules::username_module();
    tracker.values = ExpenseTracker::load_from_file(filename);
    if let Some(max_id) = tracker.values.keys().max() {
        tracker.next_id = max_id + 1;
    }

    loop {
        menu_module(&mut tracker, filename);
    }
}

fn menu_module(tracker: &mut ExpenseTracker, filename: &str) {
    let mut menu_input: String = String::new();

    println!("Please select a menu number or name:");
    println!(
        "\n 1. {:#?}\n 2. {:#?}\n 3. {:#?}\n 4. {:#?}\n q. {:#?} ",
        Menu::Add,
        Menu::View,
        Menu::Update,
        Menu::Remove,
        Menu::Quit
    );

    io::stdin()
        .read_line(&mut menu_input)
        .expect("Failed To Get Input");

    let result = match Menu::menu_select(&menu_input) {
        Some(result) => result,
        None => {
            println!("Invalid Menu");
            return menu_module(tracker, filename);
        }
    };

    match result {
        Menu::Add => {
            println!("-------------------------------------------------\n");
            println!("Add Module");
            println!("\n-------------------------------------------------\n");

            let new_expenses = modules::add_module(tracker, filename);
            tracker.save_to_file(filename);

            println!("-------------------------------------------------");
            println!("-------------------------------------------------");
            println!("New {:#?}", new_expenses);
            println!("-------------------------------------------------");
            println!("-------------------------------------------------\n");
        }
        Menu::View => {
            println!("-------------------------------------------------\n");
            println!("View Module");
            println!("\n-------------------------------------------------\n");
            let all_expenses = modules::view_module(tracker);

            println!("-------------------------------------------------");
            println!("-------------------------------------------------");
            println!("All Expenses:  {:#?}", all_expenses);
            println!("-------------------------------------------------");
            println!("-------------------------------------------------\n");
        }
        Menu::Update => {
            println!("-------------------------------------------------\n");
            println!("Update Module");
            println!("\n-------------------------------------------------\n");

            let updated_data = modules::update_module(tracker, filename);
            tracker.save_to_file(filename);

            println!("-------------------------------------------------");
            println!("-------------------------------------------------");
            println!("Updated Data  {:#?}", updated_data);
            println!("-------------------------------------------------");
            println!("-------------------------------------------------\n");
        }
        Menu::Remove => {
            println!("-------------------------------------------------\n");
            println!("\n-------------------------------------------------\n");
            println!("Remove Module");

            let result = modules::delete_module(tracker, filename);
            tracker.save_to_file(filename);
            println!("Expense Deleted: {result}");

            println!("-------------------------------------------------");
            println!("-------------------------------------------------\n");
        }
        Menu::Quit => {
            println!("-------------------------------------------------\n");
            println!("\n-------------------------------------------------\n");

            println!("Quit Module");

            modules::quit_module(tracker, filename);
            println!("\n-------------------------------------------------\n");
            println!("-------------------------------------------------\n");
        }
    }
}
