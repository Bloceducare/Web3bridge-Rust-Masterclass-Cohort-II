use emeka_expense_tracker::actions::{
    ActionTypes, add_expense, get_all_expenses, print_report, remove_expense, show_total_expenses,
    update_expense, view_expense,
};
use emeka_expense_tracker::expenses::ExpenseTracker;
use std::collections::HashMap;

fn main() {
    let command_map: HashMap<&str, ActionTypes> = HashMap::from([
        ("add", ActionTypes::AddExpense),
        ("remove", ActionTypes::RemoveExpense),
        ("update", ActionTypes::UpdateExpense),
        ("view", ActionTypes::ViewExpense),
        ("all", ActionTypes::GetAllExpenses),
        ("total", ActionTypes::GetTotalExpenses),
        ("report", ActionTypes::PrintReport),
        ("q", ActionTypes::Quit),
    ]);
    let mut tracker = ExpenseTracker::start();
    println!("Welcome to your expense tracker!");
    println!("You can add expenses, edit them, and remove items from them.");

    loop {
        let action = ActionTypes::get_action(&command_map);
        println!("\nYou chose {action:?}");
        println!("-----------------------------------\n");

        match action {
            ActionTypes::AddExpense => add_expense(&mut tracker),
            ActionTypes::RemoveExpense => remove_expense(&mut tracker),
            ActionTypes::UpdateExpense => update_expense(&mut tracker),
            ActionTypes::ViewExpense => view_expense(&tracker),
            ActionTypes::GetAllExpenses => get_all_expenses(&tracker),
            ActionTypes::GetTotalExpenses => show_total_expenses(&tracker),
            ActionTypes::PrintReport => print_report(&tracker),
            ActionTypes::Quit => {
                println!("Goodbye!");
                break;
            }
        }
        println!("-----------------------------------");
    }
}
