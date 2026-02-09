use std::io::Write;

use crate::models::{ExpenseTracker, TransactionType};
use crate::utils::cleanup_string;
use inquire::{Confirm, CustomType, Select, Text};

const EXPENSES_FILE: &str = "expenses.txt";

pub fn match_intent() {
    let mut tracker_instance = ExpenseTracker::new();

    loop {
        let intent = Text::new("what do you want to do? (create/read/update/delete/q to quit)")
            .with_help_message("Type: create, read, update, delete, or q")
            .prompt()
            .map(|s| cleanup_string(&s))
            .expect("failed to get intent");

        if intent == "q" || intent == "quit" {
            let really_quit = Confirm::new("Do you really want to quit?")
                .with_default(false)
                .prompt()
                .expect("failed to get confirmation");

            if really_quit {
                if let Err(e) = write_expenses_to_file(&tracker_instance) {
                    eprintln!("Failed to save expenses to file: {}", e);
                } else {
                    println!("Expenses saved to {}. Goodbye!", EXPENSES_FILE);
                }
                break;
            }
            continue;
        }

        match intent.as_str() {
            "create" => create_service(&mut tracker_instance),
            "read" => read_service(&tracker_instance),
            "update" => update_service(&mut tracker_instance),
            "delete" => delete_service(&mut tracker_instance),
            _ => println!("Unknown option. Use: create, read, update, delete, or q to quit."),
        }

        let again = Confirm::new("Do you want to do another operation?")
            .with_default(true)
            .prompt()
            .expect("failed to get confirmation");

        if !again {
            let really_quit = Confirm::new("Do you really want to quit?")
                .with_default(false)
                .prompt()
                .expect("failed to get confirmation");

            if really_quit {
                if let Err(e) = write_expenses_to_file(&tracker_instance) {
                    eprintln!("Failed to save expenses to file: {}", e);
                } else {
                    println!("Expenses saved to {}. Goodbye!", EXPENSES_FILE);
                }
                break;
            }
        }
    }
}

fn write_expenses_to_file(tracker: &ExpenseTracker) -> std::io::Result<()> {
    let mut file = std::fs::File::create(EXPENSES_FILE)?;
    for expense in tracker.view_all() {
        let tx_type = match expense.tx_type {
            TransactionType::Credit => "Credit",
            TransactionType::Debit => "Debit",
        };
        writeln!(
            file,
            "{}|{}|{}|{}",
            expense.id, expense.name, expense.amount, tx_type
        )?;
    }
    file.sync_all()?;
    Ok(())
}

fn create_service(tracker: &mut ExpenseTracker) {
    let expense_name = Text::new("Expense name:")
        .prompt()
        .expect("failed to read expense name");

    let amount: f64 = CustomType::new("Amount:")
        .with_help_message("Enter a number")
        .prompt()
        .expect("failed to read amount");

    let tx_type = Select::new(
        "Transaction type:",
        vec![TransactionType::Credit, TransactionType::Debit],
    )
    .prompt()
    .expect("failed to select transaction type");

    let expense = tracker.add(expense_name, amount, tx_type);
    println!("Added expense: {:?}", expense);
}

fn read_service(tracker: &ExpenseTracker) {
    let expenses = tracker.view_all_owned();
    if expenses.is_empty() {
        println!("No expenses saved.");
        return;
    }
    println!("Saved expenses:");
    for e in &expenses {
        println!("  {}", e);
    }
}

fn update_service(tracker: &mut ExpenseTracker) {
    let expenses = tracker.view_all_owned();
    if expenses.is_empty() {
        println!("No expenses to update.");
        return;
    }
    let selected = Select::new("Select expense to update:", expenses)
        .prompt()
        .expect("failed to select expense");

    let amount: f64 = CustomType::new("New amount:")
        .with_help_message("Enter a number")
        .prompt()
        .expect("failed to read amount");

    let tx_type = Select::new(
        "New transaction type:",
        vec![TransactionType::Credit, TransactionType::Debit],
    )
    .prompt()
    .expect("failed to select transaction type");

    if tracker.update(selected.id, amount, tx_type) {
        println!("Updated expense {}", selected.id);
    } else {
        println!("Update failed.");
    }
}

fn delete_service(tracker: &mut ExpenseTracker) {
    let expenses = tracker.view_all_owned();
    if expenses.is_empty() {
        println!("No expenses to delete.");
        return;
    }
    let selected = Select::new("Select expense to delete:", expenses)
        .prompt()
        .expect("failed to select expense");

    let confirmed = Confirm::new("Delete this expense?")
        .with_default(false)
        .prompt()
        .expect("failed to get confirmation");

    if confirmed && tracker.delete(selected.id) {
        println!("Deleted expense {}", selected.id);
    } else {
        println!("Not deleted.");
    }
}
