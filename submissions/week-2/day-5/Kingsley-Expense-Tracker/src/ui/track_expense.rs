use crate::model::{Expense, ExpenseTracker, TrackerActions, TransactionType};
use crate::logger;
use std::io;

// This does not have a return value because its just reponsible for io operations
pub fn start(action: &TrackerActions, tracker: &mut ExpenseTracker) {
    match action {
        TrackerActions::ViewAll => {
            let len = tracker.get_all_expenses().len();

            if len <= 0 {
                println!("Expense list is empty!")
            } else {
                for expense in tracker.get_all_expenses() {
                    println!(
                        "Expense ID: {} -> Title: {} -> Amount: N{} -> Transaction Type: {:?}",
                        expense.id, expense.title, expense.amount, expense.tx_type
                    )
                }
            }

            let _ = logger::log("All expense was viewed");
        }
        TrackerActions::Add => {
            let mut amount = String::new();
            let mut title = String::new();
            let mut tx_type = String::new();

            println!("Enter Expense title: ");

            title.clear();
            io::stdin()
                .read_line(&mut title)
                .expect("Please enter a vlaid input");

            let parsed_amount = loop {
                println!("Enter Amount: ");
                amount.clear();
                io::stdin()
                    .read_line(&mut amount)
                    .expect("Please enter a vlaid amount field, it should be an integer");

                match amount.trim().parse::<u32>() {
                    Ok(amt) => break amt,
                    Err(_) => println!("❌ Invalid entry. Please enter a positive number ‼️ "),
                }
            };

            let parsed_type = loop {
                println!("Enter Transaction type (debit/credit): ");
                tx_type.clear();
                io::stdin()
                    .read_line(&mut tx_type)
                    .expect("⚠️ Please enter a valid transaction type. (debit/credit)");
                match tx_type.trim().to_lowercase().as_str() {
                    "credit" => break TransactionType::Credit,
                    "debit" => break TransactionType::Debit,
                    _ => println!("❌ Invalid transaction type!! Please try again"),
                };
            };

            let new_expense = Expense {
                id: tracker.next_key,
                title: title.trim().to_string(),
                amount: parsed_amount,
                tx_type: parsed_type,
            };

            if let Ok(added_expense) = tracker.add(&new_expense) {
                let added_expense = format!(
                    "Added {} to the list with amount N{}",
                    added_expense.title, added_expense.amount
                );
                println!("{added_expense}");
                let _ = logger::log(added_expense.as_str());
            } else {
                println!("❌ An error occured while adding expense to the list")
            }
        }
        TrackerActions::Edit => {
            let mut expense_id = String::new();
            let mut amount = String::new();
            let mut tx_type = String::new();

            println!("Here is a list of expenses to modify\n");

            for expense in tracker.get_all_expenses() {
                println!(
                    "Expense ID: {} -> Title: {} - Amount: N{}",
                    expense.id, expense.title, expense.amount
                )
            }

            let id = loop {
                println!("\nEnter the id of the item you want to modify: ");
                expense_id.clear();
                io::stdin()
                    .read_line(&mut expense_id)
                    .expect("Please enter a valid number");

                match expense_id.trim().parse::<u8>() {
                    Ok(amt) => break amt,
                    Err(_) => println!("Invalid entry. Please enter a number"),
                }
            };

            let parsed_amount = loop {
                println!("Enter new amount: ");
                amount.clear();
                io::stdin()
                    .read_line(&mut amount)
                    .expect("Please enter a vlaid amount field, it should be an integer");

                match amount.trim().parse::<u32>() {
                    Ok(amt) => break amt,
                    Err(_) => println!("❌ Invalid entry ‼️. Please enter a number"),
                }
            };

            let parsed_type = loop {
                println!("Enter the new transaction type (debit/credit): ");
                tx_type.clear();
                io::stdin()
                    .read_line(&mut tx_type)
                    .expect("Please enter a valid transaction type. (debit/credit)");
                match tx_type.trim().to_lowercase().as_str() {
                    "debit" => break TransactionType::Credit,
                    "credit" => break TransactionType::Debit,
                    _ => println!("Invalid transaction type!! Please try again"),
                };
            };

            if let Ok(item) = tracker.update_item(id, parsed_amount, parsed_type) {
                let msg = format!(
                    "You changed expense ID {}'s amount N{} and transaction type {:?}",
                    item.id, item.amount, item.tx_type
                );
                println!("{msg}");
                let _ = logger::log(&msg);
            } else {
                println!("❌ An error occured editing item")
            }
        }
        TrackerActions::ViewSingle => {
            println!(
                "Here is a list of expenses, let me know the on you want to view more info about\n"
            );

            for expense in tracker.get_all_expenses() {
                println!("Expense ID: {} -> Title: {}", expense.id, expense.title)
            }
            let mut expense_id = String::new();

            let id = loop {
                println!("⚠️ Enter the id of the item you want to view: ");
                io::stdin()
                    .read_line(&mut expense_id)
                    .expect("Please enter a valid number");

                match expense_id.trim().parse::<u8>() {
                    Ok(amt) => break amt,
                    Err(_) => println!("❌ Invalid entry. Please enter a number"),
                }
            };

            if let Some(expense) = tracker.get_single_item(id) {
                println!("Here you go!!\n");
                println!(
                    "Expense ID: {} -> Title: {} -> Amount: N{} -> Transaction Type: {:?}",
                    expense.id, expense.title, expense.amount, expense.tx_type
                );

                let msg = format!(
                    "Expense no {} with title: {} and amount: {} and type {:?} was viewed",
                    expense.id, expense.title, expense.amount, expense.tx_type
                );

                let _ = logger::log(&msg);
            } else {
                println!("❌ An Error Occured while viewing item")
            }
        }
        TrackerActions::Delete => {
            let mut expense_id = String::new();
            println!("Here is a list of expenses, let me know the on you want to delete\n");

            for expense in tracker.get_all_expenses() {
                println!("Expense ID: {} -> Title: {}", expense.id, expense.title)
            }

            let id = loop {
                println!("Enter the id of the item you want to delete: ");

                expense_id.clear();
                io::stdin()
                    .read_line(&mut expense_id)
                    .expect("Please enter a valid number");

                match expense_id.trim().parse::<u8>() {
                    Ok(amt) => break amt,
                    Err(_) => println!("❌ Invalid entry. Please enter a number"),
                }
            };
            if let Ok(expense) = tracker.delete_expense(id) {
                let msg = format!("Expense with ID: {} was deleted", expense.title);
                println!("{msg}");
                let _ = logger::log(msg.as_str());
            }
        }

        TrackerActions::Quit => loop {
            println!("Are you sure you want to quit? (y/n)");
            let mut quit_command = String::new();
            quit_command.clear();
            io::stdin()
                .read_line(&mut quit_command)
                .expect("Please enter a valid string");

            match quit_command.trim().to_lowercase().as_str() {
                "y" => std::process::exit(1),
                "n" => return,
                _ => {
                    println!("⚠️ Invalid input please type a 'y' for yes or 'n' for no");
                }
            }
        },
    }
}
