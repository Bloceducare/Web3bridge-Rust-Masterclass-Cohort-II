use crate::models::ExpenseTracker;
use crate::utils::{
    print_expense, print_expenses_table, read_f64, read_input, read_transaction_type, read_u32,
};

pub fn handle_add(tracker: &mut ExpenseTracker) {
    let name = read_input("Enter Name: ", false).unwrap();
    let amount = read_f64("Enter Amount: ", false).unwrap();
    let tx_type = read_transaction_type(false).unwrap();

    let expense = tracker.add(name, amount, tx_type);
    print_expense(&expense);
    println!("Transaction added successfully.");
}

pub fn handle_view(tracker: &ExpenseTracker) {
    let expenses = tracker.get_all_expenses();
    if expenses.is_empty() {
        println!("No expenses found.");
    } else {
        print_expenses_table(&expenses);
    }
}

pub fn handle_update(tracker: &mut ExpenseTracker) {
    let id = read_u32("Enter ID to update: ");

    if let Some(expense) = tracker.get_expense(id) {
        print_expense(expense);
        println!("Press Enter to skip a field.");

        let new_name = read_input("New Name: ", true);
        let new_amount = read_f64("New Amount: ", true);
        let new_tx_type = read_transaction_type(true);

        if new_name.is_none() && new_amount.is_none() && new_tx_type.is_none() {
            println!("No changes made.");
            return;
        }

        if let Some(updated) = tracker.update(id, new_name, new_amount, new_tx_type) {
            print_expense(&updated);
            println!("Transaction updated successfully.");
        } else {
            println!("Error updating transaction.");
        }
    } else {
        println!("Error: No transaction found with ID {}.", id);
    }
}

pub fn handle_delete(tracker: &mut ExpenseTracker) {
    let id = read_u32("Enter ID to delete: ");

    if let Some(deleted) = tracker.delete(id) {
        print_expense(&deleted);
        println!("Transaction deleted successfully.");
    } else {
        println!("Error: No transaction found with ID {}.", id);
    }
}

pub fn handle_quit() {
    let confirm = read_input("Are you sure you want to quit? (y/n): ", false).unwrap();
    if confirm.eq_ignore_ascii_case("y") {
        println!("Exiting application");
        std::process::exit(0);
    }
}
