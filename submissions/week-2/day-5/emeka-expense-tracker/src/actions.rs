use crate::expenses::ExpenseTracker;
use crate::record::write_to_report;
use std::collections::HashMap;
use std::io;

#[derive(Clone, Debug)]
pub enum ActionTypes {
    AddExpense,
    RemoveExpense,
    UpdateExpense,
    ViewExpense,
    GetAllExpenses,
    GetTotalExpenses,
    PrintReport,
    Quit,
}

fn read_user_input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");
    user_input.trim().to_string()
}

fn get_numeric_id() -> u16 {
    let id_input = read_user_input();
    let id: u16 = match id_input.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid ID. Please enter a valid number.");
            get_numeric_id()
        }
    };
    id
}

impl ActionTypes {
    pub fn get_action(command_map: &HashMap<&str, ActionTypes>) -> ActionTypes {
        println!("\nWhat would you like to do?");
        println!(
            "Enter any of the following commands:\n\"add\", \"remove\", \"view\", \"all\", \"total\", \"report\", \"update\", or \"q\" to quit"
        );
        let user_input = read_user_input().to_lowercase();
        if let Some(action) = command_map.get(user_input.as_str()) {
            action.clone()
        } else {
            println!("Invalid command. Please try again.");
            ActionTypes::get_action(command_map)
        }
    }
}

fn get_list_of_items() -> HashMap<String, f64> {
    println!(
        "Enter items in the format: item_name:cost, separated by commas. e.g. \"coffee:3.5,lunch:12.0,groceries:45.0\""
    );
    let user_input = read_user_input();
    let mut items: HashMap<String, f64> = HashMap::new();
    for item in user_input.split(',') {
        let parts: Vec<&str> = item.trim().split(':').collect();
        if parts.len() == 2 {
            let name = parts[0].trim().to_string();
            if let Ok(cost) = parts[1].trim().parse::<f64>() {
                items.insert(name, cost);
            } else {
                println!("Invalid cost for item '{name}'. Skipping.");
            }
        } else {
            println!("Invalid format for item '{item}'. Skipping.");
        }
    }
    items
}

pub fn show_total_expenses(tracker: &ExpenseTracker) {
    let total = tracker.get_total_expenses();
    println!("Your total expenses amount to: ₦{total:.2}");
}

pub fn add_expense(tracker: &mut ExpenseTracker) {
    println!("Add a list of items and their cost to your expense tracker");
    let items = get_list_of_items();
    println!("You added the following items:");
    tracker.add_expense(items);
}

pub fn get_all_expenses(tracker: &ExpenseTracker) {
    let expenses = tracker.get_expenses();
    println!("Here are your total expenses:\n {expenses:#?}");
}

pub fn remove_expense(tracker: &mut ExpenseTracker) {
    println!("Enter the ID of the expense you want to remove:");
    let id = get_numeric_id();
    if tracker.remove_expense(id) {
        println!("Expense with ID {id} removed successfully.");
    } else {
        println!("Expense with ID {id} not found.");
    }
}

pub fn update_expense(tracker: &mut ExpenseTracker) {
    println!("Enter the ID of the expense you want to update:");
    let id = get_numeric_id();
    println!("Update an expense by providing a list of items and their cost");
    let items = get_list_of_items();
    if tracker.edit_expense(id, items) {
        println!("Expense with ID {id} updated successfully.");
    } else {
        println!("Expense with ID {id} not found.");
    }
}

pub fn view_expense(tracker: &ExpenseTracker) {
    println!("Enter the ID of the expense you want to view:");
    let id = get_numeric_id();
    if let Some(expense) = tracker.get_expense_by_id(id) {
        println!("Expense with ID {id}:\n{expense:#?}");
    } else {
        println!("Expense with ID {id} not found.");
    }
}

pub fn print_report(tracker: &ExpenseTracker) {
    let expenses = tracker.get_expenses();
    if expenses.is_empty() {
        println!("No expenses to report.");
    } else {
        let total_expense = tracker.get_total_expenses();
        println!("Generating report...");
        match write_to_report(expenses, total_expense) {
            Ok(_) => println!("Report generated successfully as 'report.txt'."),
            Err(e) => println!("Failed to generate report: {e}"),
        }
    }
}
