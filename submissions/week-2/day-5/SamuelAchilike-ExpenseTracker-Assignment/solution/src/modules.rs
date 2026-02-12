#![allow(unused)]
use std::{collections::HashMap, fs::File, io::{self, prelude::*, stdin}};
use std::fs;

// Transact type contains variants of possible transactions
#[derive(Debug, Clone, Copy, PartialEq)]
enum TransactionType {
    Credit,
    Debit,
}


#[derive(PartialEq)]
pub enum UserOptions {
    AddExpense,
    RemoveExpense,
    UpdateExpense,
    ViewExpense,
    Exit,
    Invalid
}


#[derive(Debug, Clone)]
struct Expense {
    id: u8,
    name: String,
    amount: f64,
    tx_type: TransactionType,
}


#[derive(Debug, Clone)]
pub struct ExpenseTracker {

    // Each expense is passed as a hashmap, where the key is the id of the
    // transaction and the added expense is the value

    values: HashMap<u8, Expense>,
    next_id: u8,
}

impl ExpenseTracker {

    // Constructor to create new instance of ExpenseTracker struct
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            next_id: 1,
        }
    }
        // Method to add new Expenses to the ExpenseTracker
    fn add(&mut self) -> Expense {

        let mut name: String= String::new();
        let mut amount: String = String::new();
        let tx_type:TransactionType = TransactionType::Debit;

        println!("Enter the item name below: ");
        io::stdin().read_line(&mut name);

        let name = name.trim().to_string();
        // name = name.to_string().trim();

        println!("Enter the item amount below: ");
        io::stdin().read_line(&mut amount);
        let amount = amount.trim();

        let amount: f64  = match amount.parse() {
            Ok(value) => value,
            Err(e) => {
                println!("Error parsing float: {e:?}");
                0.0
            }
        };

        
        let current_id = self.next_id;
        let new_expense = Expense {
            id: current_id,
            name,
            amount,
            tx_type,
        };
        self.values.insert(current_id, new_expense.clone());
        self.next_id += 1;
        new_expense
    }

    pub fn view_all(&self){
        let view: &Vec<&Expense> = &self.values.values().collect();

        println!("\n\nHere's the summary of your expenses so far: \n");

        for item in view {
            let item_id = item.id;
            let item_name = &item.name;
            let item_amount = item.amount;
            let item_tx_type = item.tx_type;

            println!("Item: {}\nID: {}\nAmount: ${}\nTransaction Type: {:?}\n", item_name, item_id, item_amount, item_tx_type);

            // let mut file = File::create("content.txt");

            // let mut content = String::from(("Item: {}\nID: {}\nAmount: ${}\nTransaction Type: {:?}\n", item_name, item_id, item_amount, item_tx_type));
            // let content = content.as_bytes();

            // file.write_all(content)?;
        }

        //return view;

    }

    fn update(&mut self) {
        
        println!("Enter the id of the item you would like to update: ");

        let mut id = String::new();

        io::stdin()
        .read_line(&mut id)
        .expect("Failed to read input!");

        let id: String = id.trim().to_string();

        let id: u8  = match id.parse() {
            Ok(value) => value,
            Err(e) => {
                println!("Error parsing float: {e:?}");
                0
            }
        };        

        let mut name: String= String::new();
        let mut amount: String = String::new();
        let tx_type:TransactionType = TransactionType::Debit;

        println!("Enter the item name below: ");
        io::stdin().read_line(&mut name);

        let name = name.trim().to_string();
        // name = name.to_string().trim();

        println!("Enter the item amount below: ");
        io::stdin().read_line(&mut amount);
        let amount = amount.trim();

        let amount: f64  = match amount.parse() {
            Ok(value) => value,
            Err(e) => {
                println!("Error parsing float: {e:?}");
                0.0
            }
        };

        
        let new_expense = Expense {
            id: id,
            name,
            amount,
            tx_type,
        };
        self.values.insert(id, new_expense.clone());
        self.next_id += 1;
    }

    fn delete(&mut self) -> bool {

        println!("Supply the id of the item you want to remove");

        let mut id = String::new();
        io::stdin().read_line(&mut id).expect("Invalid input");

        let id = id.trim().to_string();

        let id: u8  = match id.parse() {
            Ok(value) => value,
            Err(e) => {
                println!("Error parsing float: {e:?}");
                0
            }
        };

        return self.values.remove(&id).is_some()
    }

    // pub fn write_to_file(self) -> Result<(), dyn Eq>{
        
    //     let mut file = std::fs::File::create("expenses.txt");

    //     for expense in self.view_all() {
    //         writeln!(
    //             file?,
    //             "ID: {} -- Name: {} -- Amount: {} -- Tx-Type: {:?}\n",
    //             expense.id, expense.name, expense.amount, expense.tx_type
    //         )?;
    //     }

    //     Ok(())
    // }

    pub fn get_user_choice() -> Option<UserOptions> {

        println!("\n\n************Welcome to Sam's Online Luxury Store************\n");

        println!("What would you like to do? \n");
        println!("Press '1' to 'add' an item to cart");
        println!("Press '2' to 'update' an item in the cart");
        println!("Press '3' to 'remove' an item from the cart");
        println!("Press '4' to 'view' all items in your cart");
        println!("Press 'q' to 'quit' and end session");

        let mut choice = String::new();

        io::stdin()
        .read_line(&mut choice)
        .expect("Could not read input");

        let choice = choice.trim();

        match choice {
            "1" => Some(UserOptions::AddExpense),
            "2" => Some(UserOptions::UpdateExpense),
            "3" => Some(UserOptions::RemoveExpense),
            "4" => Some(UserOptions::ViewExpense),
            "q" => Some(UserOptions::Exit),
            _ => None
        }
    }

    pub fn execute_user_choice(&mut self, action: &UserOptions) {

        match action {
            UserOptions::AddExpense => {
                self.add();
            }

            UserOptions::UpdateExpense => {
                self.update();
            }

            UserOptions::RemoveExpense => {
                self.delete();
            }   

            UserOptions::ViewExpense => {
                self.view_all();
            }

            UserOptions::Exit => {
                println!("Exiting...")
            }

            UserOptions::Invalid => {
                println!("Invalid option, please enter a valid input...")
            }
        }
    }


}