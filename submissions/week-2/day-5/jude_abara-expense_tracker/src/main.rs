mod expense;
mod file_handler;

use expense::{Expense, ExpenseTracker, TransactionType};
use file_handler::FileHandler;
use std::io::{self, Write};

fn main() {
    let mut expense_tracker = ExpenseTracker::new();
    let file_handler = FileHandler::new("expenses.txt");

    println!("╔════════════════════════════════════════╗");
    println!("║   Welcome to Expense Tracker v0.1.0    ║");
    println!("╚════════════════════════════════════════╝");

    loop {
        print_menu();

        let command = read_input("Enter your choice: ");
        let command = command.trim().to_lowercase();

        match command.as_str() {
            "1" => handle_add(&mut expense_tracker),
            "2" => handle_view_all(&expense_tracker),
            "3" => handle_update(&mut expense_tracker),
            "4" => handle_delete(&mut expense_tracker),
            "q" => {
                if confirm_quit() {
                    // Save all transactions to file before exiting
                    if let Err(e) = file_handler.save_expenses(&expense_tracker) {
                        eprintln!("Error saving expenses: {}", e);
                    } else {
                        println!("All transactions saved successfully!");
                    }
                    println!("Goodbye!");
                    break;
                }
            }
            _ => println!("Invalid command '{}'. Please try again.\n", command),
        }
    }
}

fn print_menu() {
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("AVAILABLE OPERATIONS:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  1 → Add Transaction");
    println!("  2 → View All Transactions");
    println!("  3 → Update Transaction by ID");
    println!("  4 → Delete Transaction by ID");
    println!("  q → Quit Program");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

fn handle_add(tracker: &mut ExpenseTracker) {
    println!("\nADD NEW TRANSACTION");
    println!("─────────────────────────────────────────");

    let description = read_input("Enter description: ");
    let description = description.trim().to_string();

    if description.is_empty() {
        println!("Description cannot be empty!");
        return;
    }

    let amount = read_amount();
    let tx_type = read_transaction_type();

    let expense = tracker.add(description, amount, tx_type);

    println!("\nTransaction added successfully!");
    println!("─────────────────────────────────────────");
    print_expense(&expense);
    println!("─────────────────────────────────────────");
}

fn handle_view_all(tracker: &ExpenseTracker) {
    println!("\nALL TRANSACTIONS");
    println!("─────────────────────────────────────────");

    let expenses = tracker.view_all();

    if expenses.is_empty() {
        println!("No transactions found.");
    } else {
        let (total_credit, total_debit) = calculate_totals(&expenses);

        for expense in expenses {
            print_expense(expense);
            println!("─────────────────────────────────────────");
        }

        println!("\nSUMMARY:");
        println!("  Total Credit:  ${:.2}", total_credit);
        println!("  Total Debit:   ${:.2}", total_debit);
        println!("  Balance:       ${:.2}", total_credit - total_debit);
    }
}

fn handle_update(tracker: &mut ExpenseTracker) {
    println!("\nUPDATE TRANSACTION");
    println!("─────────────────────────────────────────");

    // First show all transactions
    let expenses = tracker.view_all();
    if expenses.is_empty() {
        println!("No transactions to update.");
        return;
    }

    println!("Current transactions:");
    for expense in expenses {
        println!(
            "  ID {}: {} - ${:.2} ({})",
            expense.id,
            expense.description,
            expense.amount,
            format!("{:?}", expense.tx_type)
        );
    }

    let id = read_u8("Enter transaction ID to update: ");

    // Check if ID exists
    if !tracker.exists(id) {
        println!("Transaction with ID {} not found!", id);
        return;
    }

    println!("\nEnter new details:");
    let description = read_input("Enter new description: ");
    let description = description.trim().to_string();

    if description.is_empty() {
        println!("Description cannot be empty!");
        return;
    }

    let amount = read_amount();
    let tx_type = read_transaction_type();

    let updated_expense = Expense {
        id,
        description,
        amount,
        tx_type,
    };

    if tracker.update(id, updated_expense.clone()) {
        println!("\nTransaction updated successfully!");
        println!("─────────────────────────────────────────");
        print_expense(&updated_expense);
        println!("─────────────────────────────────────────");
    } else {
        println!("Failed to update transaction with ID {}", id);
    }
}

fn handle_delete(tracker: &mut ExpenseTracker) {
    println!("\nDELETE TRANSACTION");
    println!("─────────────────────────────────────────");

    // First show all transactions
    let expenses = tracker.view_all();
    if expenses.is_empty() {
        println!("No transactions to delete.");
        return;
    }

    println!("Current transactions:");
    for expense in expenses {
        println!(
            "  ID {}: {} - ${:.2} ({})",
            expense.id,
            expense.description,
            expense.amount,
            format!("{:?}", expense.tx_type)
        );
    }

    let id = read_u8("Enter transaction ID to delete: ");

    // Get the expense before deleting to show it
    if let Some(expense) = tracker.get(id) {
        let expense_clone = expense.clone();

        if tracker.delete(id) {
            println!("\nTransaction deleted successfully!");
            println!("─────────────────────────────────────────");
            println!("Deleted transaction:");
            print_expense(&expense_clone);
            println!("─────────────────────────────────────────");
        } else {
            println!("Failed to delete transaction with ID {}", id);
        }
    } else {
        println!("Transaction with ID {} not found!", id);
    }
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}

fn read_amount() -> f64 {
    loop {
        let input = read_input("Enter amount: ");
        match input.trim().parse::<f64>() {
            Ok(value) if value > 0.0 => return value,
            Ok(_) => println!("Amount must be greater than 0. Please try again."),
            Err(_) => println!("Invalid amount. Please enter a valid number."),
        }
    }
}

fn read_u8(prompt: &str) -> u8 {
    loop {
        let input = read_input(prompt);
        match input.trim().parse::<u8>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid ID. Please enter a valid number."),
        }
    }
}

fn read_transaction_type() -> TransactionType {
    loop {
        let input = read_input("Enter transaction type (Credit/Debit): ");
        match input.trim().to_lowercase().as_str() {
            "credit" => return TransactionType::Credit,
            "debit" => return TransactionType::Debit,
            _ => println!("Invalid type. Please enter 'Credit' or 'Debit'."),
        }
    }
}

fn confirm_quit() -> bool {
    loop {
        let input = read_input("\nAre you sure you want to quit? (Y/n): ");
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please enter 'Y' for Yes or 'n' for No."),
        }
    }
}

fn print_expense(expense: &Expense) {
    let tx_symbol = match expense.tx_type {
        TransactionType::Credit => "Credit",
        TransactionType::Debit => "Debit",
    };

    println!("  {} ID: {}", tx_symbol, expense.id);
    println!("  Description: {}", expense.description);
    println!("  Amount: ${:.2}", expense.amount);
    println!("  Type: {:?}", expense.tx_type);
}

fn calculate_totals(expenses: &[&Expense]) -> (f64, f64) {
    let mut total_credit = 0.0;
    let mut total_debit = 0.0;

    for expense in expenses {
        match expense.tx_type {
            TransactionType::Credit => total_credit += expense.amount,
            TransactionType::Debit => total_debit += expense.amount,
        }
    }

    (total_credit, total_debit)
}
