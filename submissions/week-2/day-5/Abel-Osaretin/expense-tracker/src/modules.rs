use crate::tracker::{Expense, ExpenseTracker, TransactionType};
use std::io;
use std::process::exit;

pub fn username_module() -> String {
    let mut username: String = String::new();

    println!("Please enter Username");
    io::stdin()
        .read_line(&mut username)
        .expect("Failed To Get Input");

    println!("Welcome {username}");
    username
}

pub fn add_module(tracker: &mut ExpenseTracker, filename: &str) -> Expense {
    let mut name = String::new();
    let mut amount = String::new();
    let mut tx_type = String::new();

    println!("Enter Expense Name:");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to get Input");
    println!("Enter Expense Amount: ");
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to get Input");
    println!("Enter Transaction Type (Credit/Debit): ");
    io::stdin()
        .read_line(&mut tx_type)
        .expect("Failed to get Input");

    let name = name.trim().to_string();

    let amount = amount
        .trim()
        .parse::<f64>()
        .expect("Failed to parse amount");
    let tx_type = match tx_type.trim().to_lowercase().as_str() {
        "credit" => TransactionType::Credit,
        "debit" => TransactionType::Debit,
        _ => {
            println!("Invalid transaction type. Defaulting to Debit.");
            TransactionType::Debit
        }
    };
    let add_result = tracker.add(name, amount, tx_type);

    tracker.save_to_file(filename);

    add_result
}

pub fn view_module(tracker: &mut ExpenseTracker) -> Vec<&Expense> {
    tracker.view_all()
}

pub fn update_module(tracker: &mut ExpenseTracker, filename: &str) -> Expense {
    let mut id = String::new();
    let mut amount = String::new();
    let mut tx_type = String::new();

    println!("Enter Expense ID: ");
    io::stdin().read_line(&mut id).expect("Failed to get Input");

    println!("Enter Expense Amount: ");
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to get Input");
    println!("Enter Transaction Type (Credit/Debit): ");
    io::stdin()
        .read_line(&mut tx_type)
        .expect("Failed to get Input");

    let id = id.trim().parse().expect("Failed to parse id");
    let amount = amount
        .trim()
        .parse::<f64>()
        .expect("Failed to parse amount");
    let tx_type = match tx_type.trim().to_lowercase().as_str() {
        "credit" => TransactionType::Credit,
        "debit" => TransactionType::Debit,
        _ => {
            println!("Invalid transaction type. Defaulting to Debit.");
            TransactionType::Debit
        }
    };

    let update_result = tracker.update(id, amount, tx_type);

    tracker.save_to_file(filename);

    update_result
}

pub fn delete_module(tracker: &mut ExpenseTracker, filename: &str) -> bool {
    let mut id = String::new();

    println!("Enter Expense ID: ");
    io::stdin().read_line(&mut id).expect("Failed to get Input");
    let id = id.trim().parse().expect("Failed to parse id");

    let delete_result =tracker.delete(id);
    tracker.save_to_file(filename);
    delete_result
}

pub fn quit_module(tracker: &mut ExpenseTracker, filename: &str) {
    let mut input = String::new();
    println!("Are you Sure You Want to Quit (Y/N)");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get input");
    let input = input.trim().to_lowercase();

    if input == "y" {
        tracker.save_to_file(filename);
        exit(1)
    } else if input == "n" {
        println!("Program Resumed ---------");
    } else {
        println!("Invalid Input");
    }
}
