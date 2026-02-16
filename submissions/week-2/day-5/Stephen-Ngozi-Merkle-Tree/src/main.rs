mod expense;
mod tracker;

use std::io;
use expense::TransactionType;
use tracker::ExpenseTracker;




fn print_menu() {
    println!("\n===  Expense Tracker ===");
    println!("1 - Add Expense");
    println!("2 - View Expenses");
    println!("3 - Update Expense");
    println!("4 - Delete Expense");
    println!("q - Quit");
}
fn main() {
    // println!("Hello, world!");

    let mut tracker = ExpenseTracker::new();

        loop {
            print_menu();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed");
            let input = input.trim();
            

            if input == "q" {
                println!("Are you sure you want to quit? (y/n)");
                let mut confirm = String::new();
                io::stdin().read_line(&mut confirm).expect("Failed ");
                let confirm = confirm.trim();
                
                if confirm == "y" {
                    tracker.save_to_file("stephen_expences.txt");
                    println!("Data saved to stephen_expences.txt");
                    println!("Bye bye go home!");
                    break;
                }
            }else if input == "1" {
                println!("Enter expense name:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read input");
                let name = name.trim().to_string();
                
                println!("Enter amount:");
                let mut amount_input = String::new();
                io::stdin().read_line(&mut amount_input).expect("Failed to read input");
                let amount: f64 = amount_input.trim().parse().expect("Please enter a valid number");
                
                let tx_type = loop {
                    println!("Is this Credit or Debit? (c/d):");
                    let mut tx_input = String::new();
                    io::stdin().read_line(&mut tx_input).expect("Failed to read input");
                    let tx_input = tx_input.trim().to_lowercase();

                    match tx_input.as_str() {
                        "c" => break TransactionType::Credit,
                        "d" => break TransactionType::Debit,
                        _ => {
                            println!("Invalid input. Please enter 'c' for Credit or 'd' for Debit.");
                        }
                    }
                };
                
                let new_expense = tracker.add(name, amount, tx_type);
                println!(" Expense added successfully!");
                println!("ID: {}, Name: {}, Amount: Naria{:.2}, Type: {:?}", 
                        new_expense.id, new_expense.name, new_expense.amount, new_expense.tx_type);
            }else if input == "2" {
                let expenses = tracker.view_all();

                if expenses.is_empty() {
                    println!("No Expenses in the Tracker");
                } else {
                  println!("All Expenses");
                    for expense in expenses {
                        println!("ID: {} | Name: {} | Amount: Naria{:.2} | Type: {:?}", 
                                expense.id, expense.name, expense.amount, expense.tx_type);
                    }  
                }
            } else if input == "3" {
                println!("Enter the ID of the Expence");

                let mut new_id_input = String::new();

                io::stdin().read_line(&mut new_id_input).expect("Failed to read input");
                let id: u8 = new_id_input.trim().parse().expect("Please enter a valid ID");



                println!("Enter new amount:");
                let mut amount_input = String::new();
                io::stdin().read_line(&mut amount_input).expect("Failed to read input");
                let amount: f64 = amount_input.trim().parse().expect("Please enter a valid number");
                
                println!("Is this Credit or Debit? (c/d):");
                let mut tx_input = String::new();
                io::stdin().read_line(&mut tx_input).expect("Failed to read input");
                let tx_input = tx_input.trim();
                
                let tx_type = if tx_input == "c" {
                    TransactionType::Credit
                } else {
                    TransactionType::Debit
                };

                if tracker.update(id, amount, tx_type){
                    println!("Expense Updated Successfu");

                    if let Some(expense) = tracker.values.get(&id) {
                         println!("ID: {}, Name: {}, Amount: Naria{:.2}, Type: {:?}", 
                            expense.id, expense.name, expense.amount, expense.tx_type);
                    }else {
                        println!("Expense with ID {} not found!", id);

                    }
                }
            }else if input == "4" {
                println!("Enter the ID of the expense you to delete:");

                let  mut id_input = String::new();

                io::stdin().read_line(&mut id_input).expect("Failed to read input");
                let id: u8 = id_input.trim().parse().expect("Please enter a valid ID");

                if tracker.delete(id){
                    println!(" Your Expense is Deleted ");
                }else {
                    println!("Expense with ID {} not found!", id);
                }
            }else {
                println!("Invalid choice. Please try again.");
            }




        }

}
